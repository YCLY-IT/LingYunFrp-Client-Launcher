use tauri::Manager;
use std::sync::Mutex;
use tauri::Emitter;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use tokio::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

// 使用全局静态变量存储活动网络ID（仅内存存储）
static ACTIVE_NETWORK: Mutex<Option<String>> = Mutex::new(None);

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

    // 克隆 app handle 用于在异步任务中发送事件
    let app_handle = app.clone();
    let network_id = id.clone();

    #[cfg(target_os = "windows")]
    {
        // 创建日志文件路径（使用临时目录，系统会自动清理）
        let temp_dir = std::env::temp_dir();
        let log_file_path = temp_dir.join(format!("easytier_{}.log", network_id));
        let log_file_path_str = log_file_path.to_string_lossy().to_string();
        
        // 如果日志文件已存在，先删除
        let _ = std::fs::remove_file(&log_file_path);
        
        // 检查当前是否以管理员身份运行
        let is_admin = std::process::Command::new("powershell")
            .args(&[
                "-WindowStyle", "Hidden",
                "-NoProfile",
                "-Command",
                "([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)"
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.trim().to_lowercase() == "true"
            })
            .unwrap_or(false);
        
        if is_admin {
            // 已经是管理员，直接启动 EasyTier 并捕获输出
            let mut cmd = Command::new(&exe_path);
            cmd.args(&[
                "-i", &format!("{}/24", local_ip),
                "--network-name", &name,
                "--network-secret", &password,
                "-p", "tcp://ros.scpsl.com.cn:11010",
            ]);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.creation_flags(CREATE_NO_WINDOW);
            
            let mut child = cmd.spawn().map_err(|e| format!("启动easytier失败: {}", e))?;
            
            // 获取 stdout 和 stderr
            let stdout = child.stdout.take().ok_or("无法获取stdout")?;
            let stderr = child.stderr.take().ok_or("无法获取stderr")?;
            
            // 在后台任务中读取 stdout
            let app_handle_stdout = app_handle.clone();
            let network_id_stdout = network_id.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let _ = app_handle_stdout.emit(
                        "easytier-log",
                        serde_json::json!({
                            "network_id": network_id_stdout,
                            "level": "info",
                            "message": line
                        })
                    );
                }
            });
            
            // 在后台任务中读取 stderr
            let app_handle_stderr = app_handle.clone();
            let network_id_stderr = network_id.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let _ = app_handle_stderr.emit(
                        "easytier-log",
                        serde_json::json!({
                            "network_id": network_id_stderr,
                            "level": "error",
                            "message": line
                        })
                    );
                }
            });
            
            // 等待进程启动成功
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            // 检查进程是否还在运行
            match child.try_wait() {
                Ok(None) => {
                    // 进程仍在运行，启动成功
                    *ACTIVE_NETWORK.lock().unwrap() = Some(id.clone());
                    
                    // 在后台监控进程状态
                    tokio::spawn(async move {
                        let _ = child.wait().await;
                        // 进程结束时发送事件
                        let _ = app_handle.emit(
                            "easytier-stopped",
                            serde_json::json!({"network_id": network_id})
                        );
                    });
                    
                    return Ok(vec!["true".to_string()]);
                }
                Ok(Some(status)) => {
                    return Err(format!("easytier进程过早退出，退出码: {:?}", status.code()));
                }
                Err(e) => {
                    return Err(format!("检查进程状态失败: {}", e));
                }
            }
        }
        
        // 不是管理员，使用 PowerShell 脚本方式启动
        // 创建临时 PowerShell 脚本文件
        let script_content = format!(
            "$exePath = '{}'; $logPath = '{}'; & $exePath -i '{}/24' --network-name '{}' --network-secret '{}' -p 'tcp://ros.scpsl.com.cn:11010' 2>&1 | Out-File -FilePath $logPath -Append -Encoding UTF8; Start-Sleep -Seconds 2",
            exe_path.to_string_lossy().replace("'", "''"),
            log_file_path_str.replace("'", "''"),
            local_ip,
            name.replace("'", "''"),
            password.replace("'", "''")
        );
        let script_path = app_data_dir.join(format!("easytier_{}.ps1", network_id));
        if let Err(e) = std::fs::write(&script_path, &script_content) {
            return Err(format!("创建启动脚本失败: {}", e));
        }
        let script_path_str = script_path.to_string_lossy().to_string();
        
        // 使用 PowerShell 以管理员权限执行脚本（不等待，因为UAC会阻塞）
        let ps_command = format!(
            "Start-Process powershell -ArgumentList '-NoProfile','-ExecutionPolicy','Bypass','-File','{}' -Verb RunAs -WindowStyle Hidden",
            script_path_str
        );
        
        let mut cmd = std::process::Command::new("powershell");
        cmd.args(&[
            "-WindowStyle", "Hidden",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &ps_command
        ]);
        cmd.creation_flags(CREATE_NO_WINDOW);
        
        let _ = cmd.output();
        
        // 等待用户处理 UAC 弹窗并启动进程
        tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
        
        // 删除临时脚本
        let _ = std::fs::remove_file(&script_path);
        
        // 检查进程是否正在运行
        let check_cmd_output = std::process::Command::new("powershell")
            .args(&[
                "-WindowStyle", "Hidden",
                "-NoProfile",
                "-ExecutionPolicy", "Bypass",
                "-Command",
                "Get-Process -Name easytier-core -ErrorAction SilentlyContinue | Select-Object -First 1 | ForEach-Object { $_.Id }"
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
            
        let is_running = match check_cmd_output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                !stdout.trim().is_empty()
            }
            Err(_) => false,
        };
        
        if is_running {
            *ACTIVE_NETWORK.lock().unwrap() = Some(id.clone());
            
            // 启动后台任务读取日志文件
            let app_handle_log = app_handle.clone();
            let network_id_log = network_id.clone();
            let log_path = log_file_path.clone();
            tokio::spawn(async move {
                use tokio::fs::File;
                use tokio::io::AsyncSeekExt;
                
                let mut last_position = 0u64;
                let mut check_count = 0u32;
                
                loop {
                    // 检查进程是否还在运行
                    let check_output = std::process::Command::new("powershell")
                        .args(&[
                            "-WindowStyle", "Hidden",
                            "-NoProfile",
                            "-ExecutionPolicy", "Bypass",
                            "-Command",
                            "Get-Process -Name easytier-core -ErrorAction SilentlyContinue | Select-Object -First 1 | ForEach-Object { $_.Id }"
                        ])
                        .creation_flags(CREATE_NO_WINDOW)
                        .output();
                        
                    let still_running = match check_output {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            !stdout.trim().is_empty()
                        }
                        Err(_) => false,
                    };
                    
                    if !still_running {
                        check_count += 1;
                        if check_count >= 3 {
                            // 连续3次检查不到进程，认为已停止
                            let _ = app_handle_log.emit(
                                "easytier-stopped",
                                serde_json::json!({"network_id": network_id_log})
                            );
                            break;
                        }
                    } else {
                        check_count = 0;
                    }
                    
                    // 读取日志文件新内容
                    if let Ok(file) = File::open(&log_path).await {
                        let metadata = file.metadata().await.ok();
                        let file_size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                        
                        if file_size > last_position {
                            let mut reader = BufReader::new(file);
                            if let Ok(_) = reader.seek(std::io::SeekFrom::Start(last_position)).await {
                                let mut lines = reader.lines();
                                while let Ok(Some(line)) = lines.next_line().await {
                                    let _ = app_handle_log.emit(
                                        "easytier-log",
                                        serde_json::json!({
                                            "network_id": network_id_log,
                                            "level": "info",
                                            "message": line
                                        })
                                    );
                                }
                            }
                            last_position = file_size;
                        }
                    }
                    
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
                
                // 清理日志文件
                let _ = tokio::fs::remove_file(&log_path).await;
            });
            
            Ok(vec!["true".to_string()])
        } else {
            // 读取日志文件中的错误信息
            let error_msg = if let Ok(content) = std::fs::read_to_string(&log_file_path) {
                let _ = std::fs::remove_file(&log_file_path);
                if content.is_empty() {
                    "easytier进程启动失败，请检查是否已授予管理员权限".to_string()
                } else {
                    format!("easytier进程启动失败: {}", content.lines().take(5).collect::<Vec<_>>().join("\n"))
                }
            } else {
                "easytier进程启动失败，请检查是否已授予管理员权限".to_string()
            };
            Err(error_msg)
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = Command::new("sudo");
        cmd.args(&[
            &exe_path.to_string_lossy(),
            "-i", &format!("{}/24", local_ip),
            "--network-name", &name,
            "--network-secret", &password,
            "-p", "tcp://ros.scpsl.com.cn:11010",
        ]);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let mut child = cmd.spawn().map_err(|e| format!("启动easytier失败: {}", e))?;
        
        // 获取 stdout 和 stderr
        let stdout = child.stdout.take().ok_or("无法获取stdout")?;
        let stderr = child.stderr.take().ok_or("无法获取stderr")?;
        
        // 在后台任务中读取 stdout
        let app_handle_stdout = app_handle.clone();
        let network_id_stdout = network_id.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_handle_stdout.emit(
                    "easytier-log",
                    serde_json::json!({
                        "network_id": network_id_stdout,
                        "level": "info",
                        "message": line
                    })
                );
            }
        });
        
        // 在后台任务中读取 stderr
        let app_handle_stderr = app_handle.clone();
        let network_id_stderr = network_id.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_handle_stderr.emit(
                    "easytier-log",
                    serde_json::json!({
                        "network_id": network_id_stderr,
                        "level": "error",
                        "message": line
                    })
                );
            }
        });
        
        // 等待进程启动成功（短暂延迟后检查）
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // 检查进程是否还在运行
        match child.try_wait() {
            Ok(None) => {
                // 进程仍在运行，启动成功
                *ACTIVE_NETWORK.lock().unwrap() = Some(id.clone());
                save_active_network(&app, &id);
                
                // 在后台监控进程状态
                tokio::spawn(async move {
                    let _ = child.wait().await;
                    // 进程结束时发送事件
                    let _ = app_handle.emit(
                        "easytier-stopped",
                        serde_json::json!({"network_id": network_id})
                    );
                });
                
                Ok(vec!["true".to_string()])
            }
            Ok(Some(status)) => {
                Err(format!("easytier进程过早退出，退出码: {:?}", status.code()))
            }
            Err(e) => {
                Err(format!("检查进程状态失败: {}", e))
            }
        }
    }
}

#[tauri::command]
pub async fn stop_easytire() -> Result<bool, String> {
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
        Ok(true)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new("pkill");
        cmd.args(&["-9", "easytier-core"]);
        
        let output = cmd.output().map_err(|e| format!("停止easytier失败: {}", e))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if output.status.success() || stdout.is_empty() {
            *ACTIVE_NETWORK.lock().unwrap() = None;
            Ok(true)
        } else {
            Err(format!("停止easytier失败: {}", stderr))
        }
    }
}

#[tauri::command]
pub async fn get_active_easytire(_app: tauri::AppHandle) -> Option<String> {
    // 只从内存获取活动网络ID
    ACTIVE_NETWORK.lock().unwrap().clone()
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

