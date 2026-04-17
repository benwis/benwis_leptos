use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn FeatureCard(title: String, href: String, date: String) -> impl IntoView {
    view! {
        <a class="post-card" href=format!("/posts/{href}")>
            <div class="post-card__layout">
                <h3 class="post-card__heading">{title}</h3>
                <p class="post-card__meta">{date}</p>
            </div>
        </a>
    }
}
