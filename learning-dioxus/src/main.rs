use dioxus::prelude::*; // for element and stuff
use api::*;
use structs::*;

mod api;
mod structs;

fn main() {
    launch(App);
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
        id,
        ..
    } = story();

    let full_story = use_signal(|| None);

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} point{}", if score == 1 {"s"} else {""}); // change string dependent on values
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
            onmouseenter: move |_event| { resolve_story(full_story, preview_state, id)},
            div { font_size: "1.5rem",
                a { 
                    href: "{url}", 
                    onfocus:  move |_event| { resolve_story(full_story, preview_state, id)}, 
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