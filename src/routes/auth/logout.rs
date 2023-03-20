use leptos::*;
use leptos_router::*;
use crate::functions;

#[component]
pub fn Logout(cx: Scope, action: Action<functions::auth::Logout, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        cx,
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">"Log Out"</button>
            </ActionForm>
        </div>
    }
}