use crate::functions::auth::{Login, Logout, Signup};
use crate::functions::user::get_safe_user;
use crate::models::SafeUser;
use leptos::context::provide_context;
use leptos::prelude::*;
use leptos::server::serializers::SerdeJson;
use leptos::server::{Resource, ServerAction};
use leptos::server_fn::ServerFnError;

// TODO: actions and resources!

#[derive(Clone)]
pub struct AuthContext {
    pub login: ServerAction<Login>,
    pub logout: ServerAction<Logout>,
    pub signup: ServerAction<Signup>,
    pub user: Resource<Result<Option<SafeUser>, ServerFnError>, SerdeJson>,
}
/// Get the current user and place it in Context
pub fn provide_auth() {
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let signup = ServerAction::<Signup>::new();

    let user = Resource::new_serde(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        |_| get_safe_user(),
    );

    provide_context(AuthContext {
        user,
        login,
        logout,
        signup,
    })
}
