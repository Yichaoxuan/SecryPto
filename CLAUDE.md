# CLAUDE.md — Secrypto 项目工作指南

## 项目概述
Secrypto 是一款 Windows 11 桌面应用，集剪贴板管理器与密码管理器于一体。
基于 Tauri 2.0 + Svelte 5 构建。

## 参考文档路径
所有项目文档位于 `docs/` 目录：

| 文档 | 路径 | 说明 |
|------|------|------|
| 需求文档 | `docs/REQUIREMENTS.md` | 功能需求、用户流程、非功能需求 |
| 技术规格 | `docs/TECHNICAL_SPEC.md` | 技术栈、数据结构、依赖版本 |
| 设计规范 | `docs/DESIGN_STANDARDS.md` | UI 规范、编码规范、提交规范 |
| 架构文档 | `docs/ARCHITECTURE.md` | 系统架构图、数据流、安全架构 |
| 开发日志 | `devlog/CHANGELOG.md` | 开发进度记录，完成/待办事项 |

## 开发命令

```bash
# 启动开发服务器
cd D:/Secrypto && npm run tauri dev

# 构建生产版本
cd D:/Secrypto && npm run tauri build

# Rust 检查
cd D:/Secrypto/src-tauri && cargo check

# Rust 测试
cd D:/Secrypto/src-tauri && cargo test

# 前端开发
cd D:/Secrypto && npm run dev
```

## 工作流程

### 沟通规则
1. **收到任务后**，先读取相关文档（docs/ 下的需求/技术/设计文件）
2. **重要决策**需记录到 `devlog/CHANGELOG.md`
3. **阶段性成果**要及时提交 git，commit message 遵循 Conventional Commits

### 开发顺序
按照以下阶段推进，每阶段完成后再进入下一阶段：

1. **项目脚手架**（Tauri 初始化、依赖安装、基础配置）
2. **剪贴板监听引擎**（监听、抓取、去重、清理）
3. **全局热键 + 托盘 + 开机自启**
4. **剪贴板历史 UI**（卡片列表、搜索、交互）
5. **密码本模块**（加密、CRUD、生成器）
6. **集成打包与打磨**（MSI、性能优化）

### 每次开发前
1. 检查 `devlog/CHANGELOG.md` 了解当前进度
2. 读取相关文档确认技术方案
3. `git pull` 拉取最新代码

### 每次开发后
1. 更新 `devlog/CHANGELOG.md` 记录完成事项
2. `git add -A && git commit -m "类型: 描述"`
3. `git push` 推送代码

## 提醒
- 保持代码简洁，不要过度设计
- UI 贴合 Win11 Fluent Design 风格
- 内存占用是核心指标，避免内存泄漏
- 密码安全不容妥协，严格按照安全架构实现
