#![allow(non_snake_case)]

use dioxus::prelude::*;

  /********************************/
 /* Demonstration of use_context */
/********************************/

#[derive(Clone)]
struct ThumbnailInfo {
    title: String,
}

#[component]
fn TitleEditor() -> Element {
    let mut context = use_context::<Signal<ThumbnailInfo>>(); 
    rsx! {
        input {
            value: "{context().title}",
            oninput: move |event| context.write().title = event.value()
        }
    }
}

#[component]
fn Thumbnail() -> Element {
    let context = use_context::<Signal<ThumbnailInfo>>();
    rsx! {
        div {
            h1 { "thumbnail" }
            p { "{context().title}" }
        }
    }
}

#[component]
fn ThumbnailEditor() -> Element {

    use_context_provider(|| Signal::new(ThumbnailInfo { title: String::from("Rust 101") }));

    rsx! {
        div {
            h1 { "Thumbnail editor" }
            Thumbnail  {}
            TitleEditor {}
        }
    }
}