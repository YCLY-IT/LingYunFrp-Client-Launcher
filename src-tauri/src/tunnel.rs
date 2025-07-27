use serde_json;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::config;
use tauri::Emitter;
use tauri::Manager;
use std::io::{BufRead};

#[tauri::command]
pub async fn start_proxy(
    app: tauri::AppHandle,
    proxy_id: u32,
    token: String,
) -> Result<bool, String> {
    // 根据操作系统确定可执行文件名
    let executable_name = if cfg!(target_os = "windows") {
        "frpc.exe"
    } else {
        "frpc"
    };
    
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let frpc_path = app_data_dir.join(executable_name);
    if !frpc_path.exists() {
        return Err(format!("{} 不存在", executable_name));
    }
    let mut command = std::process::Command::new(&frpc_path);
    command
        .arg("-t").arg(token)
        .arg("-p").arg(proxy_id.to_string());

    // 这里判断开发环境，追加 -u <api_url>
    if cfg!(debug_assertions) {
        let api_url = config::api_url();
        command.arg("-u").arg(api_url);
    }

    command
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