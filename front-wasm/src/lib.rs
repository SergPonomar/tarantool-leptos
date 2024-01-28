use front_app::*;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

/// Running 'Hydration' process in browser
#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    log!("hydrate mode - hydrating");

    leptos::mount_to_body(|| {
        view! { <App/> }
    });
}
