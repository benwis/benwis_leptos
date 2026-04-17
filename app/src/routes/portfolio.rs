use crate::components::PortfolioCard;
use leptos::prelude::*;
use leptos::{IntoView, component, view};
use leptos_meta::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
        <Meta property="og:title" content="My Portfolio" />
        <Title text="My Portfolio" />
        <Meta name="description" content="A collection of things I've built or helped build." />
        <Meta
            property="og:description"
            content="A collection of things I've built or helped build."
        />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg" />
        <article id="portfolio">
            <div id="portfolio__header">
                <h1 id="portfolio__heading">"Portfolio"</h1>
            </div>
            <main id="portfolio__posts" role="list">
                <PortfolioCard
                    heading="Leptos".to_string()
                    sub_heading="Full Stack Rust Web Framework".to_string()
                    href="https://github.com/leptos-rs/leptos".to_string()
                    img="https://benwis.imgix.net/leptos_dev.png?auto=format&auto=compress"
                        .to_string()
                    description="A full-stack web framework that lets you leverage the power of Rust and fine-grained reactivity to deliver interactive, stable, and powerful web applications"
                        .to_string()
                />
                <PortfolioCard
                    heading="tower-governor".to_string()
                    sub_heading="Tower Middleware to Rate Limit APIs".to_string()
                    href="https://crates.io/crates/tower_governor".to_string()
                    img="https://benwis.imgix.net/tower-governor.png?auto=format&auto=compress"
                        .to_string()
                    description="A Rust crate implementing a Tower middleware that rate limits APIs based on the Generic Cell Rate Algorithm. Can use a wide variety of keys, including IP or user ID"
                        .to_string()
                />
                <PortfolioCard
                    heading="Femark".to_string()
                    sub_heading="Fastest Rust and Webassembly Markdown to HTML Compiler with Syntax Highlighting"
                        .to_string()
                    href="https://crates.io/crates/femark".to_string()
                    img="https://benwis.imgix.net/femark.png?auto=format&auto=compress".to_string()
                    description="Blazingly fast Markdown to HTML compiler with syntax highlighting for a variety of languages. Runs in the browser and on the server"
                        .to_string()
                />
                <PortfolioCard
                    heading="Praxis Cycles".to_string()
                    sub_heading="Ecommerce Store".to_string()
                    href="https://praxiscycles.com".to_string()
                    img="/img/praxiscycles_square.png".to_string()
                    description="Praxis Cycles is a bike parts supplier that does over a million dollars in annual sales. I redesigned the site to a media heavy layout, optimized image distribution, and massively decreased page load time"
                        .to_string()
                />
                <PortfolioCard
                    heading="Praxis OEM Site".to_string()
                    sub_heading="B2B Sales Site".to_string()
                    href="https://oem.praxiscycles.com".to_string()
                    img="/img/praxis_oem_square.png".to_string()
                    description="Praxis' OEM site is a login only site that lists their hundreds of available parts for dealers to purchase. I built the frontend using Remix, React, and TailwindCSS. The backend is a custom rust GraphQL server linked to Airtable and Sanity for data entry."
                        .to_string()
                />
            </main>
        </article>
    }
}
