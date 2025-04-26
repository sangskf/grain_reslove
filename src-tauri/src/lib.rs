// Tauri应用程序主模块

// 导入模块
mod commands;
mod network;
mod utils;

// 使用commands模块中的命令
use commands::{greet, send_hex_data};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_app::init())
        .invoke_handler(tauri::generate_handler![greet, send_hex_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
