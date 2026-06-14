use chrono::Utc;
use sha2::{Digest, Sha256};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use crate::clipboard::types::{ClipboardEntry, ClipboardStore, ContentType};

// Windows 剪贴板格式常量
const CF_BITMAP: u16 = 2;
const CF_DIB: u16 = 8;
const CF_DIBV5: u16 = 17;
const BI_RGB: u32 = 0;
const BI_BITFIELDS: u32 = 3;

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

            let mut last_text: Option<String> = None;
            let mut last_img_hash: Option<String> = None;

            loop {
                thread::sleep(Duration::from_millis(500));

                // === 尝试读取文本 ===
                if let Ok(text) = clipboard_win::get_clipboard_string() {
                    if !text.is_empty() {
                        let hash = hex::encode(Sha256::digest(text.as_bytes()));
                        if last_text.as_deref() != Some(&hash) {
                            last_text = Some(hash.clone());
                            log::info!("📋 New text captured");

                            let entry = ClipboardEntry {
                                id: Uuid::new_v4().to_string(),
                                content_type: ContentType::Text,
                                text_content: Some(text),
                                image_base64: None,
                                image_thumb: None,
                                created_at: Utc::now().to_rfc3339(),
                                is_pinned: false,
                                expiry_days: expiry,
                                content_hash: hash,
                            };

                            if monitor_tx.send(ClipboardEvent::NewEntry(entry)).is_err() {
                                break;
                            }
                            continue; // 文本优先，跳过本轮图片检查
                        }
                    }
                }

                // === 尝试读取图片 ===
                // 文本无变化时检查是否是图片更新
                if let Some((image_b64, thumb_b64, hash)) = capture_image_from_clipboard() {
                    if last_img_hash.as_deref() != Some(&hash) {
                        last_img_hash = Some(hash.clone());
                        log::info!("📷 New image captured ({} bytes base64)", image_b64.len());

                        let entry = ClipboardEntry {
                            id: Uuid::new_v4().to_string(),
                            content_type: ContentType::Image,
                            text_content: None,
                            image_base64: Some(image_b64),
                            image_thumb: Some(thumb_b64),
                            created_at: Utc::now().to_rfc3339(),
                            is_pinned: false,
                            expiry_days: expiry,
                            content_hash: hash,
                        };

                        if monitor_tx.send(ClipboardEvent::NewEntry(entry)).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        rx
    }

    /// 手动添加条目（外部调用）
    pub fn add_entry(
        &mut self,
        content_type: ContentType,
        text: Option<String>,
        _image_base64: Option<String>,
        _image_thumb: Option<String>,
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
            image_base64: None,
            image_thumb: None,
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
            return false;
        }
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

// ============================================================
// 以下为 Windows 剪贴板图片捕获实现
// ============================================================

/// 尝试从剪贴板捕获图片
/// 返回 (base64_png, base64_thumb, content_hash)
fn capture_image_from_clipboard() -> Option<(String, String, String)> {
    unsafe {
        use windows_sys::Win32::System::DataExchange::{
            CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
        };
        use windows_sys::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return None;
        }

        // 优先 DIBV5，然后是 DIB
        let format = if IsClipboardFormatAvailable(CF_DIBV5 as u32) != 0 {
            CF_DIBV5
        } else if IsClipboardFormatAvailable(CF_DIB as u32) != 0 {
            CF_DIB
        } else {
            CloseClipboard();
            return None;
        };

        let handle = GetClipboardData(format as u32);
        if handle.is_null() {
            CloseClipboard();
            return None;
        }

        let ptr = GlobalLock(handle);
        if ptr.is_null() {
            CloseClipboard();
            return None;
        }

        // 读取 DIB 数据
        let size = GlobalSize(handle) as usize;
        let data_slice = std::slice::from_raw_parts(ptr as *const u8, size);
        let data = data_slice.to_vec();
        GlobalUnlock(handle);
        CloseClipboard();

        // 解析 DIB 并转为 PNG
        let (png_b64, thumb_b64, hash) = dib_to_png(&data)?;
        Some((png_b64, thumb_b64, hash))
    }
}

/// DIB → PNG 转换
/// 返回 (base64_png, base64_thumb, content_hash)
fn dib_to_png(dib_data: &[u8]) -> Option<(String, String, String)> {
    if dib_data.len() < 40 {
        return None;
    }

    // 解析 BITMAPINFOHEADER
    let bi_size = read_u32_le(&dib_data[0..4]);
    if bi_size < 40 {
        return None;
    }

    let width = read_i32_le(&dib_data[4..8]);
    let height = read_i32_le(&dib_data[8..12]);
    let _planes = read_u16_le(&dib_data[12..14]);
    let bit_count = read_u16_le(&dib_data[14..16]);
    let compression = read_u32_le(&dib_data[16..20]);

    if width <= 0 || height == 0 || (bit_count != 24 && bit_count != 32) {
        return None;
    }

    let abs_height = height.unsigned_abs() as u32;
    let w = width as u32;
    let h = abs_height;
    let bottom_up = height > 0;

    // 计算像素数据偏移
    // header 大小可能是 40 (BITMAPINFOHEADER) 或更大
    let header_size = bi_size as usize;
    let palette_size = if bit_count <= 8 {
        (1 << bit_count) * 4
    } else if bit_count == 16 && compression == BI_BITFIELDS {
        3 * 4 // 3 DWORD color masks
    } else {
        0
    };

    let pixel_offset = header_size + palette_size;
    if pixel_offset >= dib_data.len() {
        return None;
    }

    let pixels_raw = &dib_data[pixel_offset..];

    // 每行字节数（对齐到 4 字节）
    let row_size = ((w * bit_count as u32 + 31) / 32) * 4;
    let expected_size = row_size as usize * h as usize;
    if pixels_raw.len() < expected_size {
        return None;
    }

    // 构建 RGBA 像素数据
    let mut rgba = Vec::with_capacity((w * h * 4) as usize);

    for y in 0..h {
        let row = if bottom_up { h - 1 - y } else { y } as usize;
        let row_start = row * row_size as usize;

        for x in 0..w {
            let px = row_start + x as usize * (bit_count as usize / 8);
            if bit_count == 32 {
                let b = pixels_raw.get(px).copied().unwrap_or(0);
                let g = pixels_raw.get(px + 1).copied().unwrap_or(0);
                let r = pixels_raw.get(px + 2).copied().unwrap_or(0);
                let a = pixels_raw.get(px + 3).copied().unwrap_or(255);
                rgba.extend_from_slice(&[r, g, b, a]);
            } else {
                // 24-bit: BGR
                let b = pixels_raw.get(px).copied().unwrap_or(0);
                let g = pixels_raw.get(px + 1).copied().unwrap_or(0);
                let r = pixels_raw.get(px + 2).copied().unwrap_or(0);
                rgba.extend_from_slice(&[r, g, b, 255]);
            }
        }
    }

    // 使用 image crate 编码为 PNG
    let img = image::RgbaImage::from_raw(w, h, rgba)?;

    // 全尺寸 PNG → base64
    let mut full_buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut full_buf), image::ImageFormat::Png).ok()?;
    let hash = hex::encode(Sha256::digest(&full_buf));
    let png_b64 = base64_encode(&full_buf);

    // 缩略图（最大 240px 宽）
    let thumb_w = if w > 240 { 240u32 } else { w };
    let thumb_h = (h as f64 * thumb_w as f64 / w as f64) as u32;
    let thumb_img = image::imageops::resize(
        &img,
        thumb_w.max(1),
        thumb_h.max(1),
        image::imageops::FilterType::Lanczos3,
    );

    let mut thumb_buf = Vec::new();
    thumb_img
        .write_to(&mut std::io::Cursor::new(&mut thumb_buf), image::ImageFormat::Png)
        .ok()?;
    let thumb_b64 = base64_encode(&thumb_buf);

    Some((png_b64, thumb_b64, hash))
}

// ============================================================
// 辅助函数
// ============================================================

fn read_u16_le(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]])
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn read_i32_le(bytes: &[u8]) -> i32 {
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}
