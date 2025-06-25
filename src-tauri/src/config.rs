use serde_json;

pub fn api_url() -> String {
    let conf_str = include_str!("../config.json");
    let conf: serde_json::Value = serde_json::from_str(conf_str).expect("config.json 解析失败");
    conf["api_url"].as_str().expect("api_url 字段不存在").to_string()
}

pub fn version() -> String {
    let conf_str = include_str!("../config.json");
    let conf: serde_json::Value = serde_json::from_str(conf_str).expect("config.json 解析失败");
    conf["version"].as_str().expect("version 字段不存在").to_string()
}

pub fn debug() -> bool {
    let conf_str = include_str!("../config.json");
    let conf: serde_json::Value = serde_json::from_str(conf_str).expect("config.json 解析失败");
    conf["debug"].as_bool().unwrap_or(false)
} 