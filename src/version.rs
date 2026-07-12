//! Single source of truth for launcher identity shown in the UI.
//!
//! The version is compiled from the workspace package version in `Cargo.toml`.
//! When bumping the app version, update these files together:
//! - `Cargo.toml`
//! - `src-tauri/Cargo.toml`
//! - `src-tauri/tauri.conf.json`
//! - `package.json`
//!
//! Frontend displays always read from this module (or backend `get_app_info`).

pub const APP_NAME: &str = "SillyTavern 启动器";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_LICENSE: &str = "GPL-3.0";
pub const ABOUT_DESCRIPTION: &str =
    concat!("版本 ", env!("CARGO_PKG_VERSION"), " · 基于 GPL-3.0 协议开源，完全免费");

pub fn app_title() -> String {
    format!("{APP_NAME} {APP_VERSION}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_semver_like() {
        let parts: Vec<_> = APP_VERSION.split('.').collect();
        assert!(parts.len() >= 2, "version should look like x.y or x.y.z");
        assert!(parts.iter().all(|part| part.chars().all(|c| c.is_ascii_digit())));
    }

    #[test]
    fn about_description_includes_version_and_license() {
        assert!(ABOUT_DESCRIPTION.contains(APP_VERSION));
        assert!(ABOUT_DESCRIPTION.contains(APP_LICENSE));
        assert!(!ABOUT_DESCRIPTION.contains("1.0.0 Build 128"));
        assert!(!ABOUT_DESCRIPTION.contains("MIT"));
    }

    #[test]
    fn app_title_includes_shared_version() {
        let title = app_title();
        assert!(title.contains(APP_NAME));
        assert!(title.contains(APP_VERSION));
    }
}
