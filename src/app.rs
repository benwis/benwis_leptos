use crate::components::{ColorScheme, DarkModeToggle, DarkModeToggleProps};
use crate::error_template::*;
use crate::functions;
use crate::functions::user::get_user;
use crate::routes::auth::{Join, JoinProps, Login, LoginProps, Logout, LogoutProps};
use crate::routes::blog::*;
use crate::routes::todos::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn BenwisApp(cx: Scope) -> impl IntoView {
    let login = create_server_action::<functions::auth::Login>(cx);
    let logout = create_server_action::<functions::auth::Logout>(cx);
    let signup = create_server_action::<functions::auth::Signup>(cx);

    let color_scheme_signal = create_rw_signal(cx, false);
    provide_context(cx, ColorScheme(color_scheme_signal));
    let t = move || {
        println!("Current Color Scheme is: {:#?}", color_scheme_signal());
    };

    let user = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(cx),
    );
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/benwis_leptos.css"/>
        <Router>
        <div class:dark=move || color_scheme_signal()>
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
                        <Join action=signup/>
                    }/>

                    <Route path="posts" view=move |cx| view! {
                        cx,
                        <Blog/>
                    }/>
                    <Route
                        path="posts/:slug"
                        view=move |cx| view! { cx,  <Post/> }
                        />
                    <Route path="login" view=move |cx| view! {
                        cx,
                        <Login action=login />
                    }/>
                    <Route path="settings" view=move |cx| view! {
                        cx,
                        <h1>"Settings"</h1>
                        <Logout action=logout />
                    }/>
                </Routes>
            </main>
            </div>
        </Router>
    }
}
