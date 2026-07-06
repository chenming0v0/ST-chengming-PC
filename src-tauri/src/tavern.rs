use std::path::{Path, PathBuf};
use tokio::process::Command;
use serde::{Deserialize, Serialize};

use crate::runtime::RuntimePaths;

const ST_REPO: &str = "https://gitcode.com/GitHub_Trending/si/SillyTavern.git";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TavernStatus {
    pub installed: bool,
    pub path: PathBuf,
    pub running: bool,
    pub version: Option<String>,
}

pub fn tavern_dir(install_dir: &Path) -> PathBuf {
    install_dir.join("SillyTavern")
}

pub fn is_installed(install_dir: &Path) -> bool {
    let st_dir = tavern_dir(install_dir);
    st_dir.join("package.json").exists()
}

pub async fn get_status(install_dir: &Path) -> TavernStatus {
    let path = tavern_dir(install_dir);
    let installed = is_installed(install_dir);
    let version = if installed {
        read_version(&path).await.ok()
    } else {
        None
    };

    TavernStatus {
        installed,
        path,
        running: false,
        version,
    }
}

async fn read_version(st_dir: &Path) -> Result<String, String> {
    let pkg_json = st_dir.join("package.json");
    let content = tokio::fs::read_to_string(&pkg_json)
        .await
        .map_err(|e| format!("读取 package.json 失败: {}", e))?;
    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析 package.json 失败: {}", e))?;
    json.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "package.json 中无 version 字段".to_string())
}

pub async fn install(install_dir: &Path, runtime: &RuntimePaths) -> Result<String, String> {
    let st_dir = tavern_dir(install_dir);

    if st_dir.exists() {
        return Err("SillyTavern 目录已存在，请先卸载或使用更新功能".to_string());
    }

    let env_path = build_env_path(runtime);

    let output = Command::new(&runtime.git_exe)
        .arg("clone")
        .arg(ST_REPO)
        .arg(&st_dir)
        .env("PATH", &env_path)
        .output()
        .await
        .map_err(|e| format!("执行 git clone 失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git clone 失败: {}", stderr));
    }

    let npm_install = Command::new(&runtime.npm_cmd)
        .arg("install")
        .current_dir(&st_dir)
        .env("PATH", &env_path)
        .output()
        .await
        .map_err(|e| format!("执行 npm install 失败: {}", e))?;

    if !npm_install.status.success() {
        let stderr = String::from_utf8_lossy(&npm_install.stderr);
        return Err(format!("npm install 失败: {}", stderr));
    }

    let version = read_version(&st_dir).await.unwrap_or_else(|_| "unknown".to_string());
    Ok(format!("SillyTavern {} 安装成功", version))
}

pub async fn update(install_dir: &Path, runtime: &RuntimePaths) -> Result<String, String> {
    let st_dir = tavern_dir(install_dir);

    if !st_dir.join(".git").exists() {
        return Err("SillyTavern 未安装或不是 git 仓库".to_string());
    }

    let env_path = build_env_path(runtime);

    let output = Command::new(&runtime.git_exe)
        .args(["pull", "--rebase"])
        .current_dir(&st_dir)
        .env("PATH", &env_path)
        .output()
        .await
        .map_err(|e| format!("执行 git pull 失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git pull 失败: {}", stderr));
    }

    let pull_msg = String::from_utf8_lossy(&output.stdout).to_string();

    if !pull_msg.contains("Already up to date") {
        let npm_install = Command::new(&runtime.npm_cmd)
            .arg("install")
            .current_dir(&st_dir)
            .env("PATH", &env_path)
            .output()
            .await
            .map_err(|e| format!("执行 npm install 失败: {}", e))?;

        if !npm_install.status.success() {
            let stderr = String::from_utf8_lossy(&npm_install.stderr);
            return Err(format!("更新后 npm install 失败: {}", stderr));
        }
    }

    let version = read_version(&st_dir).await.unwrap_or_else(|_| "unknown".to_string());
    Ok(format!("SillyTavern 已更新到 {}\n{}", version, pull_msg.trim()))
}

fn build_env_path(runtime: &RuntimePaths) -> String {
    runtime.env_path()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn tavern_child_path_uses_only_embedded_runtime() {
        std::env::set_var("PATH", r"C:\Windows\System32;C:\Program Files\Git\cmd");
        let runtime = RuntimePaths::new(&PathBuf::from(r"C:\launcher"));

        let path = build_env_path(&runtime);

        assert!(path.contains(r"C:\launcher\runtime\node"));
        assert!(path.contains(r"C:\launcher\runtime\git\cmd"));
        assert!(path.contains(r"C:\launcher\runtime\git\bin"));
        assert!(!path.contains(r"C:\Windows\System32"));
        assert!(!path.contains(r"C:\Program Files\Git\cmd"));
    }
}
