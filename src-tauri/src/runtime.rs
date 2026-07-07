use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::process_runner;
use crate::runtime_download::download_file;

pub(crate) const NODE_VERSION: &str = "v24.18.0";
pub(crate) const NODE_URL: &str =
    "https://cnb.cool/clya.top/cloudnodegit/-/git/raw/main/node-v24.18.0-win-x64.zip";
pub(crate) const GIT_URL: &str =
    "https://cnb.cool/clya.top/cloudnodegit/-/git/raw/main/PortableGit-2.55.0.2-64-bit.7z.exe";
pub const NPM_REGISTRY: &str = "https://registry.npmmirror.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePaths {
    pub base_dir: PathBuf,
    pub node_dir: PathBuf,
    pub git_dir: PathBuf,
    pub node_exe: PathBuf,
    pub npm_cmd: PathBuf,
    pub npmrc: PathBuf,
    pub git_exe: PathBuf,
}

impl RuntimePaths {
    pub fn new(install_dir: &Path) -> Self {
        let base_dir = install_dir.join("runtime");
        let node_dir = base_dir.join("node");
        let git_dir = base_dir.join("git");
        let node_exe = node_dir.join("node.exe");
        let npm_cmd = node_dir.join("npm.cmd");
        let npmrc = base_dir.join("npmrc");
        let git_exe = git_dir.join("cmd").join("git.exe");
        Self {
            base_dir,
            node_dir,
            git_dir,
            node_exe,
            npm_cmd,
            npmrc,
            git_exe,
        }
    }

    pub fn node_installed(&self) -> bool {
        self.node_exe.exists()
    }

    pub fn git_installed(&self) -> bool {
        self.git_exe.exists()
    }

    pub fn env_path(&self) -> String {
        runtime_path_entries(self)
            .into_iter()
            .chain(windows_system_path_entries())
            .collect::<Vec<_>>()
            .join(";")
    }

    pub fn env_vars(&self) -> BTreeMap<String, String> {
        let mut vars = preserved_windows_env_vars();
        vars.extend([
            ("PATH".to_string(), self.env_path()),
            ("NPM_CONFIG_REGISTRY".to_string(), NPM_REGISTRY.to_string()),
            (
                "NPM_CONFIG_USERCONFIG".to_string(),
                self.npmrc.to_string_lossy().to_string(),
            ),
        ]);
        vars
    }

    pub fn npm_cli_js(&self) -> PathBuf {
        self.node_dir
            .join("node_modules")
            .join("npm")
            .join("bin")
            .join("npm-cli.js")
    }
}

fn runtime_path_entries(paths: &RuntimePaths) -> Vec<String> {
    vec![
        paths.node_dir.to_string_lossy().to_string(),
        paths.git_dir.join("cmd").to_string_lossy().to_string(),
        paths.git_dir.join("bin").to_string_lossy().to_string(),
    ]
}

fn windows_system_path_entries() -> Vec<String> {
    let windows_dir = std::env::var("SystemRoot")
        .or_else(|_| std::env::var("WINDIR"))
        .unwrap_or_else(|_| r"C:\Windows".to_string());
    let windows_dir = PathBuf::from(windows_dir);

    [
        windows_dir.join("System32"),
        windows_dir.clone(),
        windows_dir.join("System32").join("Wbem"),
        windows_dir
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0"),
    ]
    .into_iter()
    .map(|path| path.to_string_lossy().to_string())
    .collect()
}

fn preserved_windows_env_vars() -> BTreeMap<String, String> {
    [
        "SystemRoot",
        "WINDIR",
        "ComSpec",
        "TEMP",
        "TMP",
        "PATHEXT",
        "USERPROFILE",
        "APPDATA",
        "LOCALAPPDATA",
        "ProgramData",
        "HOMEDRIVE",
        "HOMEPATH",
    ]
    .into_iter()
    .filter_map(|key| {
        std::env::var(key)
            .ok()
            .map(|value| (key.to_string(), value))
    })
    .collect()
}

pub async fn install_node(install_dir: &Path) -> Result<RuntimePaths, String> {
    let paths = RuntimePaths::new(install_dir);
    if paths.node_installed() {
        return Ok(paths);
    }

    let tmp_zip = paths.base_dir.join("node.zip");
    fs::create_dir_all(&paths.base_dir)
        .await
        .map_err(|e| format!("创建 runtime 目录失败: {}", e))?;

    download_file(NODE_URL, &tmp_zip, |_| {}).await?;

    extract_zip(&tmp_zip, &paths.base_dir).await?;

    let extracted_dir = paths
        .base_dir
        .join(format!("node-{}-win-x64", NODE_VERSION));
    if extracted_dir.exists() {
        if paths.node_dir.exists() {
            fs::remove_dir_all(&paths.node_dir)
                .await
                .map_err(|e| format!("清理旧 node 目录失败: {}", e))?;
        }
        fs::rename(&extracted_dir, &paths.node_dir)
            .await
            .map_err(|e| format!("重命名 node 目录失败: {}", e))?;
    }

    let _ = fs::remove_file(&tmp_zip).await;
    configure_npm(&paths).await?;
    Ok(paths)
}

pub async fn install_git(install_dir: &Path) -> Result<RuntimePaths, String> {
    let paths = RuntimePaths::new(install_dir);
    if paths.git_installed() {
        return Ok(paths);
    }

    fs::create_dir_all(&paths.base_dir)
        .await
        .map_err(|e| format!("创建 runtime 目录失败: {}", e))?;

    let tmp_exe = paths.base_dir.join("PortableGit.exe");
    download_file(GIT_URL, &tmp_exe, |_| {}).await?;

    fs::create_dir_all(&paths.git_dir)
        .await
        .map_err(|e| format!("创建 git 目录失败: {}", e))?;

    let git_dir_str = paths.git_dir.to_string_lossy().to_string();
    let tmp_exe_str = tmp_exe.to_string_lossy().to_string();
    let status = process_runner::hidden_tokio_command(Path::new(&tmp_exe_str))
        .args(["-o", &git_dir_str, "-y"])
        .status()
        .await
        .map_err(|e| format!("解压 Git 失败: {}", e))?;

    if !status.success() {
        return Err("Git 解压返回非零退出码".to_string());
    }

    let _ = fs::remove_file(&tmp_exe).await;
    Ok(paths)
}

pub async fn configure_npm(paths: &RuntimePaths) -> Result<(), String> {
    fs::create_dir_all(&paths.base_dir)
        .await
        .map_err(|e| format!("创建 runtime 目录失败: {}", e))?;
    fs::write(&paths.npmrc, format!("registry={}\n", NPM_REGISTRY))
        .await
        .map_err(|e| format!("写入 npm 配置失败: {}", e))
}

async fn extract_zip(zip_path: &Path, dest: &Path) -> Result<(), String> {
    let zip_path = zip_path.to_path_buf();
    let dest = dest.to_path_buf();

    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&zip_path).map_err(|e| format!("打开 zip 失败: {}", e))?;
        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("解析 zip 失败: {}", e))?;

        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| format!("读取 zip 条目失败: {}", e))?;
            let outpath = dest.join(entry.mangled_name());

            if entry.is_dir() {
                std::fs::create_dir_all(&outpath).map_err(|e| format!("创建目录失败: {}", e))?;
            } else {
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("创建父目录失败: {}", e))?;
                }
                let mut outfile =
                    std::fs::File::create(&outpath).map_err(|e| format!("创建文件失败: {}", e))?;
                std::io::copy(&mut entry, &mut outfile)
                    .map_err(|e| format!("写入文件失败: {}", e))?;
            }
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("解压任务失败: {}", e))?
}
