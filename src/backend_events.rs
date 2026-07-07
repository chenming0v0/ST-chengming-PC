use crate::model::{ProgressPayload, ServerStatus, TerminalOutputPayload};
use crate::tauri_api::{listen, tauri_available};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

pub fn bind_backend_events(
    active_session: ReadSignal<Option<String>>,
    set_current_stage: WriteSignal<String>,
    set_progress: WriteSignal<u32>,
    set_logs: WriteSignal<Vec<String>>,
    set_status: WriteSignal<ServerStatus>,
) {
    spawn_local(async move {
        if !tauri_available() {
            return;
        }

        let install_handler = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
            let payload = js_sys::Reflect::get(&event, &JsValue::from_str("payload"))
                .ok()
                .and_then(|value| serde_wasm_bindgen::from_value::<ProgressPayload>(value).ok());
            if let Some(payload) = payload {
                set_current_stage.set(payload.stage.clone());
                set_progress.set(payload.percent);
                set_logs.update(|items| {
                    items.push(format!(
                        "[{}:{}%] {}",
                        payload.stage, payload.percent, payload.message
                    ))
                });
            }
        });
        let _ = listen("install-progress", install_handler.as_ref().unchecked_ref()).await;
        install_handler.forget();

        let terminal_handler = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
            let payload = js_sys::Reflect::get(&event, &JsValue::from_str("payload"))
                .ok()
                .and_then(|value| {
                    serde_wasm_bindgen::from_value::<TerminalOutputPayload>(value).ok()
                });
            if let Some(payload) = payload {
                let show = active_session
                    .get_untracked()
                    .as_deref()
                    .map(|id| id == payload.session_id)
                    .unwrap_or(true);
                if show {
                    set_logs.update(|items| {
                        for line in payload.data.replace("\r\n", "\n").split('\n') {
                            if !line.is_empty() {
                                if is_tavern_ready_line(line) {
                                    set_status.set(ServerStatus::Running);
                                }
                                items.push(line.to_string());
                            }
                        }
                    });
                }
            }
        });
        let _ = listen("terminal-output", terminal_handler.as_ref().unchecked_ref()).await;
        terminal_handler.forget();
    });
}

fn is_tavern_ready_line(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.contains("sillytavern is listening")
        || lower.contains("server is listening")
        || lower.contains("listening on")
        || lower.contains("http://127.0.0.1:8000")
        || lower.contains("http://localhost:8000")
}
