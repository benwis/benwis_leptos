use cfg_if::cfg_if;
use leptos::*;

// boilerplate to run in different modes
cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        response::{Response, IntoResponse},
        routing::{post, get},
        extract::{Path, Extension},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };
    use crate::todo::*;
    use crate::auth::*;
    use todo_app_sqlite_axum::*;
    use crate::fallback::file_and_error_handler;
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use std::sync::Arc;
    use rand::Rng;
    use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
    use axum_database_sessions::{SessionConfig, SessionLayer, SessionStore};
    use axum_sessions_auth::{AuthSessionLayer, Authentication, AuthConfig, SessionSqlitePool};

    async fn server_fn_handler(auth_session: AuthSession, path: Path<String>, headers: HeaderMap, request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);

        handle_server_fns_with_context(path, headers, move |cx| {
            provide_context(cx, auth_session.clone());
        }, request).await
    }

    async fn leptos_routes_handler(auth_session: AuthSession, Extension(options): Extension<Arc<LeptosOptions>>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_app_to_stream_with_context((*options).clone(),
            move |cx| {
                // provide_context(cx, id.clone());
                provide_context(cx, auth_session.clone())
            },
            |cx| view! { cx, <TodoApp/> }
        );
        handler(req).await.into_response()
    }

    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

        // let secret = rand::thread_rng().gen::<[u8; 64]>();

        let mut conn = db().await.expect("couldn't connect to DB");

        let pool = SqlitePoolOptions::new()
            .connect("sqlite:Todos.db")
            .await
            .unwrap();

        // Auth section
        let session_config = SessionConfig::default().with_table_name("axum_sessions_list");
        let auth_config = AuthConfig::<i64>::default();
        let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config);

        session_store.initiate().await.unwrap();

        sqlx::migrate!()
            .run(&mut conn)
            .await
            .expect("could not run SQLx migrations");

        // let session_store = SqliteSessionStore::new("sqlite:Todos.db")
        //     .await
        //     .expect("Could not make Sqlite session store");
        // session_store.migrate().await.expect("Migration failed.");
        // let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);


        // let user_store = SqliteStore::<User>::new(pool);
        // let auth_layer = AuthLayer::new(user_store, &secret);

        crate::todo::register_server_functions();

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <TodoApp/> }).await;

        // build our application with a route
        let app = Router::new()
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .fallback(file_and_error_handler)
        .layer(
                AuthSessionLayer
                ::<User, i64, SessionSqlitePool, SqlitePool>
                ::new(Some(pool))
                    .with_config(auth_config),
            )
        .layer(SessionLayer::new(session_store))
        // .layer(auth_layer)
        // .layer(session_layer)
        .layer(Extension(Arc::new(leptos_options)));

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

    // client-only stuff for Trunk
    else {
        pub fn main() {
            // This example cannot be built as a trunk standalone CSR-only app.
            // Only the server may directly connect to the database.
        }
    }
}
