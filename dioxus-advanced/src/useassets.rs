#![allow(dead_code)]
#![allow(non_snake_case)]
use dioxus::prelude::*;

fn HungarianFlower () -> Element {
    rsx! {
        "Hungarian Flower"
        img {
            src: "/HungarianFlower1.svg"
        }
    }
}