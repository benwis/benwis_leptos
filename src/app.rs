use crate::components::{Nav, NavProps, Footer, FooterProps};
use crate::error_template::*;
use crate::providers::{provide_auth, provide_color_scheme, AuthContext};
use crate::routes::auth::{Join, JoinProps, Login, LoginProps, Logout, LogoutProps};
use crate::routes::blog::*;
use crate::routes::{Index, IndexProps, About, AboutProps, Portfolio, PortfolioProps};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn BenwisApp(cx: Scope) -> impl IntoView {
    // Create Actions for the Auth methods and provide the current user
    provide_auth(cx);
    let auth_context = use_context::<AuthContext>(cx).expect("Failed to get AuthContezt");

    provide_meta_context(cx);
    let color_scheme_signal = provide_color_scheme(cx);

    view! {
        cx,
        <Html class=move || {
            let classes = "h-full";
            let theme = match color_scheme_signal() {
            true => "dark",
            false => "",
            };
            format!("{} {}",classes,theme)}
            />
        <Body class="h-screen bg-white dark:bg-gray-900 max-w-5xl mx-auto flex flex-col"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/benwis_leptos.css"/>
        <Router>
            <Nav/>
            <main class="mx-auto flex w-full flex-col items-center justify-center border-gray-200 px-4 pb-16 md:pt-4 dark:border-gray-900 sm:px-8">
                <Routes>
                    <Route path="" view=|cx| view! {
                        cx,
                        <ErrorBoundary fallback=|cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
                            <Index/>
                        </ErrorBoundary>
                    }/>
                    <Route path="signup" view=move |cx| view! {
                        cx,
                        <Join action=auth_context.signup/>
                    }/>
                    <Route path="test" view=move |cx| view!{cx, <h1 class="dark:text-red-500">"POTATO"</h1>}/>
                    <Route path="about" view=move |cx| view! {
                        cx,
                        <About/>
                    }/>
                    <Route path="portfolio" view=move |cx| view! {
                        cx,
                        <Portfolio/>
                    }/>
                    <Route path="posts" view=move |cx| view! {
                        cx,
                        <Blog/>
                    }/>
                    <Route
                    path="posts/add"
                    view=move |cx| view! { cx,  <AddPost/> }
                    />
                    <Route
                        path="posts/:slug"
                        view=move |cx| view! { cx,  <Post/> }
                        />
                        <Route
                        path="posts/:slug/edit"
                        view=move |cx| view! { cx,  <EditPost/> }
                        />
                    <Route path="login" view=move |cx| view! {
                        cx,
                        <Login action=auth_context.login />
                    }/>
                    <Route path="logout" view=move |cx| view! {
                        cx,
                        <h1>"Settings"</h1>
                        <Logout action=auth_context.logout />
                    }/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
