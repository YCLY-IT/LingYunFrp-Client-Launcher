use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;
use crate::commands::forward_request;

#[cfg(target_os = "windows")]
use std::sync::Arc;
#[cfg(target_os = "windows")]
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use wintun;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use tun::platform::Device as TunDevice;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::io::{Read, Write};

const CREATE_NO_WINDOW: u32 = 0x08000000;

// ========== 虚拟组网核心结构 ==========
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetworkConfig {
    pub network_id: String,
    pub name: String,
    pub password: Option<String>,
    pub is_public: bool,
    pub max_players: u32,
    pub virtual_subnet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNode {
    pub id: String,
    pub nickname: String,
    pub is_owner: bool,
    pub is_online: bool,
    pub virtual_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetwork {
    pub config: VirtualNetworkConfig,
    pub nodes: Vec<VirtualNode>,
    pub status: String, // "Active" or "Inactive"
}

// 全局虚拟网络状态（仅单用户/单进程原型，后续可持久化/多用户）
lazy_static::lazy_static! {
    static ref CURRENT_NETWORK: Mutex<Option<VirtualNetwork>> = Mutex::new(None);
    static ref POLLING_RUNNING: AtomicBool = AtomicBool::new(false);
    static ref MY_PUBLIC_ADDR: OnceLock<String> = OnceLock::new();
    static ref MY_LOCAL_PORT: OnceLock<u16> = OnceLock::new();
}

// ========== Tauri 命令接口 ==========

#[tauri::command]
pub async fn create_virtual_network(
    name: String,
    password: Option<String>,
    is_public: bool,
    max_players: u32,
    headers: Option<serde_json::Value>
) -> Result<serde_json::Value, String> {
    let url = "/user/virtual_network/create";
    let method = "POST";
    let data = serde_json::json!({
        "name": name,
        "password": password.clone().unwrap_or_else(|| "".to_string()),
        "is_public": is_public,
        "max_players": max_players
    });
    let headers = headers.unwrap_or_else(|| serde_json::json!({}));
    let resp = forward_request(url.to_string(), method.to_string(), data, headers).await?;
    if resp.get("code").and_then(|v| v.as_i64()) == Some(0) {
        if let Some(data) = resp.get("data") {
            let mut data = data.clone();
            if let Some(nodes_obj) = data.get_mut("nodes") {
                if nodes_obj.is_object() {
                    let arr: Vec<_> = nodes_obj.as_object().unwrap().values().cloned().collect();
                    *nodes_obj = serde_json::Value::Array(arr);
                }
            }
            match serde_json::from_value::<VirtualNetwork>(data) {
                Ok(network) => {
                    let mut global = CURRENT_NETWORK.lock().unwrap();
                    *global = Some(network.clone());

                    // 自动连接 peers
                    if let (Some(my_public_addr), Some(my_local_port)) = (MY_PUBLIC_ADDR.get(), MY_LOCAL_PORT.get()) {
                        let nodes_json = serde_json::to_value(&network.nodes).unwrap_or_default();
                        let _ = auto_connect_peers(
                            nodes_json.as_array().unwrap_or(&vec![]).clone(),
                            my_public_addr.clone(),
                            *my_local_port
                        );
                    }
                }
                Err(e) => {
                    println!("反序列化 VirtualNetwork 失败: {:?}", e);
                }
            }
        }
    }
    Ok(resp)
}

#[tauri::command]
pub async fn join_virtual_network(
    network_id: String,
    password: Option<String>,
    headers: Option<serde_json::Value>
) -> Result<serde_json::Value, String> {
    let url = "/user/virtual_network/join";
    let method = "POST";
    let data = serde_json::json!({
        "network_id": network_id,
        "password": password.clone().unwrap_or_else(|| "".to_string())
    });
    let headers = headers.unwrap_or_else(|| serde_json::json!({}));
    let resp = forward_request(url.to_string(), method.to_string(), data, headers).await?;
    if resp.get("code").and_then(|v| v.as_i64()) == Some(0) {
        if let Some(data) = resp.get("data") {
            let mut data = data.clone();
            if let Some(nodes_obj) = data.get_mut("nodes") {
                if nodes_obj.is_object() {
                    let arr: Vec<_> = nodes_obj.as_object().unwrap().values().cloned().collect();
                    *nodes_obj = serde_json::Value::Array(arr);
                }
            }
            match serde_json::from_value::<VirtualNetwork>(data) {
                Ok(network) => {
                    let mut global = CURRENT_NETWORK.lock().unwrap();
                    *global = Some(network.clone());

                    // 自动连接 peers
                    if let (Some(my_public_addr), Some(my_local_port)) = (MY_PUBLIC_ADDR.get(), MY_LOCAL_PORT.get()) {
                        let nodes_json = serde_json::to_value(&network.nodes).unwrap_or_default();
                        let _ = auto_connect_peers(
                            nodes_json.as_array().unwrap_or(&vec![]).clone(),
                            my_public_addr.clone(),
                            *my_local_port
                        );
                    }
                }
                Err(e) => {
                    println!("反序列化 VirtualNetwork 失败: {:?}", e);
                }
            }
        }
    }
    Ok(resp)
}

#[tauri::command]
pub async fn leave_virtual_network(network_id: String, headers: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
    // 只校验，不修改
    {
        let global = CURRENT_NETWORK.lock().unwrap();
        if let Some(current) = global.as_ref() {
            if current.config.network_id != network_id {
                return Err("网络ID不匹配，无法离开该网络".to_string());
            }
        } else {
            return Err("当前未加入任何虚拟网络".to_string());
        }
    }
    let req = serde_json::json!({
        "network_id": network_id,
    });
    // 异步请求服务器，直接传递整体JSON数据
    let url = "/user/virtual_network/leave";
    let method = "POST";
    let headers = headers.unwrap_or_else(|| serde_json::json!({}));
    let resp = forward_request(url.to_string(), method.to_string(), req, headers).await?;
    if resp.get("code").and_then(|v| v.as_i64()).unwrap_or(1) != 0 {
        return Err(resp.get("message").and_then(|v| v.as_str()).unwrap_or("离开网络失败").to_string());
    }
    // 服务器成功后再清空本地状态
    // 先关闭虚拟网卡和P2P
    let _ = stop_virtual_network().await;
    let mut global = CURRENT_NETWORK.lock().unwrap();
    *global = None;
    Ok(resp)
}

#[tauri::command]
pub async fn get_current_virtual_network(headers: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
    let headers = headers.unwrap_or_else(|| serde_json::json!({}));
    
    // 启动轮询任务（如果还没启动）
    if !POLLING_RUNNING.load(Ordering::SeqCst) {
        POLLING_RUNNING.store(true, Ordering::SeqCst);
        
        // 启动轮询任务
        let headers_clone = headers.clone();
        tokio::spawn(async move {
            poll_network_status(headers_clone).await;
        });
    }
    
    // 优先从缓存获取数据
    {
        let global = CURRENT_NETWORK.lock().unwrap();
        if let Some(network) = global.as_ref() {
            match serde_json::to_value(network) {
                Ok(data) => {
                    return Ok(serde_json::json!({
                        "code": 0,
                        "data": data,
                        "message": "success"
                    }));
                }
                Err(e) => {
                    println!("序列化缓存数据失败: {:?}", e);
                }
            }
        }
    }
    
    // 如果缓存中没有数据，则从API获取
    let url = "/user/virtual_network/get";
    let method = "GET";
    let data = serde_json::json!({});
    let resp = forward_request(url.to_string(), method.to_string(), data, headers).await?;
    
    // 如果API返回成功且有数据，则存储到本地状态
    if resp.get("code").and_then(|v| v.as_i64()) == Some(0) {
        if let Some(data) = resp.get("data") {
            let mut data = data.clone();
            if let Some(nodes_obj) = data.get_mut("nodes") {
                if nodes_obj.is_object() {
                    let arr: Vec<_> = nodes_obj.as_object().unwrap().values().cloned().collect();
                    *nodes_obj = serde_json::Value::Array(arr);
                }
            }
            match serde_json::from_value::<VirtualNetwork>(data) {
                Ok(network) => {
                    let mut global = CURRENT_NETWORK.lock().unwrap();
                    *global = Some(network.clone());

                    // 自动连接 peers
                    if let (Some(my_public_addr), Some(my_local_port)) = (MY_PUBLIC_ADDR.get(), MY_LOCAL_PORT.get()) {
                        let nodes_json = serde_json::to_value(&network.nodes).unwrap_or_default();
                        let _ = auto_connect_peers(
                            nodes_json.as_array().unwrap_or(&vec![]).clone(),
                            my_public_addr.clone(),
                            *my_local_port
                        );
                    }
                }
                Err(e) => {
                    println!("反序列化 VirtualNetwork 失败: {:?}", e);
                }
            }
        }
    }
    
    Ok(resp)
}

/// 轮询虚拟网络状态的核心函数
async fn poll_network_status(headers: serde_json::Value) {
    while POLLING_RUNNING.load(Ordering::SeqCst) {
        // 获取当前网络状态
        match poll_current_network_status(&headers).await {
            Ok(_) => {
                println!("轮询: 成功获取虚拟网络状态");
            }
            Err(e) => {
                println!("轮询: 获取虚拟网络状态失败: {}", e);
            }
        }
        
        // 等待30秒后再次轮询
        sleep(Duration::from_secs(30)).await;
    }
}

/// 轮询获取当前网络状态并更新本地缓存
async fn poll_current_network_status(headers: &serde_json::Value) -> Result<(), String> {
    let url = "/user/virtual_network/get";
    let method = "GET";
    let data = serde_json::json!({});
    
    let resp = forward_request(url.to_string(), method.to_string(), data, headers.clone()).await?;
    
    // 如果API返回成功且有数据，则更新本地状态
    if resp.get("code").and_then(|v| v.as_i64()) == Some(0) {
        if let Some(data) = resp.get("data") {
            let mut data = data.clone();
            if let Some(nodes_obj) = data.get_mut("nodes") {
                if nodes_obj.is_object() {
                    let arr: Vec<_> = nodes_obj.as_object().unwrap().values().cloned().collect();
                    *nodes_obj = serde_json::Value::Array(arr);
                }
            }
            match serde_json::from_value::<VirtualNetwork>(data) {
                Ok(network) => {
                    let mut global = CURRENT_NETWORK.lock().unwrap();
                    *global = Some(network.clone());

                    // 自动连接 peers
                    if let (Some(my_public_addr), Some(my_local_port)) = (MY_PUBLIC_ADDR.get(), MY_LOCAL_PORT.get()) {
                        let nodes_json = serde_json::to_value(&network.nodes).unwrap_or_default();
                        let _ = auto_connect_peers(
                            nodes_json.as_array().unwrap_or(&vec![]).clone(),
                            my_public_addr.clone(),
                            *my_local_port
                        );
                    }
                }
                Err(e) => {
                    println!("轮询: 反序列化 VirtualNetwork 失败: {:?}", e);
                }
            }
        } else {
            // 如果没有数据，说明用户没有加入任何网络，清空本地状态
            let mut global = CURRENT_NETWORK.lock().unwrap();
            *global = None;
        }
    } else {
        // API返回错误，清空本地状态
        let mut global = CURRENT_NETWORK.lock().unwrap();
        *global = None;
    }
    
    Ok(())
}

static TUN_RUNNING: OnceLock<AtomicBool> = OnceLock::new();
#[cfg(target_os = "windows")]
static TUN_HANDLE: OnceLock<Mutex<Option<Arc<wintun::Session>>>> = OnceLock::new();
#[cfg(any(target_os = "linux", target_os = "macos"))]
static TUN_HANDLE: OnceLock<Mutex<Option<TunDevice>>> = OnceLock::new();

#[tauri::command]
pub async fn start_virtual_network(virtual_ip: String) -> Result<String, String> {
    // 启动前先检测驱动是否已安装
    match is_tap_driver_installed().await {
        Ok(true) => {},
        Ok(false) => {
            return Err("未检测到虚拟网卡驱动，请先安装 TAP/Wintun 驱动后重试。".to_string());
        },
        Err(e) => {
            return Err(format!("检测驱动状态失败: {e}"));
        }
    }

    // 防止重复启动
    let running = TUN_RUNNING.get_or_init(|| AtomicBool::new(false));
    if running.load(Ordering::SeqCst) {
        return Err("虚拟网卡已启动".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        // 判断系统架构，选择合适的 wintun.dll 路径
        use std::env;
        let arch = env::consts::ARCH;
        let dll_path = match arch {
            "x86_64" => "driver/amd64/wintun.dll",
            "x86" => "driver/x86/wintun.dll",
            "aarch64" => "driver/arm64/wintun.dll",
            "arm" => "driver/arm/wintun.dll",
            _ => "driver/amd64/wintun.dll", // 默认用 amd64
        };
        let wintun = unsafe { wintun::load_from_path(dll_path) }
            .map_err(|e| format!("加载 {dll_path} 失败: {e}"))?;
        let adapter = match wintun::Adapter::open(&wintun, "LingYunTun") {
            Ok(a) => a,
            Err(_) => wintun::Adapter::create(&wintun, "LingYunTun", "Example", None)
                .map_err(|e| format!("创建 Wintun 适配器失败: {e}"))?,
        };
        let session = Arc::new(adapter.start_session(wintun::MAX_RING_CAPACITY)
            .map_err(|e| format!("启动 Wintun Session 失败: {e}"))?);
        // 保存 session 句柄
        TUN_HANDLE.get_or_init(|| Mutex::new(Some(session)));
        // 设置 IP 地址（调用 netsh）
        let output = Command::new("netsh")
            .args(&["interface", "ip", "set", "address", &format!("name=LingYunTun"), "static", &virtual_ip, "255.255.255.0"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        match output {
            Ok(out) => {
                if !out.status.success() {
                    let err = String::from_utf8_lossy(&out.stderr);
                    return Err(format!("设置IP失败: {}", err));
                }
            },
            Err(e) => {
                return Err(format!("调用netsh失败: {e}"));
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        // 创建 TUN 设备（原有逻辑）
        let mut config = tun::Configuration::default();
        let ip: [u8; 4] = virtual_ip
            .split('.')
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("IP解析失败: {e}"))?
            .try_into()
            .map_err(|_| "IP格式错误".to_string())?;
        config.address((ip[0], ip[1], ip[2], ip[3]));
        config.netmask((255, 255, 255, 0));
        config.up();
        config.tun_name("LingYunTun");
        let dev = TunDevice::new(&config)
            .map_err(|e| format!("创建TUN失败: {}", e))?;
        TUN_HANDLE.get_or_init(|| Mutex::new(Some(dev)));
    }
    running.store(true, Ordering::SeqCst);
    Ok("虚拟网卡已启动".to_string())
}

#[tauri::command]
pub async fn stop_virtual_network() -> Result<String, String> {
    let running = TUN_RUNNING.get_or_init(|| AtomicBool::new(false));
    if !running.load(Ordering::SeqCst) {
        return Ok("虚拟网卡未启动".to_string());
    }
    if let Some(lock) = TUN_HANDLE.get() {
        let mut guard = lock.lock().unwrap();
        *guard = None; // Drop TunDevice 或 Session 关闭网卡
    }
    running.store(false, Ordering::SeqCst);
    Ok("虚拟网卡已关闭".to_string())
}

#[cfg(target_os = "windows")]
fn start_p2p_forward(
    tun: &'static Mutex<Option<Arc<wintun::Session>>>,
    peer_addr: String,
    local_port: u16,
) {
    let session = {
        let tun_guard = tun.lock().unwrap();
        match tun_guard.as_ref() {
            Some(s) => s.clone(),
            None => return,
        }
    };

    // UDP -> Wintun
    let udp_to_tun = std::thread::spawn({
        let session = session.clone();
        move || {
            let socket = std::net::UdpSocket::bind(("0.0.0.0", local_port + 1)).unwrap();
            let mut buf = [0u8; 1500];
            loop {
                match socket.recv(&mut buf) {
                    Ok(n) => {
                        if let Ok(mut pkt) = session.allocate_send_packet(n.try_into().unwrap_or(0)) {
                            pkt.bytes_mut()[..n].copy_from_slice(&buf[..n]);
                            session.send_packet(pkt);
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    });
// Wintun -> UDP
let tun_to_udp = std::thread::spawn({
    let session = session.clone();
    move || {
        let socket = std::net::UdpSocket::bind(("0.0.0.0", local_port)).unwrap();
        if socket.connect(&peer_addr).is_err() {
            return;
        }
        loop {
            match session.try_receive() {
                Ok(Some(pkt)) => {
                    let data = pkt.bytes();
                    let _ = socket.send(data);
                }
                Ok(None) => {
                    // 没有数据包可用，短暂休眠
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(_) => {
                    // 发生错误，可以选择重试或退出
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
    }
});



    let _ = udp_to_tun.join();
    let _ = tun_to_udp.join();
}


#[cfg(any(target_os = "linux", target_os = "macos"))]
fn start_p2p_forward(
    tun: &'static Mutex<Option<TunDevice>>,
    peer_addr: String,
    local_port: u16,
) {
    // TUN -> UDP
    let tun_to_udp = std::thread::spawn({
        let tun = tun;
        let peer_addr = peer_addr.clone();
        move || {
            let socket = std::net::UdpSocket::bind(("0.0.0.0", local_port)).unwrap();
            socket.connect(&peer_addr).unwrap();
            let mut buf = [0u8; 1500];
            loop {
                let n = {
                    let mut tun_guard = tun.lock().unwrap();
                    let tun_dev = tun_guard.as_mut().unwrap();
                    tun_dev.read(&mut buf).unwrap()
                };
                socket.send(&buf[..n]).unwrap();
            }
        }
    });
    // UDP -> TUN
    let udp_to_tun = std::thread::spawn({
        let tun = tun;
        move || {
            let socket = std::net::UdpSocket::bind(("0.0.0.0", local_port + 1)).unwrap();
            let mut buf = [0u8; 1500];
            loop {
                let n = socket.recv(&mut buf).unwrap();
                let mut tun_guard = tun.lock().unwrap();
                let tun_dev = tun_guard.as_mut().unwrap();
                tun_dev.write_all(&buf[..n]).unwrap();
            }
        }
    });
    let _ = tun_to_udp.join();
    let _ = udp_to_tun.join();
}

#[cfg(target_os = "windows")]
pub fn start_p2p_forward_cmd(peer_addr: String, local_port: u16) -> Result<String, String> {
    let tun_mutex = TUN_HANDLE.get().ok_or("虚拟网卡未启动")?;
    std::thread::spawn({
        let tun = tun_mutex;
        let peer_addr = peer_addr.clone();
        move || {
            start_p2p_forward(tun, peer_addr, local_port);
        }
    });
    Ok("P2P转发已启动".to_string())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn start_p2p_forward_cmd(peer_addr: String, local_port: u16) -> Result<String, String> {
    let tun_mutex = TUN_HANDLE.get().ok_or("虚拟网卡未启动")?;
    std::thread::spawn({
        let tun = tun_mutex;
        let peer_addr = peer_addr.clone();
        move || {
            start_p2p_forward(tun, peer_addr, local_port);
        }
    });
    Ok("P2P转发已启动".to_string())
}

#[tauri::command]
pub fn auto_connect_peers(nodes: Vec<serde_json::Value>, my_public_addr: String, my_local_port: u16) -> Result<String, String> {
    let mut base_port = my_local_port;
    for node in nodes {
        if let Some(peer_addr) = node.get("public_addr").and_then(|v| v.as_str()) {
            if peer_addr != my_public_addr {
                // 为每个对端分配不同的本地端口，避免冲突
                let local_port = base_port + 2; // 每个连接需要2个端口，所以间隔2
                let _ = start_p2p_forward_cmd(peer_addr.to_string(), local_port);
                base_port = local_port; // 更新基础端口
            }
        }
    }
    Ok("已自动连接所有对端节点".to_string())
}

#[tauri::command]
pub async fn is_tap_driver_installed() -> Result<bool, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            use std::path::Path;
            use std::env;
            let arch = env::consts::ARCH;
            let dll_path = match arch {
                "x86_64" => "driver/amd64/wintun.dll",
                "x86" => "driver/x86/wintun.dll",
                "aarch64" => "driver/arm64/wintun.dll",
                "arm" => "driver/arm/wintun.dll",
                _ => "driver/amd64/wintun.dll",
            };
            Ok(Path::new(dll_path).exists())
        }
        #[cfg(target_os = "macos")]
        {
            let output = std::process::Command::new("ifconfig")
                .output()
                .map_err(|e| format!("无法执行命令: {}", e))?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let found = stdout.contains("utun") || stdout.contains("wg");
            Ok(found)
        }
        #[cfg(target_os = "linux")]
        {
            Ok(std::path::Path::new("/dev/net/tun").exists())
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err("暂不支持此操作系统".to_string())
        }
    })
    .await
    .map_err(|e| format!("检测线程异常: {e}"))?
}