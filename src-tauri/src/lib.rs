mod commands;
mod types;
mod utils;
use crate::commands::{eth, trin};
use crate::types::node::NodeStats;
use std::sync::Mutex;
use tauri::async_runtime::JoinHandle;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_shell::process::CommandChild;

#[derive(Default)]
struct AppData {
    trin_handle: Option<CommandChild>,
    // todo: double check that we need this
    log_handle: Option<JoinHandle<()>>,
    // todo: double check that we need this
    status_handle: Option<JoinHandle<()>>,
    node_stats: NodeStats,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // plugin ensures that only one instance of the app is running at a time
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // To focus the window of the running instance when user tries
            // to open a new instance
            #[cfg(desktop)]
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            // args that are passed to your app on startup
            None,
        ))
        // By default the plugin logs to stdout and to a file
        // in the recommended log directory...
        // Linux: /home/user/.config/com.trin-desktop.app
        // macOS: /Users/user/Library/Logs/com.trin-desktop.app
        // Windows: C:\Users\user\AppData\Roaming\com.trin-desktop.app
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        // initializes the store used in frontend to persist state
        .plugin(tauri_plugin_store::Builder::new().build())
        // initializes the shell plugin which allows us to spawn child processes
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data = AppData::default();
            app.manage(Mutex::new(app_data));
            Ok(())
        })
        // adds the commands that can be called from the frontend
        .invoke_handler(tauri::generate_handler![
            trin::launch_trin,
            trin::shutdown_trin,
            eth::eth_getBlockByNumber,
            eth::eth_getBlockByHash,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
