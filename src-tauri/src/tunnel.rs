use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::config;
use tauri::Emitter;
use tauri::Manager;
use std::io::{BufRead};
use std::time::Duration;
use tokio::time::timeout;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// 存储隧道启动状态
pub struct TunnelStatus {
    pub status: Mutex<HashMap<u32, Arc<tokio::sync::Notify>>>,
}

impl TunnelStatus {
    pub fn new() -> Self {
        Self {
            status: Mutex::new(HashMap::new()),
        }
    }
}

#[tauri::command]
pub async fn start_proxy(
    app: tauri::AppHandle,
    proxy_id: u32,
    token: String,
) -> Result<u32, String> {
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

    #[cfg(target_os = "windows")]
    command.creation_flags(0x08000000);

    // 这里判断开发环境，追加 -u <api_url>
    if cfg!(debug_assertions) {
        let api_url = config::api_url();
        command.arg("-u").arg(api_url);
    }

    command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let mut child = command.spawn().map_err(|e| format!("启动隧道失败: {}", e))?;
    
    // 创建通知对象用于接收启动状态
    let notify = Arc::new(tokio::sync::Notify::new());
    let notify_stdout = notify.clone();
    let notify_stderr = notify.clone();
    
    // 保存通知对象到状态管理中
    app.state::<TunnelStatus>()
        .status
        .lock()
        .unwrap()
        .insert(proxy_id, notify);
    
    let app_handle = app.clone();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    
    // 启动stdout监控线程
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                // 检测隧道启动成功关键字
                if line.contains("隧道启动成功") || line.contains("tunnel started successfully") {
                    notify_stdout.notify_one();
                }
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
    // 启动stderr监控线程
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                // 检测隧道启动成功关键字（可能在stderr中）
                if line.contains("隧道启动成功") || line.contains("started successfully") {
                    notify_stderr.notify_one();
                }
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
    
    let pid = child.id();
    
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
            "message": format!("[FRPC] 启动 (隧道#{}) 进程 PID: {}",proxy_id, pid)
        }),
    );
    
    // 保存进程
    app.state::<Mutex<HashMap<u32, std::process::Child>>>()
        .lock()
        .unwrap()
        .insert(proxy_id, child);
    
    // 立即返回进程ID
    Ok(pid)
}

#[tauri::command]
pub async fn wait_for_tunnel_start(
    app: tauri::AppHandle,
    proxy_id: u32,
) -> Result<bool, String> {
    // 从状态管理中获取通知对象
    let tunnel_status = app.state::<TunnelStatus>();
    let notify = {
        let status_map = tunnel_status.status.lock().unwrap();
        match status_map.get(&proxy_id) {
            Some(n) => n.clone(),
            None => return Err("未找到对应的隧道状态监控".to_string()),
        }
    };
    
    // 等待隧道启动成功或超时（30秒）
    let wait_result = timeout(Duration::from_secs(30), notify.notified()).await;
    
    // 清理状态
    app.state::<TunnelStatus>()
        .status
        .lock()
        .unwrap()
        .remove(&proxy_id);
    
    match wait_result {
        Ok(()) => {
            let _ = app.emit(
                "log",
                serde_json::json!({
                    "message": format!("[FRPC] 隧道 #{} 启动成功", proxy_id)
                }),
            );
            Ok(true)
        }
        _ => {
            // 超时，返回false
            let _ = app.emit(
                "log",
                serde_json::json!({
                    "message": format!("[FRPC] 隧道 #{} 启动检测超时", proxy_id)
                }),
            );
            Ok(false)
        }
    }
}

#[tauri::command]
pub async fn stop_proxy(app: tauri::AppHandle, proxy_id: u32) -> Result<bool, String> {
    let processes = app.state::<Mutex<HashMap<u32, std::process::Child>>>();
    let mut processes = processes.lock().unwrap();
    if let Some(mut child) = processes.remove(&proxy_id) {
        child.kill().map_err(|e| format!("停止隧道失败: {}", e))?;
        // 向前端发送停止事件
        let _ = app.emit("log", 
            serde_json::json!({
                "message": format!("[FRPC] 停止进程 PID: {}", child.id())
            })
        );
        Ok(true)
    } else {
        Err("未找到对应的隧道进程".to_string())
    }
}
