#[tauri::command]
pub async fn window_minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn window_hide(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn window_toggle_maximize(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().map_err(|e| e.to_string())? {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn window_close(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn window_start_dragging(window: tauri::Window) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_app_info() -> Result<crate::paths::AppInfo, String> {
    crate::paths::app_info()
}

#[tauri::command]
pub async fn open_install_path(relative: String) -> Result<(), String> {
    let base = crate::paths::current_install_dir()?;
    let target = crate::paths::resolve_open_path(&base, &relative);
    if !target.exists() {
        std::fs::create_dir_all(&target)
            .map_err(|e| format!("无法创建目录 {}: {}", target.display(), e))?;
    }

    open_path_in_explorer(&target)
}

fn open_path_in_explorer(path: &std::path::Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        std::process::Command::new("explorer")
            .arg(path.as_os_str())
            .spawn()
            .map_err(|e| format!("打开路径失败: {}", e))?;
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        tauri_plugin_opener::open_path(path, None::<&str>)
            .map_err(|e| format!("打开路径失败: {}", e))
    }
}