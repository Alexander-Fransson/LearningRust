use dioxus::prelude::*; // for element and stuff
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

//Define the hackernews type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,

    #[serde(default)]
    pub by: String,
    
    #[serde(default)]
    pub text: String,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,

    #[serde(default)]
    pub kids: Vec<i64>,

    #[serde(default)]
    pub sub_comments: Vec<Comment>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    
    #[serde(default)]
    pub score: i64,

    #[serde(default)]
    pub by: String,

    #[serde(default)]
    pub descendants: i64,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,

    #[serde(default)]
    pub kids: Vec<i64>,

    pub r#type: String // r# is for rust to allow using the word type
}

fn main() {
    launch(App);
}

#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData)
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset)); // to globally manage preview state

    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { width: "50%", Stories {} }
            div { width: "50%", Preview {} }
        }
    }
}

#[allow(non_snake_case)]
fn Stories() -> Element {
    rsx! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    }
}



#[allow(non_snake_case)]
fn Preview() -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();
    match preview_state() {
        PreviewState::Unset => rsx! {"Hover over a story to preview it here"},
        PreviewState::Loading => rsx! {"Loading..."},
        PreviewState::Loaded(story) => {
            rsx! {
                div { padding: "0.5rem",
                    div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}" } }
                    div { dangerous_inner_html: story.item.text }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn Comment(comment: Comment) -> Element {
    rsx!{
        div { padding: "0.5rem",
            div { color: "gray", "by {comment.by}" }
            div { dangerous_inner_html: "{comment.text}" }
            for kids in &comment.sub_comments {
                Comment { comment: kids.clone() }
                
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let mut preview_state = consume_context::<Signal<PreviewState>>();
    let StoryItem {
        title, 
        by,
        score,
        time,
        kids,
        url,
        ..
    } = &*story.read();

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} point{}", if *score == 1 {"s"} else {""}); // change string dependent on values
    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {"comment"} else {"comments"}
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div { 
            padding: "0.5rem", 
            position: "relative",
            onmouseenter: move |_event| {
                *preview_state
                    .write() = PreviewState::Loaded(StoryPageData {
                        item: story(),
                        comments: vec![],
                    });
            },
            div { font_size: "1.5rem",
                a { 
                    href: "{url}", 
                    onfocus: move |_event| {
                        *preview_state
                            .write() = PreviewState::Loaded(StoryPageData {
                            item: story(),
                            comments: vec![],
                        });
                    }, 
                    "{title}" 
                },
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div { "{score}" }
                div { padding_left: "0.5rem", "by {by}" }
                div { padding_left: "0.5rem", "{time}" }
                div { padding_left: "0.5rem", "{comments}" }
            }
        }
    }
}