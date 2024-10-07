use dioxus::prelude::*; // for element and stuff
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use futures::future::join_all;

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";
const COMMENT_DEPTH: i8 = 2;

pub async fn get_story_preview(id: i64) -> Result<StoryItem, reqwest::Error> {
    let url = format!("{BASE_API_URL}{ITEM_API}{id}.json");
    return reqwest::get(url).await?.json().await;
}

pub async fn get_stories(count:usize) -> Result<Vec<StoryItem>, reqwest::Error> {
    let url = format!("{BASE_API_URL}topstories.json");
    let stories_ids = &reqwest::get(&url).await?.json::<Vec<i64>>().await?[..count];

    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_story_preview(story_id));
    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect();
    Ok(stories)
}

pub async fn get_story(id:i64) -> Result<StoryPageData, reqwest::Error> {
    let url = format!("{BASE_API_URL}{ITEM_API}{id}.json");
    let mut story = reqwest::get(&url).await?.json::<StoryPageData>().await?;
    let comment_futures = story.item.kids.iter().map(|&id| get_comment(id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|comment| comment.ok())
        .collect();
    story.comments = comments;
    Ok(story)
}

pub async fn get_comment_with_depth(comment_id: i64, depth: i8) -> Result<Comment, reqwest::Error> {
    let url = format!("{BASE_API_URL}{ITEM_API}{comment_id}.json");
    let mut comment = reqwest::get(&url).await?.json::<Comment>().await?;
    if depth > 0 {
        let sub_comment_futures = comment
            .kids
            .iter()
            .map(|story_id| get_comment_with_depth(*story_id, depth - 1));
        comment.sub_comments = join_all(sub_comment_futures)
            .await
            .into_iter()
            .filter_map(|comment| comment.ok())
            .collect();
    }
    Ok(comment)
}

pub async fn get_comment(comment_id: i64) -> Result<Comment, reqwest::Error> {
    let comment = get_comment_with_depth(comment_id, COMMENT_DEPTH).await?;
    Ok(comment)
}

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
    // use_resource will 
    let stories = use_resource(move|| get_stories(10));
    
    match &*stories.read_unchecked() {
        Some(Ok(list)) => {
            rsx!{
                div {
                    for story in list {
                        StoryListing { story: story.clone() }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx!{"An error occurred: {err}"}
        }
        None => { rsx!{"Loading..."} }
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