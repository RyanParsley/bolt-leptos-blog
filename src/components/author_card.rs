use leptos::*;

#[component]
pub fn AuthorCard(
    #[prop] name: String,
    #[prop] url: Option<String>,
) -> impl IntoView {
    view! {
        <div class="p-author h-card">
            {match url {
                Some(url) => view! {
                    <a href={url} class="u-url p-name">{name}</a>
                }.into_view(),
                None => view! {
                    <span class="p-name">{name}</span>
                }.into_view(),
            }}
        </div>
    }
}