use dioxus::prelude::*; // for element and stuff

fn main() {
    launch(App);
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx!(MyFirstComponent{})
}

#[allow(non_snake_case)]
pub fn MyFirstComponent() -> Element {
    let title = "title";
    let author = "author";
    let date = chrono::Utc::now();

    rsx! {
        div { padding: "0.5rem", position: "relative",
            "{title} by {author} on {date}"
        }
    }
}