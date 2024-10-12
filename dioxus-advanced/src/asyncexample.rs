use dioxus::prelude::*;
use std::time::Duration;
use async_std::task::sleep;

#[allow(non_snake_case)]
#[allow(dead_code)]

#[component]
fn SpawnExample() -> Element {
   let mut clicked = use_signal(|| false);

   let handler = { move |_| {spawn(async move{
    clicked.set(true);
    sleep(Duration::from_secs(3)).await;
    clicked.set(false);
});()}};

   rsx!{
        div {
            if clicked() {
                p {
                    color: "black",
                    "Operation in progress, UI is not blocked..."   
                }
            } else {
                p {
                    button {
                        onclick: handler,
                        "Start process"
                    }
                }
            }
        }
   }
}