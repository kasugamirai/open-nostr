#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

use capybastr::App;

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
