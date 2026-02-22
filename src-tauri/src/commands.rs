use tauri::Manager;
use tauri::Runtime;
use tauri::Emitter;
use tauri::command;
use std::fs::{File, self};
use std::sync::Mutex;
use crate::config;
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
    let _ = kill_all_processes(vec!["frpc.exe".to_string(), "easytire-cli.exe".to_string()]);
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
    //let icon_path = window.app_handle().path().resolve("icons/icon.png", BaseDirectory::Resource).unwrap();
    // let _ = window.app_handle().notification()
    //     .builder()
    //     .title("LingYunFRP")
    //     .body("LingYunFRP客户端已最小化到托盘")
    //     .icon(icon_path.to_string_lossy())
    //     .show();
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
pub fn kill_all_processes(processes: Vec<String>) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        
        for process in processes {
            let output = std::process::Command::new("taskkill")
                .arg("/F")
                .arg("/IM")
                .arg(&process)
                .creation_flags(0x08000000)
                .output();
            
            if let Ok(output) = output {
                if !output.status.success() {
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
        for process in processes {
            let process_name = process.strip_suffix(".exe").unwrap_or(&process);
            let output = std::process::Command::new("killall")
                .arg(process_name)
                .output();
            
            if let Ok(output) = output {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.contains("No matching processes") {
                        eprintln!("终止进程 {} 失败: {}", process_name, stderr);
                    }
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        for process in processes {
            let process_name = process.strip_suffix(".exe").unwrap_or(&process);
            let output = std::process::Command::new("pkill")
                .arg("-f")
                .arg(process_name)
                .output();
            
            if let Ok(output) = output {
                if !output.status.success() {
                    if output.status.code() != Some(1) {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("终止进程 {} 失败: {}", process_name, stderr);
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

#[command]
pub async fn delete_file<R: Runtime>(
    app: tauri::AppHandle<R>,
    file_name: String,
) -> Result<bool, String> {
    // 解析到 app_data_dir
    let mut path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(&file_name);

    // 标准化路径，防止 “../../../etc/passwd” 之类攻击
    path = path.canonicalize().map_err(|_| "文件不存在".to_string())?;

    // 必须仍在 app_data_dir 之内
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .canonicalize()
        .map_err(|_| "无法获取应用目录")?;
    if !path.starts_with(&app_data_dir) {
        return Err("非法路径".to_string());
    }

    // 确认是普通文件再删除
    if !path.is_file() {
        return Err("路径不是文件".to_string());
    }

    tokio::fs::remove_file(&path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(true)
}

#[tauri::command]
pub async fn auto_update<R: Runtime>(
    app: tauri::AppHandle<R>,
    download_url: String,
    file_name: String,
) -> Result<String, String> {
    use tokio::io::AsyncWriteExt;
    use reqwest::Client;
    use futures::StreamExt;

    let save_path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(&file_name);

    if let Some(parent) = save_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }

    let _ = app.emit("update-download-start", serde_json::json!({
        "url": download_url,
        "file_name": file_name
    }));

    let client = Client::new();
    let res = client.get(&download_url).send().await.map_err(|e| e.to_string())?;

    let total = res.content_length();
    let mut stream = res.bytes_stream();
    let mut file = tokio::fs::File::create(&save_path)
        .await
        .map_err(|e| e.to_string())?;

    let mut downloaded: u64 = 0;
    const REPORT_INTERVAL: u64 = 512 * 1024;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;

        downloaded += chunk.len() as u64;
        if downloaded % REPORT_INTERVAL < chunk.len() as u64 {
            let _ = app.emit("update-download-progress", serde_json::json!({
                "downloaded": downloaded,
                "total": total,
                "percentage": if let Some(total) = total {
                    (downloaded as f64 / total as f64 * 100.0) as u32
                } else {
                    0
                }
            }));
        }
    }

    let _ = app.emit("update-download-complete", serde_json::json!({
        "file_path": save_path.to_string_lossy().to_string()
    }));

    Ok(save_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn install_and_restart<R: Runtime>(
    app: tauri::AppHandle<R>,
    installer_path: String,
) -> Result<(), String> {
    use std::process::Command;

    let _ = app.emit("update-install-start", serde_json::json!({}));

    #[cfg(target_os = "windows")]
    {
        let installer_path = std::path::Path::new(&installer_path);
        
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("获取当前程序路径失败: {}", e))?;
        
        let install_dir = current_exe.parent()
            .ok_or("无法获取安装目录")?;
        
        let mut cmd = Command::new(installer_path);
        cmd.arg("/S");
        cmd.arg(format!("/D={}", install_dir.to_string_lossy()));
        
        let _ = cmd.spawn()
            .map_err(|e| format!("启动安装程序失败: {}", e))?;
        
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        app.exit(0);
    }

    #[cfg(target_os = "macos")]
    {
        use std::fs;
        
        let installer_path = std::path::Path::new(&installer_path);
        
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("获取当前程序路径失败: {}", e))?;
        
        let applications_dir = std::path::Path::new("/Applications");
        
        let output = Command::new("hdiutil")
            .arg("attach")
            .arg(installer_path)
            .output()
            .map_err(|e| format!("挂载 DMG 失败: {}", e))?;
        
        if !output.status.success() {
            return Err(format!("挂载 DMG 失败: {}", String::from_utf8_lossy(&output.stderr)));
        }
        
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        let mount_point = std::path::Path::new("/Volumes");
        if let Ok(entries) = fs::read_dir(mount_point) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("app") {
                    let app_name = path.file_name().unwrap();
                    let dest_path = applications_dir.join(app_name);
                    
                    if dest_path.exists() {
                        let _ = Command::new("rm")
                            .arg("-rf")
                            .arg(&dest_path)
                            .output();
                    }
                    
                    let _ = Command::new("cp")
                        .arg("-R")
                        .arg(&path)
                        .arg(applications_dir)
                        .output();
                    
                    break;
                }
            }
        }
        
        let _ = Command::new("hdiutil")
            .arg("detach")
            .arg("/Volumes")
            .output();
        
        let current_app_name = current_exe
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .ok_or("无法获取应用名称")?;
        
        let new_app_path = applications_dir.join(current_app_name);
        
        let _ = Command::new("open")
            .arg(&new_app_path)
            .spawn();
        
        app.exit(0);
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        let installer_path = std::path::Path::new(&installer_path);
        
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("获取当前程序路径失败: {}", e))?;
        
        if installer_path.extension().and_then(|s| s.to_str()) == Some("AppImage") {
            let _ = fs::copy(installer_path, &current_exe)
                .map_err(|e| format!("复制 AppImage 失败: {}", e))?;
            
            let _ = Command::new("chmod")
                .arg("+x")
                .arg(&current_exe)
                .output();
            
            let _ = Command::new(&current_exe)
                .spawn();
            
            app.exit(0);
        } else if installer_path.extension().and_then(|s| s.to_str()) == Some("deb") {
            let output = Command::new("pkexec")
                .arg("dpkg")
                .arg("-i")
                .arg(installer_path)
                .output()
                .map_err(|e| format!("安装 deb 包失败: {}", e))?;
            
            if !output.status.success() {
                return Err(format!("安装 deb 包失败: {}", String::from_utf8_lossy(&output.stderr)));
            }
            
            let _ = Command::new(&current_exe)
                .spawn();
            
            app.exit(0);
        } else {
            return Err("不支持的安装包格式".to_string());
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        return Err("当前平台不支持自动更新".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn extract_zip<R: Runtime>(
    _app: tauri::AppHandle<R>,
    zip_path: String,
    extract_to: String,
    inner_folder_name: Option<String>,
    rename_to: Option<String>,
) -> Result<(), String> {
    use std::path::Path;
    
    let extract_path = Path::new(&extract_to);
    
    // 1. 解压 zip 文件到临时目录
    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipArchive::new(file)
        .map_err(|e| e.to_string())?;
    zip.extract(&extract_to).map_err(|e| e.to_string())?;
    
    // 2. 确定最终目标目录（zip 文件所在目录 + 重命名后的文件夹名）
    let zip_path_obj = Path::new(&zip_path);
    let parent_dir = zip_path_obj.parent().unwrap_or(Path::new("."));
    let final_target_name = rename_to.clone().unwrap_or_else(|| {
        inner_folder_name.clone().unwrap_or_else(|| "extracted".to_string())
    });
    let final_target_path = parent_dir.join(&final_target_name);
    
    // 3. 如果指定了内层文件夹，将其内容移出到最终目标位置
    if let Some(inner_name) = inner_folder_name {
        // 内层文件夹路径: extract_to/inner_name/easytier/
        let inner_path = extract_path.join(&inner_name);
        
        if inner_path.exists() && inner_path.is_dir() {
            // 查找内层文件夹中的子文件夹（如 easytier）
            let mut source_folder = inner_path.clone();
            
            // 如果内层文件夹中只有一个子文件夹，进入它
            let entries: Vec<_> = fs::read_dir(&inner_path)
                .map_err(|e| e.to_string())?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .collect();
            
            if entries.len() == 1 {
                source_folder = entries[0].path();
            }
            
            // 如果最终目标已存在，先删除
            if final_target_path.exists() {
                fs::remove_dir_all(&final_target_path).map_err(|e| e.to_string())?;
            }
            
            // 创建最终目标文件夹
            fs::create_dir_all(&final_target_path).map_err(|e| e.to_string())?;
            
            // 将源文件夹的内容移动到最终目标位置
            for entry in fs::read_dir(&source_folder).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let src = entry.path();
                let dest = final_target_path.join(entry.file_name());
                
                if src.is_dir() {
                    // 递归复制文件夹
                    copy_dir_recursive(&src, &dest).map_err(|e| e.to_string())?;
                    fs::remove_dir_all(&src).map_err(|e| e.to_string())?;
                } else {
                    fs::rename(&src, &dest).map_err(|e| e.to_string())?;
                }
            }
            
            // 删除临时解压目录
            fs::remove_dir_all(&extract_path).map_err(|e| e.to_string())?;
        }
    }
    
    // 4. 删除 zip 文件
    fs::remove_file(&zip_path).map_err(|e| e.to_string())?;
    
    Ok(())
}

// 递归复制文件夹
fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dest)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    
    Ok(())
}

/// TCPing - 测试 TCP 连接延迟
/// 参数: host - 主机地址(域名或IP), port - 端口号, timeout_ms - 超时时间(毫秒)
/// 返回: 延迟时间(毫秒), 如果连接失败返回错误信息
#[tauri::command]
pub async fn tcping(host: String, port: u16, timeout_ms: u64) -> Result<u64, String> {
    use std::net::TcpStream;
    use std::time::{Duration, Instant};
    use std::net::ToSocketAddrs;
    
    // 解析地址
    let addr = format!("{}:{}", host, port);
    let socket_addrs: Vec<_> = addr.to_socket_addrs()
        .map_err(|e| format!("地址解析失败: {}", e))?
        .collect();
    
    if socket_addrs.is_empty() {
        return Err("无法解析地址".to_string());
    }
    
    let socket_addr = &socket_addrs[0];
    let timeout = Duration::from_millis(timeout_ms);
    
    // 记录开始时间
    let start = Instant::now();
    
    // 尝试连接
    match TcpStream::connect_timeout(socket_addr, timeout) {
        Ok(_) => {
            let elapsed = start.elapsed();
            Ok(elapsed.as_millis() as u64)
        }
        Err(e) => Err(format!("连接失败: {}", e)),
    }
}