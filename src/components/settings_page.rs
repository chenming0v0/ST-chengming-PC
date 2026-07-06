use leptos::prelude::*;

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
pub fn SettingsPage(dark: ReadSignal<bool>, set_dark: WriteSignal<bool>) -> impl IntoView {
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
