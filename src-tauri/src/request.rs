use serde_json;
use reqwest;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use crate::config;
use crate::commands::get_system_info;
use tauri::Manager;

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

    // 检查是否为文件上传
    let is_file_upload = data.get("file").is_some();

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

    let response = if method.to_uppercase() == "POST" && is_file_upload {
        // 处理 multipart/form-data 文件上传
        let mut form = reqwest::multipart::Form::new();
        if let Some(file_base64) = data.get("file").and_then(|v| v.as_str()) {
            // 解码 base64
            let file_bytes = match STANDARD.decode(file_base64) {
                Ok(bytes) => bytes,
                Err(e) => return Err(format!("文件base64解码失败: {}", e)),
            };
            form = form.part(
                "avatar",
                reqwest::multipart::Part::bytes(file_bytes)
                    .file_name("avatar.jpg")
                    .mime_str("image/jpeg").unwrap(),
            );
        }
        // 你可以根据需要添加更多字段
        request_builder.multipart(form).send().await
    } else if method.to_uppercase() == "POST" {
        request_builder.form(&data).send().await
    } else {
        request_builder.send().await
    }.map_err(|e| e.to_string())?;

    let resp_text = response.text().await.map_err(|e| e.to_string())?;
    let cleaned_text = resp_text
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
pub async fn download_frpc(app: tauri::AppHandle) -> Result<(), String> {
    // 根据操作系统确定可执行文件名
    let executable_name = if cfg!(target_os = "windows") {
        "frpc.exe"
    } else {
        "frpc"
    };
    
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let frpc_path = app_data_dir.join(executable_name);
    if frpc_path.exists() {
        return Err(format!("{}已存在", executable_name));
    }
    let info = get_system_info();
    let mut parts = info.split_whitespace();
    let system = parts.next().unwrap_or("unknown");
    let arch = parts.next().unwrap_or("unknown");
    
    let version = config::version();

    // 拼接下载链接
    let frpc_url = format!(
        "{}{}{}{}{}{}{}",
        config::api_url(),
        "/frp/updates/latest?software=Frpc&system=",
        system,
        "&arch=",
        arch,
        "&version=",
        version
    );

    // 下载文件
    let response = reqwest::get(&frpc_url)
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
    let bytes = file_response.bytes().await.map_err(|e| format!("读取内容失败: {}", e))?;

    // 写入文件
    std::fs::write(&frpc_path, &bytes).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_image_base64(url: String) -> Result<String, String> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("请求API失败: {}", e))?;
    let url = resp.url().to_string();

    let img_resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("请求图片失败: {}", e))?;
    let bytes = img_resp.bytes().await.map_err(|e| format!("读取图片失败: {}", e))?;
    let base64_str = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(base64_str)
} 
