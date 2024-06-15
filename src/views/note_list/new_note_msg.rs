use dioxus::prelude::*;
use nostr_sdk::SubscriptionId;

use crate::init::{MODAL_MANAGER, SUB_COUNTERS};


#[component]
pub fn NewNoteMsg(sub_id: SubscriptionId) -> Element {
    let count = SUB_COUNTERS.read().get(&sub_id).unwrap_or(0);

    rsx! {
        div {
            class: "new-note-msg",
            onclick: move |_| {
                SUB_COUNTERS.write().clear(&sub_id);
                MODAL_MANAGER.write().close_modal("sub-new-msg");
            },
            {format!("Received {} New Events !!", count)}
        }
    }
}