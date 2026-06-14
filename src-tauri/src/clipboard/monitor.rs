use chrono::Utc;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use crate::clipboard::types::{ClipboardEntry, ClipboardStore, ContentType};

/// 剪贴板管理器
pub struct ClipboardManager {
    store: ClipboardStore,
    last_content_hash: Option<String>,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            store: ClipboardStore::new(),
            last_content_hash: None,
        }
    }

    pub fn load(&mut self, entries: Vec<ClipboardEntry>) {
        self.store = ClipboardStore { entries };
    }

    pub fn get_all(&self) -> &Vec<ClipboardEntry> {
        &self.store.entries
    }

    /// 添加新条目（自动去重）
    pub fn add_entry(&mut self, content_type: ContentType, text: Option<String>, image_base64: Option<String>, image_thumb: Option<String>, expiry_days: u32) -> Option<String> {
        // 计算内容哈希用于去重
        let hash_input = match &text {
            Some(t) => t.clone(),
            None => image_base64.clone().unwrap_or_default(),
        };
        let content_hash = hex::encode(Sha256::digest(hash_input.as_bytes()));

        // 去重检查
        if let Some(last) = &self.last_content_hash {
            if last == &content_hash {
                return None; // 重复内容，不记录
            }
        }

        let entry = ClipboardEntry {
            id: Uuid::new_v4().to_string(),
            content_type,
            text_content: text,
            image_base64,
            image_thumb,
            created_at: Utc::now().to_rfc3339(),
            is_pinned: false,
            expiry_days,
            content_hash: content_hash.clone(),
        };

        self.last_content_hash = Some(content_hash);
        self.store.entries.insert(0, entry); // 插入到最前面

        Some(self.store.entries[0].id.clone())
    }

    /// 删除条目
    pub fn delete_entry(&mut self, id: &str) -> bool {
        let len_before = self.store.entries.len();
        self.store.entries.retain(|e| e.id != id);
        self.store.entries.len() < len_before
    }

    /// 切换置顶状态
    pub fn toggle_pin(&mut self, id: &str) -> bool {
        if let Some(entry) = self.store.entries.iter_mut().find(|e| e.id == id) {
            entry.is_pinned = !entry.is_pinned;
            true
        } else {
            false
        }
    }

    /// 清理过期条目
    pub fn clean_expired(&mut self) -> usize {
        let now = Utc::now();
        let before = self.store.entries.len();
        self.store.entries.retain(|e| {
            if e.expiry_days == 0 {
                return true; // 永不过期
            }
            if let Ok(created) = chrono::DateTime::parse_from_rfc3339(&e.created_at) {
                let created_utc = created.with_timezone(&Utc);
                let days = (now - created_utc).num_days();
                days < e.expiry_days as i64
            } else {
                true
            }
        });
        before - self.store.entries.len()
    }

    /// 搜索剪贴板历史
    pub fn search(&self, query: &str) -> Vec<&ClipboardEntry> {
        let q = query.to_lowercase();
        self.store.entries.iter()
            .filter(|e| {
                if let Some(ref text) = e.text_content {
                    text.to_lowercase().contains(&q)
                } else {
                    false
                }
            })
            .collect()
    }

    /// 获取排序后的条目（置顶优先，再按时间降序）
    pub fn get_sorted(&self) -> Vec<&ClipboardEntry> {
        let mut sorted: Vec<&ClipboardEntry> = self.store.entries.iter().collect();
        sorted.sort_by(|a, b| {
            b.is_pinned.cmp(&a.is_pinned)
                .then_with(|| b.created_at.cmp(&a.created_at))
        });
        sorted
    }

    pub fn to_store(&self) -> &ClipboardStore {
        &self.store
    }
}
