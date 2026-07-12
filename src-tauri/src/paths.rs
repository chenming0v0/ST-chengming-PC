use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DATA_DIR_NAME: &str = "chengming";
const WRITE_PROBE_FILE: &str = ".st-launcher-write-test";

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

#[derive(Debug, Clone)]
pub struct ResolvedInstallDir {
    pub path: PathBuf,
    pub note: String,
}

pub fn current_install_dir() -> Result<PathBuf, String> {
    Ok(resolve_install_location()?.path)
}

pub fn current_app_paths() -> Result<AppPaths, String> {
    current_install_dir().map(AppPaths::from_install_dir)
}

pub fn display_install_dir() -> Result<String, String> {
    current_install_dir().map(|path| path.to_string_lossy().to_string())
}

pub fn resolve_open_path(install_dir: &Path, relative: &str) -> PathBuf {
    let tavern_root = install_dir.join("SillyTavern");
    if relative.is_empty() || relative == "." {
        tavern_root
    } else {
        tavern_root.join(relative)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub install_dir: String,
    pub note: String,
}

pub fn app_info() -> Result<AppInfo, String> {
    let resolved = resolve_install_location()?;
    Ok(AppInfo {
        name: "SillyTavern 启动器".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        install_dir: resolved.path.to_string_lossy().to_string(),
        note: resolved.note,
    })
}

pub fn resolve_install_location() -> Result<ResolvedInstallDir, String> {
    let candidates = install_dir_candidates()?;
    resolve_install_dir(&candidates)
}

fn install_dir_candidates() -> Result<Vec<(PathBuf, &'static str)>, String> {
    let profile = std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .ok_or_else(|| "无法获取用户目录 (USERPROFILE)".to_string())?;

    let mut candidates = Vec::new();

    for folder in ["Documents", "文档"] {
        push_unique(
            &mut candidates,
            profile.join(folder).join(DATA_DIR_NAME),
            "用户文档目录（推荐，避免 MSI/C 盘权限问题）",
        );
    }

    for folder in ["Desktop", "桌面"] {
        push_unique(
            &mut candidates,
            profile.join(folder).join(DATA_DIR_NAME),
            "桌面目录（兼容旧数据）",
        );
    }

    if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA").map(PathBuf::from) {
        push_unique(
            &mut candidates,
            local_app_data.join(DATA_DIR_NAME),
            "本地应用数据目录",
        );
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_parent) = exe.parent() {
            if is_protected_install_location(exe_parent) {
                // Intentionally skip Program Files / Windows.
            } else {
                push_unique(
                    &mut candidates,
                    exe_parent.to_path_buf(),
                    "程序所在目录",
                );
            }
        }
    }

    if candidates.is_empty() {
        return Err("没有可用的安装目录候选".to_string());
    }

    Ok(candidates)
}

