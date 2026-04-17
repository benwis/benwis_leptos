use cfg_if::cfg_if;

// boilerplate to run in different modes
cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        response::{Response, IntoResponse},
        routing::get,
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };
    use tokio::net::TcpListener;
    use tower_http::trace::TraceLayer;
    use benwis_leptos::*;
    use benwis_leptos::fallback::file_and_error_handler;

    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use leptos::prelude::*;
    use leptos::logging::log;
    use leptos::config::{get_configuration, LeptosOptions};
    use leptos_meta::MetaTags;
    use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthSessionLayer, AuthConfig};
    use axum_session_sqlx::SessionSqlitePool;
    use crate::functions::auth::AuthSession;
    use crate::app::*;
    use crate::models::User;
    use tower_http::compression::CompressionLayer;
    use benwis_leptos::state::AppState;

    #[cfg(not(target_env = "msvc"))]
    use jemallocator::Jemalloc;

    fn shell(options: LeptosOptions) -> impl IntoView {
        view! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <AutoReload options=options.clone() />
                    <HydrationScripts options=options.clone() islands=true/>
                    <link rel="stylesheet" id="leptos" href="/pkg/benwis_leptos.css"/>
                    <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                    <MetaTags/>
                </head>
                <body>
                    <BenwisApp/>
                </body>
            </html>
        }
    }

    #[tracing::instrument(level = "info", fields(error))]
    async fn server_fn_handler(State(app_state): State<AppState>, auth_session: AuthSession, path: Path<String>, _headers: HeaderMap, _raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);

        handle_server_fns_with_context(move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        }, request).await
    }
    #[tracing::instrument(level = "info", fields(error))]
    async fn leptos_routes_handler(auth_session: AuthSession, State(app_state): State<AppState>, req: Request<AxumBody>) -> Response {
        let options = app_state.leptos_options.clone();
        let handler = leptos_axum::render_app_to_stream_with_context(
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.pool.clone());
            },
            move || shell(options.clone()),
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

        // Auth section
        let session_config =
            SessionConfig::default().with_table_name("axum_sessions");
        let auth_config = AuthConfig::<i64>::default();
        let session_store = SessionStore::<SessionSqlitePool>::new(
            Some(SessionSqlitePool::from(pool.clone())),
            session_config,
        )
        .await
        .expect("Failed to get Session!");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("could not run SQLx migrations");

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).expect("Failed to get config");
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(BenwisApp);
        println!("routes = {routes:#?}");

        let app_state = AppState{
            leptos_options,
            pool: pool.clone(),
        };
        // build our application with a route
        let app = Router::new()
        .route("/api/{*fn_name}", get(server_fn_handler).post(server_fn_handler))
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
        let listener = TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app)
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
