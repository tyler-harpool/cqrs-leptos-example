mod pages;
mod routes;
mod application;
mod repositories;
#[cfg(feature = "ssr")]
mod server;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use routes::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(AppRouter);
    }
}
}
