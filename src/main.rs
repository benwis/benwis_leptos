use benwis_leptos::telemetry::TracingSettings;
use cfg_if::cfg_if;

// boilerplate to run in different modes
cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        response::{Response, IntoResponse},
        routing::{post, get},
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };
    use tower_http::trace::TraceLayer;
    use benwis_leptos::*;
    use benwis_leptos::fallback::file_and_error_handler;
    use benwis_leptos::telemetry::{get_subscriber,get_subscriber_with_tracing, init_subscriber};

    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use leptos::{log, view, provide_context, get_configuration};
    use std::env;
    use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
    use axum_session::{SessionConfig, SessionLayer, SessionMode, SessionStore};
    use axum_session_auth::{AuthSessionLayer, AuthConfig, SessionSqlitePool};
    use crate::functions::auth::{AuthSession};
    use crate::app::*;
    use crate::models::User;
    use tower_http::{compression::CompressionLayer};
    use benwis_leptos::state::AppState;

    #[cfg(not(target_env = "msvc"))]
    use jemallocator::Jemalloc;

    #[tracing::instrument(level = "info", fields(error))]
    #[axum::debug_handler]
    async fn server_fn_handler(State(app_state): State<AppState>, auth_session: AuthSession, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);

        handle_server_fns_with_context(path, headers, raw_query, move |cx| {
            provide_context(cx, auth_session.clone());
            provide_context(cx, app_state.pool.clone());
        }, request).await
    }
    #[tracing::instrument(level = "info", fields(error))]
    #[axum::debug_handler]
 async fn leptos_routes_handler(auth_session: AuthSession, State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
            move |cx| {
                provide_context(cx, auth_session.clone());
                provide_context(cx, app_state.pool.clone());
            },
            |cx| view! { cx, <BenwisApp/> }
        );
        handler(req).await.into_response()
    }
    #[tokio::main]
    async fn main() {
        log!("BENWIS LEPTOS APP STARTING!");

        #[cfg(not(target_env = "msvc"))]
        #[global_allocator]
        static GLOBAL: Jemalloc = Jemalloc;

        // Load .env file if one is present(should only happen in local dev)
        dotenvy::dotenv().ok();

        let pool = SqlitePoolOptions::new()
            .connect("sqlite:db/App.db")
            .await
            .expect("Could not make pool.");

        //let parallelism = std::thread::available_parallelism().unwrap().get();
        //log!("PARALLELISM: {parallelism}");

        let tracing_conf = TracingSettings{
            honeycomb_team: Some("G93VcSQ2fiX0QrSo65YCFA".to_string()),
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
        } else if env::var("LEPTOS_ENVIRONMENT").expect("Failed to find LEPTOS_ENVIRONMENT Env Var") == "prod_no_trace" {
            init_subscriber(get_subscriber(
                "benws_leptos".into(),
                "INFO".into(),
                std::io::stdout,
             ));
        } else{
            init_subscriber(
                get_subscriber_with_tracing(
                    "benwis_leptos".into(),
                    &tracing_conf,
                    "INFO".into(),
                    std::io::stdout,
                )
                .await);
            }

        // Auth section
        let session_config = SessionConfig::default().with_table_name("axum_sessions").with_mode(SessionMode::Storable);
        let auth_config = AuthConfig::<i64>::default();
        let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config).await.expect("Failed to get Session!");
        session_store.initiate().await.expect("Failed to create session store!");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("could not run SQLx migrations");

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).await.expect("Failed to get config");
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <BenwisApp/> }).await;

        let app_state = AppState{
            leptos_options,
            pool: pool.clone(),
        };
        // build our application with a route
        let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .fallback(file_and_error_handler)
        .layer(TraceLayer::new_for_http())
        .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
        .with_config(auth_config))
        .layer(SessionLayer::new(session_store))
        .layer(CompressionLayer::new())
        .with_state(app_state);

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to start hyper server");
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
