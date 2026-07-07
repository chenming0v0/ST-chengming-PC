use crate::events::ProgressEvent;
use crate::paths;
use crate::process_runner;
use crate::runtime::{self, RuntimePaths};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::Emitter;

#[derive(Clone, Serialize, Deserialize)]
pub struct RuntimeStatus {
    pub node_installed: bool,
    pub git_installed: bool,
    pub node_version: Option<String>,
    pub git_version: Option<String>,
}

#[tauri::command]
pub async fn get_runtime_status(app: tauri::AppHandle) -> Result<RuntimeStatus, String> {
    let base = get_base_dir(&app)?;
    let paths = RuntimePaths::new(&base);
    let env_vars = runtime_env_vars(&paths);
    Ok(RuntimeStatus {
        node_installed: paths.node_installed(),
        git_installed: paths.git_installed(),
        node_version: detect_version(&paths.node_exe, &env_vars),
        git_version: detect_version(&paths.git_exe, &env_vars),
    })
}

#[tauri::command]
pub async fn install_runtime(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
    let base = get_base_dir(&app)?;

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "node".into(),
                percent: 0,
                message: "正在下载 Node.js 24...".into(),
            },
        )
        .ok();

    runtime::install_node(&base).await?;
    runtime::configure_npm(&RuntimePaths::new(&base)).await?;

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "node".into(),
                percent: 50,
                message: "Node.js 安装完成".into(),
            },
        )
        .ok();

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "git".into(),
                percent: 50,
                message: "正在下载 Git...".into(),
            },
        )
        .ok();

    runtime::install_git(&base).await?;

    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: "git".into(),
                percent: 100,
                message: "Git 安装完成".into(),
            },
        )
        .ok();

    Ok(())
}

fn get_base_dir(_app: &tauri::AppHandle) -> Result<PathBuf, String> {
    paths::current_app_paths().map(|paths| paths.install_dir)
}

fn detect_version(exe: &PathBuf, env_vars: &HashMap<String, String>) -> Option<String> {
    if !exe.exists() {
        return None;
    }
    process_runner::hidden_std_command(exe)
        .arg("--version")
        .envs(env_vars)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

fn runtime_env_vars(paths: &RuntimePaths) -> HashMap<String, String> {
    paths.env_vars().into_iter().collect()
}
