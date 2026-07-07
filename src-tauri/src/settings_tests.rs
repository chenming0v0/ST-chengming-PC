use std::collections::HashMap;
use std::path::PathBuf;

use crate::settings::{
    apply_proxy_env, build_tavern_launch_args, launcher_settings_path, CloseAction, Language,
    LauncherSettings,
};

#[test]
fn default_launcher_settings_match_current_ui_defaults() {
    let settings = LauncherSettings::default();

    assert_eq!(settings.port, 8000);
    assert!(!settings.listen_lan);
    assert!(settings.whitelist);
    assert!(!settings.basic_auth);
    assert!(settings.auto_browser);
    assert!(settings.auto_update);
    assert_eq!(settings.memory_limit_mb, None);
    assert_eq!(settings.proxy_url, "");
    assert!(settings.dark_mode);
    assert_eq!(settings.close_action, CloseAction::MinimizeToTray);
    assert_eq!(settings.language, Language::ZhCn);
}

#[test]
fn launch_args_place_node_memory_limit_before_server_script() {
    let settings = LauncherSettings {
        memory_limit_mb: Some(4096),
        ..LauncherSettings::default()
    };

    let args = build_tavern_launch_args(&settings);

    assert_eq!(args[0], "--max-old-space-size=4096");
    assert_eq!(args[1], "server.js");
}

#[test]
fn launch_args_include_real_sillytavern_cli_options() {
    let settings = LauncherSettings {
        port: 9001,
        listen_lan: true,
        whitelist: false,
        basic_auth: true,
        auto_browser: false,
        ..LauncherSettings::default()
    };

    let args = build_tavern_launch_args(&settings);

    assert!(args.contains(&"--port=9001".to_string()));
    assert!(args.contains(&"--listen=true".to_string()));
    assert!(args.contains(&"--whitelist=false".to_string()));
    assert!(args.contains(&"--basicAuthMode=true".to_string()));
    assert!(args.contains(&"--browserLaunchEnabled=false".to_string()));
}

#[test]
fn empty_memory_limit_does_not_emit_node_heap_flag() {
    let args = build_tavern_launch_args(&LauncherSettings::default());

    assert_eq!(args[0], "server.js");
    assert!(!args
        .iter()
        .any(|arg| arg.starts_with("--max-old-space-size=")));
}

#[test]
fn proxy_setting_updates_child_process_proxy_environment() {
    let mut env = HashMap::new();
    let settings = LauncherSettings {
        proxy_url: "http://127.0.0.1:7890".to_string(),
        ..LauncherSettings::default()
    };

    apply_proxy_env(&mut env, &settings);

    assert_eq!(env.get("HTTP_PROXY").unwrap(), "http://127.0.0.1:7890");
    assert_eq!(env.get("HTTPS_PROXY").unwrap(), "http://127.0.0.1:7890");
    assert_eq!(env.get("ALL_PROXY").unwrap(), "http://127.0.0.1:7890");
    assert_eq!(
        env.get("npm_config_proxy").unwrap(),
        "http://127.0.0.1:7890"
    );
    assert_eq!(
        env.get("npm_config_https_proxy").unwrap(),
        "http://127.0.0.1:7890"
    );
}

#[test]
fn blank_proxy_setting_removes_proxy_environment_overrides() {
    let mut env = HashMap::from([
        ("HTTP_PROXY".to_string(), "http://old".to_string()),
        ("HTTPS_PROXY".to_string(), "http://old".to_string()),
        ("ALL_PROXY".to_string(), "http://old".to_string()),
        ("npm_config_proxy".to_string(), "http://old".to_string()),
        (
            "npm_config_https_proxy".to_string(),
            "http://old".to_string(),
        ),
    ]);

    apply_proxy_env(&mut env, &LauncherSettings::default());

    assert!(!env.contains_key("HTTP_PROXY"));
    assert!(!env.contains_key("HTTPS_PROXY"));
    assert!(!env.contains_key("ALL_PROXY"));
    assert!(!env.contains_key("npm_config_proxy"));
    assert!(!env.contains_key("npm_config_https_proxy"));
}

#[test]
fn launcher_settings_are_stored_next_to_the_launcher() {
    let install_dir = PathBuf::from(r"C:\launcher");

    assert_eq!(
        launcher_settings_path(&install_dir),
        install_dir.join("launcher-settings.json")
    );
}
