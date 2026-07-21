use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use russh::client::{self, Handle};
use russh::keys::key::PrivateKeyWithHashAlg;
use russh::keys::load_secret_key;
use russh::{ChannelMsg, Disconnect};
use russh_sftp::client::SftpSession;
use tauri::ipc::Channel;
use tokio::sync::mpsc;

use crate::fs::FileEntry;
use crate::session::{OutputEvent, Session};
use crate::sysmon::ResourceStats;

#[allow(dead_code)]
pub struct SshParams {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub auth_method: Option<String>,
    pub key_path: Option<String>,
    pub credential_id: Option<String>,
    pub password: Option<String>,
}

/// Commands sent from the Tauri command thread to the per-session shell task.
enum Cmd {
    Data(Vec<u8>),
    Resize(u16, u16),
    Close,
}

pub struct SshSession {
    tx: mpsc::UnboundedSender<Cmd>,
    runtime: Arc<tokio::runtime::Runtime>,
    handle: Arc<Handle<ClientHandler>>,
    /// Last (idle, total) CPU jiffies, for computing remote CPU% deltas.
    last_cpu: Mutex<Option<(u64, u64)>>,
}

struct ClientHandler {
    host: String,
    port: u16,
}

#[async_trait]
impl client::Handler for ClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        // TOFU known_hosts: accept & record unknown hosts, reject on key change.
        match russh::keys::check_known_hosts(&self.host, self.port, key) {
            Ok(true) => Ok(true),
            Ok(false) => {
                let _ = russh::keys::known_hosts::learn_known_hosts(&self.host, self.port, key);
                Ok(true)
            }
            Err(russh::keys::Error::KeyChanged { .. }) => Ok(false),
            Err(_) => {
                let _ = russh::keys::known_hosts::learn_known_hosts(&self.host, self.port, key);
                Ok(true)
            }
        }
    }
}

async fn connect_and_auth(params: &SshParams) -> anyhow::Result<Handle<ClientHandler>> {
    let config = Arc::new(client::Config {
        inactivity_timeout: Some(Duration::from_secs(3600)),
        ..Default::default()
    });
    let handler = ClientHandler {
        host: params.host.clone(),
        port: params.port,
    };
    let mut handle =
        client::connect(config, (params.host.as_str(), params.port), handler).await?;

    let use_key = params.auth_method.as_deref() == Some("key")
        || (params.auth_method.is_none() && params.key_path.is_some());
    let authed = if use_key {
        let key_path = params
            .key_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("private key path is required"))?;
        let key = load_secret_key(PathBuf::from(key_path), None)?;
        handle
            .authenticate_publickey(&params.user, PrivateKeyWithHashAlg::new(Arc::new(key), None)?)
            .await?
    } else if let Some(pw) = &params.password {
        handle.authenticate_password(&params.user, pw).await?
    } else {
        anyhow::bail!("no authentication method provided");
    };

    if !authed {
        anyhow::bail!("authentication failed");
    }
    Ok(handle)
}

impl SshSession {
    pub fn spawn(params: SshParams, on_output: Channel<OutputEvent>) -> anyhow::Result<Self> {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()?,
        );

        // Connect + auth synchronously so errors surface to the caller/UI.
        let handle = match runtime.block_on(connect_and_auth(&params)) {
            Ok(h) => Arc::new(h),
            Err(e) => {
                let msg = format!("\r\n\x1b[31m[ SSH error: {e} ]\x1b[0m\r\n");
                let _ = on_output.send(OutputEvent {
                    bytes: msg.into_bytes(),
                });
                return Err(e);
            }
        };

        // Open the interactive shell channel.
        let mut channel = runtime.block_on(handle.channel_open_session())?;
        runtime.block_on(async {
            channel
                .request_pty(false, "xterm-256color", 80, 24, 0, 0, &[])
                .await?;
            channel.request_shell(true).await?;
            Ok::<(), anyhow::Error>(())
        })?;

        let (tx, mut rx) = mpsc::unbounded_channel::<Cmd>();
        let out = on_output.clone();
        let keepalive = handle.clone();
        runtime.spawn(async move {
            // hold a reference so the connection outlives the shell loop
            let _keep = keepalive;
            loop {
                tokio::select! {
                    cmd = rx.recv() => {
                        match cmd {
                            Some(Cmd::Data(d)) => { let _ = channel.data(&d[..]).await; }
                            Some(Cmd::Resize(c, r)) => {
                                let _ = channel.window_change(c as u32, r as u32, 0, 0).await;
                            }
                            Some(Cmd::Close) | None => { let _ = channel.eof().await; break; }
                        }
                    }
                    msg = channel.wait() => {
                        match msg {
                            Some(ChannelMsg::Data { ref data }) => {
                                let _ = out.send(OutputEvent { bytes: data.to_vec() });
                            }
                            Some(ChannelMsg::ExtendedData { ref data, .. }) => {
                                let _ = out.send(OutputEvent { bytes: data.to_vec() });
                            }
                            Some(ChannelMsg::Eof) | Some(ChannelMsg::Close) | None => break,
                            _ => {}
                        }
                    }
                }
            }
        });

