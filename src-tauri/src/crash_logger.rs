use std::fs::{self, OpenOptions, File};
use std::io::Write;
use std::path::PathBuf;
use chrono::{DateTime, Local, Utc};
use std::backtrace::Backtrace;
use std::panic;
use log::error;

/// 获取崩溃日志目录
pub fn get_crash_log_dir() -> PathBuf {
    let base_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("grain_reslove");
    base_dir.join("crash_logs")
}

/// 初始化崩溃日志处理
pub fn init_crash_handler() {
    // 确保崩溃日志目录存在
    let crash_dir = get_crash_log_dir();
    if !crash_dir.exists() {
        if let Err(e) = fs::create_dir_all(&crash_dir) {
            error!("创建崩溃日志目录失败: {}", e);
            return;
        }
    }

    // 设置panic处理器
    panic::set_hook(Box::new(move |panic_info| {
        let backtrace = Backtrace::capture();
        let current_time = Utc::now().with_timezone(&Local);
        let formatted_time = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
        
        let crash_file_path = crash_dir.join(format!("crash_{}.log", formatted_time));
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&crash_file_path)
        {
            // 写入崩溃时间
            let _ = writeln!(file, "崩溃时间: {}", current_time.format("%Y-%m-%d %H:%M:%S%.3f %z"));
            
            // 写入panic信息
            let _ = writeln!(file, "\n崩溃信息:");
            let _ = writeln!(file, "{}",  panic_info);
            
            // 写入发生位置
            if let Some(location) = panic_info.location() {
                let _ = writeln!(file, "\n发生位置:");
                let _ = writeln!(file, "文件: {}", location.file());
                let _ = writeln!(file, "行号: {}", location.line());
                let _ = writeln!(file, "列号: {}", location.column());
            }
            
            // 写入调用栈
            let _ = writeln!(file, "\n调用栈:");
            let _ = writeln!(file, "{}", backtrace);
            
            error!("应用程序崩溃，崩溃日志已保存到: {}", crash_file_path.display());
        } else {
            error!("无法创建崩溃日志文件: {}", crash_file_path.display());
        }
    }));
}

/// 获取最近的崩溃日志列表
pub fn get_recent_crash_logs(limit: usize) -> Vec<(DateTime<Local>, PathBuf)> {
    let crash_dir = get_crash_log_dir();
    let mut crash_logs = Vec::new();
    
    if let Ok(entries) = fs::read_dir(crash_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(modified) = metadata.modified() {
                        // Convert SystemTime to DateTime<Local> via Utc
                        let datetime_utc: DateTime<Utc> = DateTime::<Utc>::from(modified);
                        let datetime_local: DateTime<Local> = datetime_utc.with_timezone(&Local);
                        crash_logs.push((datetime_local, entry.path()));
                    }
                }
            }
        }
    }
    
    // 按时间倒序排序
    crash_logs.sort_by(|a, b| b.0.cmp(&a.0));
    
    // 限制返回数量
    crash_logs.truncate(limit);
    
    crash_logs
} 