use crate::model::{CloseAction, Language, LauncherSettings};
use crate::version::ABOUT_DESCRIPTION;
use leptos::prelude::*;

#[component]
fn Toggle(checked: Signal<bool>, on_change: Callback<bool>) -> impl IntoView {
    view! {
        <button
            type="button"
            class=move || if checked.get() { "toggle on" } else { "toggle" }
            on:click=move |_| on_change.run(!checked.get_untracked())
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
pub fn SettingsPage(
    dark: ReadSignal<bool>,
    set_dark: WriteSignal<bool>,
    settings: ReadSignal<LauncherSettings>,
    set_settings: WriteSignal<LauncherSettings>,
    saved: ReadSignal<bool>,
    set_saved: WriteSignal<bool>,
    save_error: ReadSignal<Option<String>>,
    on_save: Callback<LauncherSettings>,
) -> impl IntoView {
    let save = move |_| {
        on_save.run(settings.get_untracked());
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
            {move || save_error.get().map(|message| view! { <p class="settings-message error">{message}</p> })}

            <section class="settings-section">
                <h2>"网络"</h2>
                <div class="settings-card">
                    <SettingRow title="服务端口" desc="SillyTavern 监听的端口号，默认 8000">
                        <input
                            class="setting-input"
                            type="number"
                            min="1"
                            max="65535"
                            prop:value=move || settings.get().port.to_string()
                            on:input=move |ev| {
                                if let Ok(port) = event_target_value(&ev).parse::<u16>() {
                                    set_settings.update(|value| value.port = port);
                                    set_saved.set(false);
                                }
                            }
                        />
                    </SettingRow>
                    <SettingRow title="局域网监听" desc="开启后局域网内其他设备可通过本机 IP 访问酒馆">
                        <Toggle
                            checked=Signal::derive(move || settings.get().listen_lan)
                            on_change=Callback::new(move |value| {
                                set_settings.update(|settings| settings.listen_lan = value);
                                set_saved.set(false);
                            })
                        />
                    </SettingRow>
                    <SettingRow title="IP 白名单" desc="对应 SillyTavern 的 --whitelist 参数，关闭后允许非白名单 IP 访问">
                        <Toggle
                            checked=Signal::derive(move || settings.get().whitelist)
                            on_change=Callback::new(move |value| {
                                set_settings.update(|settings| settings.whitelist = value);
                                set_saved.set(false);
                            })
                        />
                    </SettingRow>
                    <SettingRow title="基础身份验证" desc="对应 SillyTavern 的 --basicAuthMode 参数，账号密码使用 config.yaml">
                        <Toggle
                            checked=Signal::derive(move || settings.get().basic_auth)
                            on_change=Callback::new(move |value| {
                                set_settings.update(|settings| settings.basic_auth = value);
                                set_saved.set(false);
                            })
                        />
                    </SettingRow>
                    <SettingRow title="网络代理" desc="为 git / npm / SillyTavern 子进程注入 HTTP(S)_PROXY，留空则关闭">
                        <input
                            class="setting-input"
                            placeholder="http://127.0.0.1:7890"
                            prop:value=move || settings.get().proxy_url
                            on:input=move |ev| {
                                let proxy_url = event_target_value(&ev);
                                set_settings.update(|settings| settings.proxy_url = proxy_url);
                                set_saved.set(false);
                            }
                        />
                    </SettingRow>
                </div>
            </section>

            <section class="settings-section">
                <h2>"启动"</h2>
                <div class="settings-card">
                    <SettingRow title="启动后自动打开浏览器" desc="服务启动完成后自动在默认浏览器中打开酒馆页面">
                        <Toggle
                            checked=Signal::derive(move || settings.get().auto_browser)
                            on_change=Callback::new(move |value| {
                                set_settings.update(|settings| settings.auto_browser = value);
                                set_saved.set(false);
                            })
                        />
                    </SettingRow>
                    <SettingRow title="启动前自动检查更新" desc="每次启动前检查 SillyTavern 是否有新版本">
                        <Toggle
                            checked=Signal::derive(move || settings.get().auto_update)
                            on_change=Callback::new(move |value| {
                                set_settings.update(|settings| settings.auto_update = value);
                                set_saved.set(false);
                            })
                        />
                    </SettingRow>
                    <SettingRow title="Node.js 内存上限" desc="传给 node 的 --max-old-space-size，单位 MB；留空则使用默认值">
                        <input
                            class="setting-input"
                            type="number"
                            min="128"
                            max="131072"
                            placeholder="默认"
                            prop:value=move || settings.get().memory_limit_mb.map(|value| value.to_string()).unwrap_or_default()
                            on:input=move |ev| {
                                let input = event_target_value(&ev);
                                let parsed = if input.trim().is_empty() {
                                    Some(None)
                                } else {
                                    input.parse::<u32>().ok().map(Some)
                                };
                                if let Some(memory_limit_mb) = parsed {
                                    set_settings.update(|settings| settings.memory_limit_mb = memory_limit_mb);
                                    set_saved.set(false);
                                }
                            }
                        />
                    </SettingRow>
                    <SettingRow title="关闭窗口时" desc="点击关闭按钮后的行为">
                        <select
                            class="setting-input"
                            prop:value=move || settings.get().close_action.as_value()
                            on:change=move |ev| {
                                let close_action = CloseAction::from_value(&event_target_value(&ev));
                                set_settings.update(|settings| settings.close_action = close_action);
                                set_saved.set(false);
                            }
                        >
                            <option value="minimize_to_tray">"最小化到托盘"</option>
                            <option value="exit_and_stop">"退出并停止服务"</option>
                        </select>
                    </SettingRow>
                </div>
            </section>

            <section class="settings-section">
                <h2>"外观"</h2>
                <div class="settings-card">
                    <SettingRow title="夜间模式" desc="切换启动器的明暗主题（与侧边栏按钮同步）">
                        <Toggle
                            checked=Signal::derive(move || dark.get())
                            on_change=Callback::new(move |value| {
                                set_dark.set(value);
                                set_settings.update(|settings| settings.dark_mode = value);
                                set_saved.set(false);
                            })
                        />
                    </SettingRow>
                    <SettingRow title="界面语言" desc="启动器界面显示语言">
                        <select
                            class="setting-input"
                            prop:value=move || settings.get().language.as_value()
                            on:change=move |ev| {
                                let language = Language::from_value(&event_target_value(&ev));
                                set_settings.update(|settings| settings.language = language);
                                set_saved.set(false);
                            }
                        >
                            <option value="zh-cn">"简体中文"</option>
                            <option value="zh-tw">"繁體中文"</option>
                            <option value="en-us">"English"</option>
                            <option value="ja-jp">"日本語"</option>
                        </select>
                    </SettingRow>
                </div>
            </section>

            <section class="settings-section">
                <h2>"关于"</h2>
                <div class="settings-card">
                    <SettingRow title="SillyTavern 启动器" desc=ABOUT_DESCRIPTION>
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

#[cfg(test)]
mod tests {
    const SETTINGS_PAGE_SOURCE: &str = include_str!("settings_page.rs");

    #[test]
    fn settings_page_uses_shared_version_description() {
        let production_source = SETTINGS_PAGE_SOURCE
            .split("#[cfg(test)]")
            .next()
            .expect("settings page source should contain production code");
        assert!(production_source.contains("use crate::version::ABOUT_DESCRIPTION;"));
        assert!(production_source.contains("desc=ABOUT_DESCRIPTION"));
        assert!(!production_source.contains("1.0.0 Build 128"));
        assert!(!production_source.contains("MIT 协议"));
    }

    #[test]
    fn settings_page_exposes_all_runtime_launch_settings() {
        let production_source = SETTINGS_PAGE_SOURCE
            .split("#[cfg(test)]")
            .next()
            .expect("settings page source should contain production code");

        for label in [
            "服务端口",
            "局域网监听",
            "IP 白名单",
            "基础身份验证",
            "网络代理",
            "启动后自动打开浏览器",
            "启动前自动检查更新",
            "Node.js 内存上限",
        ] {
            assert!(
                production_source.contains(label),
                "settings page should expose {label}"
            );
        }
    }
}
