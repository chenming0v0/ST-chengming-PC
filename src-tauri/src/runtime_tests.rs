use super::runtime::{RuntimePaths, GIT_URL, NODE_URL, NODE_VERSION, NPM_REGISTRY};
use std::path::PathBuf;

#[test]
fn runtime_download_sources_use_cloudnodegit_mirror() {
    assert!(NODE_VERSION.starts_with("v24."));
    assert_eq!(
        NODE_URL,
        "https://cnb.cool/clya.top/cloudnodegit/-/git/raw/main/node-v24.18.0-win-x64.zip"
    );
    assert_eq!(
        GIT_URL,
        "https://cnb.cool/clya.top/cloudnodegit/-/git/raw/main/PortableGit-2.55.0.2-64-bit.7z.exe"
    );
}

#[test]
fn runtime_paths_are_project_local() {
    let install_dir = PathBuf::from(r"C:\launcher");
    let paths = RuntimePaths::new(&install_dir);

    assert_eq!(paths.base_dir, install_dir.join("runtime"));
    assert_eq!(paths.node_exe, install_dir.join("runtime").join("node").join("node.exe"));
    assert_eq!(paths.npm_cmd, install_dir.join("runtime").join("node").join("npm.cmd"));
    assert_eq!(
        paths.npm_cli_js(),
        install_dir
            .join("runtime")
            .join("node")
            .join("node_modules")
            .join("npm")
            .join("bin")
            .join("npm-cli.js")
    );
    assert_eq!(paths.npmrc, install_dir.join("runtime").join("npmrc"));
    assert_eq!(paths.git_exe, install_dir.join("runtime").join("git").join("cmd").join("git.exe"));
}

#[test]
fn runtime_env_uses_private_path_and_taobao_registry() {
    let install_dir = PathBuf::from(r"C:\launcher");
    let paths = RuntimePaths::new(&install_dir);
    let env = paths.env_vars();

    assert_eq!(env.get("PATH").unwrap(), &paths.env_path());
    assert_eq!(env.get("NPM_CONFIG_REGISTRY").unwrap(), NPM_REGISTRY);
    assert_eq!(
        env.get("NPM_CONFIG_USERCONFIG").unwrap(),
        &install_dir.join("runtime").join("npmrc").to_string_lossy().to_string()
    );
}

#[test]
fn runtime_env_preserves_windows_process_basics() {
    let paths = RuntimePaths::new(&PathBuf::from(r"C:\launcher"));
    let env = paths.env_vars();

    assert!(env.contains_key("SystemRoot"));
    assert!(env.contains_key("ComSpec"));
    assert!(env.contains_key("TEMP"));
    assert!(env.contains_key("TMP"));
    assert!(env.contains_key("PATHEXT"));
}

#[test]
fn runtime_path_keeps_windows_system_dirs_without_external_node_or_git() {
    let paths = RuntimePaths::new(&PathBuf::from(r"C:\launcher"));
    let path = paths.env_path();
    let lower_path = path.to_ascii_lowercase();

    assert!(path.starts_with(r"C:\launcher\runtime\node"));
    assert!(path.contains(r"C:\launcher\runtime\git\cmd"));
    assert!(path.contains(r"C:\launcher\runtime\git\bin"));
    assert!(lower_path.contains(r"c:\windows\system32"));
    assert!(lower_path.contains(r"c:\windows"));
    assert!(!lower_path.contains(r"c:\program files\nodejs"));
    assert!(!lower_path.contains(r"c:\program files\git"));
}
