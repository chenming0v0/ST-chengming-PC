use crate::events::ProgressEvent;
use crate::paths;
use crate::process_runner;
use crate::runtime::{self, RuntimePaths};
use crate::settings::{self, LauncherSettings};
use crate::tavern;
use crate::terminal::SharedTerminalManager;
use crate::terminal::TerminalSession;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{Emitter, State};

#[derive(Clone, Serialize, Deserialize)]
pub struct TavernStatus {
    pub installed: bool,
    pub path: String,
    pub version: Option<String>,
    pub running: bool,
}

// --- Launcher Settings Commands ---

#[tauri::command]
pub async fn get_launcher_settings(app: tauri::AppHandle) -> Result<LauncherSettings, String> {
    let base = get_base_dir(&app)?;
    settings::load_launcher_settings(&base).await
}

#[tauri::command]
pub async fn save_launcher_settings(
    app: tauri::AppHandle,
    settings: LauncherSettings,
) -> Result<LauncherSettings, String> {
    let base = get_base_dir(&app)?;
    settings::save_launcher_settings(&base, settings).await
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
pub async fn install_tavern(
    app: tauri::AppHandle,
    window: tauri::Window,
) -> Result<String, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);
    let settings = settings::load_launcher_settings(&base).await?;

    if !paths.node_installed() || !paths.git_installed() {
        return Err("请先安装运行时环境 (Node.js + Git)".into());
    }

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "tavern".into(),
                percent: 0,
                message: "正在克隆 SillyTavern...".into(),
            },
        )
        .ok();

    let mut env_vars = runtime_env_vars(&paths);
    settings::apply_proxy_env(&mut env_vars, &settings);
    let result = tavern::install_with_env(&base, &paths, &env_vars).await?;

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "tavern".into(),
                percent: 100,
                message: "SillyTavern 安装完成".into(),
            },
        )
        .ok();

    Ok(result)
}

#[tauri::command]
pub async fn update_tavern(
    app: tauri::AppHandle,
    window: tauri::Window,
    settings: Option<LauncherSettings>,
) -> Result<String, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);
    let settings = match settings {
        Some(settings) => settings.normalized()?,
        None => settings::load_launcher_settings(&base).await?,
    };

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "update".into(),
                percent: 0,
                message: "正在检查更新...".into(),
            },
        )
        .ok();

    let mut env_vars = runtime_env_vars(&paths);
    settings::apply_proxy_env(&mut env_vars, &settings);
    let result = tavern::update_with_env(&base, &paths, &env_vars).await?;

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "update".into(),
                percent: 100,
                message: format!("更新完成: {}", result),
            },
        )
        .ok();

    Ok(result)
}

#[tauri::command]
pub async fn start_tavern(
    app: tauri::AppHandle,
    terminal_mgr: State<'_, SharedTerminalManager>,
    settings: Option<LauncherSettings>,
) -> Result<String, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);
    let st_dir = tavern::tavern_dir(&base);
    let settings = match settings {
        Some(settings) => settings.normalized()?,
        None => settings::load_launcher_settings(&base).await?,
    };

    if !paths.node_installed() || !paths.git_installed() {
        return Err("请先安装运行时环境 (Node.js + Git)".into());
    }

    if !st_dir.exists() {
        return Err("SillyTavern 未安装".into());
    }

    runtime::configure_npm(&paths).await?;
    let mut env_vars = runtime_env_vars(&paths);
    settings::apply_proxy_env(&mut env_vars, &settings);
    let startup_session = "startup".to_string();

    process_runner::run_hidden_command(
        &app,
        &startup_session,
        &paths.node_exe,
        &[
            paths.npm_cli_js().to_string_lossy().to_string(),
            "install".to_string(),
        ],
        &st_dir,
        &env_vars,
    )
    .await?;

    let mut mgr = terminal_mgr.lock().map_err(|e| e.to_string())?;
    let session_id = mgr.spawn_process_session(
        app,
        "SillyTavern".into(),
        st_dir,
        env_vars,
        paths.node_exe,
        settings::build_tavern_launch_args(&settings),
    )?;

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
    let env_vars = runtime_env_vars(&paths);

    let mut mgr = terminal_mgr.lock().map_err(|e| e.to_string())?;
    mgr.spawn_session(
        app,
        title.unwrap_or_else(|| "PowerShell".into()),
        base,
        Some(env_vars),
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

// --- Helpers ---

fn get_base_dir(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    paths::current_app_paths().map(|paths| paths.install_dir)
}

fn runtime_env_vars(paths: &RuntimePaths) -> HashMap<String, String> {
    paths.env_vars().into_iter().collect()
}
