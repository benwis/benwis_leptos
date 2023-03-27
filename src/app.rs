use crate::error_template::*;
use crate::providers::{provide_auth, provide_color_scheme, AuthContext};
use crate::routes::auth::{Join, JoinProps, Login, LoginProps, Logout, LogoutProps};
use crate::routes::blog::*;
use crate::routes::{
    About, AboutProps, Index, IndexProps, Portfolio, PortfolioProps, Rss, RssProps, Nedry, NedryProps,
};
use crate::layouts::{Default, DefaultProps};
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
                    path=""
                    view=|cx| {
                        view! { cx,
                            <Default>
                                <ErrorBoundary fallback=|cx, errors| {
                                    view! { cx, <ErrorTemplate errors=errors/> }
                                }>
                                    <Index/>
                                </ErrorBoundary>
                            </Default>
                        }
                    }
                />
                <Route
                    path="signup"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <Join action=auth_context.signup/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="about"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <About/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="portfolio"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <Portfolio/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="posts"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <Blog/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="posts/add"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <AddPost/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="posts/:slug"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <Post/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="posts/:slug/edit"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <EditPost/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="login"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <Login action=auth_context.login/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="logout"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <Logout action=auth_context.logout/>
                            </Default>
                        }
                    }
                />
                <Route
                    path="/rss.xml"
                    view=move |cx| {
                        view! { cx, <Rss/> }
                    }
                    ssr=SsrMode::Async
                />
                <Route
                    path="nedry"
                    view=move |cx| {
                        view! { cx,
                            <Default>
                                <Nedry/>
                            </Default>
                        }
                    }
                />
            </Routes>
        </Router>
    }
}
