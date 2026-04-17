// Ensure the `app` crate is linked so that #[island] components are registered.
extern crate app;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_islands();
}
