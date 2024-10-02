use dioxus::prelude::*; // for element and stuff

fn main() {
    launch(App);
}

pub fn App() -> Element {
    rsx! {
        "story"
    }
}