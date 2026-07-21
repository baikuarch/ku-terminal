use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::Serialize;

use crate::ssh::remote::SshSession;

/// A raw output chunk streamed to the frontend Channel.
#[derive(Clone, Serialize)]
pub struct OutputEvent {
    pub bytes: Vec<u8>,
}

/// Unified handle over a live session (local PTY or SSH channel).
/// Implementors own the write/resize/close side; output is pushed to the
/// frontend Channel by a reader task set up at creation time.
pub trait Session: Send + Sync {
    fn write(&self, data: &[u8]) -> anyhow::Result<()>;
    fn resize(&self, cols: u16, rows: u16) -> anyhow::Result<()>;
    fn close(&self);
}

#[derive(Default)]
pub struct SessionManager {
    inner: Mutex<HashMap<String, Arc<dyn Session>>>,
    /// SSH sessions kept concretely so exec / sftp / stats are reachable.
    ssh: Mutex<HashMap<String, Arc<SshSession>>>,
}

impl SessionManager {
    pub fn insert(&self, id: String, session: Arc<dyn Session>) {
        self.inner.lock().unwrap().insert(id, session);
    }

    pub fn insert_ssh(&self, id: String, session: Arc<SshSession>) {
        self.ssh.lock().unwrap().insert(id.clone(), session.clone());
        self.inner.lock().unwrap().insert(id, session);
    }

    pub fn get_ssh(&self, id: &str) -> Option<Arc<SshSession>> {
        self.ssh.lock().unwrap().get(id).cloned()
    }

    pub fn write(&self, id: &str, data: &[u8]) -> anyhow::Result<()> {
        let map = self.inner.lock().unwrap();
        match map.get(id) {
            Some(s) => s.write(data),
            None => Err(anyhow::anyhow!("session not found: {id}")),
        }
    }

    pub fn resize(&self, id: &str, cols: u16, rows: u16) -> anyhow::Result<()> {
        let map = self.inner.lock().unwrap();
        match map.get(id) {
            Some(s) => s.resize(cols, rows),
            None => Err(anyhow::anyhow!("session not found: {id}")),
        }
    }

    pub fn close(&self, id: &str) {
        self.ssh.lock().unwrap().remove(id);
        if let Some(s) = self.inner.lock().unwrap().remove(id) {
            s.close();
        }
    }
}
