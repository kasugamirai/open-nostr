#![allow(non_snake_case)]

mod components;
mod router;
mod state;
mod utils;
mod views;
mod nostr;

use dioxus::prelude::*;
use tracing::Level;

use router::Route;

use crate::state::CustomSubs;

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    let state = use_context_provider(|| Signal::new(String::from("light")));
    let _subs = use_context_provider(|| Signal::new(CustomSubs::new()));

    rsx! {
        div {
            id: "app",
            class: "{state}",
            Router::<Route> {}
        }
    }
}
