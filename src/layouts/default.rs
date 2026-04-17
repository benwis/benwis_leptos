use leptos::children::Children;
use leptos::reactive::owner::use_context;
use leptos::{IntoView, component};
use leptos::{prelude::*, view};
use leptos_meta::*;

use crate::components::{Footer, Nav};
use crate::providers::color_scheme::ColorScheme;

#[component]
pub fn Default(children: Children) -> impl IntoView {
    let color_scheme = use_context::<ColorScheme>().expect("Failed to find ColorScheme");

    view! {
        <Html attr:lang="en-US" attr:class=move || {
            let classes = "h-full";
            let theme = match color_scheme.prefers_dark.get() {
                true => "dark",
                false => "",
            };
            format!("{} {}", classes, theme)
        } />

        <Body />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Link
            rel="alternate"
            type_="application/rss+xml"
            href="https://benw.is/rss.xml"
            title="benwis Blog"
        />
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
        <Link
            href="https://fonts.googleapis.com/css2?family=Archivo:ital,wght@0,100..900;1,100..900&display=swap"
            rel="stylesheet"
        />
        <Link
            href="https://fonts.googleapis.com/css2?family=Archivo+Black&family=Orbitron:wght@400..900&display=swap"
            rel="stylesheet"
        />
        // Fathom - beautiful, simple website analytics
        <script
            src="https://cdn.usefathom.com/script.js"
            data-site="MTMCOBMG"
            defer
        ></script>
        <Stylesheet id="leptos" href="/pkg/benwis_leptos.css" />
        <Meta charset="utf-8" />

        <div id="site-viewport">
            <div id="site-layout">
                <Nav />
                {children()}
                <Footer />
            </div>
        </div>
    }
}
