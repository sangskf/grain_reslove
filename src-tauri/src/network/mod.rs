// 网络通信模块
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::utils::hex_utils::{format_bytes_to_hex, hex_string_to_bytes};

/// 发送16进制数据并接收响应
///
/// # 参数
///
/// * `ip` - 目标IP地址
/// * `port` - 目标端口号
/// * `data` - 16进制数据字符串，以空格分隔
///
/// # 返回值
///
/// 成功时返回响应的16进制字符串，失败时返回错误信息
pub fn send_hex_data(ip: &str, port: u16, data: &str) -> Result<String, String> {
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
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("设置读取超时失败: {}", e))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("设置写入超时失败: {}", e))?;

    // 发送16进制数据
    match stream.write_all(&hex_data) {
        Ok(_) => {}
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
        }
        Err(e) => return Err(format!("接收数据失败: {}", e)),
    }

    // 将响应转换为16进制字符串
    let hex_response = format_bytes_to_hex(&response_data);

    Ok(hex_response)
}
