use dioxus::prelude::*;

use crate::components::CustomSub;
#[component]
pub fn Home() -> Element {
    rsx! {
       h1 { "Home" }
       CustomSub {}
    }
}
