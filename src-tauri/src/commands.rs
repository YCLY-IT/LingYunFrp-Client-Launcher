use tauri::Manager;
use tauri::Runtime;
use tauri::Emitter;
use tauri::command;
use std::sync::Mutex;
use crate::config;
use tauri::path::BaseDirectory;
use tauri_plugin_notification::NotificationExt;
use std::path::Path;

#[tauri::command]
pub fn close_window(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
pub fn quit_window(window: tauri::Window, app: tauri::AppHandle, is_keep: bool) {
    *app.state::<Mutex<bool>>().lock().unwrap() = true;
    if is_keep {
        let _ = window.close();
        let app_clone = app.clone();
        let _ = window.emit("before-quit", ());
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(1));
            app_clone.exit(0);
        });
        return;
    }
    let _ = kill_all_processes();
    let _ = window.close();
    let app_clone = app.clone();
    let _ = window.emit("before-quit", ());
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        app_clone.exit(0);
    });
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
    let icon_path = window.app_handle().path().resolve("icons/icon.png", BaseDirectory::Resource).unwrap();
    let _ = window.app_handle().notification()
        .builder()
        .title("LingYunFRP")
        .body("LingYunFRP客户端已最小化到托盘")
        .icon(icon_path.to_string_lossy())
        .show();
}