        Ok(Self {
            tx,
            runtime,
            handle,
            last_cpu: Mutex::new(None),
        })
    }

    /// Run a command over a fresh exec channel and return its stdout.
    pub fn exec_capture(&self, command: &str) -> anyhow::Result<String> {
        let handle = self.handle.clone();
        let command = command.to_string();
        self.runtime.block_on(async move {
            let mut ch = handle.channel_open_session().await?;
            ch.exec(true, command.as_bytes()).await?;
            let mut out = Vec::new();
            while let Some(msg) = ch.wait().await {
                match msg {
                    ChannelMsg::Data { ref data } => out.extend_from_slice(data),
                    ChannelMsg::Eof | ChannelMsg::Close => break,
                    ChannelMsg::ExitStatus { .. } => break,
                    _ => {}
                }
            }
            Ok::<String, anyhow::Error>(String::from_utf8_lossy(&out).to_string())
        })
    }

    /// Sample remote resource usage via /proc + coreutils (Linux hosts).
    pub fn remote_stats(&self) -> anyhow::Result<ResourceStats> {
        // One round-trip: memory, cpu jiffies, and root-fs usage.
        let cmd = "grep '^cpu ' /proc/stat; free -b | awk '/^Mem:/{print \"MEM\", $2, $3}'; df -B1 / | awk 'NR==2{print \"DISK\", $2, $3}'";
        let out = self.exec_capture(cmd)?;
        parse_remote_stats(&out, &self.last_cpu)
    }

    /// List a remote directory over SFTP.
    pub fn read_dir(&self, path: &str) -> anyhow::Result<Vec<FileEntry>> {
        let handle = self.handle.clone();
        let path = path.to_string();
        self.runtime.block_on(async move {
            let ch = handle.channel_open_session().await?;
            ch.request_subsystem(true, "sftp").await?;
            let sftp = SftpSession::new(ch.into_stream()).await?;
            let mut entries = Vec::new();
            for entry in sftp.read_dir(&path).await? {
                let meta = entry.metadata();
                entries.push(FileEntry {
                    name: entry.file_name(),
                    path: entry.path(),
                    is_dir: meta.is_dir(),
                    size: meta.size.unwrap_or(0),
                });
            }
            entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });
            Ok::<Vec<FileEntry>, anyhow::Error>(entries)
        })
    }

    pub fn home_dir(&self) -> anyhow::Result<String> {
        let out = self.exec_capture("echo $HOME")?;
        let home = out.trim();
        Ok(if home.is_empty() { "/".to_string() } else { home.to_string() })
    }
}

fn parse_remote_stats(
    out: &str,
    last_cpu: &Mutex<Option<(u64, u64)>>,
) -> anyhow::Result<ResourceStats> {
    let mut mem_total = 0u64;
    let mut mem_used = 0u64;
    let mut disk_total = 0u64;
    let mut disk_used = 0u64;
    let mut cpu = 0.0f32;

    for line in out.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.first() == Some(&"cpu") && f.len() >= 8 {
            // user nice system idle iowait irq softirq steal ...
            let nums: Vec<u64> = f[1..].iter().filter_map(|x| x.parse().ok()).collect();
            let idle = nums.get(3).copied().unwrap_or(0) + nums.get(4).copied().unwrap_or(0);
            let total: u64 = nums.iter().sum();
            let mut guard = last_cpu.lock().unwrap();
            if let Some((pidle, ptotal)) = *guard {
                let dt = total.saturating_sub(ptotal);
                let di = idle.saturating_sub(pidle);
                if dt > 0 {
                    cpu = ((dt - di) as f32 / dt as f32) * 100.0;
                }
            }
            *guard = Some((idle, total));
        } else if f.first() == Some(&"MEM") && f.len() >= 3 {
            mem_total = f[1].parse().unwrap_or(0);
            mem_used = f[2].parse().unwrap_or(0);
        } else if f.first() == Some(&"DISK") && f.len() >= 3 {
            disk_total = f[1].parse().unwrap_or(0);
            disk_used = f[2].parse().unwrap_or(0);
        }
    }

    Ok(ResourceStats {
        cpu,
        mem_used,
        mem_total,
        disk_used,
        disk_total,
        net_rx: 0,
        net_tx: 0,
    })
}

impl Session for SshSession {
    fn write(&self, data: &[u8]) -> anyhow::Result<()> {
        self.tx
            .send(Cmd::Data(data.to_vec()))
            .map_err(|_| anyhow::anyhow!("ssh session closed"))
    }

    fn resize(&self, cols: u16, rows: u16) -> anyhow::Result<()> {
        let _ = self.tx.send(Cmd::Resize(cols, rows));
        Ok(())
    }

    fn close(&self) {
        let _ = self.tx.send(Cmd::Close);
        let handle = self.handle.clone();
        let rt = self.runtime.clone();
        std::thread::spawn(move || {
            let _ = rt.block_on(handle.disconnect(Disconnect::ByApplication, "", ""));
            std::thread::sleep(Duration::from_millis(100));
            drop(rt);
        });
    }
}
