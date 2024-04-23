#![allow(non_snake_case)]

mod components;
mod nostr;
mod router;
mod state;
mod utils;
mod views;

use dioxus::prelude::*;
use tracing::Level;

use router::Route;

use crate::state::subscription::CustomSub;

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    log::info!("Starting Dioxus");
    launch(App);
}

fn App() -> Element {
    let state = use_context_provider(|| Signal::new(String::from("light")));
    let _custom_sub_global = use_context_provider(|| Signal::new(CustomSub::default()));

    rsx! {
        div {
            id: "app",
            class: "{state}",
            Router::<Route> {}
        }
    }
}
