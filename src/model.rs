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
