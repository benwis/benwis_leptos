use crate::error_template::*;
use crate::layouts::{Default};
use crate::providers::{provide_auth, provide_color_scheme, AuthContext};
use crate::routes::auth::{Join, Login, Logout};
use crate::routes::blog::*;
use crate::routes::{
    About, Index, Nedry, Portfolio,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn BenwisApp(cx: Scope) -> impl IntoView {
    // Create Actions for the Auth methods and provide the current user
    provide_auth(cx);
    let auth_context = use_context::<AuthContext>(cx).expect("Failed to get AuthContezt");
    _ = provide_color_scheme(cx);

    provide_meta_context(cx);

    view! { cx,
        <Router>
            <Routes>
                <Route
                path="minimal"
                view=move |cx| {
                    view! { cx, <Index/> }
                }
            />
                <Route
                    path=""
                    view=|cx| {
                        view! { cx,
                            <Default>
                                <ErrorBoundary fallback=|cx, errors| {
                                    view! { cx, <ErrorTemplate errors=errors/> }
                                }>
                                    <Outlet/>
                                </ErrorBoundary>
                            </Default>
                        }
                    }
                >
                    <Route
                        path=""
                        view=move |cx| {
                            view! { cx, <Index/> }
                        }
                    />
                    <Route
                        path="signup"
                        view=move |cx| {
                            view! { cx, <Join action=auth_context.signup/> }
                        }
                    />
                    <Route
                        path="about"
                        view=move |cx| {
                            view! { cx, <About/> }
                        }
                    />
                    <Route
                        path="portfolio"
                        view=move |cx| {
                            view! { cx, <Portfolio/> }
                        }
                    />
                    <Route
                        path="posts"
                        view=move |cx| {
                            view! { cx, <Blog/> }
                        }
                    />
                    <Route
                        path="posts/add"
                        view=move |cx| {
                            view! { cx, <AddPost/> }
                        }
                    />
                    <Route
                        path="posts/:slug"
                        view=move |cx| {
                            view! { cx, <Post/> }
                        }
                        ssr=SsrMode::Async
                    />
                    <Route
                        path="posts/:slug/edit"
                        view=move |cx| {
                            view! { cx, <EditPost/> }
                        }
                    />
                    <Route
                        path="login"
                        view=move |cx| {
                            view! { cx, <Login action=auth_context.login/> }
                        }
                    />
                    <Route
                        path="logout"
                        view=move |cx| {
                            view! { cx, <Logout action=auth_context.logout/> }
                        }
                    />
                    <Route
                        path="nedry"
                        view=move |cx| {
                            view! { cx, <Nedry/> }
                        }
                    />
                </Route>
            //     <Route
            //         path="/rss.xml"
            //         view=move |cx| {
            //             view! { cx, <Rss/> }
            //         }
            //         ssr=SsrMode::Async
            //     />
            </Routes>
        </Router>
    }
}
