// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

// 导入utils模块
mod utils;
use utils::hex_utils::{hex_string_to_bytes, format_bytes_to_hex};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn send_hex_data(ip: &str, port: u16, data: &str) -> Result<String, String> {
    // 将用户输入的16进制字符串转换为字节数组
    let hex_data = match hex_string_to_bytes(data) {
        Ok(bytes) => bytes,
        Err(e) => return Err(format!("解析16进制数据失败: {}", e)),
    };
    
    // 连接到指定的IP和端口
    let address = format!("{}:{}", ip, port);
    let mut stream = match TcpStream::connect(&address) {
        Ok(stream) => stream,
        Err(e) => return Err(format!("连接失败: {}", e)),
    };
    
    // 设置读写超时
    stream.set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("设置读取超时失败: {}", e))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("设置写入超时失败: {}", e))?;
    
    // 发送16进制数据
    match stream.write_all(&hex_data) {
        Ok(_) => {},
        Err(e) => return Err(format!("发送数据失败: {}", e)),
    }
    
    // 读取响应
    let mut buffer = vec![0u8; 1024]; // 足够大的缓冲区
    let mut response_data = Vec::new();
    
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                response_data.extend_from_slice(&buffer[0..bytes_read]);
            } else {
                return Err("没有收到任何数据".to_string());
            }
        },
        Err(e) => return Err(format!("接收数据失败: {}", e)),
    }
    
    // 将响应转换为16进制字符串
    let hex_response = format_bytes_to_hex(&response_data);
    
    Ok(hex_response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, send_hex_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
