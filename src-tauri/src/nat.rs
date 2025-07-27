use serde::Deserialize;
use crate::commands::check_software_file;
use regex::Regex;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::commands::get_system_info;
use crate::config;

use tokio::process::{Command as TokioCommand};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::collections::HashMap;
use tauri::Emitter;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct Network {
    pub id: String,
    pub local_ip: String,
    pub local_port: u16,
}


#[tauri::command]
pub async fn nat_start(network: Network, app: tauri::AppHandle) -> Result<(bool, String), String> {
    let natter_path = match check_software_file(app.clone(), Some("natter".to_string())) {
        Ok((true, path)) => path,
        _ => return Err("natter软件文件不存在".to_string()),
    };
    
    println!("启动natter: {} -b {} -t {} -p 0", natter_path, network.local_port, network.local_ip);
    
    let mut command = TokioCommand::new(&natter_path);
    command.args(["-b", &network.local_port.to_string(), "-t", &network.local_ip, "-p", "0"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    
    #[cfg(target_os = "windows")]
    command.creation_flags(0x08000000);
    
    // 更新 nat_start 以捕获 stderr
    let mut child = command.spawn().map_err(|e| format!("启动失败: {}", e))?;
    
    let stdout = child.stdout.take().ok_or("无法捕获stdout")?;
    let stderr = child.stderr.take().ok_or("无法捕获stderr")?;
    
    let network_id = network.id.clone();
    let _app_clone = app.clone();
    
    // 使用共享的 accumulated
    let accumulated_arc = Arc::new(Mutex::new(String::new()));
    let accumulated_clone1 = accumulated_arc.clone();
    let accumulated_clone2 = accumulated_arc.clone();
    
    // stdout 读取任务
    tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let mut acc = accumulated_clone1.lock().await;
            acc.push_str(&line);
            acc.push('\n');
            drop(acc);
            // 提取等
        }
    });
    
    // stderr 读取任务
    tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let mut acc = accumulated_clone2.lock().await;
            acc.push_str(&line);
            acc.push('\n');
            drop(acc);
            // 提取等
        }
    });
    
    // 提取任务（例如每秒检查一次 accumulated）
    let accumulated_extract = accumulated_arc.clone();
    let app_extract = app.clone();
    let processes = NATTER_PROCESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let processes_clone = processes.clone();  // 新增：用于提取任务
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            // 新增：检查进程是否还在运行
            let mut processes_guard = processes_clone.lock().await;
            let is_running = if let Some(child) = processes_guard.get_mut(&network_id) {
                matches!(child.try_wait(), Ok(None))
            } else {
                false
            };
            drop(processes_guard);
            
            if !is_running {
                println!("进程已退出，停止提取任务 for {}", network_id);
                break;
            }
            
            let acc = accumulated_extract.lock().await;
            if let Some(addr) = extract_public_address(&acc) {
                process_address(&app_extract, &network_id, &addr).await;
            }
            // 清空 acc 以避免重复提取
            // acc.clear();
            drop(acc);
        }
    });
    
    // 存储进程和 accumulated
    let processes = NATTER_PROCESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut guard = processes.lock().await;
    let network_id = network.id.clone();
    guard.insert(network_id.clone(), child);
    drop(guard);
    
    let acc_global = ACCUMULATED_OUTPUTS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut acc_guard = acc_global.lock().await;
    acc_guard.insert(network_id.clone(), accumulated_arc.lock().await.clone());
    drop(acc_guard);
    
    Ok((true, "等待获取".to_string()))
}

