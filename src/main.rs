#![allow(non_snake_case)]

use capybastr::App;
use dioxus::prelude::*;
use tracing::Level;

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
