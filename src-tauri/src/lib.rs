mod commands;
mod config;
mod credentials;
mod fs;
mod pty;
mod session;
mod ssh;
mod sysmon;

use session::SessionManager;
use sysmon::SysMonitor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SessionManager::default())
        .manage(SysMonitor::default())
        .invoke_handler(tauri::generate_handler![
            commands::create_local_session,
            commands::create_ssh_session,
            commands::save_ssh_password,
            commands::load_ssh_password,
            commands::delete_ssh_password,
            commands::write_session,
            commands::resize_session,
            commands::close_session,
            commands::get_resource_stats,
            commands::read_dir,
            commands::home_dir,
            commands::load_config,
            commands::save_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
