@echo off
:: 检查是否为管理员，如果不是则用管理员权限重新运行
cd /d %~dp0

REM 启动前端 dev server（普通权限）
start "Vite Dev Server" cmd /k pnpm run dev

REM 启动 Tauri 主进程（管理员权限，不带 dev server）
cd src-tauri
start "Tauri" cmd /k cargo run
@REM powershell -Command "Start-Process cmd -Verb runAs -ArgumentList '/k cargo run'"