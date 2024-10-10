#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        p { "Hello, world!" }
    }
}
