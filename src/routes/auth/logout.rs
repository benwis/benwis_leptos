use leptos::*;
use leptos_router::*;
use crate::functions;

#[component]
pub fn Logout(cx: Scope, action: Action<functions::auth::Logout, Result<(), ServerFnError>>) -> impl IntoView {
    _ = &action.dispatch(functions::auth::Logout{});

    view! { cx, <Redirect path="/"/> }
}