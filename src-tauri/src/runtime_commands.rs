use crate::events::ProgressEvent;
use crate::paths;
use crate::process_runner;
use crate::runtime::{self, RuntimePaths};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
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
    emit_progress(
        &window,
        "check",
        2,
        "正在检测可写安装目录与运行时状态...",
    );

    let resolved = paths::resolve_install_location().map_err(|error| {
        format!("安装前检测失败：{error}")
    })?;
    let base = resolved.path;

    emit_progress(
        &window,
        "check",
        8,
        &format!(
            "{}。安装目录：{}",
            resolved.note,
            base.display()
        ),
    );

    // Fail fast if the selected data root cannot host runtime downloads.
    std::fs::create_dir_all(base.join("runtime")).map_err(|error| {
        format!(
            "无法创建 runtime 目录（{}）：{error}\n启动器会优先使用“文档\\\\chengming”，请确认该位置可写。",
            base.join("runtime").display()
        )
    })?;

    emit_progress(&window, "node", 10, "准备下载 Node.js 24...");
    install_node_with_progress(&window, &base).await?;

    emit_progress(&window, "node", 48, "正在配置 npm 镜像源...");
    runtime::configure_npm(&RuntimePaths::new(&base)).await?;
    emit_progress(&window, "node", 52, "Node.js 安装完成");

    emit_progress(&window, "git", 55, "准备下载 Portable Git...");
    install_git_with_progress(&window, &base).await?;
    emit_progress(&window, "git", 100, "Git 安装完成");

    Ok(())
}

async fn install_node_with_progress(window: &tauri::Window, base: &PathBuf) -> Result<(), String> {
    let paths = RuntimePaths::new(base);
    if paths.node_installed() {
        emit_progress(window, "node", 45, "检测到 Node.js 已安装，跳过下载");
        return Ok(());
    }

    let tmp_zip = paths.base_dir.join("node.zip");
    tokio::fs::create_dir_all(&paths.base_dir)
        .await
        .map_err(|e| format!("创建 runtime 目录失败: {e}"))?;

    let last_percent = Arc::new(AtomicU64::new(0));
    let last_percent_cb = Arc::clone(&last_percent);
    let window_cb = window.clone();
    crate::runtime_download::download_file(crate::runtime::NODE_URL, &tmp_zip, move |progress| {
        let mapped = 10 + ((progress.percent / 100.0) * 28.0).round() as u32;
        let mapped = mapped.min(38);
        let prev = last_percent_cb.load(Ordering::Relaxed) as u32;
        if mapped > prev {
            last_percent_cb.store(mapped as u64, Ordering::Relaxed);
            let total = progress
                .total
                .map(|value| format!("{:.1} MB", value as f64 / 1_048_576.0))
                .unwrap_or_else(|| "未知大小".to_string());
            emit_progress(
                &window_cb,
                "node",
                mapped,
                &format!(
                    "正在下载 Node.js... {:.1}%（{:.1} MB / {total}）",
                    progress.percent,
                    progress.downloaded as f64 / 1_048_576.0
                ),
            );
        }
    })
    .await
    .map_err(|error| format!("Node.js 下载失败：{error}"))?;

    emit_progress(window, "node", 40, "正在解压 Node.js...");
    extract_node_zip(window, &tmp_zip, &paths).await?;
    let _ = tokio::fs::remove_file(&tmp_zip).await;
    Ok(())
}

