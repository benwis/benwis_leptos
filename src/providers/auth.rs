use crate::functions::auth::{Login, Logout, Signup};
use crate::functions::user::get_safe_user;
use crate::models::user::SafeUser;
use leptos::*;

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<Login, Result<(), ServerFnError>>,
    pub logout: Action<Logout, Result<(), ServerFnError>>,
    pub signup: Action<Signup, Result<(), ServerFnError>>,
    pub user: Resource<(usize, usize, usize), Result<Option<SafeUser>, ServerFnError>>,
}
/// Get the current user and place it in Context
pub fn provide_auth(cx: Scope) {
    let login = create_server_action::<Login>(cx);
    let logout = create_server_action::<Logout>(cx);
    let signup = create_server_action::<Signup>(cx);

    let user = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_safe_user(cx),
    );

    provide_context(
        cx,
        AuthContext {
            user,
            login,
            logout,
            signup,
        },
    )
}
