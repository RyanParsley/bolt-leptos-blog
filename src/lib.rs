pub mod components;
pub mod pages;
pub mod types;
pub mod utils;
pub mod generated;

use leptos::*;
use pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <HomePage/>
        </div>
    }
}