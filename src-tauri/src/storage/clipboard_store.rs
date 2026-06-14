use std::fs;
use std::path::PathBuf;
use crate::clipboard::types::ClipboardEntry;

/// 剪贴板历史 JSON 存储
pub struct ClipboardStoreIO {
    path: PathBuf,
}

impl ClipboardStoreIO {
    /// 创建存储，路径为 %APPDATA%/Secrypto/clipboard_history.json
    pub fn new() -> Self {
        let app_dir = Self::app_data_dir();
        fs::create_dir_all(&app_dir).ok();
        Self {
            path: app_dir.join("clipboard_history.json"),
        }
    }

    fn app_data_dir() -> PathBuf {
        let base = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        base.join("Secrypto")
    }

    /// 读取所有条目
    pub fn load(&self) -> Vec<ClipboardEntry> {
        if !self.path.exists() {
            return Vec::new();
        }
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_default()
            }
            Err(_) => Vec::new(),
        }
    }

    /// 保存所有条目
    pub fn save(&self, entries: &[ClipboardEntry]) -> Result<(), String> {
        let json = serde_json::to_string_pretty(entries)
            .map_err(|e| format!("Serialize error: {}", e))?;
        fs::write(&self.path, json)
            .map_err(|e| format!("Write error: {}", e))
    }

    /// 获取存储路径（用于调试）
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
