use crate::components::{InstallPage, LaunchPage, SettingsPage, Sidebar, TerminalPage, TitleBar};
use crate::model::{
    Page, ProgressPayload, RuntimeStatus, ServerStatus, TavernStatus, TerminalOutputPayload,
};
use crate::tauri_api::{
    command_args, empty_args, listen, tauri_available, tauri_invoke, tauri_invoke_string,
};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = signal(Page::Launch);
    let (dark, set_dark) = signal(true);
    let (runtime_status, set_runtime_status) = signal(RuntimeStatus::default());
    let (tavern_status, set_tavern_status) = signal(TavernStatus::default());
    let (status, set_status) = signal(ServerStatus::Stopped);
    let (logs, set_logs) = signal(Vec::<String>::new());
    let (installing, set_installing) = signal(false);
    let (progress, set_progress) = signal(0u32);
    let (current_stage, set_current_stage) = signal("check".to_string());
    let (terminal_input, set_terminal_input) = signal(String::new());
    let (active_session, set_active_session) = signal(Option::<String>::None);

    let add_log = move |message: String| {
        set_logs.update(|items| items.push(message));
    };

    let refresh_status = move || {
        spawn_local(async move {
            let args = empty_args();
            if let Ok(status) = tauri_invoke::<RuntimeStatus>("get_runtime_status", &args).await {
                set_runtime_status.set(status);
            }
            if let Ok(status) = tauri_invoke::<TavernStatus>("get_tavern_status", &args).await {
                set_tavern_status.set(status);
            }
        });
    };

    bind_backend_events(
        active_session,
        set_current_stage,
        set_progress,
        set_logs,
        set_status,
    );
    refresh_status();

    let on_install = Callback::new(move |_| {
        if installing.get_untracked() {
            return;
        }
        set_installing.set(true);
        set_progress.set(0);
        set_current_stage.set("check".to_string());
        add_log("[启动器] 开始安装流程。".to_string());

        spawn_local(async move {
            let args = empty_args();
            match tauri_invoke::<()>("install_runtime", &args).await {
                Ok(_) => {
                    set_progress.set(60);
                    set_current_stage.set("tavern".to_string());
                    match tauri_invoke_string("install_tavern", &args).await {
                        Ok(message) => {
                            add_log(format!("[OK] {}", message));
                            set_progress.set(100);
                            set_current_stage.set("done".to_string());
                        }
                        Err(error) => add_log(format!("[错误] 安装 SillyTavern 失败: {}", error)),
                    }
                }
                Err(error) => add_log(format!("[错误] 安装运行时失败: {}", error)),
            }
            set_installing.set(false);
            refresh_status();
        });
    });

    let on_update = Callback::new(move |_| {
        set_installing.set(true);
        set_current_stage.set("tavern".to_string());
        add_log("[启动器] 正在检查更新。".to_string());
        spawn_local(async move {
            let args = empty_args();
            match tauri_invoke_string("update_tavern", &args).await {
                Ok(message) => add_log(format!("[OK] {}", message)),
                Err(error) => add_log(format!("[错误] 更新失败: {}", error)),
            }
            set_installing.set(false);
            refresh_status();
        });
    });

    let on_install_for_launch = on_install.clone();
    let on_launch = Callback::new(move |_| {
        if status.get_untracked() != ServerStatus::Stopped {
            return;
        }
        if !tavern_status.get_untracked().installed {
            add_log("[启动器] SillyTavern 尚未安装，已切换到安装页面并开始安装。".to_string());
            set_page.set(Page::Install);
            on_install_for_launch.run(());
            return;
        }
        set_status.set(ServerStatus::Starting);
        set_page.set(Page::Terminal);
        add_log("> npm install && node server.js --browserLaunchEnabled=false".to_string());
        add_log("[启动器] 正在启动 SillyTavern 服务...".to_string());

        spawn_local(async move {
            let args = empty_args();
            match tauri_invoke_string("start_tavern", &args).await {
                Ok(session_id) => {
                    set_active_session.set(Some(session_id));
                    add_log("[启动器] SillyTavern 启动命令已发送，等待服务监听成功。".to_string());
                }
                Err(error) => {
                    set_status.set(ServerStatus::Stopped);
                    add_log(format!("[错误] 启动失败: {}", error));
                }
            }
        });
    });

    let on_stop = Callback::new(move |_| {
        let session_id = active_session.get_untracked();
        spawn_local(async move {
            if let Some(session_id) = session_id {
                let args = command_args(&[("sessionId", JsValue::from_str(&session_id))]);
                let _ = tauri_invoke::<()>("terminal_kill", &args).await;
            }
            set_active_session.set(None);
            set_status.set(ServerStatus::Stopped);
            add_log("[启动器] 服务已停止。".to_string());
        });
    });

    let on_send = Callback::new(move |_| {
        let input = terminal_input.get_untracked();
        if input.trim().is_empty() {
            return;
        }
        let session_id = active_session.get_untracked();
        set_terminal_input.set(String::new());
        add_log(format!("> {}", input));
        spawn_local(async move {
            if let Some(session_id) = session_id {
                let args = command_args(&[
                    ("sessionId", JsValue::from_str(&session_id)),
                    ("input", JsValue::from_str(&input)),
                ]);
                if let Err(error) = tauri_invoke::<()>("terminal_write", &args).await {
                    add_log(format!("[错误] 写入终端失败: {}", error));
                }
            } else {
                add_log("[错误] 当前没有活动终端会话。".to_string());
            }
        });
    });

    let on_clear = Callback::new(move |_| set_logs.set(Vec::new()));
    let installed = Signal::derive(move || tavern_status.get().installed);

    view! {
        <div class=move || if dark.get() { "app dark" } else { "app" }>
            <TitleBar />
            <div class="row">
                <Sidebar page=page set_page=set_page dark=dark set_dark=set_dark status=status />
                <main class="main">
                    {move || match page.get() {
                        Page::Launch => view! {
                            <LaunchPage
                                runtime_status=runtime_status
                                tavern_status=tavern_status
                                status=status
                                set_page=set_page
                                on_launch=on_launch
                                on_stop=on_stop
                            />
                        }.into_any(),
                        Page::Install => view! {
                            <InstallPage
                                runtime_status=runtime_status
                                tavern_status=tavern_status
                                installing=installing
                                progress=progress
                                current_stage=current_stage
                                on_install=on_install
                                on_update=on_update
                            />
                        }.into_any(),
                        Page::Terminal => view! {
                            <TerminalPage
                                logs=logs
                                status=status
                                terminal_input=terminal_input
                                set_terminal_input=set_terminal_input
                                on_clear=on_clear
                                on_launch=on_launch
                                on_stop=on_stop
                                on_send=on_send
                                installed=installed
                            />
                        }.into_any(),
                        Page::Settings => view! { <SettingsPage dark=dark set_dark=set_dark /> }.into_any(),
                    }}
                </main>
            </div>
        </div>
    }
}

