#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info, Level};

mod pages;
mod layout;
use crate::layout::Route;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
