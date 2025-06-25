// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_single_instance::init as single_instance_init;
mod config;
mod commands;
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
    download_frpc,
    toggle_auto_start,
    kill_all_processes,
    get_cpl_version,
    get_client_version,
    start_proxy,
    stop_proxy,
    quit_window,
    open_url,
    api_url,
    forward_request,
    get_now_mode,
    get_system_info,
    get_api_url
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
    .manage(Mutex::new(HashMap::<u32, std::process::Child>::new()))
    .manage(Mutex::new(false)) // 添加退出状态标志
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(single_instance_init(|app, _argv, _cwd| {
        // 第二实例启动时，激活主窗口
        let window = app.get_webview_window("main").unwrap();
        let _ = window.show();
        let _ = window.set_focus();
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
            download_frpc,
            toggle_auto_start,
            kill_all_processes,
            get_cpl_version,
            get_client_version,
            start_proxy,
            stop_proxy,
            quit_window,
            open_url,
            api_url,
            forward_request,
            get_now_mode,
            get_system_info,
            get_api_url
        ])
        .setup(|app| {
            // 确保应用数据目录存在
            let app_data_dir = app.path().app_data_dir().unwrap();
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir).unwrap();
            }
            
            create_tray_menu(app)?;
            let window = app.get_webview_window("main").unwrap();
            window.set_decorations(false).unwrap();
            
            // 修改后的窗口事件处理
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        let _ = window_clone.hide();
                        
                        // 检查是否是退出操作
                        let is_quitting = *window_clone.app_handle().state::<Mutex<bool>>().lock().unwrap();
                        if !is_quitting && !window_clone.is_visible().unwrap_or(true) {
                            let _ = window_clone.app_handle().notification()
                                .builder()
                                .title("FRP客户端")
                                .body("FRP客户端已最小化到托盘")
                                .show();
                        }
                    }
                    tauri::WindowEvent::Moved { .. } => {
                        // 处理拖动事件
                    }
                    _ => {}
                }
            });
            
            #[cfg(target_os = "windows")]
            window.set_ignore_cursor_events(false).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error in running tauri application");
}

fn create_tray_menu(app: &tauri::App) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?,
            &MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?,
        ],
    )?;

    let _app_handle = app.handle().clone();
    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(move |tray, event| {
            if let tauri::tray::TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                // 设置退出标志
                *app.state::<Mutex<bool>>().lock().unwrap() = true;
                kill_all_processes().unwrap();
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(tray)
}
