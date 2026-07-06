mod app;
mod components;
mod model;
#[cfg(test)]
mod style_tests;
mod tauri_api;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