fn resolve_install_dir(
    candidates: &[(PathBuf, &'static str)],
) -> Result<ResolvedInstallDir, String> {
    if let Some((path, label)) = candidates.iter().find(|(path, _)| has_existing_data(path)) {
        ensure_writable(path).map_err(|error| {
            format!(
                "检测到已有数据目录，但当前不可写：{}\n{}\n请检查权限后重试。",
                path.display(),
                error
            )
        })?;
        return Ok(ResolvedInstallDir {
            path: path.clone(),
            note: format!("继续使用已有数据目录（{label}）"),
        });
    }

    let mut errors = Vec::new();
    for (candidate, label) in candidates {
        match ensure_writable(candidate) {
            Ok(()) => {
                return Ok(ResolvedInstallDir {
                    path: candidate.clone(),
                    note: format!("已自动选择可写目录：{label}"),
                });
            }
            Err(error) => errors.push(format!("{} ({label}): {error}", candidate.display())),
        }
    }

    Err(format!(
        "安装目录不可用：所有候选路径都无法写入。\n已尝试:\n{}\n\n常见原因：MSI 安装到 C 盘/Program Files 后没有写权限。启动器会优先使用“文档\\\\chengming”。",
        errors.join("\n")
    ))
}

fn has_existing_data(path: &Path) -> bool {
    path.join("SillyTavern").join("package.json").is_file()
        || path.join("runtime").join("node").join("node.exe").is_file()
        || path.join("launcher-settings.json").is_file()
}

fn ensure_writable(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|error| format!("创建目录失败: {error}"))?;

    let probe = path.join(WRITE_PROBE_FILE);
    fs::write(&probe, b"ok").map_err(|error| format!("目录不可写: {error}"))?;
    let _ = fs::remove_file(&probe);
    Ok(())
}

fn is_protected_install_location(path: &Path) -> bool {
    let text = path.to_string_lossy().to_ascii_lowercase();
    text.contains(r"\program files")
        || text.contains(r"\program files (x86)")
        || text.contains(r"\windows\")
        || text.ends_with(r"\windows")
}

fn push_unique(items: &mut Vec<(PathBuf, &'static str)>, path: PathBuf, label: &'static str) {
    if !items.iter().any(|(item, _)| item == &path) {
        items.push((path, label));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("st-paths-{label}-{nanos}"))
    }

    #[test]
    fn app_paths_stay_under_install_dir() {
        let paths = AppPaths::from_install_dir(r"C:\launcher");
        assert_eq!(paths.install_dir, PathBuf::from(r"C:\launcher"));
    }

    #[test]
    fn resolve_install_dir_prefers_existing_data_root() {
        let root = temp_root("existing");
        let docs = root.join("Documents").join(DATA_DIR_NAME);
        let desktop = root.join("Desktop").join(DATA_DIR_NAME);
        fs::create_dir_all(desktop.join("SillyTavern")).expect("desktop dir");
        fs::write(desktop.join("SillyTavern").join("package.json"), "{}").expect("pkg");
        fs::create_dir_all(&docs).expect("docs dir");

        let resolved = resolve_install_dir(&[
            (docs.clone(), "docs"),
            (desktop.clone(), "desktop"),
        ])
        .expect("resolve");
        assert_eq!(resolved.path, desktop);
        assert!(resolved.note.contains("已有数据"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_install_dir_uses_first_writable_candidate() {
        let root = temp_root("writable");
        let docs = root.join("Documents").join(DATA_DIR_NAME);
        let desktop = root.join("Desktop").join(DATA_DIR_NAME);

        let resolved = resolve_install_dir(&[
            (docs.clone(), "docs"),
            (desktop.clone(), "desktop"),
        ])
        .expect("resolve");
        assert_eq!(resolved.path, docs);
        assert!(docs.is_dir());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_install_dir_skips_unwritable_and_falls_back() {
        let root = temp_root("fallback");
        let docs = root.join("Documents").join(DATA_DIR_NAME);
        let blocked_parent = root.join("blocked-parent").join("not-a-dir");
        fs::create_dir_all(root.join("blocked-parent")).expect("parent");
        fs::write(&blocked_parent, b"file").expect("blocker file");
        let unusable = blocked_parent.join(DATA_DIR_NAME);

        let resolved =
            resolve_install_dir(&[(unusable, "blocked"), (docs.clone(), "docs")]).expect("resolve");
        assert_eq!(resolved.path, docs);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn is_protected_install_location_detects_program_files() {
        assert!(is_protected_install_location(Path::new(
            r"C:\Program Files\st-of-chengming"
        )));
        assert!(is_protected_install_location(Path::new(
            r"C:\Program Files (x86)\st-of-chengming"
        )));
        assert!(!is_protected_install_location(Path::new(
            r"C:\Users\chengming\Documents\chengming"
        )));
    }

    #[test]
    fn resolve_open_path_uses_tavern_root_for_dot() {
        let path = resolve_open_path(Path::new(r"C:\Users\a\Documents\chengming"), ".");
        assert_eq!(
            path,
            PathBuf::from(r"C:\Users\a\Documents\chengming\SillyTavern")
        );
    }

    #[test]
    fn resolve_open_path_joins_relative_segments() {
        let path = resolve_open_path(
            Path::new(r"C:\Users\a\Documents\chengming"),
            "data/default-user/characters",
        );
        assert_eq!(
            path,
            PathBuf::from(
                r"C:\Users\a\Documents\chengming\SillyTavern\data\default-user\characters"
            )
        );
    }

    #[test]
    fn app_info_version_matches_package() {
        let info = app_info().expect("app info");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert!(!info.install_dir.is_empty());
        assert!(!info.note.is_empty());
    }
}