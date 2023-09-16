use crate::error_template::*;
use crate::layouts::Default;
use crate::providers::{provide_auth, provide_color_scheme, AuthContext};
use crate::routes::auth::{Join, Login, Logout};
use crate::routes::blog::*;
use crate::routes::{About, Index, Nedry, Portfolio};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn BenwisApp() -> impl IntoView {
    // Create Actions for the Auth methods and provide the current user
    provide_auth();
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContezt");
    _ = provide_color_scheme();

    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route
                path="minimal"
                view=move || {
                    view! {  <Index/> }
                }
            />
                <Route
                    path=""
                    view=|| {
                        view! {
                            <Default>
                                <ErrorBoundary fallback=| errors| {
                                    view! {  <ErrorTemplate errors=errors/> }
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
                            view! {  <Index/> }
                        }
                    />
                    <Route
                        path="signup"
                        view=move || {
                            view! {  <Join action=auth_context.signup/> }
                        }
                    />
                    <Route
                        path="about"
                        view=move || {
                            view! {  <About/> }
                        }
                    />
                    <Route
                        path="portfolio"
                        view=move || {
                            view! {  <Portfolio/> }
                        }
                    />
                    <Route
                        path="posts"
                        view=move || {
                            view! {  <Blog/> }
                        }
                    />
                    <Route
                        path="posts/add"
                        view=move || {
                            view! {  <AddPost/> }
                        }
                    />
                    <Route
                        path="posts/:slug"
                        view=move || {
                            view! {  <Post/> }
                        }
                        ssr=SsrMode::Async
                    />
                    <Route
                        path="posts/:slug/edit"
                        view=move || {
                            view! {  <EditPost/> }
                        }
                    />
                    <Route
                        path="login"
                        view=move || {
                            view! {  <Login action=auth_context.login/> }
                        }
                    />
                    <Route
                        path="logout"
                        view=move || {
                            view! {  <Logout action=auth_context.logout/> }
                        }
                    />
                    <Route
                        path="nedry"
                        view=move || {
                            view! {  <Nedry/> }
                        }
                    />
                </Route>
            //     <Route
            //         path="/rss.xml"
            //         view=move || {
            //             view! {  <Rss/> }
            //         }
            //         ssr=SsrMode::Async
            //     />
            </Routes>
        </Router>
    }
}
