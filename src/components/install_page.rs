use crate::model::{icon, RuntimeStatus, TavernStatus};
use leptos::prelude::*;

#[component]
pub fn InstallPage(
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
                    <div class="path-row"><input value="桌面\\chengming\\SillyTavern" readonly /><button type="button">"固定"</button></div>
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    const INSTALL_PAGE_SOURCE: &str = include_str!("install_page.rs");

    #[test]
    fn git_step_icon_asset_exists() {
        assert!(
            INSTALL_PAGE_SOURCE.contains("(\"安装 Git\", \"下载并配置 Portable Git\", \"git\")"),
            "the Git installation step should keep a dedicated git icon key"
        );
        assert!(
            Path::new("desktop-launcher/assets/icons/vercel/git.svg").is_file(),
            "the Git installation step references git.svg, so the copied asset must exist"
        );
    }
}
