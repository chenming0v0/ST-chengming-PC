use std::path::{Path, PathBuf};

const DESKTOP_DATA_DIR_NAME: &str = "chengming";

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub install_dir: PathBuf,
}

impl AppPaths {
    pub fn from_install_dir(install_dir: impl AsRef<Path>) -> Self {
        let install_dir = install_dir.as_ref().to_path_buf();
        Self { install_dir }
    }
}

pub fn current_install_dir() -> Result<PathBuf, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("无法获取当前程序路径: {}", e))?;
    let exe_parent = exe
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "当前程序路径没有父目录".to_string())?;
    let desktop_dir = std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .map(|profile| profile.join("Desktop"));
    Ok(resolve_install_dir(&exe_parent, desktop_dir.as_deref()))
}

pub fn current_app_paths() -> Result<AppPaths, String> {
    current_install_dir().map(AppPaths::from_install_dir)
}

fn resolve_install_dir(exe_parent: &Path, desktop_dir: Option<&Path>) -> PathBuf {
    desktop_dir
        .map(|dir| dir.join(DESKTOP_DATA_DIR_NAME))
        .unwrap_or_else(|| exe_parent.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_paths_stay_under_install_dir() {
        let paths = AppPaths::from_install_dir(r"C:\launcher");

        assert_eq!(paths.install_dir, PathBuf::from(r"C:\launcher"));
    }

    #[test]
    fn install_dir_prefers_user_desktop_chengming_dir() {
        let exe_parent = Path::new(r"C:\Program Files\st-of-chengming");
        let desktop_dir = Some(Path::new(r"C:\Users\chengming\Desktop"));

        let install_dir = resolve_install_dir(exe_parent, desktop_dir);

        assert_eq!(
            install_dir,
            PathBuf::from(r"C:\Users\chengming\Desktop\chengming")
        );
    }

    #[test]
    fn install_dir_falls_back_to_exe_parent_without_desktop() {
        let exe_parent = Path::new(r"C:\launcher");

        let install_dir = resolve_install_dir(exe_parent, None);

        assert_eq!(install_dir, PathBuf::from(r"C:\launcher"));
    }
}
