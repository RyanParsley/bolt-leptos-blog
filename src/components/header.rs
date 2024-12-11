use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="header-container">
                <h1 class="header-title">My Blog</h1>
                <nav class="nav">
                    <ul class="nav-list">
                        <li><a href="/" class="nav-link">Home</a></li>
                        <li><a href="/about" class="nav-link">About</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}