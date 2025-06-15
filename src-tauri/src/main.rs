// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::Runtime;
use tauri::Emitter;
use tauri::command;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri_plugin_notification::NotificationExt;
use std::io::BufRead;

#[tauri::command]
fn close_window(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
fn quit_window(window: tauri::Window, app: tauri::AppHandle) {
    // 设置退出标志
    *app.state::<Mutex<bool>>().lock().unwrap() = true;
    kill_all_processes().unwrap();
    window.close().unwrap();
    std::process::exit(0);
}

#[tauri::command]
fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn toggle_maximize(window: tauri::Window) {
    let is_maximized = window.is_maximized().unwrap();
    if is_maximized {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
fn hide_to_tray(window: tauri::Window) {
    window.hide().unwrap();
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
    .manage(Mutex::new(HashMap::<u32, std::process::Child>::new()))
    .manage(Mutex::new(false)) // 添加退出状态标志
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_opener::init())
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
            get_now_mode
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

#[tauri::command]
fn check_frpc_exists(app: tauri::AppHandle) -> bool {
    // 1. 检查应用数据目录
    let app_data_dir = app.path().app_data_dir().unwrap();
    let frpc_path = app_data_dir.join("frpc.exe");
    //println!("检查路径1: {:?}", frpc_path);
    
    if frpc_path.exists() {
        return true;
    }

    // 2. 检查程序所在目录
    if let Ok(exe_dir) = std::env::current_exe() {
        let exe_path = exe_dir.parent().unwrap().join("frpc.exe");
        //println!("检查路径2: {:?}", exe_path);
        if exe_path.exists() {
            return true;
        }
    }

    false
}

const API_URL: &str = "http://localhost:8081/";
const VERSION: &str = "1.0.0";
const DEBUG: bool = true;

#[tauri::command]
async fn api_url() -> String {
    API_URL.to_string()
}


#[command]
async fn emit_event<R: Runtime>(
    app: tauri::AppHandle<R>,
    event: String,
    payload: serde_json::Value,
) -> Result<(), String> {
    app.emit(&event, payload)
        .map_err(|e| format!("发送事件失败: {}", e))
}

#[tauri::command]
async fn get_app_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let app_data_dir_str = app_data_dir.to_str().ok_or("路径转换失败")?.to_string();
    Ok(app_data_dir_str)
}

#[tauri::command]
async fn open_app_data_dir(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    std::process::Command::new("explorer")
        .arg(app_data_dir)
        .spawn()
        .map_err(|e| format!("打开目录失败: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn download_frpc(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let frpc_path = app_data_dir.join("frpc.exe");
    if frpc_path.exists() {
        return Err("frpc.exe已存在".to_string());
    }
    let response = reqwest::get(API_URL.to_string() + "frp/download")
       .await
       .map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    std::fs::write(frpc_path, bytes).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn toggle_auto_start(enable: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::{RegKey, enums::HKEY_CURRENT_USER, enums::KEY_WRITE};
        
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
        
        match hkcu.open_subkey_with_flags(path, KEY_WRITE) {
            Ok(key) => {
                let app_name = "LingYunFrp";
                let exe_path = std::env::current_exe()
                    .map_err(|e| format!("获取当前程序路径失败: {}", e))?
                    .to_string_lossy()
                    .into_owned();
                
                if enable {
                    key.set_value(app_name, &exe_path)
                        .map_err(|e| format!("设置自启动失败: {}", e))?;
                } else {
                    key.delete_value(app_name)
                        .map_err(|e| format!("取消自启动失败: {}", e))?;
                }
            }
            Err(e) => return Err(format!("打开注册表失败: {}", e)),
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        return Err("当前仅支持Windows系统的开机自启动".to_string());
    }
    
    Ok(())
}

#[tauri::command]
fn get_cpl_version() -> String {
    VERSION.to_string()
}

#[tauri::command]
fn kill_all_processes() -> Result<(), String> {
    let _output = std::process::Command::new("taskkill")
        .arg("/F")
        .arg("/IM")
        .arg("frpc.exe")
        .output()
        .map_err(|e| format!("终止进程失败: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn start_proxy(
    app: tauri::AppHandle,
    proxy_id: u32,
    token: String,
) -> Result<bool, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let frpc_path = app_data_dir.join("frpc.exe");
    
    if !frpc_path.exists() {
        return Err("frpc.exe 不存在".to_string());
    }

    let mut command = std::process::Command::new(&frpc_path);
    
    command
        .arg("-t").arg(token)
        .arg("-p").arg(proxy_id.to_string())
        .stdout(std::process::Stdio::piped())  // 新增：捕获标准输出
        .stderr(std::process::Stdio::piped());  // 新增：捕获错误输出

    let mut child = command.spawn().map_err(|e| format!("启动隧道失败: {}", e))?;

    // 新增：创建线程读取输出
    let app_handle = app.clone();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    // 处理标准输出
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = app_handle.emit(
                    "tunnel-event",
                    serde_json::json!({
                        "type": "log",
                        "tunnelId": proxy_id,
                        "message": line
                    }),
                );
            }
        }
    });

    // 处理错误输出
    let app_handle_err = app.clone(); // 新增克隆
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = app_handle_err.emit( // 使用克隆后的handle
                    "tunnel-event", 
                    serde_json::json!({
                        "type": "error",
                        "tunnelId": proxy_id,
                        "message": line
                    }),
                );
            }
        }
    });

    // 发送启动日志事件
    let _ = app.emit(
        "tunnel-event",
        serde_json::json!({
            "type": "start",
            "tunnelId": proxy_id,
            "message": format!("隧道 #{} 启动进程", proxy_id)
        }),
    );

    // 发送到前端日志系统
    let _ = app.emit(
        "log",
        serde_json::json!({
            "message": format!("[FRPC] 启动进程 PID: {}", child.id())
        }),
    );

    // 保存进程ID以便后续管理
    app.state::<Mutex<HashMap<u32, std::process::Child>>>()
        .lock()
        .unwrap()
        .insert(proxy_id, child);

    Ok(true)
}

