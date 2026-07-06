use crate::paths;
use crate::runtime::{self, RuntimePaths};
use crate::tavern;
use crate::terminal::SharedTerminalManager;
use crate::terminal::TerminalSession;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{Emitter, State};

#[derive(Clone, Serialize)]
pub struct ProgressEvent {
    pub stage: String,
    pub percent: u32,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RuntimeStatus {
    pub node_installed: bool,
    pub git_installed: bool,
    pub node_version: Option<String>,
    pub git_version: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TavernStatus {
    pub installed: bool,
    pub path: String,
    pub version: Option<String>,
    pub running: bool,
}

// --- Runtime Commands ---

#[tauri::command]
pub async fn get_runtime_status(app: tauri::AppHandle) -> Result<RuntimeStatus, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);
    Ok(RuntimeStatus {
        node_installed: paths.node_installed(),
        git_installed: paths.git_installed(),
        node_version: detect_version(&paths.node_exe),
        git_version: detect_version(&paths.git_exe),
    })
}

#[tauri::command]
pub async fn install_runtime(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
    let base = get_base_dir(&app)?;

    window.emit("install-progress", ProgressEvent {
        stage: "node".into(),
        percent: 0,
        message: "正在下载 Node.js 24...".into(),
    }).ok();

    runtime::install_node(&base).await?;

    window.emit("install-progress", ProgressEvent {
        stage: "node".into(),
        percent: 50,
        message: "Node.js 安装完成".into(),
    }).ok();

    window.emit("install-progress", ProgressEvent {
        stage: "git".into(),
        percent: 50,
        message: "正在下载 Git...".into(),
    }).ok();

    runtime::install_git(&base).await?;

    window.emit("install-progress", ProgressEvent {
        stage: "git".into(),
        percent: 100,
        message: "Git 安装完成".into(),
    }).ok();

    Ok(())
}

// --- Tavern Commands ---

#[tauri::command]
pub async fn get_tavern_status(app: tauri::AppHandle) -> Result<TavernStatus, String> {
    let base = get_base_dir(&app)?;
    let status = tavern::get_status(&base).await;
    Ok(TavernStatus {
        installed: status.installed,
        path: status.path.to_string_lossy().to_string(),
        version: status.version,
        running: status.running,
    })
}

#[tauri::command]
pub async fn install_tavern(app: tauri::AppHandle, window: tauri::Window) -> Result<String, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);

    if !paths.node_installed() || !paths.git_installed() {
        return Err("请先安装运行时环境 (Node.js + Git)".into());
    }

    window.emit("install-progress", ProgressEvent {
        stage: "tavern".into(),
        percent: 0,
        message: "正在克隆 SillyTavern...".into(),
    }).ok();

    let result = tavern::install(&base, &paths).await?;

    window.emit("install-progress", ProgressEvent {
        stage: "tavern".into(),
        percent: 100,
        message: "SillyTavern 安装完成".into(),
    }).ok();

    Ok(result)
}

#[tauri::command]
pub async fn update_tavern(app: tauri::AppHandle, window: tauri::Window) -> Result<String, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);

    window.emit("install-progress", ProgressEvent {
        stage: "update".into(),
        percent: 0,
        message: "正在检查更新...".into(),
    }).ok();

    let result = tavern::update(&base, &paths).await?;

    window.emit("install-progress", ProgressEvent {
        stage: "update".into(),
        percent: 100,
        message: format!("更新完成: {}", result),
    }).ok();

    Ok(result)
}

#[tauri::command]
pub async fn start_tavern(
    app: tauri::AppHandle,
    terminal_mgr: State<'_, SharedTerminalManager>,
) -> Result<String, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);
    let st_dir = tavern::tavern_dir(&base);

    if !st_dir.exists() {
        return Err("SillyTavern 未安装".into());
    }

    let env_path = paths.env_path();

    let mut mgr = terminal_mgr.lock().map_err(|e| e.to_string())?;
    let session_id = mgr.spawn_session(
        app,
        "SillyTavern".into(),
        st_dir,
        Some(env_path),
    )?;

    mgr.write_to_session(&session_id, "node server.js")?;

    Ok(session_id)
}

// --- Terminal Commands ---

#[tauri::command]
pub async fn terminal_create(
    app: tauri::AppHandle,
    terminal_mgr: State<'_, SharedTerminalManager>,
    title: Option<String>,
) -> Result<String, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);
    let env_path = paths.env_path();

    let mut mgr = terminal_mgr.lock().map_err(|e| e.to_string())?;
    mgr.spawn_session(
        app,
        title.unwrap_or_else(|| "PowerShell".into()),
        base,
        Some(env_path),
    )
}

#[tauri::command]
pub async fn terminal_write(
    terminal_mgr: State<'_, SharedTerminalManager>,
    session_id: String,
    input: String,
) -> Result<(), String> {
    let mut mgr = terminal_mgr.lock().map_err(|e| e.to_string())?;
    mgr.write_to_session(&session_id, &input)
}

#[tauri::command]
pub async fn terminal_kill(
    terminal_mgr: State<'_, SharedTerminalManager>,
    session_id: String,
) -> Result<(), String> {
    let mut mgr = terminal_mgr.lock().map_err(|e| e.to_string())?;
    mgr.kill_session(&session_id)
}

#[tauri::command]
pub async fn terminal_list(
    terminal_mgr: State<'_, SharedTerminalManager>,
) -> Result<Vec<TerminalSession>, String> {
    let mgr = terminal_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.list_sessions())
}

// --- Window Commands ---

#[tauri::command]
pub async fn window_minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
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

// --- Helpers ---

fn get_base_dir(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    paths::current_app_paths().map(|paths| paths.install_dir)
}

fn detect_version(exe: &PathBuf) -> Option<String> {
    if !exe.exists() {
        return None;
    }
    std::process::Command::new(exe)
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}
