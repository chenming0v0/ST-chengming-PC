use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use tokio::fs;
use tokio::time::timeout;

const CONNECT_TIMEOUT_SECS: u64 = 20;
const STALL_TIMEOUT_SECS: u64 = 90;

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
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
        .pool_idle_timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建下载客户端失败: {e}"))?;

    let resp = timeout(
        Duration::from_secs(CONNECT_TIMEOUT_SECS + 10),
        client.get(url).send(),
    )
    .await
    .map_err(|_| {
        format!(
            "连接下载源超时（{CONNECT_TIMEOUT_SECS}s）。请检查网络，或稍后重试。\nURL: {url}"
        )
    })?
    .map_err(|e| format!("下载请求失败: {e}\nURL: {url}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "下载源返回错误状态 {}: {}\nURL: {url}",
            resp.status().as_u16(),
            resp.status()
        ));
    }

    let total = resp.content_length();
    let mut stream = resp.bytes_stream();

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("创建目录失败: {e}"))?;
    }

    let mut file = fs::File::create(dest)
        .await
        .map_err(|e| format!("创建文件失败: {e}（可能是目标目录不可写）"))?;

    let mut downloaded: u64 = 0;
    use tokio::io::AsyncWriteExt;

    loop {
        let next = timeout(Duration::from_secs(STALL_TIMEOUT_SECS), stream.next()).await;
        let chunk = match next {
            Ok(Some(item)) => item.map_err(|e| format!("下载数据失败: {e}"))?,
            Ok(None) => break,
            Err(_) => {
                return Err(format!(
                    "下载长时间无进度（超过 {STALL_TIMEOUT_SECS} 秒），已自动中止。\n请检查网络后重试。\nURL: {url}"
                ));
            }
        };

        file.write_all(&chunk)
            .await
            .map_err(|e| format!("写入文件失败: {e}（目录可能不可写或磁盘空间不足）"))?;
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

    if downloaded == 0 {
        return Err(format!("下载内容为空，可能被网络拦截。\nURL: {url}"));
    }

    file.flush()
        .await
        .map_err(|e| format!("保存下载文件失败: {e}"))?;

    Ok(())
}