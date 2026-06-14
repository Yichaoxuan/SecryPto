mod clipboard;
pub mod common;
mod hotkey;
mod storage;
mod tray;
mod vault;

use std::sync::Mutex;
use std::thread;

use clipboard::monitor::ClipboardManager;
use clipboard::types::{ClipboardEntry, ContentType};
use common::AppError;
use storage::clipboard_store::ClipboardStoreIO;
use storage::vault_store::VaultStoreIO;
use tauri::{Emitter, Manager};
use vault::manager::VaultManager;

/// 应用全局状态
struct AppState {
    clipboard_mgr: Mutex<ClipboardManager>,
    vault_mgr: Mutex<VaultManager>,
    clipboard_store: ClipboardStoreIO,
    vault_store: VaultStoreIO,
}

// ==================== 剪贴板命令 ====================

#[tauri::command]
fn get_clipboard_history(state: tauri::State<AppState>) -> Result<Vec<ClipboardEntry>, String> {
    let mgr = state.clipboard_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get_sorted().into_iter().cloned().collect())
}

#[tauri::command]
fn delete_clipboard_entry(state: tauri::State<AppState>, id: String) -> Result<bool, String> {
    let mut mgr = state.clipboard_mgr.lock().map_err(|e| e.to_string())?;
    let result = mgr.delete_entry(&id);
    if result {
        state
            .clipboard_store
            .save(mgr.to_store().entries.as_slice())?;
    }
    Ok(result)
}

#[tauri::command]
fn toggle_pin_entry(state: tauri::State<AppState>, id: String) -> Result<bool, String> {
    let mut mgr = state.clipboard_mgr.lock().map_err(|e| e.to_string())?;
    let result = mgr.toggle_pin(&id);
    if result {
        state
            .clipboard_store
            .save(mgr.to_store().entries.as_slice())?;
    }
    Ok(result)
}

#[tauri::command]
fn search_clipboard(
    state: tauri::State<AppState>,
    query: String,
) -> Result<Vec<ClipboardEntry>, String> {
    let mgr = state.clipboard_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.search(&query).into_iter().cloned().collect())
}

#[tauri::command]
fn clean_expired(state: tauri::State<AppState>) -> Result<usize, String> {
    let mut mgr = state.clipboard_mgr.lock().map_err(|e| e.to_string())?;
    let count = mgr.clean_expired();
    if count > 0 {
        state
            .clipboard_store
            .save(mgr.to_store().entries.as_slice())?;
    }
    Ok(count)
}

// ==================== 密码本命令 ====================

#[tauri::command]
fn vault_initialize(
    state: tauri::State<AppState>,
    master_password: String,
) -> Result<bool, String> {
    let mut mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    mgr.initialize(&master_password)?;
    state.vault_store.save_vault(mgr.store())?;
    Ok(true)
}

#[tauri::command]
fn vault_unlock(
    state: tauri::State<AppState>,
    master_password: String,
) -> Result<bool, String> {
    let mut mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    let result = mgr.unlock(&master_password)?;
    Ok(result)
}

#[tauri::command]
fn vault_lock(state: tauri::State<AppState>) -> Result<(), String> {
    let mut mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    mgr.lock();
    Ok(())
}

#[tauri::command]
fn vault_is_unlocked(state: tauri::State<AppState>) -> Result<bool, String> {
    let mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.is_unlocked())
}

#[tauri::command]
fn vault_is_initialized(state: tauri::State<AppState>) -> Result<bool, String> {
    let mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    Ok(!mgr.store().master_password_hash.is_empty())
}

#[tauri::command]
fn vault_add_entry(
    state: tauri::State<AppState>,
    name: String,
    url: String,
    username: String,
    password: String,
    notes: String,
    tags: Vec<String>,
) -> Result<String, String> {
    let mut mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    let id = mgr.add_entry(&name, &url, &username, &password, &notes, tags)?;
    state.vault_store.save_vault(mgr.store())?;
    Ok(id)
}

#[tauri::command]
fn vault_update_entry(
    state: tauri::State<AppState>,
    id: String,
    name: String,
    url: String,
    username: String,
    password: String,
    notes: String,
    tags: Vec<String>,
) -> Result<bool, String> {
    let mut mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    let result = mgr.update_entry(&id, &name, &url, &username, &password, &notes, tags)?;
    if result {
        state.vault_store.save_vault(mgr.store())?;
    }
    Ok(result)
}

#[tauri::command]
fn vault_delete_entry(state: tauri::State<AppState>, id: String) -> Result<bool, String> {
    let mut mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    let result = mgr.delete_entry(&id);
    if result {
        state.vault_store.save_vault(mgr.store())?;
    }
    Ok(result)
}

