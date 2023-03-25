use crate::functions::auth::{Login, Logout, Signup};
use crate::functions::user::get_user;
use crate::models::user::User;
use leptos::*;

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<Login, Result<(), ServerFnError>>,
    pub logout: Action<Logout, Result<(), ServerFnError>>,
    pub signup: Action<Signup, Result<(), ServerFnError>>,
    pub user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>,
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
        move |_| get_user(cx),
    );

    // let user = Signal::derive(cx, move || {
    //     {
    //         {
    //             user_resource.read(cx).map(|user| match user {
    //                 Err(_) => None,
    //                 Ok(None) => None,
    //                 Ok(Some(user)) => Some(user),
    //             })
    //         }
    //     }
    //     .unwrap_or(None)
    // });
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
