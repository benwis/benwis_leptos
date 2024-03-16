pub mod app;
pub mod routes;
pub use routes::*;
pub mod components;
pub mod error_template;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod fallback;
#[allow(clippy::too_many_arguments)]
pub mod functions;
pub mod layouts;
pub mod models;
pub mod providers;
#[cfg(feature = "ssr")]
pub mod state;
pub mod telemetry;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::BenwisApp;

    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    leptos::hydrate_body(BenwisApp);
}
