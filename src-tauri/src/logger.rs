use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use lazy_static::lazy_static;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

// 初始化一个静态日志目录
lazy_static! {
    static ref LOG_DIR: RwLock<String> = RwLock::new(String::new());
}

/// 日志条目结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    /// 时间戳
    pub time: String,
    /// 日志级别
    pub level: String,
    /// 日志消息
    pub message: String,
}

/// 设置日志目录
pub fn set_log_dir(dir: String) -> io::Result<()> {
    // 确保日志目录存在
    let path = Path::new(&dir);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    // 设置全局日志目录
    if let Ok(mut log_dir) = LOG_DIR.write() {
        *log_dir = dir;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "无法写入日志目录"))
    }
}

/// 获取当前日志文件路径
pub fn get_log_file_path() -> io::Result<PathBuf> {
    let log_dir = if let Ok(dir) = LOG_DIR.read() {
        if dir.is_empty() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "日志目录未设置"));
        }
        dir.clone()
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "无法读取日志目录"));
    };

    // 使用本地时区获取当前日期
    let now = Local::now();
    let today = now.format("%Y-%m-%d").to_string();
    
    // 修正文件名，不要在format中添加.log后缀，因为tauri-plugin-log会自动添加
    let file_name = format!("app_{}.log", today);
    Ok(PathBuf::from(&log_dir).join(file_name))
}

/// 检查并创建日志目录
pub fn ensure_log_dir() -> io::Result<()> {
    let log_dir = if let Ok(dir) = LOG_DIR.read() {
        if dir.is_empty() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "日志目录未设置"));
        }
        dir.clone()
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "无法读取日志目录"));
    };

    let path = Path::new(&log_dir);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 往日志文件写入一条日志
pub fn append_to_log(level: &str, message: &str) -> io::Result<()> {
    ensure_log_dir()?;

    let log_path = get_log_file_path()?;
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    // 使用系统本地时间，而不是UTC+8
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let time_str = now.format("%H:%M:%S%.3f").to_string();
    
    // 使用带括号的格式，与Tauri内置日志一致
    let log_line = format!("[{}][{}][{}][grain_reslove_lib::commands] {}\n", 
                         date_str, time_str, level, message);

    use std::io::Write;
    let mut writer = io::BufWriter::new(file);
    writer.write_all(log_line.as_bytes())?;

    // 根据日志级别记录到相应的日志系统
    match level {
        "INFO" => info!("{}", message),
        "WARN" => warn!("{}", message),
        "ERROR" => error!("{}", message),
        _ => info!("{}", message),
    }

    Ok(())
}

/// 从日志文件读取日志
pub fn read_logs(level_filter: Option<String>, limit: Option<usize>) -> io::Result<Vec<LogEntry>> {
    ensure_log_dir()?;

    let log_path = get_log_file_path()?;

    if !log_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(log_path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(entry) = parse_log_line(&line) {
                if let Some(ref filter) = level_filter {
                    if entry.level != *filter && filter != "all" {
                        continue;
                    }
                }
                entries.push(entry);
            }
        }
    }

    // 按时间倒序排序，使用本地时区解析时间
    entries.sort_by(|a, b| {
        // 解析时间字符串为本地时间
        let parse_time = |time_str: &str| -> DateTime<Local> {
            // 尝试解析为没有时区的日期时间
            match NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S%.3f") {
                Ok(dt) => {
                    // 将NaiveDateTime转换为当地时区
                    match Local.from_local_datetime(&dt).single() {
                        Some(local_dt) => local_dt,
                        None => Local::now()
                }
                },
                Err(_) => Local::now()
            }
        };

        let a_time = parse_time(&a.time);
        let b_time = parse_time(&b.time);

        // 倒序排序（新的在前）
        b_time.cmp(&a_time)
    });

    // 限制返回数量
    if let Some(limit) = limit {
        if entries.len() > limit {
            entries.truncate(limit);
        }
    }

    Ok(entries)
}

/// 清空日志文件
pub fn clear_logs() -> io::Result<()> {
    ensure_log_dir()?;

    let log_path = get_log_file_path()?;

    if log_path.exists() {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(log_path)?;

        file.set_len(0)?;
    }

    info!("日志已清空");
    Ok(())
}

/// 解析日志行
fn parse_log_line(line: &str) -> Option<LogEntry> {
    // 检查是否是新的带括号格式：[YYYY-MM-DD][HH:MM:SS.sss][LEVEL][module] message
    if line.starts_with('[') {
        let parts: Vec<&str> = line.splitn(5, ']').collect();
        if parts.len() >= 4 {
            // 日期 [YYYY-MM-DD
            let date = parts[0].trim_start_matches('[');
            // 时间 [HH:MM:SS.sss
            let time = parts[1].trim_start_matches('[');
            // 级别 [LEVEL
            let level = parts[2].trim_start_matches('[');
            
            // 消息内容 (可能包含模块名)
            let mut message = "";
            if parts.len() > 4 {
                message = parts[4].trim_start();
            } else if !parts[3].is_empty() {
                // 如果没有模块名，直接取第4部分
                message = parts[3].trim_start_matches('[').trim();
            }
            
            return Some(LogEntry {
                time: format!("[{}][{}]", date, time),
                level: level.to_string(),
                message: message.to_string(),
            });
        }
    }
    
    // 兼容旧格式: YYYY-MM-DD HH:MM:SS.MMM [LEVEL] Message
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return None;
    }

    let date = parts[0];
    let time_part = parts[1];
    
    let rest = parts[2];
    if let Some(level_start) = rest.find('[') {
        if let Some(level_end) = rest.find(']') {
            let level = &rest[level_start + 1..level_end];
            let message = rest[level_end + 2..].to_string();

            return Some(LogEntry {
                time: format!("[{}][{}]", date, time_part),
                level: level.to_string(),
                message,
            });
        }
    }

    None
}
