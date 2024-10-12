#![allow(non_snake_case)]
#![allow(dead_code)]
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/contactus")]
    ContactUs {},
}

fn RoutMaker() -> Element {
    rsx! {
        Router::<Route>{}
    }
}

fn Home() -> Element {
    rsx! {
       "Home Page"
    }
}

fn ContactUs() -> Element {
    rsx! {
        "Contact Us"
    }
}