#![allow(non_snake_case)]


use dioxus::prelude::*;
use std::time::Duration;
use async_std::task::sleep;

mod context;
mod conditionalrenders;
mod routhandling;
mod useassets;
mod asyncexample;

fn main() {
    launch(App);
}

fn App() -> Element {
    let mut count = use_signal(|| 0);
    let mut running = use_signal(|| true);

    let mut f = use_future(move || async move {
        loop {
            if running() {
                count += 1;
            }
            sleep(Duration::from_secs(1)).await;
       }
    });

    rsx!{
        div {
            color: "black",
            h1 { "Count: {count}" }
            button {
                onclick: move |_| running.toggle(),
                "Start/Stop"
            }
            button {
                onclick: move |_| count.set(0),
                "Reset"
            }
            button {
                onclick: move |_| f.cancel(),
                "Cancel"
            }
        }
    }
}