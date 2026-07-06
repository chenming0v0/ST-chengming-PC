use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Launch,
    Install,
    Terminal,
    Settings,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ServerStatus {
    Stopped,
    Starting,
    Running,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct RuntimeStatus {
    node_installed: bool,
    git_installed: bool,
    node_version: Option<String>,
    git_version: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct TavernStatus {
    installed: bool,
    path: String,
    version: Option<String>,
    running: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct ProgressPayload {
    stage: String,
    percent: u32,
    message: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct TerminalOutputPayload {
    session_id: String,
    data: String,
}

fn tauri_available() -> bool {
    web_sys::window()
        .and_then(|window| js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI__")).ok())
        .map(|value| !value.is_undefined() && !value.is_null())
        .unwrap_or(false)
}

async fn tauri_invoke<T: for<'de> Deserialize<'de>>(cmd: &str, args: &JsValue) -> Result<T, String> {
    if !tauri_available() {
        return Err("当前不在 Tauri 运行环境中".to_string());
    }

    let result = invoke(cmd, args.clone()).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| format!("反序列化失败: {}", e))
}

async fn tauri_invoke_string(cmd: &str, args: &JsValue) -> Result<String, String> {
    if !tauri_available() {
        return Err("当前不在 Tauri 运行环境中".to_string());
    }

    let result = invoke(cmd, args.clone()).await;
    result.as_string().ok_or_else(|| {
        serde_wasm_bindgen::from_value::<String>(result.clone())
            .unwrap_or_else(|_| format!("调用 {} 失败", cmd))
    })
}

fn empty_args() -> JsValue {
    JsValue::from(js_sys::Object::new())
}

fn command_args(pairs: &[(&str, JsValue)]) -> JsValue {
    let obj = js_sys::Object::new();
    for (key, value) in pairs {
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(key), value);
    }
    JsValue::from(obj)
}

fn icon(name: &'static str) -> String {
    format!("assets/icons/vercel/{name}.svg")
}

fn status_text(status: ServerStatus) -> &'static str {
    match status {
        ServerStatus::Stopped => "已停止",
        ServerStatus::Starting => "启动中",
        ServerStatus::Running => "运行中",
    }
}

#[component]
fn TitleBar() -> impl IntoView {
    let start_dragging = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_start_dragging", &empty_args()).await;
        });
    };
    let minimize = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_minimize", &empty_args()).await;
        });
    };
    let toggle_maximize = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_toggle_maximize", &empty_args()).await;
        });
    };
    let close = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_close", &empty_args()).await;
        });
    };
    let stop_titlebar_drag = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <header class="titlebar" on:mousedown=start_dragging>
            <div class="brand">
                <img class="titlebar-logo" src="assets/chengming.png" alt="辰林" />
                <span>"SillyTavern 启动器 1.0.0"</span>
            </div>
            <div class="wc">
                <button type="button" title="帮助" on:mousedown=stop_titlebar_drag>"?"</button>
                <button type="button" title="最小化" on:mousedown=stop_titlebar_drag on:click=minimize>"—"</button>
                <button type="button" title="最大化" on:mousedown=stop_titlebar_drag on:click=toggle_maximize>"□"</button>
                <button type="button" class="x" title="关闭" on:mousedown=stop_titlebar_drag on:click=close>"×"</button>
            </div>
        </header>
    }
}

