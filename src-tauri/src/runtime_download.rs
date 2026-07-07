use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

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
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
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
        let percent = total
            .map(|total| (downloaded as f64 / total as f64) * 100.0)
            .unwrap_or(0.0);
        on_progress(DownloadProgress {
            stage: "downloading".to_string(),
            downloaded,
            total,
            percent,
        });
    }

    Ok(())
}
