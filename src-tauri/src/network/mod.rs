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
/// * `timeout_ms` - 超时时间（毫秒）
///
/// # 返回值
///
/// 成功时返回响应的16进制字符串，失败时返回错误信息
pub fn send_hex_data(ip: &str, port: u16, data: &str, timeout_ms: Option<u64>) -> Result<String, String> {
    // 将用户输入的16进制字符串转换为字节数组
    let hex_data = match hex_string_to_bytes(data) {
        Ok(bytes) => bytes,
        Err(e) => return Err(format!("解析16进制数据失败: {}", e)),
    };

    // 连接到指定的IP和端口
    let address = format!("{}:{}", ip, port);
    let mut stream = match TcpStream::connect_timeout(
        &address.parse().map_err(|e| format!("无效的地址格式: {}", e))?,
        Duration::from_millis(timeout_ms.unwrap_or(5000))
    ) {
        Ok(stream) => stream,
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            if error_msg.contains("connection refused") {
                return Err("连接被拒绝，请检查设备是否在线或端口是否正确".to_string());
            } else if error_msg.contains("timed out") {
                return Err("连接超时，请检查网络连接和设备状态".to_string());
            } else if error_msg.contains("network is unreachable") {
                return Err("网络不可达，请检查网络连接".to_string());
            } else {
                return Err(format!("连接失败: {}", e));
            }
        }
    };

    // 设置读写超时
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(5000));
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|e| format!("设置读取超时失败: {}", e))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|e| format!("设置写入超时失败: {}", e))?;

    // 发送16进制数据
    if let Err(e) = stream.write_all(&hex_data) {
        let error_msg = e.to_string().to_lowercase();
        if error_msg.contains("broken pipe") {
            return Err("连接已断开，请检查设备状态".to_string());
        } else if error_msg.contains("timed out") {
            return Err("发送数据超时，请检查设备状态".to_string());
        } else {
            return Err(format!("发送数据失败: {}", e));
        }
    }

    // 读取响应
    let mut buffer = vec![0u8; 2048]; // 增大缓冲区
    let mut response_data = Vec::new();

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                response_data.extend_from_slice(&buffer[0..bytes_read]);
            } else {
                return Err("设备未返回数据，请检查设备状态".to_string());
            }
        }
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            if error_msg.contains("timed out") {
                return Err("接收数据超时，请检查设备状态".to_string());
            } else {
                return Err(format!("接收数据失败: {}", e));
            }
        }
    }

    // 将响应转换为16进制字符串
    let hex_response = format_bytes_to_hex(&response_data);

    Ok(hex_response)
}
