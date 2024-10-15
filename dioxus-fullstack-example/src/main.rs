#![allow(non_snake_case)]

use dioxus::prelude::*;
use http::response;
use serde::{Deserialize, Serialize};
use std::time::Duration;
//use dioxus_logger::tracing;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Request {
    name: String,
}

fn main() {
    // Init logger
    // dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    // tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    //let result = use_resource(get_data); // if you want to load in data fater the component renders
    let result = use_server_future(|| get_data(Request {
        name: "from client here".to_string()
    }))?; // if you want the page to wait on the server

    rsx! {
        p { "Server returned: {result.value():?}" }
    }
}

#[server]
async fn get_data(req:Request) -> Result<String, ServerFnError> {
    let headers: http::HeaderMap = extract().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let response = format!("Hi form server {:?}, data {:?}", headers[http::header::USER_AGENT], req.name); 
    Ok(response)
}