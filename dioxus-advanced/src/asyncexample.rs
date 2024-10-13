use dioxus::prelude::*;
use std::time::Duration;
use async_std::task::sleep;
use crate::asyncexample::Action::Idle;
use crate::asyncexample::Action::SendMessage;
use crate::asyncexample::Action::CloseChat;
use async_std::stream::StreamExt;
use core::fmt::Display;

#[allow(non_snake_case)]
#[allow(dead_code)]


enum Action {
    Idle,
    SendMessage(String),
    CloseChat
}

impl Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Idle => write!(f, "Idle"),
            SendMessage(msg) => write!(f, "Sending Message: {}", msg),
            CloseChat => write!(f, "Closing Conversation"),
        }
    }
}

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

#[component]
fn UseFuture() -> Element {
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

#[component]
fn useResource() -> Element {
    let mut future =  use_resource(|| async move {
        sleep(Duration::from_secs(3)).await; // simulate API call
        "hi world" // use resource is similar to use future except it can return a value
    });
    
    match &*future.read_unchecked() {
        Some(data) => rsx!{
            p {"{data}"},
            button {
                onclick: move |_| future.restart(),
                "Restart"
            }
        },
        _ => rsx!{
            p {"Loading..."},
        }
    }
}

#[component]
fn Corutine() -> Element {
    let mut current_action: Signal<Action> = use_signal(|| Idle);
    
    let x = use_coroutine(|mut rx: UnboundedReceiver<Action>| {async move {
        while let Some(msg) = rx.next().await {
            match msg {
                Idle => current_action.set(Idle),
                SendMessage(msg) => {
                    current_action.set(SendMessage(msg));
                    sleep(Duration::from_secs(3)).await;
                    current_action.set(Idle);
                },
                CloseChat => current_action.set(CloseChat),
            }            
        }
    }});
    
    rsx!{
        div {
            p {
                color: "black",
                "{current_action}"
            }
            button {
                onclick: move |_| x.send(SendMessage(String::from("hello"))),
                "Send"
            }
            button {
                onclick: move |_| x.send(CloseChat),
                "Close"
            }
        }
    }

}