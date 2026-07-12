use crate::model::{icon, RuntimeStatus, TavernStatus};
use leptos::prelude::*;

#[component]
pub fn InstallPage(
    runtime_status: ReadSignal<RuntimeStatus>,
    tavern_status: ReadSignal<TavernStatus>,
    install_dir: ReadSignal<String>,
    install_note: ReadSignal<String>,
    install_message: ReadSignal<Option<String>>,
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
                    <div class="path-row">
                        <input
                            type="text"
                            readonly
                            prop:value=move || {
                                let base = install_dir.get();
                                if base.is_empty() {
                                    "文档\\chengming\\SillyTavern".to_string()
                                } else {
                                    format!("{base}\\SillyTavern")
                                }
                            }
                        />
                        <button type="button" class="path-fixed" disabled title="优先写入用户文档\\chengming；若桌面已有旧数据会继续使用">"自动"</button>
                    </div>
                </div>
            </div>

            {move || {
                let note = install_note.get();
                if note.is_empty() {
                    view! { <></> }.into_any()
                } else {
                    view! { <p class="install-note">{note}</p> }.into_any()
                }
            }}
            {move || install_message.get().map(|message| {
                let class_name = if message.contains("失败") { "install-message error" } else { "install-message" };
                view! { <p class=class_name>{message}</p> }
            })}
            <section class="install-progress-card">
                <div class="progress-head">
                    <div>
                        <strong>"安装进度"</strong>
                        <small class="progress-stage">{move || {
                            let stage = current_stage.get();
                            match stage.as_str() {
                                "check" => "环境检测".to_string(),
                                "node" => "安装 Node.js".to_string(),
                                "git" => "安装 Git".to_string(),
                                "tavern" => "克隆 SillyTavern".to_string(),
                                "deps" => "安装依赖".to_string(),
                                "done" => "完成".to_string(),
                                other => other.to_string(),
                            }
                        }}</small>
                    </div>
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