#[tauri::command]
pub fn check_frpc_exists(app: tauri::AppHandle) -> bool {
    // 根据操作系统确定可执行文件名
    let executable_name = if cfg!(target_os = "windows") {
        "frpc.exe"
    } else {
        "frpc"
    };
    
    let app_data_dir = app.path().app_data_dir().unwrap();
    let frpc_path = app_data_dir.join(executable_name);
    if frpc_path.exists() {
        return true;
    }
    if let Ok(exe_dir) = std::env::current_exe() {
        let exe_path = exe_dir.parent().unwrap().join(executable_name);
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
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(app_data_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(app_data_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(app_data_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        return Err("不支持的操作系统".to_string());
    }
    
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
pub fn kill_all_processes() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        
        // 分别终止 frpc.exe 和 natter.exe
        let processes = vec!["frpc.exe", "natter.exe"];
        
        for process in processes {
            let output = std::process::Command::new("taskkill")
                .arg("/F")
                .arg("/IM")
                .arg(process)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output();
            
            // 忽略错误，因为进程可能不存在
            if let Ok(output) = output {
                if !output.status.success() {
                    // 检查是否是因为进程不存在而失败
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.contains("找不到") && !stderr.contains("not found") {
                        eprintln!("终止进程 {} 失败: {}", process, stderr);
                    }
                }
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        let processes = vec!["frpc", "natter"];
        
        for process in processes {
            let output = std::process::Command::new("killall")
                .arg(process)
                .output();
            
            // 忽略错误，因为进程可能不存在
            if let Ok(output) = output {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.contains("No matching processes") {
                        eprintln!("终止进程 {} 失败: {}", process, stderr);
                    }
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        let processes = vec!["frpc", "natter"];
        
        for process in processes {
            let output = std::process::Command::new("pkill")
                .arg("-f")
                .arg(process)
                .output();
            
            // 忽略错误，因为进程可能不存在
            if let Ok(output) = output {
                if !output.status.success() {
                    // pkill 返回 1 表示没有找到匹配的进程，这是正常的
                    if output.status.code() != Some(1) {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("终止进程 {} 失败: {}", process, stderr);
                    }
                }
            }
        }
    }
    Ok(())
}


#[tauri::command]
pub async fn get_frpc_cli_version(app: tauri::AppHandle) -> Result<String, String> {
    // 根据操作系统确定可执行文件名
    let executable_name = if cfg!(target_os = "windows") {
        "frpc.exe"
    } else {
        "frpc"
    };
    
    let frpc_path = {
        let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        let data_path = app_data_dir.join(executable_name);
        if data_path.exists() {
            data_path
        } else if let Ok(exe_dir) = std::env::current_exe() {
            let exe_path = exe_dir.parent()
                .ok_or("无法获取父目录")?
                .join(executable_name);
            exe_path
        } else {
            return Err("frpc executable not found".into());
        }
    };
    let frpc_path_clone = frpc_path.clone();
    
    // 使用 tokio::spawn 在线程池中异步执行命令
    let output = tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            std::process::Command::new(frpc_path)
                .arg("--version")
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output()
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new(frpc_path)
                .arg("--version")
                .output()
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new(frpc_path)
                .arg("--version")
                .output()
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            std::process::Command::new(frpc_path)
                .arg("--version")
                .output()
        }
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
    .map_err(|e| format!("执行失败: {}", e))?;
    
    let version = String::from_utf8(output.stdout)
        .map(|v| v.trim().replace(['\r', '\n'], ""))
        .map_err(|e| format!("编码错误: {}", e))?;
    
    // 根据操作系统处理路径分隔符
    let path_str = if cfg!(target_os = "windows") {
        frpc_path_clone.to_string_lossy().replace('\\', "\\\\")
    } else {
        frpc_path_clone.to_string_lossy().to_string()
    };
    
    Ok(serde_json::json!({
        "code": 0,
        "version": version,
        "path": path_str
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

#[tauri::command]
pub async fn check_auto_start_status() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::{RegKey, enums::HKEY_CURRENT_USER, enums::KEY_READ};
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
        match hkcu.open_subkey_with_flags(path, KEY_READ) {
            Ok(key) => {
                let app_name = "LingYunFrp";
                let exe_path = std::env::current_exe()
                    .map_err(|e| format!("获取当前程序路径失败: {}", e))?
                    .to_string_lossy()
                    .into_owned();
                match key.get_value::<String, _>(app_name) {
                    Ok(reg_path) => Ok(reg_path == exe_path),
                    Err(_) => Ok(false),
                }
            }
            Err(_) => Ok(false),
        }
    }
    #[cfg(target_os = "macos")]
    {
        use std::env;
        use std::path::PathBuf;
        let home_dir = env::var("HOME").map_err(|_| "无法获取HOME目录".to_string())?;
        let plist_path = PathBuf::from(home_dir).join("Library/LaunchAgents/cn.lyfrp.autostart.plist");
        Ok(plist_path.exists())
    }
    #[cfg(target_os = "linux")]
    {
        use std::env;
        use std::path::PathBuf;
        let home_dir = env::var("HOME").map_err(|_| "无法获取HOME目录".to_string())?;
        let desktop_path = PathBuf::from(home_dir).join(".config/autostart/cn.lyfrp.desktop");
        Ok(desktop_path.exists())
    }
}



/// 检查软件数据目录中是否存在指定文件
fn check_software_exists(app: &tauri::AppHandle, software_name: &str) -> Result<String, String> {
    // 根据操作系统确定可执行文件名
    let executable_name = if cfg!(target_os = "windows") {
        if !software_name.ends_with(".exe") {
            format!("{}.exe", software_name)
        } else {
            software_name.to_string()
        }
    } else {
        software_name.to_string()
    };
    
    // 首先检查应用数据目录
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        let software_path = app_data_dir.join(&executable_name);
        if Path::new(&software_path).exists() {
            return Ok(software_path.to_string_lossy().to_string());
        }
    }
    
    // 然后检查当前可执行文件所在目录
    if let Ok(exe_dir) = std::env::current_exe() {
        if let Some(parent_dir) = exe_dir.parent() {
            let software_path = parent_dir.join(&executable_name);
            if Path::new(&software_path).exists() {
                return Ok(software_path.to_string_lossy().to_string());
            }
        }
    }
    
    Err(format!("软件文件不存在: {}", executable_name))
}

/// 检查软件文件是否存在
#[tauri::command]
pub fn check_software_file(app: tauri::AppHandle, software_name: Option<String>) -> Result<(bool, String), String> {
    let software_name = software_name.unwrap_or_else(|| "frpc".to_string());
    
    match check_software_exists(&app, &software_name) {
        Ok(path) => Ok((true, path)),
        Err(error) => Ok((false, error))
    }
}