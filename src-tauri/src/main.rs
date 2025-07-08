#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_single_instance::init as single_instance_init;
use tauri::Listener;
mod config;
mod commands;
mod virtual_net;


use virtual_net::{
    create_virtual_network,
    join_virtual_network,
    leave_virtual_network,
    get_current_virtual_network,
    start_virtual_network,
    stop_virtual_network,
    auto_connect_peers,
    is_tap_driver_installed,
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
    download_frpc,
    toggle_auto_start,
    kill_all_processes,
    get_client_version,
    start_proxy,
    stop_proxy,
    quit_window,
    open_url,
    api_url,
    forward_request,
    get_now_mode,
    get_system_info,
    get_api_url,
    is_admin,
    check_auto_start_status,
    get_bing_wallpaper_base64,
};
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;

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
        get_client_version,
        start_proxy,
        stop_proxy,
        quit_window,
        open_url,
        api_url,
        forward_request,
        get_now_mode,
        get_system_info,
        get_api_url,
        create_virtual_network,
        join_virtual_network,
        leave_virtual_network,
        get_current_virtual_network,
        start_virtual_network,
        stop_virtual_network,
        auto_connect_peers,
        is_tap_driver_installed,
        is_admin,
        check_auto_start_status,
        get_bing_wallpaper_base64,
    ])
    .setup(|app| {
        // 确保应用数据目录存在
        let app_data_dir = app.path().app_data_dir().unwrap();
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir).unwrap();
        }
        
        let auto_start_enabled = tauri::async_runtime::block_on(check_auto_start_status()).unwrap_or(false);
        let tray = create_tray_menu(app, auto_start_enabled)?;
        app.manage(tray);
        let window = app.get_webview_window("main").unwrap();
        window.set_decorations(false).unwrap();
        
        // 修改后的窗口事件处理
        window.on_window_event(move |_event| {
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                use std::env;
                use std::process::Command;
                if nix::unistd::Uid::effective().is_root() {
                    // 已是 root
                    return;
                }
                let exe = env::current_exe().unwrap();
                let args: Vec<String> = env::args().skip(1).collect();
                let mut cmd = Command::new("sudo");
                cmd.arg(exe);
                for arg in args { cmd.arg(arg); }
                let _status = cmd.status().expect("无法请求 sudo 权限");
                std::process::exit(0);
            }
        });
        
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
                            .title("LingYunFRP客户端")
                            .body("LingYunFRP客户端已最小化到托盘")
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

        // 监听主窗口事件
        if let Some(main_window) = app.app_handle().get_webview_window("main") {
            main_window.listen("request_admin", |_event| {
                #[cfg(target_os = "windows")]
                {
                    run_as_admin();
                }
                #[cfg(any(target_os = "macos", target_os = "linux"))]
                {
                    run_as_admin();
                }
            });
        }

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error in running tauri application");
}

fn create_tray_menu(app: &tauri::App, _auto_start_enabled: bool) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?,
            &MenuItem::with_id(app, "hide", "隐藏主窗口", true, None::<&str>)?,
            &MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?,
            &MenuItem::with_id(app, "auto_start", "开/关闭自启", true, None::<&str>)?,
            &MenuItem::with_id(app, "app_data_dir", "打开数据目录", true, None::<&str>)?,
            &MenuItem::with_id(app, "quit_without_frpc", "退出不关闭FRPC", true, None::<&str>)?,
            &MenuItem::with_id(app, "quit", "完全退出", true, None::<&str>)?,
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
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.eval("window.location.href = '/dashboard/settings'");
                }
            }
            "auto_start" => {
                let app_clone = app.clone();
                std::thread::spawn(move || {
                    if let Ok(current_status) = tauri::async_runtime::block_on(check_auto_start_status()) {
                        let new_status = !current_status;
                        if let Err(e) = tauri::async_runtime::block_on(toggle_auto_start(new_status)) {
                            eprintln!("切换开机自启失败: {}", e);
                        } else {
                            if let Some(window) = app_clone.get_webview_window("main") {
                                let _ = window.app_handle().notification()
                                    .builder()
                                    .title("LingYunFRP")
                                    .body(if new_status { 
                                        "开机自启已开启 ✓" 
                                    } else { 
                                        "开机自启已关闭 ✗" 
                                    })
                                    .show();
                            }
                            // 状态已通过通知告知用户
                        }
                    } else {
                        eprintln!("检查开机自启状态失败");
                    }
                });
            }
            "app_data_dir" => {
                let app_clone = app.clone();
                std::thread::spawn(move || {
                    if let Err(e) = tauri::async_runtime::block_on(open_app_data_dir(app_clone)) {
                        eprintln!("打开数据目录失败: {}", e);
                    }
                });
            }
            "quit_without_frpc" => {
                app.exit(0);
            }
            "quit" => {
                *app.state::<Mutex<bool>>().lock().unwrap() = true;
                if let Err(e) = kill_all_processes() {
                    eprintln!("关闭进程失败: {}", e);
                }
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(tray)
}







#[cfg(target_os = "windows")]
fn run_as_admin() {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::UI::Shell::{ShellExecuteW};
    use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    let exe = std::env::current_exe().unwrap();
    let exe_wide: Vec<u16> = OsStr::new(exe.to_str().unwrap()).encode_wide().chain(once(0)).collect();
    let params = "--elevated";
    let params_wide: Vec<u16> = OsStr::new(params).encode_wide().chain(once(0)).collect();

    unsafe {
        ShellExecuteW(
            None,
            PCWSTR::from_raw(wide_null("runas").as_ptr()),
            PCWSTR::from_raw(exe_wide.as_ptr()),
            PCWSTR::from_raw(params_wide.as_ptr()),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        );
    }
    std::process::exit(0);
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn run_as_admin() {
    std::thread::spawn(|| {
        use std::env;
        use std::process::Command;
        if nix::unistd::Uid::effective().is_root() {
            return;
        }
        let exe = env::current_exe().unwrap();
        let args: Vec<String> = env::args().skip(1).collect();
        let mut cmd = Command::new("sudo");
        cmd.arg(exe);
        for arg in args { cmd.arg(arg); }
        let _status = cmd.status().expect("无法请求 sudo 权限");
        std::process::exit(0);
    });
}
#[cfg(target_os = "windows")]
fn wide_null(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}


