const APP_STYLES: &str = include_str!("../styles.css");

#[test]
fn app_theme_variables_default_to_light_and_dark_is_explicit() {
    let light_theme = css_rule(".app");
    let dark_theme = css_rule(".app.dark");

    assert!(light_theme.contains("--bg: #f4f4f5;"));
    assert!(light_theme.contains("--sidebar: #ffffff;"));
    assert!(light_theme.contains("--card: #ffffff;"));
    assert!(light_theme.contains("--text: #18181b;"));
    assert!(light_theme.contains("--muted: #71717a;"));

    assert!(dark_theme.contains("--bg: #1f1f1f;"));
    assert!(dark_theme.contains("--sidebar: #181818;"));
    assert!(dark_theme.contains("--card: #2b2b2b;"));
    assert!(dark_theme.contains("--text: #f4f4f5;"));
    assert!(dark_theme.contains("--muted: #a1a1aa;"));
}

#[test]
fn launch_shell_uses_theme_variables_for_light_mode_surfaces() {
    for selector in [".titlebar", ".sidebar", ".page", ".fitem", ".ann", ".btn"] {
        let rule = css_rule(selector);
        assert!(
            rule.contains("var("),
            "{selector} should use theme variables instead of fixed dark colors"
        );
    }

    assert!(css_rule(".ann").contains("color: var(--text);"));
    assert!(css_rule(".btn").contains("background: var(--button);"));
    assert!(css_rule(".btn").contains("color: var(--button-text);"));
}

fn css_rule(selector: &str) -> &str {
    let start = APP_STYLES
        .rfind(&format!("{selector} {{"))
        .unwrap_or_else(|| panic!("missing CSS rule for {selector}"));
    let after_start = &APP_STYLES[start..];
    let end = after_start
        .find("\n}")
        .unwrap_or_else(|| panic!("unterminated CSS rule for {selector}"));
    &after_start[..end]
}
