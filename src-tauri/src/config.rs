use serde_json;

fn get_mode<'a>(_conf: &'a serde_json::Value) -> &'a str {
    #[cfg(debug_assertions)]
    {
        "dev"
    }
    #[cfg(not(debug_assertions))]
    {
        "prod"
    }
}

fn get_config<'a>() -> serde_json::Value {
    // 只用include_str!，不依赖外部文件
    let conf_str = include_str!("../config.json");
    serde_json::from_str(conf_str).expect("config.json 解析失败")
}

pub fn api_url() -> String {
    let conf = get_config();
    let mode = get_mode(&conf);
    conf[mode]["api_url"].as_str().expect("api_url 字段不存在").to_string()
}

pub fn version() -> String {
    let conf = get_config();
    let mode = get_mode(&conf);
    conf[mode]["version"].as_str().expect("version 字段不存在").to_string()
}

pub fn debug() -> bool {
    let conf = get_config();
    let mode = get_mode(&conf);
    conf[mode]["debug"].as_bool().unwrap_or(false)
}
