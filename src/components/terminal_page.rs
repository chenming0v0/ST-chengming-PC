use crate::components::status_pill::StatusPill;
use crate::model::ServerStatus;
use leptos::prelude::*;

#[component]
pub fn TerminalPage(
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
                    <strong>"SillyTavern - npm install && node server.js"</strong>
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
