use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Launch,
    Install,
    Terminal,
    Settings,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Running,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct RuntimeStatus {
    pub node_installed: bool,
    pub git_installed: bool,
    pub node_version: Option<String>,
    pub git_version: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TavernStatus {
    pub installed: bool,
    pub path: String,
    pub version: Option<String>,
    pub running: bool,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub install_dir: String,
    pub note: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProgressPayload {
    pub stage: String,
    pub percent: u32,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalOutputPayload {
    pub session_id: String,
    pub data: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloseAction {
    MinimizeToTray,
    ExitAndStop,
}

impl CloseAction {
    pub fn as_value(self) -> &'static str {
        match self {
            CloseAction::MinimizeToTray => "minimize_to_tray",
            CloseAction::ExitAndStop => "exit_and_stop",
        }
    }

    pub fn from_value(value: &str) -> Self {
        match value {
            "exit_and_stop" => CloseAction::ExitAndStop,
            _ => CloseAction::MinimizeToTray,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
    ZhCn,
    ZhTw,
    EnUs,
    JaJp,
}

impl Language {
    pub fn as_value(self) -> &'static str {
        match self {
            Language::ZhCn => "zh-cn",
            Language::ZhTw => "zh-tw",
            Language::EnUs => "en-us",
            Language::JaJp => "ja-jp",
        }
    }

    pub fn from_value(value: &str) -> Self {
        match value {
            "zh-tw" => Language::ZhTw,
            "en-us" => Language::EnUs,
            "ja-jp" => Language::JaJp,
            _ => Language::ZhCn,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
            port: 8000,
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
    pub fn tavern_launch_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(memory_limit_mb) = self.memory_limit_mb {
            args.push(format!("--max-old-space-size={memory_limit_mb}"));
        }

        args.extend([
            "server.js".to_string(),
            format!("--port={}", self.port),
            format!("--listen={}", self.listen_lan),
            format!("--whitelist={}", self.whitelist),
            format!("--basicAuthMode={}", self.basic_auth),
            format!("--browserLaunchEnabled={}", self.auto_browser),
        ]);

        args
    }

    pub fn launch_command_preview(&self) -> String {
        format!(
            "npm install && node {}",
            self.tavern_launch_args().join(" ")
        )
    }
}

pub fn icon(name: &'static str) -> String {
    format!("assets/icons/vercel/{name}.svg")
}

pub fn status_text(status: ServerStatus) -> &'static str {
    match status {
        ServerStatus::Stopped => "已停止",
        ServerStatus::Starting => "启动中",
        ServerStatus::Running => "运行中",
    }
}
