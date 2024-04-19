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

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    let state = use_context_provider(|| Signal::new(String::from("light")));

    rsx! {
        div {
            id: "app",
            class: "{state}",
            Router::<Route> {}
        }
    }
}