async fn process_address(app: &tauri::AppHandle, network_id: &str, public_address: &str) {
    println!("处理提取地址: {}", public_address);
    let addresses = PUBLIC_ADDRESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut guard = addresses.lock().await;
    
    // 新增：检查是否变化
    let should_emit = if let Some(existing) = guard.get(network_id) {
        existing != public_address
    } else {
        true
    };
    
    guard.insert(network_id.to_string(), public_address.to_string());
    drop(guard);
    
    if should_emit {
        if let Err(e) = app.emit("nat-ip-update", json!({"network_id": network_id, "public_address": public_address, "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()})) {
            println!("emit失败: {}", e);
        }
    }
}

// 全局存储
static NATTER_PROCESSES: std::sync::OnceLock<Arc<Mutex<HashMap<String, tokio::process::Child>>>> = std::sync::OnceLock::new();
static PUBLIC_ADDRESSES: std::sync::OnceLock<Arc<Mutex<HashMap<String, String>>>> = std::sync::OnceLock::new();
static ACCUMULATED_OUTPUTS: std::sync::OnceLock<Arc<Mutex<HashMap<String, String>>>> = std::sync::OnceLock::new();

// 更新 nat_get_address 以检查进程状态
#[tauri::command]
pub async fn nat_get_address(network_id: String) -> Result<String, String> {
    let processes = NATTER_PROCESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut processes_guard = processes.lock().await;
    
    if let Some(child) = processes_guard.get_mut(&network_id) {
        if let Ok(Some(status)) = child.try_wait() {
            return Err(format!("进程已退出，代码: {}", status.code().unwrap_or(0)));
        }
    } else {
        return Err("未找到进程".to_string());
    }
    drop(processes_guard);
    
    let addresses = PUBLIC_ADDRESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let addresses_guard = addresses.lock().await;
    
    if let Some(addr) = addresses_guard.get(&network_id) {
        Ok(addr.clone())
    } else {
        let acc = ACCUMULATED_OUTPUTS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
        let acc_guard = acc.lock().await;
        let debug_output = if let Some(output) = acc_guard.get(&network_id) {
            output.chars().take(200).collect::<String>()
        } else {
            "无累积输出".to_string()
        };
        Err(format!("等待中... (调试输出: {})", debug_output))
    }
}

// 修改 nat_stop 以清理地址
#[tauri::command]
pub async fn nat_stop(network_id: String) -> Result<bool, String> {
    let processes = NATTER_PROCESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let mut processes_guard = processes.lock().await;
    
    if let Some(mut child) = processes_guard.remove(&network_id) {
        let _ = child.kill().await;
        println!("已停止NAT通道: {}", network_id);
        
        // 清理地址
        let addresses = PUBLIC_ADDRESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
        let mut addresses_guard = addresses.lock().await;
        addresses_guard.remove(&network_id);

        // 清理累积输出
        let acc = ACCUMULATED_OUTPUTS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
        let mut acc_guard = acc.lock().await;
        acc_guard.remove(&network_id);
        
        Ok(true)
    } else {
        Err("未找到运行中的NAT通道".to_string())
    }
}
// 新命令检查活动NAT
#[tauri::command]
pub async fn get_active_nat() -> Option<String> {
    let processes = NATTER_PROCESSES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    let guard = processes.lock().await;
    
    if let Some((&ref id, _)) = guard.iter().next() {
        Some(id.clone())
    } else {
        None
    }
}
// 合并提取函数为一个更通用的
fn extract_public_address(output: &str) -> Option<String> {
    println!("尝试提取地址 from: {}", output);
    
    // 带时间戳的通用匹配
    let timestamp_re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \[I\] .* (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d{1,5})") .ok()?;
    for cap in timestamp_re.captures_iter(output) {
        let ip = &cap[1];
        let port = &cap[2];
        if is_valid_ip(ip) && !ip.starts_with("192.168.") && !ip.starts_with("127.") && port.parse::<u16>().is_ok() {
            println!("时间戳通用匹配: {}:{}", ip, port);
            return Some(format!("{}:{}", ip, port));
        }
    }
    
    // 带时间戳的特定模式
    let timestamp_patterns = [
        r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \[I\] tcp://[^:]+:\d+ <--Natter--> tcp://([^:]+):(\d+)",
        r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \[I\] Please check \[ http://([^:]+):(\d+) \]",
        r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \[I\] tcp://([^:]+):(\d+)",
        r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \[I\] LAN > (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d{1,5})   \[ OPEN \]"
    ];
    for pat in timestamp_patterns {
        if let Ok(re) = Regex::new(pat) {
            if let Some(caps) = re.captures(output) {
                let ip = &caps[1];
                let port = &caps[2];
                if is_valid_ip(ip) && !ip.starts_with("192.168.") && port.parse::<u16>().is_ok() {
                    println!("时间戳特定匹配: {}:{}", ip, port);
                    return Some(format!("{}:{}", ip, port));
                }
            }
        }
    }
    
    // 保持原有模式作为后备
    // 忽略前缀的通用IP:port匹配（公网IP通常不是192.168.*）
    let re = Regex::new(r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d{1,5})") .ok()?;
    for cap in re.captures_iter(output) {
        let ip = &cap[1];
        let port = &cap[2];
        if is_valid_ip(ip) && !ip.starts_with("192.168.") && !ip.starts_with("127.") && port.parse::<u16>().is_ok() {
            println!("通用匹配 (非本地): {}:{}", ip, port);
            return Some(format!("{}:{}", ip, port));
        }
    }
    
    // 特定模式，允许前缀如时间戳 [I]
    let patterns = [
        r"\[I\]\s*tcp://[^:]+:\d+\s*<--Natter-->\s*tcp://([^:]+):(\d+)",
        r"\[I\]\s*Please check \[ http://([^:]+):(\d+) \]",
        r"\[I\]\s*tcp://([^:]+):(\d+)"
    ];
    for pat in patterns {
        if let Ok(re) = Regex::new(pat) {
            if let Some(caps) = re.captures(output) {
                let ip = &caps[1];
                let port = &caps[2];
                if is_valid_ip(ip) && port.parse::<u16>().is_ok() {
                    println!("特定匹配: {}:{}", ip, port);
                    return Some(format!("{}:{}", ip, port));
                }
            }
        }
    }
    
    println!("无匹配");
    None
}

fn is_valid_ip(ip: &str) -> bool {
    ip.split('.').all(|s| s.parse::<u8>().is_ok())
}

#[tauri::command]
pub async fn check_natter_exists(app: tauri::AppHandle) -> bool {
    let _natter_path = match check_software_file(app.clone(), Some("natter".to_string())) {
        Ok((true, path)) => path,
        _ => return false,
    };
    true
}

#[tauri::command]
pub async fn download_natter(app: tauri::AppHandle) -> Result<(), String> {
    // 检查程序是否已存在
    let (exists, natter_path) = check_software_file(app.clone(), Some("natter".to_string()))
        .map_err(|e| format!("检查失败: {}", e))?;
    
    if exists {
        return Err("natter软件文件已存在".to_string());
    }
    
    use std::path::Path;
    if Path::new(&natter_path).exists() {
        return Err("natter软件文件已存在".to_string());
    }
    
    let info = get_system_info();
    let mut parts = info.split_whitespace();
    let system = parts.next().unwrap_or("unknown");
    let arch = parts.next().unwrap_or("unknown");
    let version = config::version();
    
    // 拼接下载链接
    let natter_url = format!(
        "{}/frp/updates/latest?software=Natter&system={}&arch={}&version={}",
        config::api_url(),
        system,
        arch,
        version
    );
    
    // 下载文件
    let response = reqwest::get(&natter_url)
        .await
        .map_err(|e| format!("下载失败: {}", e))?;
    let status = response.status();
    let resp_text = response.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if !status.is_success() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&resp_text) {
            if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                return Err(msg.to_string());
            }
        }
        return Err(format!("下载失败，状态码: {}", status));
    }
    let json: serde_json::Value = serde_json::from_str(&resp_text).map_err(|e| format!("解析JSON失败: {}", e))?;
    let download_url = json["data"]["latest_info"]["download_url"]
        .as_str()
        .ok_or("未找到下载链接")?;
    
    // 再次请求下载文件
    let file_response = reqwest::get(download_url)
        .await
        .map_err(|e| format!("下载文件失败: {}", e))?;
    let total_size = file_response.content_length().unwrap_or(0) as usize;
    let mut downloaded: usize = 0;
    let mut source = file_response.bytes_stream();
    let mut content: Vec<u8> = Vec::new();
    
    while let Some(item) = futures::StreamExt::next(&mut source).await {
        let chunk = item.map_err(|e| format!("读取 chunk 失败: {}", e))?;
        content.extend_from_slice(&chunk);
        downloaded += chunk.len();
        let progress = if total_size > 0 { (downloaded as f64 / total_size as f64 * 100.0) as u32 } else { 0 };
        if let Err(e) = app.emit("natter-download-progress", json!({"progress": progress, "downloaded": downloaded, "total": total_size})) {
            println!("发送进度失败: {}", e);
        }
    }
    
    // 写入文件
    std::fs::write(natter_path.as_str(), &content).map_err(|e| format!("写入文件失败: {}", e))?;
    
    Ok(())
}