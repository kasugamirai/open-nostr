use dioxus::prelude::*;

#[component]
pub fn Message() -> Element {
    rsx! {
        h1 { "Message" }
    }
}
