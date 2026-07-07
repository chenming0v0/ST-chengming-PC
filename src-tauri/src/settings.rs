use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const SETTINGS_FILE: &str = "launcher-settings.json";
const DEFAULT_PORT: u16 = 8000;
const MIN_MEMORY_LIMIT_MB: u32 = 128;
const MAX_MEMORY_LIMIT_MB: u32 = 131_072;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloseAction {
    MinimizeToTray,
    ExitAndStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
    ZhCn,
    ZhTw,
    EnUs,
    JaJp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherSettings {
    pub port: u16,
    pub listen_lan: bool,
    pub whitelist: bool,
    pub basic_auth: bool,
    pub auto_browser: bool,
    pub auto_update: bool,
    pub memory_limit_mb: Option<u32>,
    pub proxy_url: String,
    pub dark_mode: bool,
    pub close_action: CloseAction,
    pub language: Language,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        Self {
            port: DEFAULT_PORT,
            listen_lan: false,
            whitelist: true,
            basic_auth: false,
            auto_browser: true,
            auto_update: true,
            memory_limit_mb: None,
            proxy_url: String::new(),
            dark_mode: true,
            close_action: CloseAction::MinimizeToTray,
            language: Language::ZhCn,
        }
    }
}

impl LauncherSettings {
    pub fn normalized(mut self) -> Result<Self, String> {
        if self.port == 0 {
            return Err("服务端口必须在 1 到 65535 之间".to_string());
        }

        self.proxy_url = self.proxy_url.trim().to_string();
        self.memory_limit_mb = match self.memory_limit_mb {
            Some(0) | None => None,
            Some(value) if (MIN_MEMORY_LIMIT_MB..=MAX_MEMORY_LIMIT_MB).contains(&value) => {
                Some(value)
            }
            Some(_) => {
                return Err(format!(
                    "内存上限必须留空，或设置为 {} 到 {} MB",
                    MIN_MEMORY_LIMIT_MB, MAX_MEMORY_LIMIT_MB
                ));
            }
        };

        Ok(self)
    }
}

pub fn launcher_settings_path(install_dir: &Path) -> PathBuf {
    install_dir.join(SETTINGS_FILE)
}

pub async fn load_launcher_settings(install_dir: &Path) -> Result<LauncherSettings, String> {
    let path = launcher_settings_path(install_dir);
    if !path.exists() {
        return Ok(LauncherSettings::default());
    }

    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取启动器设置失败: {}", e))?;
    serde_json::from_str::<LauncherSettings>(&content)
        .map_err(|e| format!("解析启动器设置失败: {}", e))?
        .normalized()
}

pub async fn save_launcher_settings(
    install_dir: &Path,
    settings: LauncherSettings,
) -> Result<LauncherSettings, String> {
    let settings = settings.normalized()?;
    let path = launcher_settings_path(install_dir);
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("序列化启动器设置失败: {}", e))?;
    tokio::fs::write(&path, content)
        .await
        .map_err(|e| format!("保存启动器设置失败: {}", e))?;
    Ok(settings)
}

pub fn build_tavern_launch_args(settings: &LauncherSettings) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(memory_limit_mb) = settings.memory_limit_mb {
        args.push(format!("--max-old-space-size={memory_limit_mb}"));
    }

    args.extend([
        "server.js".to_string(),
        format!("--port={}", settings.port),
        format!("--listen={}", settings.listen_lan),
        format!("--whitelist={}", settings.whitelist),
        format!("--basicAuthMode={}", settings.basic_auth),
        format!("--browserLaunchEnabled={}", settings.auto_browser),
    ]);

    args
}

pub fn apply_proxy_env(env: &mut HashMap<String, String>, settings: &LauncherSettings) {
    const KEYS: &[&str] = &[
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
        "http_proxy",
        "https_proxy",
        "all_proxy",
        "npm_config_proxy",
        "npm_config_https_proxy",
    ];

    for key in KEYS {
        env.remove(*key);
    }

    if settings.proxy_url.is_empty() {
        return;
    }

    for key in KEYS {
        env.insert((*key).to_string(), settings.proxy_url.clone());
    }
}
