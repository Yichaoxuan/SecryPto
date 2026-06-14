# Secrypto 设计与编码规范

## UI 设计规范

### Win11 Fluent Design 风格

```
┌─────────────────────────────────────┐
│ 🗂️ 剪贴板    🔒 密码本    ⚙️ 设置   │ ← Tab 导航
├─────────────────────────────────────┤
│ 🔍 搜索历史...          ┌──┐       │ ← 搜索栏
│                         │全部▼│     │ ← 过滤下拉
├─────────────────────────────────────┤
│ ┌─────────────────────────────┐     │
│ │ 文本内容预览文字...    📌🗑️ │     │ ← 卡片
│ │ 3 分钟前                    │     │
│ └─────────────────────────────┘     │
│ ┌─────────────────────────────┐     │
│ │ [图片缩略图]          📌🗑️ │     │ ← 图片卡片
│ │ 1 小时前                    │     │
│ └─────────────────────────────┘     │
└─────────────────────────────────────┘
```

### 颜色体系

| 令牌 | 亮色模式 | 暗色模式 |
|------|---------|---------|
| --bg-primary | #FFFFFF | #1C1C1C |
| --bg-secondary | #F3F3F3 | #2D2D2D |
| --bg-card | #F9F9F9 | #252525 |
| --accent | #005FB8 | #60CDFF |
| --text-primary | #1A1A1A | #FFFFFF |
| --text-secondary | #757575 | #9D9D9D |
| --border | #E5E5E5 | #404040 |

### 尺寸规范

- 窗口默认尺寸：800 × 600px
- 最小尺寸：480 × 400px
- 卡片圆角：8px
- 卡片间距：8px
- 窗口圆角：使用 Mica 背景自动圆角
- 字体：Segoe UI Variable（Win11 默认字体）
- 字号：正文 14px，标题 18px，小字 12px

### 交互规范

- 所有可点击元素悬停时颜色加深 10%
- 卡片点击有涟漪动画
- 删除操作需二次确认（Toast + 撤销）
- 置顶卡片显示 📌 标记，始终在列表最前
- 搜索输入防抖 300ms

## Rust 编码规范

### 命名约定
- 类型、trait、enum 使用 `PascalCase`
- 函数、方法、变量使用 `snake_case`
- 常量使用 `SCREAMING_SNAKE_CASE`
- 错误类型使用 `PascalCase + Error` 后缀

### 模块结构
```
src-tauri/src/
├── main.rs              # Tauri 入口
├── clipboard/
│   ├── mod.rs           # 模块导出
│   ├── monitor.rs       # 剪贴板监听
│   └── types.rs         # 数据结构
├── vault/
│   ├── mod.rs
│   ├── crypto.rs        # 加密解密
│   └── types.rs
├── storage/
│   ├── mod.rs
│   ├── clipboard_store.rs  # 剪贴板持久化
│   └── vault_store.rs      # 密码库持久化
├── hotkey/
│   └── mod.rs           # 全局热键
└── tray/
    └── mod.rs           # 系统托盘
```

### 错误处理
- 使用 `thiserror` 定义业务错误类型
- 所有公开函数返回 `Result<T, AppError>`
- 前端调用的 Tauri command 返回 `Result<T, String>`

## Svelte 编码规范

### 组件结构
```
src/
├── App.svelte           # 根组件
├── main.ts              # 入口
├── app.css              # 全局样式
├── lib/
│   ├── components/
│   │   ├── ClipboardCard.svelte
│   │   ├── PasswordEntry.svelte
│   │   ├── SearchBar.svelte
│   │   ├── TabNav.svelte
│   │   ├── Toast.svelte
│   │   └── ...
│   ├── stores/
│   │   ├── clipboard.ts    # 剪贴板状态
│   │   ├── vault.ts        # 密码本状态
│   │   └── settings.ts     # 设置状态
│   ├── types/
│   │   └── index.ts        # TypeScript 类型定义
│   └── utils/
│       ├── time.ts         # 时间格式化
│       └── clipboard.ts    # 前端剪贴板工具
```

### 状态管理
- 使用 Svelte 5 `$state` rune 管理组件状态
- 全局状态使用 Svelte 的 module context + `$state`
- 避免引入外部状态管理库

## 提交规范

使用 Conventional Commits：
```
feat: 新功能
fix: 修复
docs: 文档变更
style: 代码风格变更（不影响功能）
refactor: 重构
perf: 性能优化
test: 测试
chore: 构建/工具变更
```
