use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

/// 初始化系统托盘
pub fn create_tray(app: &AppHandle) {
    let icon = app.default_window_icon().unwrap().clone();

    let _tray = TrayIconBuilder::new()
        .tooltip("Secrypto - 剪贴板管理器")
        .icon(icon)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_main_window(tray.app_handle());
            }
        })
        .build(app);
}

/// 切换主窗口显示/隐藏
pub fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

/// 显示主窗口
pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
    }
}

/// 隐藏主窗口
pub fn hide_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

/// 设置开机自启（通过写注册表）
pub fn set_auto_start(app: &AppHandle, enable: bool) -> Result<(), String> {
    let app_name = "Secrypto";

    if enable {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get exe path: {}", e))?;
        let path_str = exe_path.to_string_lossy().to_string();

        let key = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
            .open_subkey_with_flags(
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                winreg::enums::KEY_SET_VALUE,
            )
            .map_err(|e| format!("Failed to open registry key: {}", e))?;

        key.set_value(app_name, &path_str)
            .map_err(|e| format!("Failed to set registry value: {}", e))?;

        log::info!("Auto-start enabled: {}", path_str);
    } else {
        let key = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
            .open_subkey_with_flags(
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                winreg::enums::KEY_SET_VALUE,
            )
            .map_err(|e| format!("Failed to open registry key: {}", e))?;

        let _ = key.delete_value(app_name);
        log::info!("Auto-start disabled");
    }

    Ok(())
}
