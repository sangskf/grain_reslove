// 命令模块，包含所有Tauri命令

use crate::network;

/// 问候命令
/// 
/// 一个简单的示例命令，用于测试Tauri的命令调用
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 发送16进制数据命令
/// 
/// 向指定IP和端口发送16进制数据，并返回响应
#[tauri::command]
pub fn send_hex_data(ip: &str, port: u16, data: &str) -> Result<String, String> {
    network::send_hex_data(ip, port, data)
} 