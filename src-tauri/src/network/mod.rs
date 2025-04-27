// 网络通信模块
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use log::{debug, info, warn, error};

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
pub fn send_hex_data(
    ip: &str,
    port: u16,
    data: &str,
    timeout_ms: Option<u64>,
) -> Result<String, String> {
    debug!("网络模块: 开始处理发送请求 -> {}:{}", ip, port);
    let actual_timeout = timeout_ms.unwrap_or(100000);
    debug!("网络模块: 使用超时时间: {}ms", actual_timeout);
    
    // 将用户输入的16进制字符串转换为字节数组
    let parse_start = std::time::Instant::now();
    let hex_data = match hex_string_to_bytes(data) {
        Ok(bytes) => {
            let elapsed = parse_start.elapsed();
            debug!("网络模块: 16进制字符串解析成功, 字节数: {}, 耗时: {:?}", bytes.len(), elapsed);
            bytes
        },
        Err(e) => {
            let elapsed = parse_start.elapsed();
            error!("网络模块: 解析16进制数据失败: {}, 耗时: {:?}", e, elapsed);
            return Err(format!("解析16进制数据失败: {}", e));
        }
    };

    // 连接到指定的IP和端口
    let address = format!("{}:{}", ip, port);
    info!("网络模块: 开始连接到 {}", address);
    let connect_start = std::time::Instant::now();
    
    let mut stream = match TcpStream::connect_timeout(
        &address
            .parse()
            .map_err(|e| format!("无效的地址格式: {}", e))?,
        Duration::from_millis(actual_timeout),
    ) {
        Ok(stream) => {
            let elapsed = connect_start.elapsed();
            info!("网络模块: 连接 {} 成功, 耗时: {:?}", address, elapsed);
            stream
        },
        Err(e) => {
            let elapsed = connect_start.elapsed();
            let error_msg = e.to_string().to_lowercase();
            
            if error_msg.contains("connection refused") {
                error!("网络模块: 连接被拒绝 {}, 耗时: {:?}", address, elapsed);
                return Err("连接被拒绝，请检查设备是否在线或端口是否正确".to_string());
            } else if error_msg.contains("timed out") {
                error!("网络模块: 连接超时 {}, 耗时: {:?}", address, elapsed);
                return Err("连接超时，请检查网络连接和设备状态".to_string());
            } else if error_msg.contains("network is unreachable") {
                error!("网络模块: 网络不可达 {}, 耗时: {:?}", address, elapsed);
                return Err("网络不可达，请检查网络连接".to_string());
            } else {
                error!("网络模块: 连接失败 {}: {}, 耗时: {:?}", address, e, elapsed);
                return Err(format!("连接失败: {}", e));
            }
        }
    };

    // 设置读写超时
    debug!("网络模块: 设置socket读写超时: {}ms", actual_timeout);
    let timeout = Duration::from_millis(actual_timeout);
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|e| {
            error!("网络模块: 设置读取超时失败: {}", e);
            format!("设置读取超时失败: {}", e)
        })?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|e| {
            error!("网络模块: 设置写入超时失败: {}", e);
            format!("设置写入超时失败: {}", e)
        })?;

    // 发送16进制数据
    debug!("网络模块: 开始发送数据, 字节数: {}", hex_data.len());
    let send_start = std::time::Instant::now();
    
    if let Err(e) = stream.write_all(&hex_data) {
        let elapsed = send_start.elapsed();
        let error_msg = e.to_string().to_lowercase();
        
        if error_msg.contains("broken pipe") {
            error!("网络模块: 连接已断开: {}, 耗时: {:?}", address, elapsed);
            return Err("连接已断开，请检查设备状态".to_string());
        } else if error_msg.contains("timed out") {
            error!("网络模块: 发送数据超时: {}, 耗时: {:?}", address, elapsed);
            return Err("发送数据超时，请检查设备状态".to_string());
        } else {
            error!("网络模块: 发送数据失败: {}, 错误: {}, 耗时: {:?}", address, e, elapsed);
            return Err(format!("发送数据失败: {}", e));
        }
    }
    
    let send_elapsed = send_start.elapsed();
    info!("网络模块: 数据发送成功, 字节数: {}, 耗时: {:?}", hex_data.len(), send_elapsed);

    // 读取响应
    debug!("网络模块: 等待接收响应");
    let recv_start = std::time::Instant::now();
    let mut buffer = vec![0u8; 4096]; // 增大缓冲区以处理大型响应
    let mut response_data = Vec::new();

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let elapsed = recv_start.elapsed();
            if bytes_read > 0 {
                response_data.extend_from_slice(&buffer[0..bytes_read]);
                info!("网络模块: 成功接收数据, 字节数: {}, 耗时: {:?}", bytes_read, elapsed);
                
                // 尝试读取更多数据（如果有）
                let mut total_bytes = bytes_read;
                while let Ok(more_bytes) = stream.read(&mut buffer) {
                    if more_bytes == 0 {
                        break;
                    }
                    response_data.extend_from_slice(&buffer[0..more_bytes]);
                    total_bytes += more_bytes;
                    debug!("网络模块: 额外接收到 {} 字节数据, 总计: {}", more_bytes, total_bytes);
                }
            } else {
                warn!("网络模块: 设备返回了0字节数据, 耗时: {:?}", elapsed);
                return Err("设备未返回数据，请检查设备状态".to_string());
            }
        }
        Err(e) => {
            let elapsed = recv_start.elapsed();
            let error_msg = e.to_string().to_lowercase();
            
            if error_msg.contains("timed out") {
                error!("网络模块: 接收数据超时: {}, 耗时: {:?}", address, elapsed);
                return Err("接收数据超时，请检查设备状态".to_string());
            } else {
                error!("网络模块: 接收数据失败: {}, 错误: {}, 耗时: {:?}", address, e, elapsed);
                return Err(format!("接收数据失败: {}", e));
            }
        }
    }

    // 将响应转换为16进制字符串
    debug!("网络模块: 开始将响应转换为16进制字符串, 原始字节数: {}", response_data.len());
    let format_start = std::time::Instant::now();
    let hex_response = format_bytes_to_hex(&response_data);
    let format_elapsed = format_start.elapsed();
    
    debug!("网络模块: 转换为16进制完成, 字符串长度: {}, 耗时: {:?}", 
           hex_response.len(), format_elapsed);
    
    // 计算总共耗时
    let total_elapsed = connect_start.elapsed();
    info!("网络模块: 整个网络交互过程完成, 发送: {} 字节, 接收: {} 字节, 总耗时: {:?}", 
          hex_data.len(), response_data.len(), total_elapsed);

    Ok(hex_response)
}
