use crate::common::{PasswordEntry, VaultStore};
use crate::vault::crypto::VaultCrypto;
use uuid::Uuid;
use chrono::Utc;

/// 密码库管理器
pub struct VaultManager {
    store: VaultStore,
    key: Option<[u8; 32]>,  // 当前会话密钥（仅内存中）
    unlocked: bool,
}

impl VaultManager {
    pub fn new() -> Self {
        Self {
            store: VaultStore::new(),
            key: None,
            unlocked: false,
        }
    }

    pub fn load(&mut self, store: VaultStore) {
        self.store = store;
    }

    pub fn store(&self) -> &VaultStore {
        &self.store
    }

    /// 初始化密码库（首次使用）
    pub fn initialize(&mut self, master_password: &str) -> Result<(), String> {
        let salt = VaultCrypto::generate_salt();
        let hash = VaultCrypto::hash_master_password(master_password, &salt)?;
        let key = VaultCrypto::derive_key(master_password, &salt)?;

        self.store.salt = salt;
        self.store.master_password_hash = hash;
        self.key = Some(key);
        self.unlocked = true;
        Ok(())
    }

    /// 解锁密码库
    pub fn unlock(&mut self, master_password: &str) -> Result<bool, String> {
        if self.store.master_password_hash.is_empty() {
            return Err("Vault not initialized".to_string());
        }

        let valid = VaultCrypto::verify_master_password(master_password, &self.store.master_password_hash)?;
        if valid {
            let key = VaultCrypto::derive_key(master_password, &self.store.salt)?;
            self.key = Some(key);
            self.unlocked = true;
        }
        Ok(valid)
    }

    /// 锁定密码库
    pub fn lock(&mut self) {
        self.key = None;
        self.unlocked = false;
    }

    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    /// 添加密码条目
    pub fn add_entry(&mut self, name: &str, url: &str, username: &str, password: &str, notes: &str, tags: Vec<String>) -> Result<String, String> {
        let key = self.key.as_ref().ok_or("Vault is locked")?;
        let encrypted = VaultCrypto::encrypt(password, key)?;

        let now = Utc::now().to_rfc3339();
        let entry = PasswordEntry {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            url: url.to_string(),
            username: username.to_string(),
            encrypted_password: encrypted,
            notes: notes.to_string(),
            tags,
            created_at: now.clone(),
            updated_at: now,
        };

        let id = entry.id.clone();
        self.store.encrypted_entries.push(entry);
        Ok(id)
    }

    /// 更新密码条目
    pub fn update_entry(&mut self, id: &str, name: &str, url: &str, username: &str, password: &str, notes: &str, tags: Vec<String>) -> Result<bool, String> {
        let key = self.key.as_ref().ok_or("Vault is locked")?;
        let encrypted = VaultCrypto::encrypt(password, key)?;

        if let Some(entry) = self.store.encrypted_entries.iter_mut().find(|e| e.id == id) {
            entry.name = name.to_string();
            entry.url = url.to_string();
            entry.username = username.to_string();
            entry.encrypted_password = encrypted;
            entry.notes = notes.to_string();
            entry.tags = tags;
            entry.updated_at = Utc::now().to_rfc3339();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 删除密码条目
    pub fn delete_entry(&mut self, id: &str) -> bool {
        let len_before = self.store.encrypted_entries.len();
        self.store.encrypted_entries.retain(|e| e.id != id);
        self.store.encrypted_entries.len() < len_before
    }

    /// 解密并获取密码（返回明文密码）
    pub fn get_password(&self, id: &str) -> Result<String, String> {
        let key = self.key.as_ref().ok_or("Vault is locked")?;
        let entry = self.store.encrypted_entries.iter()
            .find(|e| e.id == id)
            .ok_or("Entry not found")?;
        VaultCrypto::decrypt(&entry.encrypted_password, key)
    }

    /// 获取所有加密条目（不解密密码）
    pub fn get_entries(&self) -> &Vec<PasswordEntry> {
        &self.store.encrypted_entries
    }

    /// 搜索条目
    pub fn search(&self, query: &str) -> Vec<&PasswordEntry> {
        let q = query.to_lowercase();
        self.store.encrypted_entries.iter()
            .filter(|e| {
                e.name.to_lowercase().contains(&q)
                    || e.url.to_lowercase().contains(&q)
                    || e.username.to_lowercase().contains(&q)
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    /// 生成随机密码
    pub fn generate_password(length: u32, use_upper: bool, use_lower: bool, use_digits: bool, use_symbols: bool) -> String {
        VaultCrypto::generate_password(length, use_upper, use_lower, use_digits, use_symbols)
    }
}