#[tauri::command]
async fn stop_proxy(app: tauri::AppHandle, proxy_id: u32) -> Result<bool, String> {
    let processes = app.state::<Mutex<HashMap<u32, std::process::Child>>>();
    let mut processes = processes.lock().unwrap();
    
    if let Some(mut child) = processes.remove(&proxy_id) {
        child.kill().map_err(|e| format!("停止隧道失败: {}", e))?;
        Ok(true)
    } else {
        Err("未找到对应的隧道进程".to_string())
    }
}

#[tauri::command]
fn get_frpc_cli_version(app: tauri::AppHandle) -> Result<String, String> {
    // 获取frpc可执行文件路径
    let frpc_path = {
        let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        let data_path = app_data_dir.join("frpc.exe");
        
        if data_path.exists() {
            data_path
        } else if let Ok(exe_dir) = std::env::current_exe() {
            let exe_path = exe_dir.parent()
                .ok_or("无法获取父目录")?
                .join("frpc.exe");
            exe_path
        } else {
            return Err("frpc executable not found".into());
        }
    };

    // 克隆路径用于后续使用
    let frpc_path_clone = frpc_path.clone();

    let output = std::process::Command::new(frpc_path)
        .arg("--version")
        .output()
        .map_err(|e| format!("执行失败: {}", e))?;

    // 添加版本变量声明
    let version = String::from_utf8(output.stdout)
        .map(|v| v.trim().replace(['\r', '\n'], ""))
        .map_err(|e| format!("编码错误: {}", e))?;

    // 使用克隆的路径
    Ok(serde_json::json!({
        "code": 0,
        "version": version,
        "path": frpc_path_clone.to_string_lossy().replace('\\', "\\\\")
    }).to_string())
}

#[tauri::command]
async fn get_client_version() -> String {
    VERSION.to_string()
}

#[tauri::command]
fn open_url(url: String) {
    // 使用系统默认浏览器打开URL
    if let Err(e) = open::that(&url) {
        eprintln!("打开浏览器失败: {}", e);
    }
}

#[tauri::command]
async fn forward_request(
    url: String,
    method: String,
    data: serde_json::Value,
    headers: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    // 如果URL已经是完整的URL（以http://或https://开头），则直接使用
    let api_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        API_URL.to_string() + url.trim_start_matches('/')
    };
    
    let mut request_builder = match method.to_uppercase().as_str() {
        "POST" => client.post(&api_url),
        "GET" => client.get(&api_url),
        _ => return Err("不支持的请求方法".to_string()),
    };

    // 添加请求头
    if let Some(headers_map) = headers.as_object() {
        for (key, value) in headers_map {
            if let Some(value_str) = value.as_str() {
                request_builder = request_builder.header(key, value_str);
            }
        }
    }

    // 发送请求
    let response = match method.to_uppercase().as_str() {
        "POST" => {
            request_builder.form(&data).send().await
        },
        "GET" => request_builder.send().await,
        _ => return Err("不支持的请求方法".to_string()),
    }.map_err(|e| e.to_string())?;

    // 获取响应文本
    let response_text = response.text().await.map_err(|e| e.to_string())?;
    
    // 清理响应文本
    let cleaned_text = response_text
        .trim_start_matches('\u{FEFF}')
        .trim() // 移除首尾空白字符
        .lines() // 按行分割
        .filter(|line| !line.trim().is_empty()) 
        .collect::<Vec<&str>>() 
        .join("");

    // 尝试解析JSON
    let response_json = serde_json::from_str(&cleaned_text)
        .map_err(|e| format!("JSON解析错误: {} - 原始文本: {}", e, cleaned_text))?;

    Ok(response_json)
}

#[tauri::command]
fn get_now_mode() -> bool {
    DEBUG
}