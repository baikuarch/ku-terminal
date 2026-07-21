use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const CURRENT_CONFIG_VERSION: u32 = 3;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshDef {
    pub host: String,
    pub port: u16,
    pub user: String,
    #[serde(default)]
    pub auth_method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDef {
    pub id: String,
    pub name: String,
    /// "local" | "ssh"
    pub kind: String,
    /// Production / Staging / Development
    pub group: String,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SshDef>,
}

fn default_status() -> String {
    "idle".to_string()
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Versioned envelope for future imports and migrations.
    #[serde(default = "legacy_config_version")]
    pub version: u32,
    pub sessions: Vec<SessionDef>,
    #[serde(default)]
    pub custom_groups: Vec<String>,
    #[serde(default)]
    pub collapsed_groups: Vec<String>,
    #[serde(default)]
    pub hidden_groups: Vec<String>,
    #[serde(default)]
    pub sidebar_collapsed: bool,
}

fn legacy_config_version() -> u32 {
    1
}

impl AppConfig {
    fn seeded() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION,
            sessions: default_sessions(),
            custom_groups: Vec::new(),
            collapsed_groups: Vec::new(),
            hidden_groups: Vec::new(),
            sidebar_collapsed: false,
        }
    }
}

fn config_dir() -> PathBuf {
    let base = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join(".ku-terminal")
}

fn config_path() -> PathBuf {
    config_dir().join("sessions.json")
}

/// Returns the full persisted application configuration, seeding defaults on first run.
/// Files created by versions before v2 only contain `sessions` and remain readable.
pub fn load_config() -> anyhow::Result<AppConfig> {
    let path = config_path();
    if !path.exists() {
        let config = AppConfig::seeded();
        save_config(config.clone())?;
        return Ok(config);
    }
    let data = std::fs::read_to_string(&path)?;
    let mut config: AppConfig = serde_json::from_str(&data)?;
    normalize_auth_methods(&mut config);
    Ok(config)
}

fn normalize_auth_methods(config: &mut AppConfig) {
    for session in &mut config.sessions {
        if let Some(ssh) = &mut session.ssh {
            if ssh.auth_method.is_empty() {
                ssh.auth_method = if ssh.key_path.is_some() {
                    "key".to_string()
                } else {
                    "password".to_string()
                };
            }
            if ssh.credential_id.is_none() {
                ssh.credential_id = Some(session.id.clone());
            }
        }
    }
}

pub fn save_config(mut config: AppConfig) -> anyhow::Result<()> {
    config.version = CURRENT_CONFIG_VERSION;
    let dir = config_dir();
    std::fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(&config)?;
    std::fs::write(config_path(), json)?;
    Ok(())
}

fn default_sessions() -> Vec<SessionDef> {
    let ssh = |host: &str, user: &str| {
        Some(SshDef {
            host: host.to_string(),
            port: 22,
            user: user.to_string(),
            auth_method: "password".to_string(),
            key_path: None,
            credential_id: None,
        })
    };
    let mk = |id: &str, group: &str, status: &str, host: &str, user: &str| SessionDef {
        id: id.to_string(),
        name: id.to_string(),
        kind: "ssh".to_string(),
        group: group.to_string(),
        status: status.to_string(),
        ssh: ssh(host, user),
    };
    vec![
        mk("prod-web-01", "Production", "online", "10.0.0.11", "admin"),
        mk("prod-db-master", "Production", "idle", "10.0.0.12", "admin"),
        mk(
            "prod-worker-03",
            "Production",
            "error",
            "10.0.0.13",
            "admin",
        ),
        mk("stage-api-01", "Staging", "online", "10.1.0.11", "deploy"),
        mk("stage-redis-a", "Staging", "idle", "10.1.0.12", "deploy"),
        mk("dev-sandbox", "Development", "online", "127.0.0.1", "dev"),
    ]
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    #[test]
    fn reads_the_pre_v2_session_only_format() {
        let config: AppConfig = serde_json::from_str(r#"{"sessions":[]}"#).unwrap();

        assert_eq!(config.version, 1);
        assert!(config.custom_groups.is_empty());
        assert!(config.collapsed_groups.is_empty());
        assert!(config.hidden_groups.is_empty());
        assert!(!config.sidebar_collapsed);
    }
}
