use crate::model::{RuntimeStatus, Page, ServerStatus, TavernStatus};
use leptos::prelude::*;

#[component]
pub fn LaunchPage(
    runtime_status: ReadSignal<RuntimeStatus>,
    tavern_status: ReadSignal<TavernStatus>,
    status: ReadSignal<ServerStatus>,
    set_page: WriteSignal<Page>,
    on_launch: Callback<()>,
    on_stop: Callback<()>,
) -> impl IntoView {
    let folders = [
        ("根目录", "."),
        ("角色卡", "data/default-user/characters"),
        ("聊天记录", "data/default-user/chats"),
        ("世界书", "data/default-user/worlds"),
        ("扩展插件", "data/default-user/extensions"),
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
                        {folders.into_iter().map(|(name, path)| view! {
                            <button type="button" class="fitem">
                                <div><strong>{name}</strong><span>{path}</span></div>
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
