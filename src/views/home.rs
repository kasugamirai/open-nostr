use std::future;

use dioxus::prelude::*;
use futures::{FutureExt, TryFutureExt};
use wasm_bindgen_test::console_log;

use crate::{components::CustomSub, store::CBWebDatabase};
use crate::store::DEFAULT_RELAY_SET_KEY;

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
