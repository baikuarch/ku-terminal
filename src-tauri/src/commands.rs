use std::sync::Arc;

use tauri::ipc::Channel;
use tauri::State;

use crate::config::{self, AppConfig};
use crate::credentials;
use crate::fs::{self, FileEntry};
use crate::pty::local::LocalSession;
use crate::session::{OutputEvent, SessionManager};
use crate::ssh::remote::{SshParams, SshSession};
use crate::sysmon::{ResourceStats, SysMonitor};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub auth_method: Option<String>,
    pub key_path: Option<String>,
    pub credential_id: Option<String>,
}

#[tauri::command]
pub fn create_local_session(
    cwd: Option<String>,
    shell: Option<String>,
    on_output: Channel<OutputEvent>,
    manager: State<'_, SessionManager>,
) -> Result<String, String> {
    let session = LocalSession::spawn(cwd, shell, on_output).map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    manager.insert(id.clone(), Arc::new(session));
    Ok(id)
}

#[tauri::command]
pub fn create_ssh_session(
    cfg: SshConfig,
    password: Option<String>,
    on_output: Channel<OutputEvent>,
    manager: State<'_, SessionManager>,
) -> Result<String, String> {
    let params = SshParams {
        host: cfg.host,
        port: cfg.port,
        user: cfg.user,
        auth_method: cfg.auth_method,
        key_path: cfg.key_path,
        credential_id: cfg.credential_id,
        password,
    };
    let session = SshSession::spawn(params, on_output).map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    manager.insert_ssh(id.clone(), Arc::new(session));
    Ok(id)
}

#[tauri::command]
pub fn save_ssh_password(credential_id: String, password: String) -> Result<(), String> {
    credentials::save_ssh_password(&credential_id, &password).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_ssh_password(credential_id: String) -> Option<String> {
    credentials::load_ssh_password(&credential_id)
}

#[tauri::command]
pub fn delete_ssh_password(credential_id: String) -> Result<(), String> {
    credentials::delete_ssh_password(&credential_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_session(
    id: String,
    data: String,
    manager: State<'_, SessionManager>,
) -> Result<(), String> {
    manager.write(&id, data.as_bytes()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn resize_session(
    id: String,
    cols: u16,
    rows: u16,
    manager: State<'_, SessionManager>,
) -> Result<(), String> {
    manager.resize(&id, cols, rows).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_session(id: String, manager: State<'_, SessionManager>) {
    manager.close(&id);
}

#[tauri::command]
pub fn get_resource_stats(
    session_id: Option<String>,
    monitor: State<'_, SysMonitor>,
    manager: State<'_, SessionManager>,
) -> Result<ResourceStats, String> {
    if let Some(id) = session_id {
        if let Some(ssh) = manager.get_ssh(&id) {
            return ssh.remote_stats().map_err(|e| e.to_string());
        }
    }
    Ok(monitor.sample())
}

#[tauri::command]
pub fn read_dir(
    path: String,
    session_id: Option<String>,
    manager: State<'_, SessionManager>,
) -> Result<Vec<FileEntry>, String> {
    if let Some(id) = session_id {
        if let Some(ssh) = manager.get_ssh(&id) {
            return ssh.read_dir(&path).map_err(|e| e.to_string());
        }
    }
    fs::read_dir(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn home_dir(
    session_id: Option<String>,
    manager: State<'_, SessionManager>,
) -> String {
    if let Some(id) = session_id {
        if let Some(ssh) = manager.get_ssh(&id) {
            if let Ok(h) = ssh.home_dir() {
                return h;
            }
        }
    }
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string())
}

#[tauri::command]
pub fn load_config() -> Result<AppConfig, String> {
    config::load_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_config(config: AppConfig) -> Result<(), String> {
    config::save_config(config).map_err(|e| e.to_string())
}
