use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::LeptosOptions;
        use leptos_router::RouteListing;
        use sqlx::SqlitePool;
        use axum::extract::FromRef;
        use crate::models::PostsContainer;
       use crate::errors::BenwisAppError;
        /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
        /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
        #[derive(FromRef, Debug, Clone)]
        pub struct AppState{
            pub leptos_options: LeptosOptions,
            pub pool: SqlitePool,
            pub posts: PostsContainer,
            pub routes: Vec<RouteListing>,
        }
        impl AppState{
            pub async fn new_with_posts(leptos_options: LeptosOptions, pool: SqlitePool, routes: Vec<RouteListing>)-> Result<Self, BenwisAppError>{
                Ok(Self{
                    leptos_options,
                    pool,
                    posts: PostsContainer::new_with_posts().await?,
                    routes
                })
            }
        }
}
    }
