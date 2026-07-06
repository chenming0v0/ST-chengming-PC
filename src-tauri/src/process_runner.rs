use crate::terminal::TerminalOutputEvent;
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command as StdCommand, Stdio};
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command as TokioCommand;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub async fn run_hidden_command(
    app: &AppHandle,
    session_id: &str,
    program: &Path,
    args: &[String],
    cwd: &Path,
    env_vars: &HashMap<String, String>,
) -> Result<(), String> {
    emit_line(app, session_id, format!("> {}", command_label(program, args)));

    let mut command = hidden_tokio_command(program);
    command
        .args(args)
        .current_dir(cwd)
        .envs(env_vars)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|e| format!("启动命令失败 {}: {}", program.display(), e))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdout_task = stdout.map(|out| pipe_output(app.clone(), session_id.to_string(), out));
    let stderr_task = stderr.map(|err| pipe_output(app.clone(), session_id.to_string(), err));

    let status = child
        .wait()
        .await
        .map_err(|e| format!("等待命令结束失败 {}: {}", program.display(), e))?;

    if let Some(task) = stdout_task {
        let _ = task.await;
    }
    if let Some(task) = stderr_task {
        let _ = task.await;
    }

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "命令退出码异常 {}: {}",
            status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "terminated".to_string()),
            command_label(program, args)
        ))
    }
}

pub fn hidden_tokio_command(program: &Path) -> TokioCommand {
    let mut command = TokioCommand::new(program);
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

pub fn hidden_std_command(program: &Path) -> StdCommand {
    let mut command = StdCommand::new(program);
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

fn pipe_output<R>(
    app: AppHandle,
    session_id: String,
    mut reader: R,
) -> tokio::task::JoinHandle<()>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf).await {
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
    })
}

fn emit_line(app: &AppHandle, session_id: &str, line: String) {
    let _ = app.emit(
        "terminal-output",
        TerminalOutputEvent {
            session_id: session_id.to_string(),
            data: format!("{line}\n"),
        },
    );
}

fn command_label(program: &Path, args: &[String]) -> String {
    let mut parts = vec![program.display().to_string()];
    parts.extend(args.iter().cloned());
    parts.join(" ")
}
