use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use reqwest::Client;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

const NODE_VERSION: &str = "v24.18.0";
const NODE_URL: &str =
    "https://cnb.cool/clya.top/cloudnodegit/-/git/raw/main/node-v24.18.0-win-x64.zip";
const GIT_URL: &str =
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
        Self { base_dir, node_dir, git_dir, node_exe, npm_cmd, npmrc, git_exe }
    }

    pub fn node_installed(&self) -> bool {
        self.node_exe.exists()
    }

    pub fn git_installed(&self) -> bool {
        self.git_exe.exists()
    }

    pub fn env_path(&self) -> String {
        let node_path = self.node_dir.to_string_lossy();
        let git_cmd_path = self.git_dir.join("cmd").to_string_lossy().to_string();
        let git_bin_path = self.git_dir.join("bin").to_string_lossy().to_string();
        format!("{};{};{}", node_path, git_cmd_path, git_bin_path)
    }

    pub fn env_vars(&self) -> BTreeMap<String, String> {
        BTreeMap::from([
            ("PATH".to_string(), self.env_path()),
            ("NPM_CONFIG_REGISTRY".to_string(), NPM_REGISTRY.to_string()),
            (
                "NPM_CONFIG_USERCONFIG".to_string(),
                self.npmrc.to_string_lossy().to_string(),
            ),
        ])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub stage: String,
    pub downloaded: u64,
    pub total: Option<u64>,
    pub percent: f64,
}

pub async fn download_file(
    url: &str,
    dest: &Path,
    on_progress: impl Fn(DownloadProgress),
) -> Result<(), String> {
    let client = Client::new();
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("下载请求失败: {}", e))?;

    let total = resp.content_length();
    let mut stream = resp.bytes_stream();

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).await.map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let mut file = fs::File::create(dest)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;

    let mut downloaded: u64 = 0;
    use tokio::io::AsyncWriteExt;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("下载数据失败: {}", e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("写入文件失败: {}", e))?;
        downloaded += chunk.len() as u64;
        let percent = total.map(|t| (downloaded as f64 / t as f64) * 100.0).unwrap_or(0.0);
        on_progress(DownloadProgress {
            stage: "downloading".to_string(),
            downloaded,
            total,
            percent,
        });
    }

    Ok(())
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

    let extracted_dir = paths.base_dir.join(format!("node-{}-win-x64", NODE_VERSION));
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
    let status = tokio::process::Command::new(&tmp_exe_str)
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
        let file = std::fs::File::open(&zip_path)
            .map_err(|e| format!("打开 zip 失败: {}", e))?;
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| format!("解析 zip 失败: {}", e))?;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)
                .map_err(|e| format!("读取 zip 条目失败: {}", e))?;
            let outpath = dest.join(entry.mangled_name());

            if entry.is_dir() {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| format!("创建目录失败: {}", e))?;
            } else {
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("创建父目录失败: {}", e))?;
                }
                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| format!("创建文件失败: {}", e))?;
                std::io::copy(&mut entry, &mut outfile)
                    .map_err(|e| format!("写入文件失败: {}", e))?;
            }
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("解压任务失败: {}", e))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn runtime_download_sources_use_cloudnodegit_mirror() {
        assert!(NODE_VERSION.starts_with("v24."));
        assert_eq!(
            NODE_URL,
            "https://cnb.cool/clya.top/cloudnodegit/-/git/raw/main/node-v24.18.0-win-x64.zip"
        );
        assert_eq!(
            GIT_URL,
            "https://cnb.cool/clya.top/cloudnodegit/-/git/raw/main/PortableGit-2.55.0.2-64-bit.7z.exe"
        );
    }

    #[test]
    fn runtime_paths_are_project_local() {
        let install_dir = PathBuf::from(r"C:\launcher");
        let paths = RuntimePaths::new(&install_dir);

        assert_eq!(paths.base_dir, install_dir.join("runtime"));
        assert_eq!(paths.node_exe, install_dir.join("runtime").join("node").join("node.exe"));
        assert_eq!(paths.npm_cmd, install_dir.join("runtime").join("node").join("npm.cmd"));
        assert_eq!(paths.npmrc, install_dir.join("runtime").join("npmrc"));
        assert_eq!(paths.git_exe, install_dir.join("runtime").join("git").join("cmd").join("git.exe"));
    }

    #[test]
    fn runtime_env_uses_private_path_and_taobao_registry() {
        let install_dir = PathBuf::from(r"C:\launcher");
        let paths = RuntimePaths::new(&install_dir);

        let env = paths.env_vars();

        assert_eq!(env.get("PATH").unwrap(), &paths.env_path());
        assert_eq!(env.get("NPM_CONFIG_REGISTRY").unwrap(), NPM_REGISTRY);
        assert_eq!(
            env.get("NPM_CONFIG_USERCONFIG").unwrap(),
            &install_dir.join("runtime").join("npmrc").to_string_lossy().to_string()
        );
    }
}
