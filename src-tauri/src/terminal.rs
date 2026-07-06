use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child as ProcessChild, Command as ProcessCommand, Stdio};
use std::sync::{Arc, Mutex};

use portable_pty::{native_pty_system, Child as PtyChild, CommandBuilder, PtySize};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub title: String,
    pub cwd: String,
    pub alive: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TerminalOutputEvent {
    pub session_id: String,
    pub data: String,
}

pub struct TerminalProcess {
    child: TerminalChild,
    writer: Box<dyn Write + Send>,
    title: String,
    cwd: PathBuf,
}

enum TerminalChild {
    Pty(Box<dyn PtyChild + Send>),
    Process(ProcessChild),
}

impl TerminalChild {
    fn kill(&mut self) -> Result<(), String> {
        match self {
            TerminalChild::Pty(child) => child.kill().map_err(|e| format!("终止终端失败: {}", e)),
            TerminalChild::Process(child) => {
                child.kill().map_err(|e| format!("终止进程失败: {}", e))
            }
        }
    }
}

pub struct TerminalManager {
    sessions: HashMap<String, TerminalProcess>,
    next_id: u32,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn spawn_session(
        &mut self,
        app: AppHandle,
        title: String,
        cwd: PathBuf,
        env_vars: Option<HashMap<String, String>>,
    ) -> Result<String, String> {
        let id = format!("term-{}", self.next_id);
        self.next_id += 1;

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 30,
                cols: 120,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| format!("创建 PTY 失败: {}", e))?;

        let mut cmd = CommandBuilder::new(shell_path());
        cmd.arg("/K");
        cmd.arg("prompt $P$G");
        cmd.cwd(cwd.clone());

        if let Some(vars) = env_vars {
            for (key, value) in vars {
                cmd.env(key, value);
            }
        }

        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| format!("启动终端失败: {}", e))?;
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| format!("创建终端读取器失败: {}", e))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| format!("创建终端写入器失败: {}", e))?;

        spawn_output_thread(app, id.clone(), reader);

        self.sessions.insert(
            id.clone(),
            TerminalProcess {
                child: TerminalChild::Pty(child),
                writer,
                title,
                cwd,
            },
        );

        Ok(id)
    }

    pub fn spawn_process_session(
        &mut self,
        app: AppHandle,
        title: String,
        cwd: PathBuf,
        env_vars: HashMap<String, String>,
        program: PathBuf,
        args: Vec<String>,
    ) -> Result<String, String> {
        let id = format!("term-{}", self.next_id);
        self.next_id += 1;

        let mut cmd = ProcessCommand::new(&program);
        cmd.args(&args)
            .current_dir(cwd.clone())
            .envs(env_vars)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("启动进程失败 {}: {}", program.display(), e))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "创建进程输出读取器失败".to_string())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "创建进程错误输出读取器失败".to_string())?;
        let writer = child
            .stdin
            .take()
            .ok_or_else(|| "创建进程写入器失败".to_string())?;

        spawn_output_thread(app.clone(), id.clone(), stdout);
        spawn_output_thread(app, id.clone(), stderr);

        self.sessions.insert(
            id.clone(),
            TerminalProcess {
                child: TerminalChild::Process(child),
                writer: Box::new(writer),
                title,
                cwd,
            },
        );

        Ok(id)
    }

    pub fn write_to_session(&mut self, id: &str, input: &str) -> Result<(), String> {
        let proc = self
            .sessions
            .get_mut(id)
            .ok_or_else(|| format!("终端会话 {} 不存在", id))?;

        proc.writer
            .write_all(input.as_bytes())
            .map_err(|e| format!("写入终端失败: {}", e))?;
        proc.writer
            .write_all(b"\r\n")
            .map_err(|e| format!("写入终端失败: {}", e))?;
        proc.writer
            .flush()
            .map_err(|e| format!("刷新终端失败: {}", e))?;

        Ok(())
    }

    pub fn kill_session(&mut self, id: &str) -> Result<(), String> {
        if let Some(mut proc) = self.sessions.remove(id) {
            proc.child.kill()?;
        }
        Ok(())
    }

    pub fn list_sessions(&self) -> Vec<TerminalSession> {
        self.sessions
            .iter()
            .map(|(id, proc)| TerminalSession {
                id: id.clone(),
                title: proc.title.clone(),
                cwd: proc.cwd.to_string_lossy().to_string(),
                alive: true,
            })
            .collect()
    }
}

pub type SharedTerminalManager = Arc<Mutex<TerminalManager>>;

fn spawn_output_thread<R>(app: AppHandle, session_id: String, mut reader: R)
where
    R: Read + Send + 'static,
{
    std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app.emit(
                        "terminal-output",
                        TerminalOutputEvent {
                            session_id: session_id.clone(),
                            data,
                        },
                    );
                }
                Err(_) => break,
            }
        }
    });
}

fn shell_path() -> String {
    std::env::var("ComSpec")
        .ok()
        .filter(|path| !path.trim().is_empty())
        .unwrap_or_else(|| r"C:\Windows\System32\cmd.exe".to_string())
}
