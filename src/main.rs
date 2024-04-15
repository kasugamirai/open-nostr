#![allow(non_snake_case)]

mod components;
mod router;
mod state;
mod utils;
mod views;

use dioxus::prelude::*;
use log::LevelFilter;

use router::Route;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

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
