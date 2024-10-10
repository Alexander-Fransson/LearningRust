#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    dioxus_logger::init(Level::INFO)
        .expect("failed to init");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut count = use_signal(||0);
    let mut inputText = use_signal(||String::from(""));

    rsx! {
        button {
            onclick: move |_event| {
                info!("{:?}", _event)
            },
            "Click me!"
        }
        button {
            onclick: move |_| count += 1,
            "add to count"
        }
        input {
            value: "{inputText}",
            oninput: move |ev| {
                inputText.set(ev.value());
            }
        }
        p{"input {inputText}"}

        p { "Clicked: {count}" }

        ol{
            li { "Hello, world!" }
            li { "Hello, world!" }
            li { "Hello, world!" }
        }
        Notes {text:"poop"}
    }
}

#[derive(Props, Clone, PartialEq)] // all of these are needed for props to render and be used many times
struct CustomProps {
    text: String,

    #[props(optional)]
    color: Option<String>,
    #[props(default = 10)]
    size: i32,
}

#[component]
fn Notes(props: CustomProps) -> Element {
    rsx! {
        h1 {
            background: "yellow",
            "{props.text}"
        }
    }
}