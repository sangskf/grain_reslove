// 命令模块，包含所有Tauri命令

use crate::logger::{self, LogEntry};
use crate::network;
use log::{debug, error, info, warn};

/// 发送16进制数据命令
///
/// 向指定IP和端口发送16进制数据，并返回响应
#[tauri::command]
pub fn send_hex_data(
    ip: &str,
    port: u16,
    data: &str,
    timeout_ms: Option<u64>,
) -> Result<String, String> {
    debug!(
        "命令调用: send_hex_data - IP: {}, 端口: {}, 超时: {:?}",
        ip, port, timeout_ms
    );
    info!(
        "准备发送数据到 {}:{}, 数据长度: {} 字节",
        ip,
        port,
        data.len()
    );

    // 记录数据摘要（只显示前30个字符，避免日志过长）
    let data_preview = if data.len() > 30 {
        format!("{}...(共{}字节)", &data[0..30], data.len())
    } else {
        data.to_string()
    };
    debug!("发送数据内容: {}", data_preview);

    // 调用网络模块发送数据
    let start_time = std::time::Instant::now();
    let result = network::send_hex_data(ip, port, data, timeout_ms);
    let elapsed = start_time.elapsed();

    // 记录结果日志
    match &result {
        Ok(response) => {
            let response_len = response.len();
            let response_preview = if response.len() > 30 {
                format!("{}...(共{}字节)", &response[0..30], response_len)
            } else {
                response.to_string()
            };

            info!(
                "成功接收来自 {}:{} 的响应，长度: {} 字节，耗时: {:?}",
                ip, port, response_len, elapsed
            );
            debug!("响应数据内容: {}", response_preview);
        }
        Err(e) => {
            error!(
                "发送数据到 {}:{} 失败: {}, 耗时: {:?}",
                ip, port, e, elapsed
            );
        }
    }

    result
}

/// 获取日志
///
/// 从日志文件中读取最近的日志
#[tauri::command]
pub fn get_logs(level: Option<String>, limit: Option<usize>) -> Result<Vec<LogEntry>, String> {
    debug!(
        "命令调用: get_logs - 级别过滤: {:?}, 限制数量: {:?}",
        level, limit
    );

    let actual_limit = limit.unwrap_or(100);
    info!(
        "正在获取日志，级别过滤: {:?}，限制数量: {}",
        level, actual_limit
    );

    let start_time = std::time::Instant::now();
    match logger::read_logs(level.clone(), Some(actual_limit)) {
        Ok(logs) => {
            let elapsed = start_time.elapsed();
            info!("成功获取 {} 条日志记录，耗时: {:?}", logs.len(), elapsed);
            Ok(logs)
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            error!("读取日志失败: {}, 耗时: {:?}", e, elapsed);
            Err(format!("读取日志失败: {}", e))
        }
    }
}

/// 添加日志
///
/// 允许前端添加日志到日志文件
#[tauri::command]
pub fn add_log(level: &str, message: &str) -> Result<(), String> {
    debug!(
        "命令调用: add_log - 级别: {}, 消息长度: {}",
        level,
        message.len()
    );

    // 记录消息摘要（只显示前50个字符，避免日志过长）
    let message_preview = if message.len() > 50 {
        format!("{}...(共{}字符)", &message[0..50], message.len())
    } else {
        message.to_string()
    };

    // 根据级别记录日志
    match level {
        "info" => info!("前端日志 [{}]: {}", level, message_preview),
        "warn" => warn!("前端日志 [{}]: {}", level, message_preview),
        "error" => error!("前端日志 [{}]: {}", level, message_preview),
        _ => info!("前端日志 [{}]: {}", level, message_preview),
    }

    let start_time = std::time::Instant::now();
    match logger::append_to_log(&level.to_uppercase(), message) {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            debug!("成功添加日志，耗时: {:?}", elapsed);
            Ok(())
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            error!("添加日志失败: {}, 耗时: {:?}", e, elapsed);
            Err(format!("添加日志失败: {}", e))
        }
    }
}

/// 清空日志
///
/// 清空当天的日志文件
#[tauri::command]
pub fn clear_logs() -> Result<(), String> {
    debug!("命令调用: clear_logs");
    info!("正在清空日志文件");

    let start_time = std::time::Instant::now();
    match logger::clear_logs() {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            info!("成功清空日志文件，耗时: {:?}", elapsed);
            Ok(())
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            error!("清空日志失败: {}, 耗时: {:?}", e, elapsed);
            Err(format!("清空日志失败: {}", e))
        }
    }
}
