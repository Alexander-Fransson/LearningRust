#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
fn app() -> Element {
    let mut is_logged_in = use_signal(|| false);

    rsx! {
        button {
            onclick: move |_| is_logged_in.set(!is_logged_in()),
            if is_logged_in() {
                "Log out"
            } else {
                "Log in"
            }
        }
    }
}