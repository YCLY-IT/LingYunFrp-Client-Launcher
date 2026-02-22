#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use tauri::Manager;
use tauri::Emitter;
use tauri::tray::{TrayIcon, TrayIconBuilder};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri_plugin_single_instance::init as single_instance_init;
use tauri_plugin_deep_link::DeepLinkExt;
mod config;
mod commands;
mod request;
mod tunnel;
mod virtual_network;

use virtual_network::{
    check_easy_tire_exists,
    start_easytire,
    stop_easytire,
    get_active_easytire,
    check_easytire_process_running,
};

use request::{
    forward_request,
    get_image_base64,
    download_file
};

use tunnel::{
    start_proxy,
    stop_proxy,
    wait_for_tunnel_start,
    TunnelStatus,
};

use commands::{
    close_window,
    minimize_window,
    toggle_maximize,
    hide_to_tray,
    check_frpc_exists,
    emit_event,
    get_app_data_dir,
    open_app_data_dir,
    get_frpc_cli_version,
    toggle_auto_start,
    kill_all_processes,
    get_client_version,
    quit_window,
    api_url,
    get_now_mode,
    get_system_info,
    get_api_url,
    check_auto_start_status,
    check_software_file,
    delete_file,
    auto_update,
    install_and_restart,
    tcping
};

// 显示主窗口命令
#[tauri::command]
fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

// 隐藏主窗口命令
#[tauri::command]
fn hide_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

// 显示设置页面命令
#[tauri::command]
fn show_settings(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        window.eval("window.location.href = '/#/dashboard/settings'").map_err(|e| e.to_string())?;
    }
    Ok(())
}

// 退出应用（不关闭FRPC）命令
#[tauri::command]
fn quit_without_frpc(app: tauri::AppHandle) {
    app.exit(0);
}

// 完全退出应用命令
#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    *app.state::<Mutex<bool>>().lock().unwrap() = true;
    if let Err(e) = kill_all_processes(vec!["frpc.exe".to_string(), "easytire-cli.exe".to_string()]) {
        eprintln!("关闭进程失败: {}", e);
    }
    app.exit(0);
}

// 隐藏托盘菜单窗口
#[tauri::command]
fn hide_tray_menu(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("tray_menu") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

// 检查主窗口是否可见
#[tauri::command]
fn is_main_window_visible(app: tauri::AppHandle) -> bool {
    if let Some(window) = app.get_webview_window("main") {
        window.is_visible().unwrap_or(true)
    } else {
        true
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
    .manage(Mutex::new(HashMap::<u32, std::process::Child>::new()))
    .manage(Mutex::new(false))
    .manage(TunnelStatus::new())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_deep_link::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(single_instance_init(|app, argv, _cwd| {
        // 第二实例启动时，激活主窗口
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
        }
        if let Some(url) = argv.iter().find(|s| s.starts_with("lyfrp://")) {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("deep-link", vec![url.clone()]);
            }
        }
    }))
    .invoke_handler(tauri::generate_handler![
        close_window,
        minimize_window,
        toggle_maximize,
        hide_to_tray,
        check_frpc_exists,
        emit_event,
        get_app_data_dir,
        open_app_data_dir,
        get_frpc_cli_version,
        toggle_auto_start,
        kill_all_processes,
        get_client_version,
        start_proxy,
        stop_proxy,
        wait_for_tunnel_start,
        quit_window,
        api_url,
        forward_request,
        get_now_mode,
        get_system_info,
        get_api_url,
        check_auto_start_status,
        get_image_base64,
        check_software_file,
        check_easy_tire_exists,
        start_easytire,
        stop_easytire,
        get_active_easytire,
        check_easytire_process_running,
        download_file,
        delete_file,
        auto_update,
        install_and_restart,
        tcping,
        // 新增命令
        show_main_window,
        hide_main_window,
        show_settings,
        quit_without_frpc,
        quit_app,
        hide_tray_menu,
        is_main_window_visible
    ])
    .setup(|app| {
        // 确保应用数据目录存在
        let app_data_dir = app.path().app_data_dir().unwrap();
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir).unwrap();
        }
        
        // 配置托盘菜单窗口
        setup_tray_menu_window(app)?;
        
        // 创建托盘图标
        let tray = create_tray_menu(app)?;
        app.manage(tray);
        
        let window = app.get_webview_window("main").unwrap();
        window.set_decorations(false).unwrap();
        
        // 修改后的窗口事件处理
        let window_clone = window.clone();
        window.on_window_event(move |event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = window_clone.hide();
                }
                tauri::WindowEvent::Moved { .. } => {
                    // 处理拖动事件
                }
                _ => {}
            }
        });
        
        #[cfg(target_os = "windows")]
        window.set_ignore_cursor_events(false).unwrap();

        #[cfg(any(windows, target_os = "linux"))]
        app.deep_link().register_all().unwrap();
        
        // 运行时注册自定义 scheme（仅桌面端可用）
        #[cfg(desktop)]
        app.deep_link().register("lyfrp").unwrap();

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error in running tauri application");
}

// 配置托盘菜单窗口
fn setup_tray_menu_window(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(tray_window) = app.get_webview_window("tray_menu") {
        // 监听窗口失焦事件，自动隐藏
        let window_clone = tray_window.clone();
        tray_window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(focused) = event {
                if !focused {
                    let _ = window_clone.hide();
                }
            }
        });
    }
    
    Ok(())
}

// 创建托盘图标
fn create_tray_menu(app: &tauri::App) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(move |tray, event| {
            match event {
                // 左键点击 - 显示/隐藏主窗口
                tauri::tray::TrayIconEvent::Click { 
                    button: tauri::tray::MouseButton::Left, 
                    .. 
                } => {
                    if let Some(window) = tray.app_handle().get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                // 右键点击 - 显示自定义菜单
                tauri::tray::TrayIconEvent::Click { 
                    button: tauri::tray::MouseButton::Right,
                    rect,
                    .. 
                } => {
                    if let Some(menu_window) = tray.app_handle().get_webview_window("tray_menu") {
                        // 计算菜单位置（在托盘图标上方或下方）
                        let menu_width = 260.0;
                        let menu_height = 320.0;
                        
                        // 从 Rect 中获取位置和大小
                        let (pos_x, pos_y) = match rect.position {
                            tauri::Position::Physical(p) => (p.x as f64, p.y as f64),
                            tauri::Position::Logical(p) => (p.x, p.y),
                        };
                        
                        let (size_width, size_height) = match rect.size {
                            tauri::Size::Physical(s) => (s.width as f64, s.height as f64),
                            tauri::Size::Logical(s) => (s.width, s.height),
                        };
                        
                        let x = pos_x + size_width / 2.0 - menu_width / 2.0;
                        let y = pos_y - menu_height - 8.0; // 在图标上方显示
                        
                        // 如果上方空间不够，显示在下方
                        let y = if y < 0.0 {
                            pos_y + size_height + 8.0
                        } else {
                            y
                        };
                        
                        let _ = menu_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(x as i32, y as i32)));
                        let _ = menu_window.show();
                        let _ = menu_window.set_focus();
                        
                        // 触发菜单显示事件，让前端更新窗口状态
                        let _ = menu_window.eval(r#"
                            window.dispatchEvent(new CustomEvent('menu-shown'));
                        "#);
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(tray)
}
