# Secrypto 技术规格文档

## 技术栈

| 层级 | 技术 | 版本 | 说明 |
|------|------|------|------|
| 桌面框架 | Tauri | 2.x | Rust 驱动的跨平台桌面框架 |
| 前端框架 | Svelte | 5.x | 编译型前端框架，极致轻量 |
| 前端语言 | TypeScript | 5.x | 类型安全的 JavaScript 超集 |
| 构建工具 | Vite | 6.x | 快速的开发服务器与打包 |
| 后端语言 | Rust | 1.96+ | 系统级编程语言 |
| UI 风格 | Fluent Design | - | Win11 原生设计语言 |

## Rust 依赖

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
base64 = "0.22"
aes-gcm = "0.10"
argon2 = "0.5"
windows-sys = { version = "0.59", features = [
    "Win32_Foundation",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_System_DataExchange",
    "Win32_Graphics_Gdi",
] }
image = "0.25"
uuid = { version = "1", features = ["v4"] }
tauri = { version = "2", features = ["tray-icon"] }
```

## 前端依赖（预估）

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-clipboard-manager": "^2"
  },
  "devDependencies": {
    "svelte": "^5",
    "vite": "^6",
    "@sveltejs/vite-plugin-svelte": "^5",
    "typescript": "^5"
  }
}
```

## 数据结构

### ClipboardEntry（剪贴板条目）

| 字段 | 类型 | 说明 |
|------|------|------|
| id | String(UUID) | 唯一标识 |
| content_type | String | "text" 或 "image" |
| text_content | Option\<String> | 文本内容（文本类型时） |
| image_base64 | Option\<String> | 图片 Base64（图片类型时） |
| image_thumb | Option\<String> | 缩略图 Base64 |
| created_at | String(ISO 8601) | 创建时间 |
| is_pinned | bool | 是否置顶 |
| expiry_days | u32 | 保留天数（0=永久） |
| content_hash | String | 内容 SHA256，用于去重 |

### PasswordEntry（密码条目）

| 字段 | 类型 | 说明 |
|------|------|------|
| id | String(UUID) | 唯一标识 |
| name | String | 名称/网站名 |
| url | String | 网站地址 |
| username | String | 用户名 |
| encrypted_password | String(Base64) | AES-256-GCM 加密后的密码 |
| notes | String | 备注 |
| tags | Vec\<String> | 标签列表 |
| created_at | String(ISO 8601) | 创建时间 |
| updated_at | String(ISO 8601) | 更新时间 |

### Settings（设置）

| 字段 | 类型 | 说明 |
|------|------|------|
| hotkey | String | 全局热键组合（默认 "Win+V"） |
| default_expiry_days | u32 | 默认保留期限（默认 3 天） |
| auto_start | bool | 是否开机自启 |
| theme | String | "system" / "light" / "dark" |
| language | String | "zh" / "en" |
| vault_locked | bool | 密码本是否锁定 |

## 文件存储结构

```
%APPDATA%/Secrypto/
├── clipboard_history.json   # 剪贴板历史
├── vault.json               # 加密的密码库
├── settings.json            # 用户设置
└── images/                  # 图片缓存目录
    └── {uuid}.png
```

## 全局热键实现方案

使用 Windows 低级键盘钩子（`WH_KEYBOARD_LL`）监听 Win+V：
- 检测到 Win 键按下 + V 键按下 → 显示主窗口
- 其他按键照常传递，不影响系统其他功能
- 热键组合可在设置中自定义

## 安全方案

- 主密码 → Argon2id（迭代 3 次，192MB 内存）→ 256-bit 密钥
- 每个密码条目使用独立 96-bit 随机 nonce
- 密文格式：Base64(nonce + ciphertext + tag)
- 明文密钥仅存在于运行时内存中，从不落盘
