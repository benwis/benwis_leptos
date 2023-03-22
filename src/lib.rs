use cfg_if::cfg_if;

pub mod app;
pub mod routes;
pub use routes::*;
pub mod components;
pub mod error_template;
pub mod errors;
pub mod fallback;
pub mod functions;
pub mod models;
// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::app::*;
        use leptos::view;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Info);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(|cx| {
                view! { cx,  <BenwisApp/> }
            });
        }
    }
}
