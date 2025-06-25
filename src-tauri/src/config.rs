use std::env;

pub fn api_url() -> String {
    env::var("API_URL").expect("API_URL 未配置")
}

pub fn version() -> String {
    env::var("TAURI_APP_VERSION").expect("TAURI_APP_VERSION 未配置")
}

pub fn debug() -> bool {
    env::var("DEBUG")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false)
} 