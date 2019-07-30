use serde_derive::{
  Deserialize,
  Serialize,
};
use smithy::types::UnwrappedPromise;

pub type TrackId = usize;
pub type UnwrappedApiCall = UnwrappedPromise<SearchResults, ()>;

pub enum Page {
  Search,
  Detail(TrackId),
}

impl Page {
  pub fn go_to_detail_view(&mut self, track_id: TrackId) {
    *self = Page::Detail(track_id);
    let _ = crate::util::get_location().set_hash(&track_id.to_string());
  }

  pub fn go_to_search_view(&mut self) {
    *self = Page::Search;
    let _ = crate::util::get_location().set_hash("");
  }

  pub fn handle_hash_change(&mut self) {
    let id_opt = crate::util::get_current_hash().and_then(|hash| hash.parse::<TrackId>().ok());
    match id_opt {
      Some(id) => self.go_to_detail_view(id),
      None => self.go_to_search_view(),
    };
  }

  pub fn get_detail_api_call(&self) -> Option<UnwrappedApiCall> {
    match self {
      Page::Search => None,
      Page::Detail(track_id) => Some(smithy::unwrapped_promise_from_future(
        crate::api::fetch_details(track_id),
      )),
    }
  }
}

impl Default for Page {
  fn default() -> Page {
    let mut page = Page::Search;
    page.handle_hash_change();
    page
  }
}

// ---------------------------

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResults {
  pub result_count: usize,
  pub results: Vec<SearchItem>,
}

impl SearchResults {
  pub fn new() -> SearchResults {
    SearchResults {
      result_count: 0,
      results: vec![],
    }
  }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchItem {
  pub wrapper_type: String,
  pub kind: Option<String>,
  pub artist_id: Option<usize>,
  pub collection_id: Option<usize>,
  pub track_id: Option<usize>,
  pub artist_name: String,
  pub collection_name: Option<String>,
  pub track_name: Option<String>,
  pub collection_censored_name: Option<String>,
  pub track_censored_name: Option<String>,
  pub artist_view_url: Option<String>,
  pub collection_view_url: Option<String>,
  pub track_view_url: Option<String>,
  pub preview_url: Option<String>,
  pub artwork_url_30: Option<String>,
  pub artwork_url_60: Option<String>,
  pub artwork_url_100: Option<String>,
  pub collection_price: Option<f32>,
  pub track_price: Option<f32>,
  pub release_date: Option<String>,
  pub collection_explicitness: Option<String>,
  pub track_explicitness: Option<String>,
  pub disc_count: Option<usize>,
  pub disc_number: Option<usize>,
  pub track_count: Option<usize>,
  pub track_number: Option<usize>,
  pub track_time_millis: Option<usize>,
  pub country: Option<String>,
  pub currency: Option<String>,
  pub primary_genre_name: Option<String>,
  pub is_streamable: Option<bool>,
}