async fn extract_node_zip(
    window: &tauri::Window,
    tmp_zip: &PathBuf,
    paths: &RuntimePaths,
) -> Result<(), String> {
    // Reuse runtime extract helpers via public install_node after download is complete is harder,
    // so call the existing install path only if node still missing after download.
    // Directly reuse runtime::install_node would re-download. Keep local rename flow here.
    use zip::ZipArchive;

    let zip_path = tmp_zip.clone();
    let dest = paths.base_dir.clone();
    let node_dir = paths.node_dir.clone();
    let node_version = crate::runtime::NODE_VERSION.to_string();

    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&zip_path).map_err(|e| format!("打开 zip 失败: {e}"))?;
        let mut archive = ZipArchive::new(file).map_err(|e| format!("解析 zip 失败: {e}"))?;
        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| format!("读取 zip 条目失败: {e}"))?;
            let outpath = dest.join(entry.mangled_name());
            if entry.is_dir() {
                std::fs::create_dir_all(&outpath).map_err(|e| format!("创建目录失败: {e}"))?;
            } else {
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("创建父目录失败: {e}"))?;
                }
                let mut outfile =
                    std::fs::File::create(&outpath).map_err(|e| format!("创建文件失败: {e}"))?;
                std::io::copy(&mut entry, &mut outfile)
                    .map_err(|e| format!("写入文件失败: {e}"))?;
            }
        }

        let extracted_dir = dest.join(format!("node-{node_version}-win-x64"));
        if extracted_dir.exists() {
            if node_dir.exists() {
                std::fs::remove_dir_all(&node_dir)
                    .map_err(|e| format!("清理旧 node 目录失败: {e}"))?;
            }
            std::fs::rename(&extracted_dir, &node_dir)
                .map_err(|e| format!("重命名 node 目录失败: {e}"))?;
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("解压任务失败: {e}"))??;

    emit_progress(window, "node", 45, "Node.js 解压完成");
    Ok(())
}

async fn install_git_with_progress(window: &tauri::Window, base: &PathBuf) -> Result<(), String> {
    let paths = RuntimePaths::new(base);
    if paths.git_installed() {
        emit_progress(window, "git", 95, "检测到 Git 已安装，跳过下载");
        return Ok(());
    }

    tokio::fs::create_dir_all(&paths.base_dir)
        .await
        .map_err(|e| format!("创建 runtime 目录失败: {e}"))?;

    let tmp_exe = paths.base_dir.join("PortableGit.exe");
    let last_percent = Arc::new(AtomicU64::new(0));
    let last_percent_cb = Arc::clone(&last_percent);
    let window_cb = window.clone();
    crate::runtime_download::download_file(crate::runtime::GIT_URL, &tmp_exe, move |progress| {
        let mapped = 55 + ((progress.percent / 100.0) * 30.0).round() as u32;
        let mapped = mapped.min(85);
        let prev = last_percent_cb.load(Ordering::Relaxed) as u32;
        if mapped > prev {
            last_percent_cb.store(mapped as u64, Ordering::Relaxed);
            let total = progress
                .total
                .map(|value| format!("{:.1} MB", value as f64 / 1_048_576.0))
                .unwrap_or_else(|| "未知大小".to_string());
            emit_progress(
                &window_cb,
                "git",
                mapped,
                &format!(
                    "正在下载 Git... {:.1}%（{:.1} MB / {total}）",
                    progress.percent,
                    progress.downloaded as f64 / 1_048_576.0
                ),
            );
        }
    })
    .await
    .map_err(|error| format!("Git 下载失败：{error}"))?;

    emit_progress(window, "git", 88, "正在解压 Portable Git...");
    tokio::fs::create_dir_all(&paths.git_dir)
        .await
        .map_err(|e| format!("创建 git 目录失败: {e}"))?;

    let git_dir_str = paths.git_dir.to_string_lossy().to_string();
    let tmp_exe_str = tmp_exe.to_string_lossy().to_string();
    let status = process_runner::hidden_tokio_command(std::path::Path::new(&tmp_exe_str))
        .args(["-o", &git_dir_str, "-y"])
        .status()
        .await
        .map_err(|e| format!("解压 Git 失败: {e}"))?;

    if !status.success() {
        return Err("Git 解压返回非零退出码。可能是下载包损坏，请重试。".to_string());
    }

    let _ = tokio::fs::remove_file(&tmp_exe).await;
    if !paths.git_installed() {
        return Err("Git 解压后未找到 git.exe，安装失败。".to_string());
    }
    Ok(())
}

fn emit_progress(window: &tauri::Window, stage: &str, percent: u32, message: &str) {
    window
        .emit(
            "install-progress",
            ProgressEvent {
                stage: stage.into(),
                percent,
                message: message.into(),
            },
        )
        .ok();
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