#[tauri::command]
fn vault_get_password(state: tauri::State<AppState>, id: String) -> Result<String, String> {
    let mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    mgr.get_password(&id)
}

#[tauri::command]
fn vault_get_entries(
    state: tauri::State<AppState>,
) -> Result<Vec<common::PasswordEntry>, String> {
    let mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get_entries().clone())
}

#[tauri::command]
fn vault_search(
    state: tauri::State<AppState>,
    query: String,
) -> Result<Vec<common::PasswordEntry>, String> {
    let mgr = state.vault_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.search(&query).into_iter().cloned().collect())
}

#[tauri::command]
fn vault_generate_password(
    length: u32,
    use_upper: bool,
    use_lower: bool,
    use_digits: bool,
    use_symbols: bool,
) -> String {
    VaultManager::generate_password(length, use_upper, use_lower, use_digits, use_symbols)
}

// ==================== 应用入口 ====================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化存储
    let clipboard_store = ClipboardStoreIO::new();
    let vault_store = VaultStoreIO::new();

    // 加载数据
    let clipboard_entries = clipboard_store.load();
    let vault_data = vault_store.load_vault();

    // 初始化管理器
    let mut clipboard_mgr = ClipboardManager::new();
    clipboard_mgr.load(clipboard_entries);

    let mut vault_mgr = VaultManager::new();
    vault_mgr.load(vault_data);

    let state = AppState {
        clipboard_mgr: Mutex::new(clipboard_mgr),
        vault_mgr: Mutex::new(vault_mgr),
        clipboard_store,
        vault_store,
    };

    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 初始化系统托盘
            tray::create_tray(app.handle());

            // 注册全局 Win+V 热键
            let app_handle_hotkey = app.handle().clone();
            let (hotkey_tx, hotkey_rx) = std::sync::mpsc::channel();
            let _ = hotkey::HotkeyManager::register(hotkey_tx);

            // 处理热键事件
            std::thread::spawn(move || {
                while let Ok(action) = hotkey_rx.recv() {
                    match action {
                        hotkey::HotkeyAction::ToggleWindow => {
                            tray::toggle_main_window(&app_handle_hotkey);
                        }
                        hotkey::HotkeyAction::ShowWindow => {
                            tray::show_main_window(&app_handle_hotkey);
                        }
                        hotkey::HotkeyAction::HideWindow => {
                            tray::hide_main_window(&app_handle_hotkey);
                        }
                    }
                }
            });

            // 启动剪贴板监听
            let app_handle = app.handle().clone();
            let rx = {
                let state = app.state::<AppState>();
                let mut mgr = state.clipboard_mgr.lock().unwrap();
                mgr.start_monitoring()
            };
            // 单独线程处理剪贴板事件并存入管理器
            thread::spawn(move || {
                log::info!("📋 Clipboard event processor started");
                loop {
                    match rx.recv() {
                        Ok(event) => {
                            if let clipboard::monitor::ClipboardEvent::NewEntry(entry) = event {
                                // 通过 app_handle 访问 state 并存入
                                if let Some(state) = app_handle.try_state::<AppState>() {
                                    if let Ok(mut mgr) = state.clipboard_mgr.lock() {
                                        let is_new = mgr.process_new_entry(entry);
                                        if is_new {
                                            // 持久化保存
                                            let _ = state.clipboard_store
                                                .save(mgr.to_store().entries.as_slice());
                                            // 通知前端刷新
                                            let _ = app_handle.emit("clipboard-updated", ());
                                            log::info!("📋 Clipboard saved & frontend notified");
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            log::error!("Clipboard event channel closed");
                            break;
                        }
                    }
                }
            });

            // 清理过期剪贴板条目
            if let Some(state) = app.try_state::<AppState>() {
                if let Ok(mut mgr) = state.clipboard_mgr.lock() {
                    let cleaned = mgr.clean_expired();
                    if cleaned > 0 {
                        log::info!("Cleaned {} expired clipboard entries", cleaned);
                        let _ = state
                            .clipboard_store
                            .save(mgr.to_store().entries.as_slice());
                    }
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 剪贴板
            get_clipboard_history,
            delete_clipboard_entry,
            toggle_pin_entry,
            search_clipboard,
            clean_expired,
            // 密码本
            vault_initialize,
            vault_unlock,
            vault_lock,
            vault_is_unlocked,
            vault_is_initialized,
            vault_add_entry,
            vault_update_entry,
            vault_delete_entry,
            vault_get_password,
            vault_get_entries,
            vault_search,
            vault_generate_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
