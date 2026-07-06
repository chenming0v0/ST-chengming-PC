mod commands;
mod paths;
mod runtime;
mod tavern;
mod terminal;

use std::sync::{Arc, Mutex};
use terminal::TerminalManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let term_mgr: terminal::SharedTerminalManager = Arc::new(Mutex::new(TerminalManager::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(term_mgr)
        .invoke_handler(tauri::generate_handler![
            commands::get_runtime_status,
            commands::install_runtime,
            commands::get_tavern_status,
            commands::install_tavern,
            commands::update_tavern,
            commands::start_tavern,
            commands::terminal_create,
            commands::terminal_write,
            commands::terminal_kill,
            commands::terminal_list,
            commands::window_minimize,
            commands::window_toggle_maximize,
            commands::window_close,
            commands::window_start_dragging,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
