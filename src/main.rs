#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

use capybastr::Route;

use capybastr::CustomSub;

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing::info!("Starting Dioxus");
    launch(App);
}

fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");
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