#[component]
fn Sidebar(
    page: ReadSignal<Page>,
    set_page: WriteSignal<Page>,
    dark: ReadSignal<bool>,
    set_dark: WriteSignal<bool>,
    status: ReadSignal<ServerStatus>,
) -> impl IntoView {
    let nav = [
        (Page::Launch, "启动", "circle-play"),
        (Page::Install, "安装", "arrow-down"),
        (Page::Terminal, "终端", "box"),
    ];

    view! {
        <aside class="sidebar">
            <button
                type="button"
                class="sb-logo"
                title="启动页"
                on:click=move |_| set_page.set(Page::Launch)
            >
                <img src="assets/chengming.png" alt="辰林" />
            </button>

            <nav class="side-nav">
                {nav.into_iter().map(|(target, label, icon_name)| view! {
                    <button
                        type="button"
                        class=move || if page.get() == target { "sb-nav on" } else { "sb-nav" }
                        on:click=move |_| set_page.set(target)
                    >
                        <span class="nav-icon-wrap">
                            <img src=icon(icon_name) alt="" />
                            {move || {
                                if target == Page::Terminal && status.get() == ServerStatus::Running {
                                    view! { <span class="running-dot"></span> }.into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }}
                        </span>
                        <span>{label}</span>
                    </button>
                }).collect_view()}
            </nav>

            <div class="sb-grow"></div>

            <button
                type="button"
                class="sb-nav"
                title=move || if dark.get() { "切换到日间模式" } else { "切换到夜间模式" }
                on:click=move |_| set_dark.update(|value| *value = !*value)
            >
                <span class="nav-icon-wrap">
                    {move || if dark.get() {
                        view! { <img src=icon("circle-pause") alt="" /> }.into_any()
                    } else {
                        view! { <img src=icon("circle-play") alt="" /> }.into_any()
                    }}
                </span>
                <span>{move || if dark.get() { "日间" } else { "夜间" }}</span>
            </button>

            <button
                type="button"
                class=move || if page.get() == Page::Settings { "sb-nav on" } else { "sb-nav" }
                on:click=move |_| set_page.set(Page::Settings)
            >
                <span class="nav-icon-wrap"><img src=icon("pen-line") alt="" /></span>
                <span>"设置"</span>
            </button>
        </aside>
    }
}

#[component]
fn StatusPill(status: ReadSignal<ServerStatus>) -> impl IntoView {
    view! {
        <span class=move || match status.get() {
            ServerStatus::Running => "status-pill running",
            ServerStatus::Starting => "status-pill starting",
            ServerStatus::Stopped => "status-pill stopped",
        }>
            <span></span>
            {move || status_text(status.get())}
        </span>
    }
}

#[component]
fn LaunchPage(
    runtime_status: ReadSignal<RuntimeStatus>,
    tavern_status: ReadSignal<TavernStatus>,
    status: ReadSignal<ServerStatus>,
    set_page: WriteSignal<Page>,
    on_launch: Callback<()>,
    on_stop: Callback<()>,
) -> impl IntoView {
    let folders = [
        ("根目录", ".", "folder"),
        ("角色卡", "data/default-user/characters", "user"),
        ("聊天记录", "data/default-user/chats", "message-circle-more"),
        ("世界书", "data/default-user/worlds", "file"),
        ("扩展插件", "data/default-user/extensions", "box"),
    ];

    let announcements = [
        "公告栏可滚动，请知晓以下全部内容。",
        "首次使用请先前往「安装」页面完成环境部署（Node.js + Git + SillyTavern 本体）。",
        "启动后默认监听 http://127.0.0.1:8000，可在「设置」页面修改端口与监听地址。",
        "请勿从任何渠道购买本软件与教程，SillyTavern 是完全免费的开源项目。",
        "遇到问题请先查看「终端」页面的日志输出，大部分报错都能从日志中找到原因。",
    ];

    view! {
        <div class="page page-launch">
            <section class="banner">
                <img src="assets/bg.webp" alt="SillyTavern banner" />
                <div class="banner-t">
                    <p>"SillyTavern"</p>
                    <h1>"酒馆 - 启动器"</h1>
                    <p>"与 AI 角色畅聊，让故事随心所欲！"</p>
                </div>
            </section>

            <div class="split">
                <section class="launch-main">
                    <h2 class="h1">"文件夹"</h2>
                    <div class="grid-f">
                        {folders.into_iter().map(|(name, path, _icon_name)| view! {
                            <button type="button" class="fitem">
                                <div>
                                    <strong>{name}</strong>
                                    <span>{path}</span>
                                </div>
                            </button>
                        }).collect_view()}
                    </div>

                    <div class="launch-grow"></div>
                    <div class="launch-meta">
                        <div>"启动器版本：" <span class="val">"1.0.0 Build 128"</span></div>
                        <div>
                            "Node.js：" <span class="val">{move || {
                                let status = runtime_status.get();
                                if status.node_installed {
                                    status.node_version.unwrap_or_else(|| "已安装".to_string())
                                } else {
                                    "未检测到（请先安装）".to_string()
                                }
                            }}</span>
                        </div>
                        <div>
                            "SillyTavern：" <span class="val">{move || {
                                let status = tavern_status.get();
                                if status.installed {
                                    status.version.unwrap_or_else(|| "已安装".to_string())
                                } else {
                                    "未安装".to_string()
                                }
                            }}</span>
                        </div>
                    </div>
                </section>

                <aside class="launch-side">
                    <h2 class="h1">"公告"</h2>
                    <div class="ann">
                        {announcements.into_iter().map(|text| view! { <p>{text}</p> }).collect_view()}
                    </div>

                    <div class="launch-side-bottom">
                        {move || match status.get() {
                            ServerStatus::Stopped => {
                                let installed = tavern_status.get().installed;
                                view! {
                                    <button
                                        type="button"
                                        class="cta btn-p"
                                        on:click=move |_| {
                                            if installed {
                                                on_launch.run(());
                                            } else {
                                                set_page.set(Page::Install);
                                            }
                                        }
                                    >
                                        "一键启动"
                                    </button>
                                }.into_any()
                            }
                            ServerStatus::Starting => view! {
                                <button type="button" class="cta btn" disabled>
                                    <span class="spinner"></span>
                                    "正在启动..."
                                </button>
                            }.into_any(),
                            ServerStatus::Running => view! {
                                <button type="button" class="cta btn-r" on:click=move |_| on_stop.run(())>
                                    <span class="stop-square"></span>
                                    "停止运行"
                                </button>
                            }.into_any(),
                        }}
                        {move || if status.get() == ServerStatus::Running {
                            view! { <p class="running-url">"● 服务运行中 — http://127.0.0.1:8000"</p> }.into_any()
                        } else {
                            view! {}.into_any()
                        }}
                        <button type="button" class="btn install-entry" on:click=move |_| set_page.set(Page::Install)>
                            "前往安装（未安装态入口）"
                        </button>
                    </div>
                </aside>
            </div>
        </div>
    }
}

#[component]
fn InstallPage(
    runtime_status: ReadSignal<RuntimeStatus>,
    tavern_status: ReadSignal<TavernStatus>,
    installing: ReadSignal<bool>,
    progress: ReadSignal<u32>,
    current_stage: ReadSignal<String>,
    on_install: Callback<()>,
    on_update: Callback<()>,
) -> impl IntoView {
    let steps = [
        ("环境检测", "检测项目内运行时目录与安装状态", "check"),
        ("安装 Node.js", "下载并配置 Node.js v24 便携运行时", "box"),
        ("安装 Git", "下载并配置 Portable Git", "git"),
        ("克隆 SillyTavern", "git clone 指定镜像仓库", "folder-open"),
        ("安装依赖", "npm install 安装项目依赖包", "arrow-down"),
        ("完成配置", "安装目录内完成，不污染系统环境", "circle-check"),
    ];

    view! {
        <div class="page page-install">
            <header class="page-header">
                <h1>"安装 SillyTavern"</h1>
                <p>"一键部署运行环境，自动完成 Node.js、Git 与 SillyTavern 本体的安装。"</p>
            </header>

            <div class="install-options">
                <div class="option-card">
                    <p>"安装分支"</p>
                    <div class="segmented"><button class="selected" type="button">"release（稳定版）"</button><button type="button">"staging（测试版）"</button></div>
                </div>
                <div class="option-card">
                    <p>"下载源"</p>
                    <div class="segmented"><button class="selected" type="button">"GitCode 镜像"</button><button type="button">"官方源"</button></div>
                </div>
                <div class="option-card">
                    <p>"安装路径"</p>
                    <div class="path-row"><input value="程序所在目录" readonly /><button type="button">"固定"</button></div>
                </div>
            </div>

            <section class="install-progress-card">
                <div class="progress-head">
                    <strong>"安装进度"</strong>
                    <span>{move || format!("{}%", progress.get())}</span>
                </div>
                <div class="progress-track"><div style=move || format!("width:{}%", progress.get())></div></div>
                <ul class="step-list">
                    {steps.into_iter().enumerate().map(|(index, (name, desc, icon_name))| view! {
                        <li class=move || {
                            let stage = current_stage.get();
                            let installed = tavern_status.get().installed;
                            let done_by_status = match index {
                                0 => runtime_status.get().node_installed || runtime_status.get().git_installed || installed,
                                1 => runtime_status.get().node_installed,
                                2 => runtime_status.get().git_installed,
                                3 => installed,
                                4 => installed,
                                _ => installed,
                            };
                            if done_by_status {
                                "step done"
                            } else if installing.get() && stage_matches(&stage, index) {
                                "step active"
                            } else {
                                "step pending"
                            }
                        }>
                            <span><img src=icon(icon_name) alt="" /></span>
                            <div><strong>{name}</strong><small>{desc}</small></div>
                            <em>{move || {
                                let stage = current_stage.get();
                                if installing.get() && stage_matches(&stage, index) { "进行中" } else { "" }
                            }}</em>
                        </li>
                    }).collect_view()}
                </ul>
            </section>

            <div class="install-actions">
                {move || if tavern_status.get().installed && !installing.get() {
                    view! {
                        <>
                            <span class="installed-note">"✓ 已安装 SillyTavern"</span>
                            <div class="grow"></div>
                            <button type="button" class="secondary-button" on:click=move |_| on_update.run(())>"检查更新"</button>
                            <button type="button" class="blue-button" on:click=move |_| on_install.run(())>"重新安装"</button>
                        </>
                    }.into_any()
                } else {
                    view! {
                        <button type="button" class=move || if installing.get() { "blue-button disabled" } else { "blue-button" } disabled=move || installing.get() on:click=move |_| on_install.run(())>
                            {move || if installing.get() { "正在安装，请稍候..." } else { "开始安装" }}
                        </button>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

fn stage_matches(stage: &str, index: usize) -> bool {
    match index {
        0 => stage == "check",
        1 => stage == "node",
        2 => stage == "git",
        3 => stage == "tavern",
        4 => stage == "deps",
        5 => stage == "done",
        _ => false,
    }
}

#[component]
fn TerminalPage(
    logs: ReadSignal<Vec<String>>,
    status: ReadSignal<ServerStatus>,
    terminal_input: ReadSignal<String>,
    set_terminal_input: WriteSignal<String>,
    on_clear: Callback<()>,
    on_launch: Callback<()>,
    on_stop: Callback<()>,
    on_send: Callback<()>,
    installed: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="page page-terminal">
            <div class="terminal-header">
                <h1>"终端"</h1>
                <StatusPill status=status />
                <div class="grow"></div>
                <label class="auto-scroll"><input type="checkbox" checked />"自动滚动"</label>
                <button type="button" class="secondary-button small" on:click=move |_| on_clear.run(())>"清空日志"</button>
                {move || if status.get() == ServerStatus::Stopped {
                    view! {
                        <button type="button" class=move || if installed.get() { "blue-button small" } else { "blue-button small disabled" } disabled=move || !installed.get() on:click=move |_| on_launch.run(())>"启动服务"</button>
                    }.into_any()
                } else {
                    view! { <button type="button" class="danger-button small" on:click=move |_| on_stop.run(())>"停止服务"</button> }.into_any()
                }}
            </div>

            <section class="terminal-window">
                <div class="terminal-window-bar">
                    <span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span>
                    <strong>"SillyTavern - node server.js"</strong>
                </div>
                <div class="terminal-output">
                    {move || {
                        let list = logs.get();
                        if list.is_empty() {
                            view! { <p class="muted-line">"[启动器] 暂无日志输出。点击「启动服务」后，SillyTavern 的控制台输出将显示在这里。"</p> }.into_any()
                        } else {
                            view! {
                                <>
                                    {list.into_iter().map(|line| {
                                        let class_name = terminal_line_class(&line);
                                        view! { <p class=class_name>{line}</p> }
                                    }).collect_view()}
                                </>
                            }.into_any()
                        }
                    }}
                    {move || if status.get() != ServerStatus::Stopped {
                        view! { <p class="cursor-line"><span></span></p> }.into_any()
                    } else {
                        view! {}.into_any()
                    }}
                </div>
                <div class="terminal-input-row">
                    <span>">"</span>
                    <input
                        type="text"
                        placeholder="输入命令..."
                        prop:value=move || terminal_input.get()
                        on:input=move |ev| set_terminal_input.set(event_target_value(&ev))
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                on_send.run(());
                            }
                        }
                    />
                    <button type="button" on:click=move |_| on_send.run(())>"发送"</button>
                </div>
            </section>
        </div>
    }
}

fn terminal_line_class(line: &str) -> &'static str {
    if line.contains("[ERROR]") || line.contains("[错误]") || line.contains("error") {
        "term-error"
    } else if line.contains("[WARN]") || line.contains("警告") {
        "term-warn"
    } else if line.contains("[OK]") || line.contains("✓") || line.contains("完成") {
        "term-ok"
    } else if line.starts_with(">") {
        "term-command"
    } else {
        "term-normal"
    }
}

#[component]
fn Toggle(checked: ReadSignal<bool>, set_checked: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            type="button"
            class=move || if checked.get() { "toggle on" } else { "toggle" }
            on:click=move |_| set_checked.update(|value| *value = !*value)
        >
            <span></span>
        </button>
    }
}

#[component]
fn SettingRow(title: &'static str, desc: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="setting-row">
            <div>
                <strong>{title}</strong>
                <small>{desc}</small>
            </div>
            {children()}
        </div>
    }
}

