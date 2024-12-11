use leptos::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    view! {
        <header class="header h-card">
            <div class="header-container">
                <h1 class="header-title p-name">My Blog</h1>
                <nav class="nav">
                    <ul class="nav-list">
                        <li><a href="/" class="nav-link u-url">Home</a></li>
                        <li><a href="/about" class="nav-link">About</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}