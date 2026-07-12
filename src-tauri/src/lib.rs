mod commands;
mod events;
mod paths;
mod process_runner;
mod runtime;
mod runtime_commands;
mod runtime_download;
#[cfg(test)]
mod runtime_tests;
mod settings;
#[cfg(test)]
mod settings_tests;
mod tavern;
mod terminal;
mod window_commands;

use std::sync::{Arc, Mutex};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::Manager;
use terminal::TerminalManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let term_mgr: terminal::SharedTerminalManager = Arc::new(Mutex::new(TerminalManager::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(term_mgr)
        .setup(|app| {
            let handle = app.handle().clone();
            if let Some(icon) = app.default_window_icon().cloned() {
                TrayIconBuilder::new()
                    .tooltip("SillyTavern 启动器")
                    .icon(icon)
                    .show_menu_on_left_click(false)
                    .on_tray_icon_event(move |_tray, event| match event {
                        TrayIconEvent::Click { .. } | TrayIconEvent::DoubleClick { .. } => {
                            if let Some(window) = handle.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    })
                    .build(app)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            runtime_commands::get_runtime_status,
            runtime_commands::install_runtime,
            commands::get_launcher_settings,
            commands::save_launcher_settings,
            commands::get_tavern_status,
            commands::install_tavern,
            commands::update_tavern,
            commands::start_tavern,
            commands::terminal_create,
            commands::terminal_write,
            commands::terminal_kill,
            commands::terminal_list,
            window_commands::window_minimize,
            window_commands::window_hide,
            window_commands::window_toggle_maximize,
            window_commands::window_close,
            window_commands::window_start_dragging,
            window_commands::get_app_info,
            window_commands::open_install_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

