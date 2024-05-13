use dioxus::prelude::*;

use crate::components::icons::*;
#[component]
pub fn CustomSub() -> Element {
    rsx! {
        // Custom Sub component
        div {
            class: "custom-sub-wrapper",
            div {
                class: "custom-sub-header",
                h1 {
                    class: "title custom-sub-title font-Raleway-800 font-size-20",
                    "Custom Sub"
                }
                button {
                    class: "icon",
                    dangerous_inner_html: "{MORE}"
                }
            }
        }
    }
}