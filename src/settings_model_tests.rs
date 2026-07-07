use crate::model::{CloseAction, Language, LauncherSettings};

#[test]
fn frontend_launcher_settings_defaults_match_backend_contract() {
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
fn frontend_launcher_settings_serialize_to_tauri_field_names() {
    let settings = LauncherSettings {
        close_action: CloseAction::ExitAndStop,
        language: Language::EnUs,
        memory_limit_mb: Some(4096),
        ..LauncherSettings::default()
    };

    let json = serde_json::to_value(settings).unwrap();

    assert_eq!(json["close_action"], "exit_and_stop");
    assert_eq!(json["language"], "en-us");
    assert_eq!(json["memory_limit_mb"], 4096);
}

#[test]
fn frontend_launch_command_preview_uses_node_memory_flag_order() {
    let settings = LauncherSettings {
        memory_limit_mb: Some(2048),
        port: 9001,
        ..LauncherSettings::default()
    };

    assert_eq!(
        settings.launch_command_preview(),
        "npm install && node --max-old-space-size=2048 server.js --port=9001 --listen=false --whitelist=true --basicAuthMode=false --browserLaunchEnabled=true"
    );
}
