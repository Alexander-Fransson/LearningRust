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
    pub comments: Vec<Comment>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
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

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! {
        StoryListing{
            story: StoryItem{
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
#[component]
pub fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let StoryItem {
        title, 
        by,
        score,
        time,
        kids,
        ..
    } = &*story.read();

    let comments = kids.len();

    rsx! {
        div { padding: "0.5rem", position: "relative",
            "{title} by {by} ({score}) {time} {comments}"
        }
    }
}