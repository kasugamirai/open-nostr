use std::future;

use dioxus::prelude::*;
use futures::{FutureExt, TryFutureExt};
use wasm_bindgen_test::console_log;

use crate::store::DEFAULT_RELAY_SET_KEY;
use crate::{components::CustomSub, store::CBWebDatabase};

#[component]
pub fn Home() -> Element {
    let cb_database_db = use_context::<Signal<CBWebDatabase>>();

    use_effect(move || {
        spawn(async move {
            let binding = cb_database_db.read();
            let relay_set = binding
                .get_relay_set(DEFAULT_RELAY_SET_KEY.to_string())
                .await
                .unwrap();
            console_log!("Relay set: {:?}", relay_set);
        });
    });

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
