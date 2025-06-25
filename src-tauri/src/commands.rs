use tauri::Manager;
use tauri::Runtime;
use tauri::Emitter;
use tauri::command;
use std::collections::HashMap;
use std::sync::Mutex;
use std::io::BufRead;
use crate::config;

#[tauri::command]
pub fn close_window(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
pub fn quit_window(window: tauri::Window, app: tauri::AppHandle) {
    *app.state::<Mutex<bool>>().lock().unwrap() = true;
    let _ = kill_all_processes();
    let _ = window.close();
    app.exit(0);
}

#[tauri::command]
pub fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
pub fn toggle_maximize(window: tauri::Window) {
    let is_maximized = window.is_maximized().unwrap();
    if is_maximized {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
pub fn hide_to_tray(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
pub fn check_frpc_exists(app: tauri::AppHandle) -> bool {
    let app_data_dir = app.path().app_data_dir().unwrap();
    let frpc_path = app_data_dir.join("frpc.exe");
    if frpc_path.exists() {
        return true;
    }
    if let Ok(exe_dir) = std::env::current_exe() {
        let exe_path = exe_dir.parent().unwrap().join("frpc.exe");
        if exe_path.exists() {
            return true;
        }
    }
    false
}

#[tauri::command]
pub async fn api_url() -> String {
    config::api_url().to_string()
}

#[command]
pub async fn emit_event<R: Runtime>(
    app: tauri::AppHandle<R>,
    event: String,
    payload: serde_json::Value,
) -> Result<(), String> {
    app.emit(&event, payload)
        .map_err(|e| format!("发送事件失败: {}", e))
}

#[tauri::command]
pub async fn get_app_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let app_data_dir_str = app_data_dir.to_str().ok_or("路径转换失败")?.to_string();
    Ok(app_data_dir_str)
}

#[tauri::command]
pub async fn open_app_data_dir(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    std::process::Command::new("explorer")
        .arg(app_data_dir)
        .spawn()
        .map_err(|e| format!("打开目录失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn download_frpc(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let frpc_path = app_data_dir.join("frpc.exe");
    if frpc_path.exists() {
        return Err("frpc.exe已存在".to_string());
    }
    let response = reqwest::get(config::api_url() + "/frp/download")
       .await
       .map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    std::fs::write(frpc_path, bytes).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_auto_start(enable: bool) -> Result<(), String> {
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
    #[cfg(target_os = "macos")]
    {
        use std::fs;
        use std::env;
        use std::path::PathBuf;
        let home_dir = env::var("HOME").map_err(|_| "无法获取HOME目录".to_string())?;
        let plist_dir = PathBuf::from(home_dir).join("Library/LaunchAgents");
        let plist_path = plist_dir.join("com.lingyunfrp.autostart.plist");
        let exe_path = env::current_exe().map_err(|e| format!("获取当前程序路径失败: {}", e))?;
        if enable {
            fs::create_dir_all(&plist_dir).map_err(|e| format!("创建LaunchAgents目录失败: {}", e))?;
            let plist_content = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
                <plist version=\"1.0\">\n\
                <dict>\n\
                    <key>Label</key>\n\
                    <string>com.lingyunfrp.autostart</string>\n\
                    <key>ProgramArguments</key>\n\
                    <array>\n\
                        <string>{}</string>\n\
                    </array>\n\
                    <key>RunAtLoad</key>\n\
                    <true/>\n\
                </dict>\n\
                </plist>\n",
                exe_path.to_string_lossy()
            );
            fs::write(&plist_path, plist_content).map_err(|e| format!("写入plist失败: {}", e))?;
        } else {
            if plist_path.exists() {
                fs::remove_file(&plist_path).map_err(|e| format!("删除plist失败: {}", e))?;
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        use std::env;
        use std::path::PathBuf;
        let home_dir = env::var("HOME").map_err(|_| "无法获取HOME目录".to_string())?;
        let autostart_dir = PathBuf::from(home_dir).join(".config/autostart");
        let desktop_path = autostart_dir.join("lingyunfrp.desktop");
        let exe_path = env::current_exe().map_err(|e| format!("获取当前程序路径失败: {}", e))?;
        if enable {
            fs::create_dir_all(&autostart_dir).map_err(|e| format!("创建autostart目录失败: {}", e))?;
            let desktop_content = format!(
                "[Desktop Entry]\nType=Application\nName=LingYunFrp\nExec=\"{}\"\nX-GNOME-Autostart-enabled=true\n",
                exe_path.to_string_lossy()
            );
            fs::write(&desktop_path, desktop_content).map_err(|e| format!("写入desktop文件失败: {}", e))?;
        } else {
            if desktop_path.exists() {
                fs::remove_file(&desktop_path).map_err(|e| format!("删除desktop文件失败: {}", e))?;
            }
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        return Err("当前仅支持Windows、macOS、Linux系统的开机自启动".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn get_cpl_version() -> String {
    config::version().to_string()
}

#[tauri::command]
pub fn kill_all_processes() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let _output = std::process::Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("frpc.exe")
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()
            .map_err(|e| format!("终止进程失败: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        let _output = std::process::Command::new("killall")
            .arg("frpc")
            .output()
            .map_err(|e| format!("终止进程失败: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        let _output = std::process::Command::new("pkill")
            .arg("-f")
            .arg("frpc")
            .output()
            .map_err(|e| format!("终止进程失败: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn start_proxy(
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
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let mut child = command.spawn().map_err(|e| format!("启动隧道失败: {}", e))?;
    let app_handle = app.clone();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
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
    let app_handle_err = app.clone();
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = app_handle_err.emit(
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
    let _ = app.emit(
        "tunnel-event",
        serde_json::json!({
            "type": "start",
            "tunnelId": proxy_id,
            "message": format!("隧道 #{} 启动进程", proxy_id)
        }),
    );
    let _ = app.emit(
        "log",
        serde_json::json!({
            "message": format!("[FRPC] 启动进程 PID: {}", child.id())
        }),
    );
    app.state::<Mutex<HashMap<u32, std::process::Child>>>()
        .lock()
        .unwrap()
        .insert(proxy_id, child);
    Ok(true)
}

#[tauri::command]
pub async fn stop_proxy(app: tauri::AppHandle, proxy_id: u32) -> Result<bool, String> {
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
pub fn get_frpc_cli_version(app: tauri::AppHandle) -> Result<String, String> {
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
    let frpc_path_clone = frpc_path.clone();
    let output = std::process::Command::new(frpc_path)
        .arg("--version")
        .output()
        .map_err(|e| format!("执行失败: {}", e))?;
    let version = String::from_utf8(output.stdout)
        .map(|v| v.trim().replace(['\r', '\n'], ""))
        .map_err(|e| format!("编码错误: {}", e))?;
    Ok(serde_json::json!({
        "code": 0,
        "version": version,
        "path": frpc_path_clone.to_string_lossy().replace('\\', "\\\\")
    }).to_string())
}

#[tauri::command]
pub async fn get_client_version() -> String {
    config::version().to_string()
}

#[tauri::command]
pub fn open_url(url: String) {
    if let Err(e) = open::that(&url) {
        eprintln!("打开浏览器失败: {}", e);
    }
}

#[tauri::command]
pub async fn forward_request(
    url: String,
    method: String,
    data: serde_json::Value,
    headers: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let api_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        config::api_url().to_string() + url.trim_start_matches('/')
    };
    let mut request_builder = match method.to_uppercase().as_str() {
        "POST" => client.post(&api_url),
        "GET" => client.get(&api_url),
        _ => return Err("不支持的请求方法".to_string()),
    };
    if let Some(headers_map) = headers.as_object() {
        for (key, value) in headers_map {
            if let Some(value_str) = value.as_str() {
                request_builder = request_builder.header(key, value_str);
            }
        }
    }
    let response = match method.to_uppercase().as_str() {
        "POST" => {
            request_builder.form(&data).send().await
        },
        "GET" => request_builder.send().await,
        _ => return Err("不支持的请求方法".to_string()),
    }.map_err(|e| e.to_string())?;
    let response_text = response.text().await.map_err(|e| e.to_string())?;
    let cleaned_text = response_text
        .trim_start_matches('\u{FEFF}')
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty()) 
        .collect::<Vec<&str>>() 
        .join("");
    let response_json = serde_json::from_str(&cleaned_text)
        .map_err(|e| format!("JSON解析错误: {} - 原始文本: {}", e, cleaned_text))?;
    Ok(response_json)
}

#[tauri::command]
pub fn get_now_mode() -> bool {
    config::debug()
}

#[tauri::command]
pub fn get_system_info() -> String {
    let system = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    };
    let arch = if cfg!(target_arch = "x86") {
        "386"
    } else if cfg!(target_arch = "x86_64") {
        "amd64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else {
        "unknown"
    };
    format!("{} {}", system, arch)
} 

#[tauri::command]
pub fn get_api_url() -> String {
    config::api_url().to_string()
}