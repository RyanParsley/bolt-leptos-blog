use leptos::*;
mod components;
mod pages;
mod utils;
mod types;

use crate::pages::home::HomePage;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <HomePage/>
        </div>
    }
}