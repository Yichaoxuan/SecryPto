use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 热键消息
#[derive(Debug, Clone)]
pub enum HotkeyAction {
    ToggleWindow,
    ShowWindow,
    HideWindow,
}

/// 全局热键管理器
pub struct HotkeyManager {
    sender: mpsc::Sender<HotkeyAction>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::channel();
        Self { sender: tx }
    }

    pub fn get_sender(&self) -> mpsc::Sender<HotkeyAction> {
        self.sender.clone()
    }

    /// 注册全局 Win+V 热键（低级键盘钩子）
    /// 返回事件接收器，调用方需循环处理
    pub fn register(pressed_tx: mpsc::Sender<HotkeyAction>) -> Result<(), String> {
        thread::spawn(move || {
            log::info!("⌨️ Global hotkey monitor started (Win+V)");

            // 使用轮询方式检测按键状态
            // 低级键盘钩子 SetWindowsHookEx(WH_KEYBOARD_LL) 需要消息循环
            // 这里使用更简单的方式：定期检查 GetAsyncKeyState
            loop {
                thread::sleep(Duration::from_millis(100));

                unsafe {
                    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

                    // VK_LWIN = 0x5B, VK_RWIN = 0x5C, 'V' = 0x56
                    let state_win_l = GetAsyncKeyState(0x5B) as u16;
                    let state_win_r = GetAsyncKeyState(0x5C) as u16;
                    let state_v = GetAsyncKeyState(0x56) as u16;
                    let win_pressed = (state_win_l & 0x8000) != 0 || (state_win_r & 0x8000) != 0;
                    let v_pressed = (state_v & 0x8000) != 0;

                    if win_pressed && v_pressed {
                        thread::sleep(Duration::from_millis(50));
                        let state_win_l2 = GetAsyncKeyState(0x5B) as u16;
                        let state_win_r2 = GetAsyncKeyState(0x5C) as u16;
                        let state_v2 = GetAsyncKeyState(0x56) as u16;
                        if ((state_win_l2 & 0x8000) != 0 || (state_win_r2 & 0x8000) != 0)
                            && (state_v2 & 0x8000) != 0
                        {
                            let _ = pressed_tx.send(HotkeyAction::ToggleWindow);
                            log::info!("🔑 Win+V detected");
                            // 等待按键释放，避免重复触发
                            thread::sleep(Duration::from_millis(300));
                        }
                    }
                }
            }
        });

        Ok(())
    }
}
