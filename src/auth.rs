use crate::error_template::{ErrorTemplate, ErrorTemplateProps};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection};
        use bcrypt::{hash, verify, DEFAULT_COST};
        use axum_login::{
            axum_sessions::{async_session::MemoryStore, SessionLayer},
            secrecy::SecretVec,
            AuthLayer, AuthUser, RequireAuthorizationLayer, SqliteStore,
        };
        use crate::todo::db;
        // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct User {
            pub id: u16,
            pub username: String,
            pub password: String,
        }

        pub type AuthContext = axum_login::extractors::AuthContext<User, SqliteStore<User>>;

        impl AuthUser for User {
            fn get_id(&self) -> String {
                format!("{}", self.id)
            }

            fn get_password_hash(&self) -> SecretVec<u8> {
                SecretVec::new(self.password.clone().into())
            }
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct User {
            pub id: u16,
            pub username: String,
            pub password: String,
        }
    }
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let mut auth = use_context::<AuthContext>(cx)
        .ok_or("Auth context missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    let user: Option<User> = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let mut auth = use_context::<AuthContext>(cx)
        .ok_or("Auth context missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let user = user
        .ok_or("User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match verify(password, &user.password).map_err(|e| ServerFnError::ServerError(e.to_string()))? {
        true => auth
            .login(&user)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string())),
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String,
) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    match sqlx::query("INSERT INTO users (username, password) VALUES (?,?)")
        .bind(username)
        .bind(password_hashed)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let signup = create_server_action::<Signup>(cx);

    view! {
        cx,
        <ActionForm action=signup>
            <h1>"Sign Up"</h1>
            <label>
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input type="password" placeholder="Password again" name="password-confirmation" class="auth-input" />
            </label>
            <br/>
            <button type="submit" class="button">"Sign Up"</button>
        </ActionForm>
    }
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);

    view! {
        cx,
        <ActionForm action=login>
            <h1>"Log In"</h1>
            <label>
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <button type="submit" class="button">"Log In"</button>
        </ActionForm>
    }
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let mut auth = use_context::<AuthContext>(cx)
        .ok_or("Auth context missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    auth.logout().await;

    Ok(())
}

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let logout = create_server_action::<Logout>(cx);

    view! {
        cx,
        <div id="loginbox">
            <ActionForm action=logout>
                <button type="submit" class="button">"Log Out"</button>
            </ActionForm>
        </div>
    }
}