#[component]
fn SettingsPage(dark: ReadSignal<bool>, set_dark: WriteSignal<bool>) -> impl IntoView {
    let (listen_lan, set_listen_lan) = signal(false);
    let (auto_browser, set_auto_browser) = signal(true);
    let (auto_update, set_auto_update) = signal(true);
    let (auth, set_auth) = signal(false);
    let (saved, set_saved) = signal(false);

    let save = move |_| {
        set_saved.set(true);
    };

    view! {
        <div class="page page-settings">
            <header class="settings-header">
                <div>
                    <h1>"设置"</h1>
                    <p>"配置启动器与 SillyTavern 服务的运行参数。"</p>
                </div>
                <button type="button" class=move || if saved.get() { "save-button saved" } else { "save-button" } on:click=save>
                    {move || if saved.get() { "✓ 已保存" } else { "保存设置" }}
                </button>
            </header>

            <section class="settings-section">
                <h2>"网络"</h2>
                <div class="settings-card">
                    <SettingRow title="服务端口" desc="SillyTavern 监听的端口号，默认 8000">
                        <input class="setting-input" value="8000" />
                    </SettingRow>
                    <SettingRow title="局域网监听" desc="开启后局域网内其他设备可通过本机 IP 访问酒馆">
                        <Toggle checked=listen_lan set_checked=set_listen_lan />
                    </SettingRow>
                    <SettingRow title="基础身份验证" desc="访问酒馆时需要输入用户名和密码">
                        <Toggle checked=auth set_checked=set_auth />
                    </SettingRow>
                </div>
            </section>

            <section class="settings-section">
                <h2>"启动"</h2>
                <div class="settings-card">
                    <SettingRow title="启动后自动打开浏览器" desc="服务启动完成后自动在默认浏览器中打开酒馆页面">
                        <Toggle checked=auto_browser set_checked=set_auto_browser />
                    </SettingRow>
                    <SettingRow title="启动前自动检查更新" desc="每次启动前检查 SillyTavern 是否有新版本">
                        <Toggle checked=auto_update set_checked=set_auto_update />
                    </SettingRow>
                    <SettingRow title="关闭窗口时" desc="点击关闭按钮后的行为">
                        <select class="setting-input"><option>"最小化到托盘"</option><option>"退出并停止服务"</option></select>
                    </SettingRow>
                </div>
            </section>

            <section class="settings-section">
                <h2>"外观"</h2>
                <div class="settings-card">
                    <SettingRow title="夜间模式" desc="切换启动器的明暗主题（与侧边栏按钮同步）">
                        <Toggle checked=dark set_checked=set_dark />
                    </SettingRow>
                    <SettingRow title="界面语言" desc="启动器界面显示语言">
                        <select class="setting-input"><option>"简体中文"</option><option>"English"</option><option>"日本語"</option></select>
                    </SettingRow>
                </div>
            </section>

            <section class="settings-section">
                <h2>"关于"</h2>
                <div class="settings-card">
                    <SettingRow title="SillyTavern 启动器" desc="版本 1.0.0 Build 128 · 基于 MIT 协议开源，完全免费">
                        <button type="button" class="secondary-button">"检查启动器更新"</button>
                    </SettingRow>
                    <SettingRow title="开源仓库" desc="gitcode.com/GitHub_Trending/si/SillyTavern">
                        <a class="repo-link" href="https://gitcode.com/GitHub_Trending/si/SillyTavern" target="_blank">"前往仓库 →"</a>
                    </SettingRow>
                </div>
            </section>
        </div>
    }
}

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

    {
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
                    set_logs.update(|items| items.push(format!("[{}:{}%] {}", payload.stage, payload.percent, payload.message)));
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

    refresh_status();

    let on_install = Callback::new(move |_| {
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

    let on_launch = Callback::new(move |_| {
        if status.get_untracked() != ServerStatus::Stopped {
            return;
        }
        if !tavern_status.get_untracked().installed {
            add_log("[启动器] SillyTavern 尚未安装，已切换到安装页面。".to_string());
            set_page.set(Page::Install);
            return;
        }
        set_status.set(ServerStatus::Starting);
        set_page.set(Page::Terminal);
        add_log("> node server.js".to_string());
        add_log("[启动器] 正在启动 SillyTavern 服务...".to_string());

        spawn_local(async move {
            let args = empty_args();
            match tauri_invoke_string("start_tavern", &args).await {
                Ok(session_id) => {
                    set_active_session.set(Some(session_id));
                    set_status.set(ServerStatus::Running);
                    add_log("[OK] SillyTavern 启动命令已发送到内联终端。".to_string());
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
