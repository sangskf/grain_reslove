// Tauri应用程序主模块

// 导入模块
mod commands;
mod network;
mod utils;
mod logger;

// 使用commands模块中的命令
use commands::{send_hex_data, get_logs, add_log, clear_logs};
use tauri_plugin_log::{Target, TargetKind};
use chrono::{Local, FixedOffset, Utc};
use log::{info, LevelFilter};
use std::path::PathBuf;
use std::fs;
use std::env;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 获取应用日志目录 (使用当前目录或临时目录作为备选)
    let app_dir = env::current_dir().unwrap_or_else(|_| env::temp_dir());
    let log_dir = app_dir.join("logs");
    
    // 创建日志目录
    if !log_dir.exists() {
        if let Err(e) = fs::create_dir_all(&log_dir) {
            eprintln!("创建日志目录失败: {}", e);
        }
    }

    // 创建东八区（北京时间）时区
    let china_timezone = FixedOffset::east_opt(8 * 3600).unwrap_or(FixedOffset::east(8 * 3600));
    // 获取当前时间并转换为东八区时间
    let now = Utc::now().with_timezone(&china_timezone);
    // 生成日志文件名，按日期命名
    let today = now.format("%Y-%m-%d").to_string();
    let log_file_name = format!("app_{}", today);
    let log_file_path = log_dir.join(log_file_name);
    
    // 记录日志文件路径到全局变量，供logger模块使用
    if let Err(e) = logger::set_log_dir(log_dir.to_string_lossy().to_string()) {
        eprintln!("设置日志目录失败: {}", e);
    }

    // 启动应用
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { 
                    file_name: Some(log_file_path.to_string_lossy().to_string()) 
                }),
                Target::new(TargetKind::Webview),
            ])
            .level(LevelFilter::Info)
            .build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_app::init())
        .invoke_handler(tauri::generate_handler![
            send_hex_data, 
            get_logs, 
            add_log, 
            clear_logs
        ]);

    info!("应用程序启动");

    builder
        .run(tauri::generate_context!())
        .expect("error while running application");
}
