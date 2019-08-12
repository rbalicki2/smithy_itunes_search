#![feature(proc_macro_hygiene, slice_patterns)]

// mod api;
// mod types;
mod util;
// mod app;

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

  let css = smithy_css::css!(
    [123]
  );
  web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("css macro {:?}", css)));

  let node = util::document().query_selector("#app").unwrap().unwrap();
  // let app = app::render();
  let app = smithy::smd!(
    // <style>{ css.to_string() }</style>
    <div>SOME TEXT</div>
    // <div class={&css.classes.my_class}>harharhar</div>
  );
  smithy::mount(Box::new(app), node);
}
