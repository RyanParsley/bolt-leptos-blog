use leptos::*;
use crate::types::syndication::SyndicationLink;

#[component]
pub fn SyndicationLinks(
    #[prop] links: Vec<SyndicationLink>,
) -> impl IntoView {
    view! {
        <div class="syndication-links">
            <h4>Also available on:</h4>
            <ul>
                {links.into_iter().map(|link| view! {
                    <li>
                        <a href={link.url} class="u-syndication" rel="syndication">
                            {link.platform}
                        </a>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}