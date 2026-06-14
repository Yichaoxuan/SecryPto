use serde::{Deserialize, Serialize};

/// 剪贴板内容类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Image,
}

/// 剪贴板条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub id: String,
    pub content_type: ContentType,
    pub text_content: Option<String>,
    pub image_base64: Option<String>,
    pub image_thumb: Option<String>,
    pub created_at: String,
    pub is_pinned: bool,
    pub expiry_days: u32,      // 0 = 永不过期
    pub content_hash: String,  // SHA256 用于去重
}

/// 剪贴板历史存储
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardStore {
    pub entries: Vec<ClipboardEntry>,
}

impl ClipboardStore {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }
}
