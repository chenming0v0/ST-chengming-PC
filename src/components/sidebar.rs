use crate::model::{icon, Page, ServerStatus};
use leptos::prelude::*;

#[component]
pub fn Sidebar(
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
            <button type="button" class="sb-logo" title="启动页" on:click=move |_| set_page.set(Page::Launch)>
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
                        view! { <img src=icon("star") alt="" /> }.into_any()
                    } else {
                        view! { <img src=icon("circle-pause") alt="" /> }.into_any()
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
