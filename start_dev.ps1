# Secrypto 开发启动脚本 (PowerShell)
# 设置 VS 2022 BuildTools 环境
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
cmd /c "call `"$vcvars`" && set" | ForEach-Object {
    if ($_ -match '^([^=]+)=(.*)$') {
        [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
    }
}

$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
rustup default stable-x86_64-pc-windows-msvc | Out-Null
Set-Location D:\Secrypto
npm run tauri dev
