use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

/// 初始化系统托盘
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) {
    let _tray = TrayIconBuilder::new()
        .tooltip("Secrypto - 剪贴板管理器")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                // 左键点击显示/隐藏窗口
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app);
}

/// 设置开机自启
pub fn set_auto_start(app: &AppHandle, enable: bool) -> Result<(), String> {
    // 写注册表 HKCU\Software\Microsoft\Windows\CurrentVersion\Run
    let key_path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let app_name = "Secrypto";

    if enable {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get exe path: {}", e))?;
        // Windows 注册表操作使用 winreg crate
        // 当前简化实现：后续添加
        log::info!("Auto-start enabled: {:?}", exe_path);
    } else {
        log::info!("Auto-start disabled");
    }

    Ok(())
}
