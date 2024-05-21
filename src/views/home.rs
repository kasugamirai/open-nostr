use dioxus::prelude::*;
use crate::components::CustomSub;

#[component]
pub fn Home() -> Element {
    rsx! {
      div{
        class:"flexBox",
        div{
          class:"flexBoxLeft",
          h1 { "Home" }
        }
        div{
          CustomSub {}
        }
      }

    }
}
