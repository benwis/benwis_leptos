use benwis_leptos::telemetry::TracingSettings;
use cfg_if::cfg_if;

// boilerplate to run in different modes
cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        response::{Response, IntoResponse},
        routing::{post, get},
        extract::{Path, Extension, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };
    use tower_http::trace::TraceLayer;
    use benwis_leptos::*;
    use benwis_leptos::fallback::file_and_error_handler;
    use benwis_leptos::telemetry::{get_subscriber, get_subscriber_with_tracing, init_subscriber};

    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use leptos::{log, view, provide_context, LeptosOptions, get_configuration};
    use std::{sync::Arc, env};
    use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
    use axum_database_sessions::{SessionConfig, SessionLayer, SessionStore};
    use axum_sessions_auth::{AuthSessionLayer, AuthConfig, SessionSqlitePool};
    use crate::functions::auth::{AuthSession};
    use crate::app::*;
    use crate::models::User;
    use tower_http::{compression::CompressionLayer};

    #[tracing::instrument(level = "info", fields(error))]
    async fn server_fn_handler(Extension(pool): Extension<SqlitePool>, query: RawQuery, auth_session: AuthSession, path: Path<String>, headers: HeaderMap, request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);

        handle_server_fns_with_context(path, headers, query, move |cx| {
            provide_context(cx, auth_session.clone());
            provide_context(cx, pool.clone());
        }, request).await
    }

    #[tracing::instrument(level = "info", fields(error))]
    async fn leptos_routes_handler(Extension(pool): Extension<SqlitePool>, auth_session: AuthSession, Extension(options): Extension<Arc<LeptosOptions>>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_app_to_stream_with_context((*options).clone(),
            move |cx| {
                provide_context(cx, auth_session.clone());
                provide_context(cx, pool.clone());
            },
            |cx| view! { cx, <BenwisApp/> }
        );
        handler(req).await.into_response()
    }

    #[tokio::main]
    async fn main() {
        // Load .env file if one is present(should only happen in local dev)
        dotenvy::dotenv().ok();

        // simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

        let pool = SqlitePoolOptions::new()
            .connect("sqlite:db/App.db")
            .await
            .expect("Could not make pool.");

        let parallelism = std::thread::available_parallelism().unwrap().get();
        log!("PARALLELISM: {parallelism}");

        let tracing_conf = TracingSettings{ 
            honeycomb_team: Some("6yem4uKpKZQBMObm755EdA".to_string()), 
            honeycomb_dataset: Some("benwis_leptos".to_string()), 
            honeycomb_service_name: Some("benwis_leptos".to_string()) 
        };

        // Get telemetry layer
        if env::var("LEPTOS_ENVIRONMENT").expect("Failed to find LEPTOS_ENVIRONMENT Env Var") == "local" {
            println!("LOCAL ENVIRONMENT");
            init_subscriber(get_subscriber(
                "benws_leptos".into(),
                "INFO".into(),
                std::io::stdout,
            ));
        } else {
            init_subscriber(
                get_subscriber_with_tracing(
                    "benwis_leptos".into(),
                    &tracing_conf,
                    "INFO".into(),
                    std::io::stdout,
                )
                .await,
            );
        }

        // Auth section
        let session_config = SessionConfig::default().with_table_name("axum_sessions");
        let auth_config = AuthConfig::<i64>::default();
        let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config);
        session_store.initiate().await.unwrap();

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("could not run SQLx migrations");

        crate::functions::register_server_functions();

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <BenwisApp/> }).await;

        // build our application with a route
        let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .fallback(file_and_error_handler)
        .layer(TraceLayer::new_for_http())
        .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
            .with_config(auth_config))
        .layer(SessionLayer::new(session_store))
        .layer(Extension(Arc::new(leptos_options)))
        .layer(Extension(pool))
        .layer(CompressionLayer::new());

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
