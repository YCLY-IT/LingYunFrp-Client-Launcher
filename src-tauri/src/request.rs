use serde_json;
use reqwest;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use crate::commands::extract_zip;
use crate::config;
use tauri::Manager;

#[tauri::command]
pub async fn forward_request(
    url: String,
    method: String,
    data: serde_json::Value,
    headers: serde_json::Value,
    skip_system_proxy: Option<bool>,
) -> Result<serde_json::Value, String> {
    let client = if skip_system_proxy.unwrap_or(false) {
        reqwest::Client::builder()
            .no_proxy()
            .build()
            .map_err(|e| format!("创建HTTP客户端失败: {}", e))?
    } else {
        reqwest::Client::new()
    };
    let api_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        let base_url = config::api_url();
        let path = url.trim_start_matches('/');
        if base_url.ends_with('/') {
            format!("{}{}", base_url, path)
        } else {
            format!("{}/{}", base_url, path)
        }
    };

    // 检查是否为文件上传
    let is_file_upload = data.get("file").is_some();

    let mut request_builder = match method.to_uppercase().as_str() {
        "POST" => client.post(&api_url),
        "GET" => client.get(&api_url),
        "PATCH" => client.patch(&api_url),
        "PUT" => client.put(&api_url),
        "DELETE" => client.delete(&api_url),
        _ => return Err("不支持的请求方法".to_string()),
    };

    if let Some(headers_map) = headers.as_object() {
        for (key, value) in headers_map {
            if let Some(value_str) = value.as_str() {
                request_builder = request_builder.header(key, value_str);
            }
        }
    }

    let method_upper = method.to_uppercase();
    let response = if method_upper == "POST" && is_file_upload {
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
    } else if method_upper == "POST" || method_upper == "PATCH" || method_upper == "PUT" || method_upper == "DELETE" {
        request_builder.json(&data).send().await
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

use tauri::{Emitter, Runtime};
use tokio::io::AsyncWriteExt;
use reqwest::Client;
use futures::StreamExt;

#[derive(Clone, serde::Serialize)]
pub struct DownloadProgress {
    pub url: String,
    pub bytes_downloaded: u64,
    pub total_bytes: Option<u64>,
}

#[tauri::command]
pub async fn download_file<R: Runtime>(
    app: tauri::AppHandle<R>,
    url: String,
    file_name: String,
    need_extract: bool,
    extract_inner_folder: Option<String>,
    extract_rename_to: Option<String>,
) -> Result<(), String> {
    // 使用 Tauri 提供的 app_data_dir
    let save_path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(&file_name);

    // 确保目录已创建
    if let Some(parent) = save_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }
    let client = Client::new();
    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;

    let total = res.content_length();
    let mut stream = res.bytes_stream();
    let mut file = tokio::fs::File::create(&save_path)
        .await
        .map_err(|e| e.to_string())?;

    let mut downloaded: u64 = 0;
    let mut last_reported: u64 = 0;
    // 每 512 KB 汇报一次，避免过于频繁
    const REPORT_INTERVAL: u64 = 512 * 1024;
    let emit_name = format!("download-progress-{}", file_name.trim_end_matches(".exe").trim_end_matches(".zip"));
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;

        downloaded += chunk.len() as u64;
        // 当下载量超过上次报告位置 + 间隔时，发送进度
        if downloaded >= last_reported + REPORT_INTERVAL {
            last_reported = downloaded;
            let _ = app.emit(
                &emit_name,
                DownloadProgress {
                    url: url.clone(),
                    bytes_downloaded: downloaded,
                    total_bytes: total,
                },
            );
        }
    }

    // 下载完成后再发送一次确保 100 %
    let _ = app.emit(
        &emit_name,
        DownloadProgress {
            url: url.clone(),
            bytes_downloaded: downloaded,
            total_bytes: total,
        },
    );

    if need_extract {
        // 使用临时目录解压，避免和最终目标目录冲突
        let temp_extract_to = save_path.with_extension("").with_extension("temp");
        extract_zip(
            app, 
            save_path.to_string_lossy().to_string(), 
            temp_extract_to.to_string_lossy().to_string(),
            extract_inner_folder,
            extract_rename_to,
        ).await?;
    } else {
        // 如果不需要解压，删除 zip 文件
        let _ = tokio::fs::remove_file(&save_path).await;
    }

    Ok(())
}