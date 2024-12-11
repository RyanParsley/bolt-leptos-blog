use leptos::*;
use crate::components::site_header::SiteHeader;
use crate::components::post_card::PostCard;
use crate::utils::posts::get_posts;

#[component]
pub fn HomePage() -> impl IntoView {
    let posts = get_posts();

    view! {
        <div>
            <SiteHeader/>
            <main class="container h-feed">
                <div class="post-grid">
                    {posts.into_iter().map(|post| view! { <PostCard post={post}/> }).collect::<Vec<_>>()}
                </div>
            </main>
        </div>
    }
}