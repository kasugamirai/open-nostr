use dioxus::prelude::*;
use crate::components::CustomSub;

#[component]
pub fn Home() -> Element {
    rsx! {
      div{
        class:"flex-box",
        div{
          class:"flex-box-left",
          h1 { "Home" }
        }
        div{
          CustomSub {}
        }
      }

    }
}
