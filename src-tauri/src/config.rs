use std::env;
use std::fs;
use serde::Deserialize;
use serde_json;

pub fn api_url() -> String {
    env::var("API_URL").expect("API_URL 未配置")
}

#[derive(Deserialize)]
struct TauriConf {
    version: String,
}

pub fn version() -> String {
    let conf_path = "tauri.conf.json";
    let conf_str = fs::read_to_string(conf_path).expect("无法读取 tauri.conf.json");
    let conf: TauriConf = serde_json::from_str(&conf_str).expect("tauri.conf.json 解析失败");
    conf.version
}

pub fn debug() -> bool {
    env::var("DEBUG")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false)
} 