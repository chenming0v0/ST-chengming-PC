use std::path::{Path, PathBuf};

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
    exe.parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "当前程序路径没有父目录".to_string())
}

pub fn current_app_paths() -> Result<AppPaths, String> {
    current_install_dir().map(AppPaths::from_install_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_paths_stay_under_install_dir() {
        let paths = AppPaths::from_install_dir(r"C:\launcher");

        assert_eq!(paths.install_dir, PathBuf::from(r"C:\launcher"));
    }
}
