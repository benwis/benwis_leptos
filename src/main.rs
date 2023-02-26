use cfg_if::cfg_if;
use leptos::*;

// boilerplate to run in different modes
cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        routing::post,
        extract::{Path, Extension},
        http::{Request, header::HeaderMap},
        body::Body,
        Router,
    };
    use crate::todo::*;
    use crate::auth::*;
    use todo_app_sqlite_axum::*;
    use crate::fallback::file_and_error_handler;
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use std::sync::Arc;
    use rand::Rng;
    use axum_login::{
        axum_sessions::SessionLayer,
        secrecy::SecretVec,
        AuthLayer, SqliteStore,
    };
    use async_sqlx_session::SqliteSessionStore;
    use sqlx::sqlite::SqlitePoolOptions;


    pub type AuthContext = axum_login::extractors::AuthContext<User, SqliteStore<User>>;

    async fn server_fn_handler(auth_context: AuthContext, path: Path<String>, headers: HeaderMap, request: Request<Body>){
        handle_server_fns_with_context(path, headers, move |cx| {
            provide_context(cx, auth_context.clone());
        }, request).await;
    }

    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

        let secret = rand::thread_rng().gen::<[u8; 64]>();

        let mut conn = db().await.expect("couldn't connect to DB");

        sqlx::migrate!()
            .run(&mut conn)
            .await
            .expect("could not run SQLx migrations");

        let session_store = SqliteSessionStore::new("sqlite:Todos.db")
            .await
            .expect("Could not make Sqlite session store");
        session_store.migrate().await.expect("Migration failed.");
        let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

        let pool = SqlitePoolOptions::new()
            .connect("sqlite:Todos.db")
            .await
            .unwrap();

        let user_store = SqliteStore::<User>::new(pool);
        let auth_layer = AuthLayer::new(user_store, &secret);

        crate::todo::register_server_functions();

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <TodoApp/> }).await;

        // build our application with a route
        let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <TodoApp/> } )
        .fallback(file_and_error_handler)
        .layer(auth_layer)
        .layer(session_layer)
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
