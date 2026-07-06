use crate::tauri_api::{empty_args, tauri_invoke};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn TitleBar() -> impl IntoView {
    let start_dragging = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_start_dragging", &empty_args()).await;
        });
    };
    let minimize = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_minimize", &empty_args()).await;
        });
    };
    let toggle_maximize = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_toggle_maximize", &empty_args()).await;
        });
    };
    let close = move |_| {
        spawn_local(async move {
            let _ = tauri_invoke::<()>("window_close", &empty_args()).await;
        });
    };
    let stop_titlebar_drag = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <header class="titlebar" on:mousedown=start_dragging>
            <div class="brand">
                <img class="titlebar-logo" src="assets/chengming.png" alt="辰林" />
                <span>"SillyTavern 启动器 1.0.0"</span>
            </div>
            <div class="wc">
                <button type="button" title="帮助" on:mousedown=stop_titlebar_drag>
                    <svg class="wc-icon wc-help-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">
                        <path d="M6.05 5.95c.14-1.08.98-1.82 2.1-1.82 1.2 0 2.08.77 2.08 1.84 0 .8-.43 1.24-1.1 1.7-.7.48-1.03.86-1.03 1.62v.34" />
                        <circle cx="8.1" cy="12" r="0.48" fill="currentColor" stroke="none" />
                    </svg>
                </button>
                <button type="button" title="最小化" on:mousedown=stop_titlebar_drag on:click=minimize>
                    <svg class="wc-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true" focusable="false">
                        <path d="M4 8h8" />
                    </svg>
                </button>
                <button type="button" title="最大化" on:mousedown=stop_titlebar_drag on:click=toggle_maximize>
                    <svg class="wc-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" aria-hidden="true" focusable="false">
                        <rect x="4" y="4" width="8" height="8" rx="0.7" />
                    </svg>
                </button>
                <button type="button" class="x" title="关闭" on:mousedown=stop_titlebar_drag on:click=close>
                    <svg class="wc-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true" focusable="false">
                        <path d="M4.75 4.75l6.5 6.5" />
                        <path d="M11.25 4.75l-6.5 6.5" />
                    </svg>
                </button>
            </div>
        </header>
    }
}

#[cfg(test)]
mod tests {
    const TITLEBAR_SOURCE: &str = include_str!("titlebar.rs");
    const APP_STYLES: &str = include_str!("../../styles.css");

    #[test]
    fn window_controls_use_uniform_svg_icons() {
        assert_eq!(TITLEBAR_SOURCE.matches("<svg class=\"wc-icon").count(), 4);
        assert_eq!(TITLEBAR_SOURCE.matches("viewBox=\"0 0 16 16\"").count(), 4);
        assert!(TITLEBAR_SOURCE.contains("class=\"wc-icon wc-help-icon\""));
        assert!(
            !TITLEBAR_SOURCE.contains("class=\"wc-help-ring\""),
            "help control should render a bare question mark without a ring"
        );
        for glyph in ["\"?\"", "\"—\"", "\"□\"", "\"×\""] {
            assert!(
                !TITLEBAR_SOURCE.contains(glyph),
                "window control should not render raw text glyph {glyph}"
            );
        }
    }

    #[test]
    fn window_control_svg_box_is_stable() {
        assert!(APP_STYLES.contains(".wc-icon"));
        assert!(APP_STYLES.contains("width: 13px;"));
        assert!(APP_STYLES.contains("height: 13px;"));
        assert!(APP_STYLES.contains(".wc-help-icon"));
        assert!(APP_STYLES.contains("width: 15px;"));
        assert!(APP_STYLES.contains("height: 15px;"));
    }

    #[test]
    fn window_controls_are_flush_with_right_edge() {
        assert!(APP_STYLES.contains("padding: 0 0 0 12px;"));
        assert!(!APP_STYLES.contains("padding: 0 8px 0 12px;"));
    }
}
