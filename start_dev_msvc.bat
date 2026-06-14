@echo off
chcp 65001 >nul
title Secrypto 开发模式

REM 设置 VS 2022 BuildTools 环境（MSVC 链接器）
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

REM 添加 Rust 工具链
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

REM 确保使用 MSVC 工具链
rustup default stable-x86_64-pc-windows-msvc >nul

REM 进入项目目录
cd /d D:\Secrypto

echo.
echo ========================================
echo   Secrypto 开发模式启动中...
echo ========================================
echo.

npm run tauri dev

pause
