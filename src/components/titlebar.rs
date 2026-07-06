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
                <button type="button" title="帮助" on:mousedown=stop_titlebar_drag>"?"</button>
                <button type="button" title="最小化" on:mousedown=stop_titlebar_drag on:click=minimize>"—"</button>
                <button type="button" title="最大化" on:mousedown=stop_titlebar_drag on:click=toggle_maximize>"□"</button>
                <button type="button" class="x" title="关闭" on:mousedown=stop_titlebar_drag on:click=close>"×"</button>
            </div>
        </header>
    }
}
