use tauri::Manager;
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

static ACTIVE_NETWORK: Mutex<Option<String>> = Mutex::new(None);

fn get_active_network_file(app: &tauri::AppHandle) -> PathBuf {
    let app_data_dir = app.path().app_data_dir().unwrap();
    app_data_dir.join("active_network.txt")
}

fn save_active_network(app: &tauri::AppHandle, network_id: &str) {
    let file_path = get_active_network_file(app);
    if let Err(e) = fs::write(&file_path, network_id) {
        eprintln!("保存活动网络ID失败: {}", e);
    }
}

fn load_active_network(app: &tauri::AppHandle) -> Option<String> {
    let file_path = get_active_network_file(app);
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let network_id = content.trim().to_string();
            if !network_id.is_empty() {
                Some(network_id)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn clear_active_network(app: &tauri::AppHandle) {
    let file_path = get_active_network_file(app);
    let _ = fs::remove_file(&file_path);
}

#[tauri::command]
pub async fn check_easy_tire_exists(app: tauri::AppHandle) -> bool {
    let executable_name = if cfg!(target_os = "windows") {
        "easytier-core.exe"
    } else {
        "easytier-core"
    };
    
    let folder_name = format!("easytier");
    
    let app_data_dir = app.path().app_data_dir().unwrap();
    let frpc_path = app_data_dir.join(&folder_name).join(executable_name);
    if frpc_path.exists() {
        return true;
    }
    if let Ok(exe_dir) = std::env::current_exe() {
        let exe_path = exe_dir.parent().unwrap().join(&folder_name).join(executable_name);
        if exe_path.exists() {
            return true;
        }
    }
    false
}

#[tauri::command]
pub async fn start_easytire(app: tauri::AppHandle, name: String, password: String, id: String, local_ip: String) -> Result<Vec<String>, String> {
    let executable_name = if cfg!(target_os = "windows") {
        "easytier-core.exe"
    } else {
        "easytier-core"
    };
    
    let folder_name = format!("easytier");
    
    let app_data_dir = app.path().app_data_dir().unwrap();
    let easytier_path = app_data_dir.join(&folder_name).join(&executable_name);
    
    let exe_path = if easytier_path.exists() {
        easytier_path
    } else if let Ok(exe_dir) = std::env::current_exe() {
        exe_dir.parent().unwrap().join(&folder_name).join(&executable_name)
    } else {
        return Err(format!("找不到easytier-core可执行文件"));
    };

    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("powershell");
        cmd.args(&[
            "-WindowStyle", "Hidden",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &format!(
                "Start-Process '{}' -ArgumentList '-i', '{}/24', '--network-name', '{}', '--network-secret', '{}', '-p', 'tcp://public.easytier.cn:11010' -Verb RunAs -WindowStyle Hidden",
                exe_path.to_string_lossy(),
                local_ip,
                name,
                password
            )
        ]);
        cmd.creation_flags(CREATE_NO_WINDOW);
        
        let output = cmd.output().map_err(|e| format!("启动easytier失败: {}", e))?;
        if output.status.success() {
            *ACTIVE_NETWORK.lock().unwrap() = Some(id.clone());
            save_active_network(&app, &id);
            Ok(vec!["true".to_string()])
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let _stdout = String::from_utf8_lossy(&output.stdout);
            Err(format!("启动easytier失败: {}", stderr))
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new("sudo");
        cmd.args(&[
            &exe_path.to_string_lossy(),
            "-i", &format!("{}/24", local_ip).into(),
            "--network-name", &name,
            "--network-secret", &password,
            "-p", "tcp://public.easytier.cn:11010",
        ]);
        use std::process::Stdio;
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
        cmd.stdin(Stdio::null());
        
        let child = cmd.spawn().map_err(|e| format!("启动easytier失败: {}", e))?;
        if child.id() > 0 {
            *ACTIVE_NETWORK.lock().unwrap() = Some(id.clone());
            save_active_network(&app, &id);
            Ok(vec!["true".to_string()])
        } else {
            Err(format!("启动easytier失败"))
        }
    }
}

#[tauri::command]
pub async fn stop_easytire(app: tauri::AppHandle, _network_id: String) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        let mut check_cmd = std::process::Command::new("powershell");
        check_cmd.args(&[
            "-WindowStyle", "Hidden",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "Get-Process -Name easytier-core -ErrorAction SilentlyContinue | Select-Object -First 1 | ForEach-Object { $_.Id }"
        ]);
        check_cmd.creation_flags(CREATE_NO_WINDOW);
        
        let check_output = check_cmd.output().map_err(|e| format!("检查进程失败: {}", e))?;
        let check_stdout = String::from_utf8_lossy(&check_output.stdout);
        
        if check_stdout.trim().is_empty() {
            *ACTIVE_NETWORK.lock().unwrap() = None;
            clear_active_network(&app);
            return Ok(true);
        }
        
        let mut cmd = std::process::Command::new("powershell");
        cmd.args(&[
            "-WindowStyle", "Hidden",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "Start-Process taskkill -ArgumentList '/F', '/IM', 'easytier-core.exe' -Verb RunAs -WindowStyle Hidden"
        ]);
        cmd.creation_flags(CREATE_NO_WINDOW);
        
        let _output = cmd.output().map_err(|e| format!("停止easytier失败: {}", e))?;
        
        *ACTIVE_NETWORK.lock().unwrap() = None;
        clear_active_network(&app);
        Ok(true)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new("pkill");
        cmd.args(&["-9", "easytier-core"]);
        
        let output = cmd.output().map_err(|e| format!("停止easytier失败: {}", e))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        eprintln!("pkill stdout: {}", stdout);
        eprintln!("pkill stderr: {}", stderr);
        
        if output.status.success() || stdout.is_empty() {
            *ACTIVE_NETWORK.lock().unwrap() = None;
            clear_active_network(&app);
            Ok(true)
        } else {
            Err(format!("停止easytier失败: {}", stderr))
        }
    }
}

#[tauri::command]
pub async fn get_active_easytire(app: tauri::AppHandle) -> Option<String> {
    let memory_network = ACTIVE_NETWORK.lock().unwrap().clone();
    if memory_network.is_some() {
        memory_network
    } else {
        load_active_network(&app)
    }
}

#[tauri::command]
pub async fn check_easytire_process_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("powershell");
        cmd.args(&[
            "-WindowStyle", "Hidden",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "Get-Process -Name easytier-core -ErrorAction SilentlyContinue | Select-Object -First 1 | ForEach-Object { $_.Id }"
        ]);
        cmd.creation_flags(CREATE_NO_WINDOW);
        
        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let trimmed = stdout.trim();
                !trimmed.is_empty()
            }
            Err(_) => false,
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new("pgrep");
        cmd.args(&["-x", "easytier-core"]);
        
        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let trimmed = stdout.trim();
                !trimmed.is_empty()
            }
            Err(_) => false,
        }
    }
}

