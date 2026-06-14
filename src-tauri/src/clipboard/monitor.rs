use chrono::Utc;
use sha2::{Digest, Sha256};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use crate::clipboard::types::{ClipboardEntry, ClipboardStore, ContentType};

/// 剪贴板变化通知
#[derive(Debug, Clone)]
pub enum ClipboardEvent {
    NewEntry(ClipboardEntry),
}

/// 剪贴板管理器
pub struct ClipboardManager {
    store: ClipboardStore,
    last_content_hash: Option<String>,
    expiry_days: u32,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            store: ClipboardStore::new(),
            last_content_hash: None,
            expiry_days: 3,
        }
    }

    pub fn load(&mut self, entries: Vec<ClipboardEntry>) {
        self.store = ClipboardStore { entries };
        if let Some(last) = self.store.entries.first() {
            self.last_content_hash = Some(last.content_hash.clone());
        }
    }

    pub fn set_expiry_days(&mut self, days: u32) {
        self.expiry_days = days;
    }

    pub fn get_all(&self) -> &Vec<ClipboardEntry> {
        &self.store.entries
    }

    /// 启动后台剪贴板监听（返回事件接收器）
    pub fn start_monitoring(&mut self) -> mpsc::Receiver<ClipboardEvent> {
        let (tx, rx) = mpsc::channel();
        let monitor_tx = tx;
        let expiry = self.expiry_days;

        thread::spawn(move || {
            log::info!("🔍 Clipboard monitor started");
            let mut last_content: Option<String> = None;

            loop {
                thread::sleep(Duration::from_millis(500));

                if let Ok(text) = clipboard_win::get_clipboard_string() {
                    if !text.is_empty() {
                        let current_hash = hex::encode(Sha256::digest(text.as_bytes()));

                        if last_content.as_deref() != Some(&current_hash) {
                            last_content = Some(current_hash.clone());
                            log::info!("📋 New clipboard content captured");

                            let entry = ClipboardEntry {
                                id: Uuid::new_v4().to_string(),
                                content_type: ContentType::Text,
                                text_content: Some(text),
                                image_base64: None,
                                image_thumb: None,
                                created_at: Utc::now().to_rfc3339(),
                                is_pinned: false,
                                expiry_days: expiry,
                                content_hash: current_hash,
                            };

                            if monitor_tx.send(ClipboardEvent::NewEntry(entry)).is_err() {
                                log::error!("Clipboard monitor channel closed");
                                break;
                            }
                        }
                    }
                }
            }
        });

        rx
    }

    /// 手动添加条目
    pub fn add_entry(
        &mut self,
        content_type: ContentType,
        text: Option<String>,
        image_base64: Option<String>,
        image_thumb: Option<String>,
        expiry_days: u32,
    ) -> Option<String> {
        let hash_input = text.as_deref().unwrap_or("");
        let content_hash = hex::encode(Sha256::digest(hash_input.as_bytes()));

        if let Some(last) = &self.last_content_hash {
            if last == &content_hash {
                return None;
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
        self.store.entries.insert(0, entry);
        Some(self.store.entries[0].id.clone())
    }

    /// 处理从外部接收的新条目
    pub fn process_new_entry(&mut self, entry: ClipboardEntry) -> bool {
        let hash = entry.content_hash.clone();
        if self.last_content_hash.as_deref() == Some(&hash) {
            return false; // 重复
        }
        // 检查与列表最新是否重复
        if let Some(last) = self.store.entries.first() {
            if last.content_hash == entry.content_hash {
                return false;
            }
        }
        self.last_content_hash = Some(hash);
        self.store.entries.insert(0, entry);
        true
    }

    pub fn delete_entry(&mut self, id: &str) -> bool {
        let len_before = self.store.entries.len();
        self.store.entries.retain(|e| e.id != id);
        self.store.entries.len() < len_before
    }

    pub fn toggle_pin(&mut self, id: &str) -> bool {
        if let Some(entry) = self.store.entries.iter_mut().find(|e| e.id == id) {
            entry.is_pinned = !entry.is_pinned;
            true
        } else {
            false
        }
    }

    pub fn clean_expired(&mut self) -> usize {
        let now = Utc::now();
        let before = self.store.entries.len();
        self.store.entries.retain(|e| {
            if e.expiry_days == 0 {
                return true;
            }
            if let Ok(created) = chrono::DateTime::parse_from_rfc3339(&e.created_at) {
                let days = (now - created.with_timezone(&Utc)).num_days();
                days < e.expiry_days as i64
            } else {
                true
            }
        });
        before - self.store.entries.len()
    }

    pub fn search(&self, query: &str) -> Vec<&ClipboardEntry> {
        let q = query.to_lowercase();
        self.store
            .entries
            .iter()
            .filter(|e| {
                if let Some(ref text) = e.text_content {
                    text.to_lowercase().contains(&q)
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn get_sorted(&self) -> Vec<&ClipboardEntry> {
        let mut sorted: Vec<&ClipboardEntry> = self.store.entries.iter().collect();
        sorted.sort_by(|a, b| {
            b.is_pinned
                .cmp(&a.is_pinned)
                .then_with(|| b.created_at.cmp(&a.created_at))
        });
        sorted
    }

    pub fn to_store(&self) -> &ClipboardStore {
        &self.store
    }
}
