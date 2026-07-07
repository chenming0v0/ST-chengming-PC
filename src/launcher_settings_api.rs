use crate::model::LauncherSettings;
use crate::tauri_api::{command_args, empty_args, tauri_invoke, tauri_invoke_string};

pub async fn load_launcher_settings() -> Result<LauncherSettings, String> {
    tauri_invoke::<LauncherSettings>("get_launcher_settings", &empty_args()).await
}

pub async fn save_launcher_settings(
    settings: LauncherSettings,
) -> Result<LauncherSettings, String> {
    let args = settings_args(&settings)?;
    tauri_invoke::<LauncherSettings>("save_launcher_settings", &args).await
}

pub async fn update_tavern(settings: &LauncherSettings) -> Result<String, String> {
    let args = settings_args(settings)?;
    tauri_invoke_string("update_tavern", &args).await
}

pub async fn start_tavern(settings: &LauncherSettings) -> Result<String, String> {
    let args = settings_args(settings)?;
    tauri_invoke_string("start_tavern", &args).await
}

fn settings_args(settings: &LauncherSettings) -> Result<wasm_bindgen::JsValue, String> {
    let value = serde_wasm_bindgen::to_value(settings)
        .map_err(|error| format!("设置序列化失败: {}", error))?;
    Ok(command_args(&[("settings", value)]))
}
