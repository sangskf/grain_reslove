// 命令模块，包含所有Tauri命令

use crate::network;
use crate::logger::{self, LogEntry};
use log::{info, warn, error};

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
    // 记录操作日志
    info!("发送数据到 {}:{}", ip, port);
    
    // 调用网络模块发送数据
    let result = network::send_hex_data(ip, port, data, timeout_ms);
    
    // 记录结果日志
    match &result {
        Ok(_) => info!("成功接收来自 {}:{} 的响应", ip, port),
        Err(e) => error!("发送数据失败: {}", e),
    }
    
    result
}

/// 获取日志
///
/// 从日志文件中读取最近的日志
#[tauri::command]
pub fn get_logs(level: Option<String>, limit: Option<usize>) -> Result<Vec<LogEntry>, String> {
    info!("获取日志，级别：{:?}，限制: {:?}", level, limit);
    
    match logger::read_logs(level, limit.or(Some(100))) {
        Ok(logs) => Ok(logs),
        Err(e) => {
            error!("读取日志失败: {}", e);
            Err(format!("读取日志失败: {}", e))
        }
    }
}

/// 添加日志
///
/// 允许前端添加日志到日志文件
#[tauri::command]
pub fn add_log(level: &str, message: &str) -> Result<(), String> {
    match level {
        "info" => info!("{}", message),
        "warn" => warn!("{}", message),
        "error" => error!("{}", message),
        _ => info!("{}", message),
    }
    
    match logger::append_to_log(&level.to_uppercase(), message) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("添加日志失败: {}", e)),
    }
}

/// 清空日志
///
/// 清空当天的日志文件
#[tauri::command]
pub fn clear_logs() -> Result<(), String> {
    info!("清空日志");
    
    match logger::clear_logs() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("清空日志失败: {}", e)),
    }
}
