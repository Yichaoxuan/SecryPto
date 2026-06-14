use serde::{Deserialize, Serialize};

/// 密码条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: String,
    pub name: String,
    pub url: String,
    pub username: String,
    pub encrypted_password: String, // Base64(nonce + ciphertext + tag)
    pub notes: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 密码库存储（密文 + 元数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultStore {
    pub salt: String,                   // Argon2 盐
    pub nonce_prefix: String,           // 随机前缀
    pub encrypted_entries: Vec<PasswordEntry>,
    pub master_password_hash: String,   // 用于验证主密码
}

impl VaultStore {
    pub fn new() -> Self {
        Self {
            salt: String::new(),
            nonce_prefix: String::new(),
            encrypted_entries: Vec::new(),
            master_password_hash: String::new(),
        }
    }
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub hotkey: String,                // "Win+V"
    pub default_expiry_days: u32,      // 默认 3
    pub auto_start: bool,
    pub theme: Theme,
    pub language: String,              // "zh" | "en"
    pub vault_locked: bool,            // 密码本是否锁定
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkey: "Win+V".to_string(),
            default_expiry_days: 3,
            auto_start: true,
            theme: Theme::System,
            language: "zh".to_string(),
            vault_locked: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

/// Tauri 命令统一错误类型
#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self { message: e.to_string() }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self { message: e.to_string() }
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        Self { message: msg }
    }
}
