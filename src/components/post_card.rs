use leptos::*;
use crate::types::post::BlogPost;
use crate::components::author_card::AuthorCard;
use crate::components::post_date::PostDate;
use crate::components::webmentions::WebmentionList;
use crate::components::syndication_links::SyndicationLinks;

#[component]
pub fn PostCard(post: BlogPost) -> impl IntoView {
    let post_url = format!("/posts/{}", post.slug);

    view! {
        <article class="post-card h-entry">
            <h2 class="post-title p-name">
                <a href={post_url.clone()} class="u-url">{post.title}</a>
            </h2>
            
            <div class="post-meta">
                <AuthorCard name={post.author.clone()} url={post.author_url.clone()}/>
                <PostDate date={post.date}/>
            </div>

            // Context links (reply/like/repost)
            {post.in_reply_to.as_ref().map(|url| view! {
                <div class="context">
                    In reply to <a href={url.clone()} class="u-in-reply-to">{url}</a>
                </div>
            })}
            {post.like_of.as_ref().map(|url| view! {
                <div class="context">
                    Likes <a href={url.clone()} class="u-like-of">{url}</a>
                </div>
            })}
            {post.repost_of.as_ref().map(|url| view! {
                <div class="context">
                    Reposted from <a href={url.clone()} class="u-repost-of">{url}</a>
                </div>
            })}

            <div class="p-summary">{post.description.clone()}</div>
            
            <div class="e-content post-content"
                 inner_html={post.content.clone()}>
            </div>

            <SyndicationLinks links={post.syndication}/>
            <WebmentionList webmentions={post.webmentions}/>
        </article>
    }
}