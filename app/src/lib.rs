pub mod components;
pub mod error_template;
pub mod functions;
pub mod layouts;
pub mod providers;
pub mod routes;

use crate::error_template::ErrorTemplate;
use crate::layouts::Default;
use crate::providers::provide_color_scheme;
use crate::routes::blog::*;
use crate::routes::{About, Index, Nedry, Portfolio};
use leptos::nonce::use_nonce;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn BenwisApp() -> impl IntoView {
    // Create Actions for the Auth methods and provide the current user
    _ = provide_color_scheme();
    provide_meta_context();
    view! {
        {
            if std::env::var("LEPTOS_ENV") == Ok("production".to_string()){
                view!{
                <Meta
                    http_equiv="Content-Security-Policy"
                    content=move || {
                        use_nonce()
                            .map(|nonce| {
                                format!(
                                    "default-src 'self'; base-uri 'self'; img-src 'nonce-{nonce}' 'self' https://benwis.imgix.net https://cdn.usefathom.com; script-src 'nonce-{nonce}' 'strict-dynamic' 'wasm-unsafe-eval'; style-src 'nonce-{nonce}' 'self' https://cdn.usefathom.com/script.js;"
                                )
                            })
                            .unwrap_or_default()
                    }
                />
                    }.into_view()
            } else{
            ().into_view()
            }
        }
        <Router>
            <Routes>
                <Route
                    path="minimal"
                    view=move || {
                        view! { <Index/> }
                    }
                />

                <Route
                    path=""
                    view=|| {
                        view! {
                            <Default>
                                <ErrorBoundary fallback=|errors| {
                                    view! { <ErrorTemplate errors=errors/> }
                                }>
                                    <Outlet/>
                                </ErrorBoundary>
                            </Default>
                        }
                    }
                >

                    <Route
                        path=""
                        view=move || {
                            view! { <Index/> }
                        }
                    />

                    <Route
                        path="about"
                        view=move || {
                            view! { <About/> }
                        }
                    />

                    <Route
                        path="portfolio"
                        view=move || {
                            view! { <Portfolio/> }
                        }
                    />

                    <Route
                        path="posts"
                        view=move || {
                            view! { <Blog/> }
                        }
                    />

                    <Route
                        path="posts/:slug"
                        view=move || {
                            view! { <Post/> }
                        }

                        ssr=SsrMode::Async
                    />
                    <Route
                        path="nedry"
                        view=move || {
                            view! { <Nedry/> }
                        }
                    />

                </Route>
            // <Route
            // path="/rss.xml"
            // view=move || {
            // view! {  <Rss/> }
            // }
            // ssr=SsrMode::Async
            // />
            </Routes>
        </Router>
    }
}
