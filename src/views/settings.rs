use dioxus::prelude::*;

use crate::components::{InputCard, Switch};

#[component]
pub fn Settings() -> Element {
    let mut text = use_signal(|| "Hello".to_string());
    let handle_change = move |value| {
        text.set(if value {
            "Goodbye".to_string()
        } else {
            "Hello".to_string()
        });
    };
    let handle_input = move |value| {
        text.set(value);
    };
    rsx! {
        h1 { "Settings" }
        Switch { on_change: handle_change, value: false, close: rsx!{ "Goodbye" }, open: rsx! { "{text}" } }
        p { "{text}" }
        InputCard { on_change: handle_input, value: false }
    }
}
