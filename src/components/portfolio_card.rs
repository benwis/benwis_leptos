use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn PortfolioCard(
    img: String,
    href: String,
    heading: String,
    sub_heading: String,
    description: String,
) -> impl IntoView {
    view! {
        <a class="post-card" href=href>
            <img class="post-card__image" src=img alt=sub_heading.clone() />
            <div class="post-card__text">
                <h2 class="post-card__heading">{heading}</h2>
                <p class="post-card__excerpt">{description}</p>
            </div>
        </a>
    }
}
