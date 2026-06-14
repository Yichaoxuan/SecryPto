@echo off
chcp 65001 >nul
title Secrypto

REM 设置 VS 2022 BuildTools 环境
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

set PATH=%USERPROFILE%\.cargo\bin;%PATH%
rustup default stable-x86_64-pc-windows-msvc >nul
cd /d D:\Secrypto
npm run tauri dev
pause
