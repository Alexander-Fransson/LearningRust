use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(
        listener,
        Router::new()
            .route("/", get(app_endpoint))
            .into_make_service(),
    )
    .await
    .unwrap();
}

async fn app_endpoint() -> Html<String> {
    Html(dioxus_ssr::render_element(rsx! {
        p { "Hello, World!" }
    }))
}
