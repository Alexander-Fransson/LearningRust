#![allow(non_snake_case)]

use dioxus::prelude::*;
use context::*;
use conditionalrenders::*;
use routhandling::*;

mod context;
mod conditionalrenders;
mod routhandling;

fn main() {
    launch(HungarianFlower);
    //launch(RoutMaker); 
    //launch(app);
    //launch(ThumbnailEditor);
}

fn HungarianFlower () -> Element {
    rsx! {
        "Hungarian Flower"
        img {
            src: "/HungarianFlower1.svg"
        }
    }
}