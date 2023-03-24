use crate::components::{DarkModeToggle, DarkModeToggleProps};
use crate::error_template::*;
use crate::functions::user::get_user;
use crate::providers::{provide_auth, provide_color_scheme, AuthContext};
use crate::routes::auth::{Join, JoinProps, Login, LoginProps, Logout, LogoutProps};
use crate::routes::blog::*;
use crate::routes::todos::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn BenwisApp(cx: Scope) -> impl IntoView {
    // let login = create_server_action::<functions::auth::Login>(cx);
    // let logout = create_server_action::<functions::auth::Logout>(cx);
    // let signup = create_server_action::<functions::auth::Signup>(cx);

    // Create Actions for the Auth methods and provide the current user
    provide_auth(cx);
    let auth_context = use_context::<AuthContext>(cx).expect("Failed to get AuthContezt");

    let user = create_resource(
        cx,
        move || {
            (
                auth_context.login.version().get(),
                auth_context.signup.version().get(),
                auth_context.logout.version().get(),
            )
        },
        move |_| get_user(cx),
    );
    provide_meta_context(cx);
    let color_scheme_signal = provide_color_scheme(cx);

    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/benwis_leptos.css"/>
        <Router>
        <div  class:dark=color_scheme_signal class="dummy">
        <div class="dark:bg-gray-800">
            <header>
                <A href="/"><h1 class="text-sky-400">"My Blog"</h1></A>
                <DarkModeToggle/>
                <Transition
                    fallback=move || view! {cx, <span>"Loading..."</span>}
                >
                {move || {
                    user.read(cx).map(|user| match user {
                        Err(e) => view! {cx,
                            <A href="/signup">"Signup"</A>", "
                            <A href="/login">"Login"</A>", "
                            <span>{format!("Login error: {}", e)}</span>
                        }.into_view(cx),
                        Ok(None) => view! {cx,
                            <A href="/signup">"Signup"</A>", "
                            <A href="/login">"Login"</A>", "
                            <span>"Logged out."</span>
                        }.into_view(cx),
                        Ok(Some(user)) => view! {cx,
                            <A href="/settings">"Settings"</A>", "
                            <span>{format!("Logged in as: {} ({})", user.username, user.id)}</span>
                        }.into_view(cx)
                    })
                }}
                </Transition>
            </header>
            <hr/>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! {
                        cx,
                        <ErrorBoundary fallback=|cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
                            <Todos/>
                        </ErrorBoundary>
                    }/>
                    <Route path="signup" view=move |cx| view! {
                        cx,
                        <Join action=auth_context.signup/>
                    }/>
                    <Route path="test" view=move |cx| view!{cx, <h1 class="dark:text-red-500">"POTATO"</h1>}/>

                    <Route path="posts" view=move |cx| view! {
                        cx,
                        <Blog/>
                    }/>
                    <Route
                        path="posts/:slug"
                        view=move |cx| view! { cx,  <Post/> }
                        />
                        <Route
                        path="posts/:slug/edit"
                        view=move |cx| view! { cx,  <PostEdit/> }
                        />
                    <Route path="login" view=move |cx| view! {
                        cx,
                        <Login action=auth_context.login />
                    }/>
                    <Route path="settings" view=move |cx| view! {
                        cx,
                        <h1>"Settings"</h1>
                        <Logout action=auth_context.logout />
                    }/>
                </Routes>
            </main>
            </div>
            </div>
        </Router>
    }
}
