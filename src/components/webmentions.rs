use leptos::*;
use crate::types::webmention::{Webmention, WebmentionType};

#[component]
pub fn WebmentionList(
    #[prop] webmentions: Vec<Webmention>,
) -> impl IntoView {
    view! {
        <div class="webmentions">
            <h3>Responses</h3>
            
            // Replies
            <div class="replies">
                {webmentions.iter()
                    .filter(|w| matches!(w.interaction_type, WebmentionType::Reply))
                    .map(|mention| view! {
                        <div class="p-comment h-cite">
                            <div class="author h-card">
                                {mention.author.photo.as_ref().map(|photo| view! {
                                    <img src={photo.clone()} class="u-photo" alt={mention.author.name.clone()}/>
                                })}
                                <a href={mention.author.url.clone().unwrap_or_default()} class="p-author">{&mention.author.name}</a>
                            </div>
                            <div class="e-content">{mention.content.clone().unwrap_or_default()}</div>
                            <a href={mention.source} class="u-url">
                                <time class="dt-published">
                                    {mention.published.format("%Y-%m-%d").to_string()}
                                </time>
                            </a>
                        </div>
                    })
                    .collect::<Vec<_>>()}
            </div>

            // Likes and Reposts
            <div class="interactions">
                {webmentions.iter()
                    .filter(|w| matches!(w.interaction_type, WebmentionType::Like | WebmentionType::Repost))
                    .map(|mention| view! {
                        <div class={match mention.interaction_type {
                            WebmentionType::Like => "p-like-of",
                            WebmentionType::Repost => "p-repost-of",
                            _ => ""
                        }}>
                            <a href={mention.author.url.clone().unwrap_or_default()} class="h-card">
                                {mention.author.photo.as_ref().map(|photo| view! {
                                    <img src={photo.clone()} class="u-photo" alt={mention.author.name.clone()}/>
                                })}
                                <span class="p-name">{&mention.author.name}</span>
                            </a>
                        </div>
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}