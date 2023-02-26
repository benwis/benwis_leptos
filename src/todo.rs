use crate::auth::*;
use crate::error_template::{ErrorTemplate, ErrorTemplateProps};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection};
        // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            SqliteConnection::connect("sqlite:Todos.db").await.map_err(|e| ServerFnError::ServerError(e.to_string()))
        }

        pub fn register_server_functions() {
            _ = GetTodos::register();
            _ = AddTodo::register();
            _ = DeleteTodo::register();
            _ = Login::register();
            _ = Signup::register();
            _ = GetUser::register();
            _ = Foo::register();
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct Todo {
            id: u16,
            title: String,
            created_at: String,
            completed: bool,
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Todo {
            id: u16,
            title: String,
            created_at: String,
            completed: bool,
        }
    }
}

#[server(GetTodos, "/api")]
pub async fn get_todos(cx: Scope) -> Result<Vec<Todo>, ServerFnError> {
    // this is just an example of how to access server context injected in the handlers
    // http::Request doesn't implement Clone, so more work will be needed to do use_context() on this
    let req_parts = use_context::<leptos_axum::RequestParts>(cx);

    if let Some(req_parts) = req_parts {
        println!("Uri = {:?}", req_parts.uri);
    }

    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut todos = Vec::new();
    let mut rows = sqlx::query_as::<_, Todo>("SELECT * FROM todos").fetch(&mut conn);
    while let Some(row) = rows
        .try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
    {
        todos.push(row);
    }

    // Add a random header(because why not)
    // let mut res_headers = HeaderMap::new();
    // res_headers.insert(SET_COOKIE, HeaderValue::from_str("fizz=buzz").unwrap());

    // let res_parts = leptos_axum::ResponseParts {
    //     headers: res_headers,
    //     status: Some(StatusCode::IM_A_TEAPOT),
    // };

    // let res_options_outer = use_context::<leptos_axum::ResponseOptions>(cx);
    // if let Some(res_options) = res_options_outer {
    //     res_options.overwrite(res_parts).await;
    // }

    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(cx: Scope, title: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    let user = get_user(cx).await?;

    let id = match user {
        Some(user) => user.id,
        None => User::default().id,
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query("INSERT INTO todos (title, user_id, completed) VALUES (?, ?, false)")
        .bind(title)
        .bind(id)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: u16) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn TodoApp(cx: Scope) -> impl IntoView {
    let user = create_resource(cx, || (), move |_| get_user(cx));
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/todo_app_sqlite_axum.css"/>
        <Router>
            <header>
                <a href="/"><h1>"My Tasks"</h1></a>
                <Suspense
                    fallback=move || view! {cx, <span>"Loading..."</span>}
                >
                {move || {
                    user.read(cx).map(|user| match user {
                        Err(e) => view! {cx,
                            <a href="/signup">"Signup"</a>", "
                            <a href="/login">"Login"</a>", "
                            <span>{format!("Login error: {}", e.to_string())}</span>
                        }.into_view(cx),
                        Ok(None) => view! {cx,
                            <a href="/signup">"Signup"</a>", "
                            <a href="/login">"Login"</a>", "
                            <span>"Logged out."</span>
                        }.into_view(cx),
                        Ok(Some(user)) if user.id == User::default().id => view! {cx,
                            <a href="/signup">"Signup"</a>", "
                            <a href="/login">"Login"</a>", "
                            <a href="/settings">"Settings"</a>", "
                            <span>{format!("Logged in as: {}", user.username)}</span>
                        }.into_view(cx),
                        Ok(Some(user)) => view! {cx,
                            <a href="/settings">"Settings"</a>", "
                            <span>{format!("Logged in as: {}", user.username)}</span>
                        }.into_view(cx)
                    })
                }}
                </Suspense>
            </header>
            <hr/>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! {
                        cx,
                        <ErrorBoundary fallback=|cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
                            <Todos/>
                        </ErrorBoundary>
                    }/> //Route
                    <Route path="signup" view=|cx| view! {
                        cx,
                        <Signup/>
                    }/>
                    <Route path="login" view=|cx| view! {
                        cx,
                        <Login/>
                    }/>
                    <Route path="settings" view=|cx| view! {
                        cx,
                        <h1>"Settings"</h1>
                        <Logout/>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Todos(cx: Scope) -> impl IntoView {
    let add_todo = create_server_multi_action::<AddTodo>(cx);
    let delete_todo = create_server_action::<DeleteTodo>(cx);
    let submissions = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos = create_resource(
        cx,
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(cx),
    );

    view! {
        cx,
        <div>
            <MultiActionForm action=add_todo>
                <label>
                    "Add a Todo"
                    <input type="text" name="title"/>
                </label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
            <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                {move || {
                    let existing_todos = {
                        move || {
                            todos.read(cx)
                                .map(move |todos| match todos {
                                    Err(e) => {
                                        vec![view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_any()]
                                    }
                                    Ok(todos) => {
                                        if todos.is_empty() {
                                            vec![view! { cx, <p>"No tasks were found."</p> }.into_any()]
                                        } else {
                                            todos
                                                .into_iter()
                                                .map(move |todo| {
                                                    view! {
                                                        cx,
                                                        <li>
                                                            {todo.title}
                                                            ": "
                                                            {todo.created_at}
                                                            <ActionForm action=delete_todo>
                                                                <input type="hidden" name="id" value={todo.id}/>
                                                                <input type="submit" value="X"/>
                                                            </ActionForm>
                                                        </li>
                                                    }
                                                    .into_any()
                                                })
                                                .collect::<Vec<_>>()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };

                    let pending_todos = move || {
                        submissions
                        .get()
                        .into_iter()
                        .filter(|submission| submission.pending().get())
                        .map(|submission| {
                            view! {
                                cx,
                                <li class="pending">{move || submission.input.get().map(|data| data.title) }</li>
                            }
                        })
                        .collect::<Vec<_>>()
                    };

                    view! {
                        cx,
                        <ul>
                            {existing_todos}
                            {pending_todos}
                        </ul>
                    }
                }
            }
            </Transition>
        </div>
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
                <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input" />
            </label>
            <br/>
            <button type="submit" class="button">"Sign Up"</button>
        </ActionForm>
    }
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
