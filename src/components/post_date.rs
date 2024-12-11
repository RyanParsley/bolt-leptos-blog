use leptos::*;
use chrono::NaiveDate;

#[component]
pub fn PostDate(
    #[prop] date: NaiveDate,
) -> impl IntoView {
    let formatted_date = date.format("%Y-%m-%d").to_string();
    
    view! {
        <time class="dt-published" datetime={formatted_date.clone()}>
            {formatted_date}
        </time>
    }
}