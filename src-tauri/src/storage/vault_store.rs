use std::fs;
use std::path::PathBuf;
use crate::common::{PasswordEntry, Settings, VaultStore};

/// 密码库存储
pub struct VaultStoreIO {
    path: PathBuf,
    settings_path: PathBuf,
}

impl VaultStoreIO {
    pub fn new() -> Self {
        let base = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("Secrypto");
        fs::create_dir_all(&base).ok();
        Self {
            path: base.join("vault.json"),
            settings_path: base.join("settings.json"),
        }
    }

    /// 加载密码库
    pub fn load_vault(&self) -> VaultStore {
        if !self.path.exists() {
            return VaultStore::new();
        }
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| VaultStore::new())
    }

    /// 保存密码库
    pub fn save_vault(&self, vault: &VaultStore) -> Result<(), String> {
        let json = serde_json::to_string_pretty(vault)
            .map_err(|e| format!("Serialize error: {}", e))?;
        fs::write(&self.path, json)
            .map_err(|e| format!("Write error: {}", e))
    }

    /// 加载设置
    pub fn load_settings(&self) -> Settings {
        if !self.settings_path.exists() {
            return Settings::default();
        }
        fs::read_to_string(&self.settings_path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// 保存设置
    pub fn save_settings(&self, settings: &Settings) -> Result<(), String> {
        let json = serde_json::to_string_pretty(settings)
            .map_err(|e| format!("Serialize error: {}", e))?;
        fs::write(&self.settings_path, json)
            .map_err(|e| format!("Write error: {}", e))
    }
}
