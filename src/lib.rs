#![feature(proc_macro_hygiene, slice_patterns)]

mod api;
mod types;
mod util;
mod app;

use wasm_bindgen::{
  prelude::*,
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  let node = util::document().query_selector("#app").unwrap().unwrap();
  let app = app::render();
  smithy::mount(Box::new(app), node);
}
