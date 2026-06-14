use std::sync::mpsc;

/// 热键消息
#[derive(Debug, Clone)]
pub enum HotkeyAction {
    ShowWindow,
    HideWindow,
}

/// 全局热键管理器（Windows 低级键盘钩子）
pub struct HotkeyManager {
    sender: mpsc::Sender<HotkeyAction>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::channel();
        Self { sender: tx }
    }

    /// 发送显示窗口命令
    pub fn request_show(&self) {
        let _ = self.sender.send(HotkeyAction::ShowWindow);
    }

    /// 发送隐藏窗口命令
    pub fn request_hide(&self) {
        let _ = self.sender.send(HotkeyAction::HideWindow);
    }

    pub fn get_sender(&self) -> mpsc::Sender<HotkeyAction> {
        self.sender.clone()
    }

    /// 注册全局热键（Win+V）
    /// 注意：在正式实现中会使用 SetWindowsHookEx 或 RegisterHotKey
    /// 当前为占位实现，后续窗口管理会集成
    pub fn register(&self) -> Result<(), String> {
        // TODO: 实现 Windows 低级键盘钩子
        // - 使用 SetWindowsHookEx(WH_KEYBOARD_LL, ...)
        // - 监听 VK_LWIN + VK_V 组合键
        // - 按热键时发送 HotkeyAction::ShowWindow
        log::info!("Hotkey registration placeholder");
        Ok(())
    }
}
