use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{rand_core::CryptoRngCore, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use zeroize::Zeroize;

/// 密码库加密管理器
pub struct VaultCrypto;

impl VaultCrypto {
    /// 从主密码派生 AES-256 密钥（Argon2id）
    pub fn derive_key(master_password: &str, salt: &str) -> Result<[u8; 32], String> {
        let salt_bytes = SaltString::from_b64(salt)
            .map_err(|e| format!("Invalid salt: {}", e))?;

        let mut key = [0u8; 32];
        // Argon2id with recommended parameters for interactive use
        let argon2 = Argon2::default();
        argon2
            .hash_password_into(master_password.as_bytes(), salt_bytes.as_ref().as_bytes(), &mut key)
            .map_err(|e| format!("Key derivation failed: {}", e))?;

        Ok(key)
    }

    /// 加密明文
    pub fn encrypt(plaintext: &str, key: &[u8; 32]) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Invalid key: {}", e))?;

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // 格式: nonce(12) + ciphertext
        let mut combined = nonce_bytes.to_vec();
        combined.extend_from_slice(&ciphertext);

        Ok(BASE64.encode(&combined))
    }

    /// 解密密文
    pub fn decrypt(encrypted: &str, key: &[u8; 32]) -> Result<String, String> {
        let combined = BASE64
            .decode(encrypted)
            .map_err(|e| format!("Base64 decode failed: {}", e))?;

        if combined.len() < 12 {
            return Err("Invalid ciphertext".to_string());
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Invalid key: {}", e))?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "Decryption failed (wrong password or corrupted data)".to_string())?;

        String::from_utf8(plaintext)
            .map_err(|_| "Invalid UTF-8 in decrypted data".to_string())
    }

    /// 生成随机盐
    pub fn generate_salt() -> String {
        SaltString::generate(&mut OsRng).to_string()
    }

    /// 生成主密码验证哈希（用于验证主密码是否正确）
    pub fn hash_master_password(password: &str, salt: &str) -> Result<String, String> {
        let salt_bytes = SaltString::from_b64(salt)
            .map_err(|e| format!("Invalid salt: {}", e))?;

        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt_bytes)
            .map_err(|e| format!("Hashing failed: {}", e))?;

        Ok(hash.to_string())
    }

    /// 验证主密码
    pub fn verify_master_password(password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| format!("Invalid hash: {}", e))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// 生成随机密码
    pub fn generate_password(length: u32, use_upper: bool, use_lower: bool, use_digits: bool, use_symbols: bool) -> String {
        let mut chars = Vec::new();
        if use_upper { chars.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()); }
        if use_lower { chars.extend("abcdefghijklmnopqrstuvwxyz".chars()); }
        if use_digits { chars.extend("0123456789".chars()); }
        if use_symbols { chars.extend("!@#$%^&*()-_=+[]{}|;:,.<>?".chars()); }

        if chars.is_empty() {
            chars.extend("abcdefghijklmnopqrstuvwxyz".chars());
        }

        let len = length.max(4).min(128) as usize;
        let mut rng = rand::thread_rng();
        (0..len)
            .map(|_| {
                let idx = rng.next_u32() as usize % chars.len();
                chars[idx]
            })
            .collect()
    }
}
