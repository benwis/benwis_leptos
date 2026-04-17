use crate::error_template::NotFound;
use crate::layouts::Default;
use crate::providers::{AuthContext, provide_auth, provide_color_scheme};
use crate::routes::auth::{Join, Login, Logout};
use crate::routes::blog::*;
use crate::routes::{About, Index, Nedry, Portfolio};
use leptos::reactive::owner::use_context;
use leptos::{IntoView, component};
use leptos::{prelude::*, view};
use leptos_meta::*;
use leptos_router::components::{FlatRoutes, Route, Router};
use leptos_router::path;

#[component]
pub fn BenwisApp() -> impl IntoView {
    provide_auth();
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    provide_color_scheme();

    provide_meta_context();

    let signup_action = auth_context.signup;
    let login_action = auth_context.login;
    let logout_action = auth_context.logout;

    view! {
        <Router>
            <Default>
                <FlatRoutes fallback=NotFound>
                    <Route
                        path=path!("")
                        view=move || view! { <HomeIndex /> }
                    />
                    <Route
                        path=path!("signup")
                        view=move || view! { <Join action=signup_action /> }
                    />
                    <Route path=path!("about") view=About />
                    <Route path=path!("portfolio") view=Portfolio />
                    <Route path=path!("posts") view=Blog />
                    <Route path=path!("posts/add") view=AddPost />
                    <Route path=path!("posts/:slug") view=Post />
                    <Route path=path!("posts/:slug/edit") view=EditPost />
                    <Route
                        path=path!("login")
                        view=move || view! { <Login action=login_action /> }
                    />
                    <Route
                        path=path!("logout")
                        view=move || view! { <Logout action=logout_action /> }
                    />
                    <Route path=path!("nedry") view=Nedry />
                </FlatRoutes>
            </Default>
        </Router>
    }
}

/// Wrapper that sets <Body id="home-template"> for the home page,
/// then renders the actual Index content.
#[component]
fn HomeIndex() -> impl IntoView {
    view! {
        <Body attr:id="home-template" />
        <Index />
    }
}
