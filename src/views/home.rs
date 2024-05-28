use dioxus::prelude::*;
use futures::{FutureExt, TryFutureExt};
use wasm_bindgen_test::console_log;

use crate::{
  components::Notification,
  components::Author, 
  store::CBWebDatabase
};
use crate::store::DEFAULT_RELAY_SET_KEY;

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
          // Notification{}
          Author{}
        }
      }

    }
}
