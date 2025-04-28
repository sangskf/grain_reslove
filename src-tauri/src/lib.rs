// Tauri应用程序主模块

// 导入模块
mod commands;
mod network;
mod utils;
mod logger;
mod crash_logger;

// 使用commands模块中的命令
use commands::{send_hex_data, get_logs, add_log, clear_logs};
use tauri_plugin_log::{Target, TargetKind};
use chrono::Local;
use log::{info, LevelFilter};
use std::path::PathBuf;
use std::fs;
use std::env;
use tauri::Manager;
use dirs;

// 打开日志目录的命令
#[tauri::command]
fn open_log_directory() -> Result<String, String> {
    let app_data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| env::temp_dir())
        .join("GrainResolve");
    
    let log_dir = app_data_dir.join("logs");
    
    // 确保目录存在
    if !log_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&log_dir) {
            return Err(format!("创建日志目录失败: {}", e));
        }
    }
    
    // 使用系统默认程序打开目录
    if let Err(e) = opener::open(&log_dir) {
        return Err(format!("打开日志目录失败: {}", e));
    }
    
    Ok(log_dir.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化崩溃日志处理
    crash_logger::init_crash_handler();
    
    // 获取应用数据目录
    let app_data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| env::temp_dir())
        .join("GrainResolve");
    
    let log_dir = app_data_dir.join("logs");
    
    // 创建日志目录，使用标准库的create_dir_all
    if !log_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&log_dir) {
            eprintln!("创建日志目录失败: {}", e);
            // 如果创建失败，尝试使用临时目录
            let temp_log_dir = env::temp_dir().join("GrainResolve").join("logs");
            if let Err(e) = std::fs::create_dir_all(&temp_log_dir) {
                eprintln!("创建临时日志目录也失败: {}", e);
            } else {
                // 如果成功创建临时目录，使用它
                let _ = logger::set_log_dir(temp_log_dir.to_string_lossy().to_string());
            }
        }
    }

    // 使用本地时区获取当前时间
    let now = Local::now();
    // 生成日志文件名，按日期命名
    let today = now.format("%Y-%m-%d").to_string();
    // 修正文件名，tauri-plugin-log 会自动添加 .log 后缀
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
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_app::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            send_hex_data, 
            get_logs, 
            add_log, 
            clear_logs,
            open_log_directory
        ]);

    info!("应用程序启动");

    builder
        .run(tauri::generate_context!())
        .expect("error while running application");
}
