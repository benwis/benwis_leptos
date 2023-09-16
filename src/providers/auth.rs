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
pub fn provide_auth() {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();

    let user = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_safe_user(),
    );

    provide_context(AuthContext {
        user,
        login,
        logout,
        signup,
    })
}
