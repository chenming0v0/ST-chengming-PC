use crate::backend_events::bind_backend_events;
use crate::components::{InstallPage, LaunchPage, SettingsPage, Sidebar, TerminalPage, TitleBar};
use crate::launcher_settings_api;
use crate::model::{
    CloseAction, LauncherSettings, Page, RuntimeStatus, ServerStatus, TavernStatus,
};
use crate::tauri_api::{
    command_args, empty_args, tauri_available, tauri_invoke, tauri_invoke_string,
};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
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
    let (settings, set_settings) = signal(LauncherSettings::default());
    let (settings_saved, set_settings_saved) = signal(false);
    let (settings_error, set_settings_error) = signal(Option::<String>::None);

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

    let refresh_settings = move || {
        spawn_local(async move {
            if !tauri_available() {
                return;
            }

            match launcher_settings_api::load_launcher_settings().await {
                Ok(value) => {
                    set_dark.set(value.dark_mode);
                    set_settings.set(value);
                    set_settings_error.set(None);
                }
                Err(error) => set_settings_error.set(Some(error)),
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
    Effect::new(move |_| {
        let language = settings.get().language.as_value();
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            if let Some(root) = document.document_element() {
                let _ = root.set_attribute("lang", language);
            }
        }
    });
    refresh_status();
    refresh_settings();

    let on_save_settings = Callback::new(move |value: LauncherSettings| {
        set_settings_error.set(None);
        set_settings_saved.set(false);
        spawn_local(async move {
            match launcher_settings_api::save_launcher_settings(value).await {
                Ok(saved_settings) => {
                    set_settings.set(saved_settings);
                    set_settings_saved.set(true);
                    set_settings_error.set(None);
                }
                Err(error) => set_settings_error.set(Some(error)),
            }
        });
    });

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
        let launch_settings = settings.get_untracked();
        add_log(format!("> {}", launch_settings.launch_command_preview()));
        add_log("[启动器] 正在启动 SillyTavern 服务...".to_string());

        spawn_local(async move {
            if launch_settings.auto_update {
                add_log("[启动器] 启动前自动检查更新。".to_string());
                match launcher_settings_api::update_tavern(&launch_settings).await {
                    Ok(message) => add_log(format!("[OK] {}", message)),
                    Err(error) => add_log(format!("[警告] 自动更新失败，继续启动: {}", error)),
                }
            }

            match launcher_settings_api::start_tavern(&launch_settings).await {
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

    let on_window_close = Callback::new(move |_| {
        let close_action = settings.get_untracked().close_action;
        let session_id = active_session.get_untracked();
        spawn_local(async move {
            match close_action {
                CloseAction::MinimizeToTray => {
                    let _ = tauri_invoke::<()>("window_hide", &empty_args()).await;
                }
                CloseAction::ExitAndStop => {
                    if let Some(session_id) = session_id {
                        let args = command_args(&[("sessionId", JsValue::from_str(&session_id))]);
                        let _ = tauri_invoke::<()>("terminal_kill", &args).await;
                    }
                    set_active_session.set(None);
                    set_status.set(ServerStatus::Stopped);
                    let _ = tauri_invoke::<()>("window_close", &empty_args()).await;
                }
            }
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
            <TitleBar on_close=on_window_close />
            <div class="row">
                <Sidebar page=page set_page=set_page dark=dark set_dark=set_dark status=status />
                <main class="main">
                    {move || match page.get() {
                        Page::Launch => view! {
                            <LaunchPage
                                runtime_status=runtime_status
                                tavern_status=tavern_status
                                status=status
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
                        Page::Settings => view! {
                            <SettingsPage
                                dark=dark
                                set_dark=set_dark
                                settings=settings
                                set_settings=set_settings
                                saved=settings_saved
                                set_saved=set_settings_saved
                                save_error=settings_error
                                on_save=on_save_settings
                            />
                        }.into_any(),
                    }}
                </main>
            </div>
        </div>
    }
}
