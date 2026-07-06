use crate::model::{status_text, ServerStatus};
use leptos::prelude::*;

#[component]
pub fn StatusPill(status: ReadSignal<ServerStatus>) -> impl IntoView {
    view! {
        <span class=move || match status.get() {
            ServerStatus::Running => "status-pill running",
            ServerStatus::Starting => "status-pill starting",
            ServerStatus::Stopped => "status-pill stopped",
        }>
            <span></span>
            {move || status_text(status.get())}
        </span>
    }
}