fn bind_backend_events(
    active_session: ReadSignal<Option<String>>,
    set_current_stage: WriteSignal<String>,
    set_progress: WriteSignal<u32>,
    set_logs: WriteSignal<Vec<String>>,
    set_status: WriteSignal<ServerStatus>,
) {
    spawn_local(async move {
        if !tauri_available() {
            return;
        }

        let install_handler = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
            let payload = js_sys::Reflect::get(&event, &JsValue::from_str("payload"))
                .ok()
                .and_then(|value| serde_wasm_bindgen::from_value::<ProgressPayload>(value).ok());
            if let Some(payload) = payload {
                set_current_stage.set(payload.stage.clone());
                set_progress.set(payload.percent);
                set_logs.update(|items| {
                    items.push(format!(
                        "[{}:{}%] {}",
                        payload.stage, payload.percent, payload.message
                    ))
                });
            }
        });
        let _ = listen("install-progress", install_handler.as_ref().unchecked_ref()).await;
        install_handler.forget();

        let terminal_handler = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
            let payload = js_sys::Reflect::get(&event, &JsValue::from_str("payload"))
                .ok()
                .and_then(|value| serde_wasm_bindgen::from_value::<TerminalOutputPayload>(value).ok());
            if let Some(payload) = payload {
                let show = active_session
                    .get_untracked()
                    .as_deref()
                    .map(|id| id == payload.session_id)
                    .unwrap_or(true);
                if show {
                    set_logs.update(|items| {
                        for line in payload.data.replace("\r\n", "\n").split('\n') {
                            if !line.is_empty() {
                                if is_tavern_ready_line(line) {
                                    set_status.set(ServerStatus::Running);
                                }
                                items.push(line.to_string());
                            }
                        }
                    });
                }
            }
        });
        let _ = listen("terminal-output", terminal_handler.as_ref().unchecked_ref()).await;
        terminal_handler.forget();
    });
}

fn is_tavern_ready_line(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.contains("sillytavern is listening")
        || lower.contains("server is listening")
        || lower.contains("listening on")
        || lower.contains("http://127.0.0.1:8000")
        || lower.contains("http://localhost:8000")
}
