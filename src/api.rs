use crate::types::SearchResults;
use futures::Future;
use js_sys::Promise;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
  Request,
  RequestInit,
  RequestMode,
  Response,
};

// const SEARCH_BASE_URL: &'static str = "https://itunes.apple.com/search?term=";
const SEARCH_BASE_URL: &'static str =
  "https://uzwiof4m5m.execute-api.us-east-1.amazonaws.com/prod/search?term=";
// const DETAIL_BASE_URL: &'static str = "https://itunes.apple.com/lookup?id=";
const DETAIL_BASE_URL: &'static str =
  "https://uzwiof4m5m.execute-api.us-east-1.amazonaws.com/prod/lookup?id=";

pub fn search(term: &str) -> impl Future<Item = SearchResults, Error = ()> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let url = format!("{}{}", SEARCH_BASE_URL, term);
  let request = Request::new_with_str_and_init(&url, &opts).unwrap();

  let window = web_sys::window().unwrap();
  let request_promise = window.fetch_with_request(&request);

  let future = JsFuture::from(request_promise)
    .and_then(|resp_value| {
      let resp: Response = resp_value.unchecked_into();
      resp.json()
    })
    .and_then(|json_value: Promise| {
      // Convert this other `Promise` into a rust `Future`.
      JsFuture::from(json_value)
    })
    .map(|json| {
      // Use serde to parse the JSON into a struct.
      let todo_lists_result = json.into_serde().map_err(|e| {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("{:?}", e)));
        e
      });
      todo_lists_result.unwrap_or_else(|_| SearchResults::new())
    })
    .map_err(|_| ());
  future
}

// This is largely repeated from the previous function... -__-
// TODO DRY it up
pub fn fetch_details(
  track_id: &crate::types::TrackId,
) -> impl Future<Item = SearchResults, Error = ()> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let url = format!("{}{}", DETAIL_BASE_URL, track_id);
  let request = Request::new_with_str_and_init(&url, &opts).unwrap();

  let window = web_sys::window().unwrap();
  let request_promise = window.fetch_with_request(&request);

  let future = JsFuture::from(request_promise)
    .and_then(|resp_value| {
      let resp: Response = resp_value.unchecked_into();
      resp.json()
    })
    .and_then(|json_value: Promise| {
      // Convert this other `Promise` into a rust `Future`.
      JsFuture::from(json_value)
    })
    .map(|json| {
      // Use serde to parse the JSON into a struct.
      let todo_lists_result = json.into_serde();
      todo_lists_result.unwrap_or_else(|_| SearchResults::new())
    })
    .map_err(|_| ());
  future
}
