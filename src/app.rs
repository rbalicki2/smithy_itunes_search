use wasm_bindgen::{
  JsCast,
};
use smithy::{
  smd,
  types::{
    PromiseState,
    SmithyComponent,
    UnwrappedPromise,
  },
};
use std::{
  cell::RefCell,
  rc::Rc,
};
use crate::types::{
  Page,
  SearchResults,
  SearchItem,
};

pub fn render<'a>() -> SmithyComponent<'a> {
  let mut page = Page::default();
  let mut search_api_call_opt: Option<UnwrappedPromise<SearchResults, ()>> = None;
  let mut detail_api_call_opt = page.get_detail_api_call();

  smd!(
    on_hash_change={|_| {
      page.handle_hash_change();
      detail_api_call_opt = page.get_detail_api_call();
    }};
    { render_head() }
    { render_styles() }
    <div class="container">
      {
        match page {
          Page::Search => render_search(&mut page, &mut search_api_call_opt),
          Page::Detail(_) => render_detail_view(&mut page, &mut search_api_call_opt, &mut detail_api_call_opt),
        }
      }
    </div>
  )
}

fn render_search<'a>(
  page: &'a mut Page,
  search_api_call_opt: &'a mut Option<UnwrappedPromise<SearchResults, ()>>,
) -> SmithyComponent<'a> {
  smd!(
    <h1>
      Smithy iTunes music search
      <div><small class="text-muted">Search for your favorite artists and songs</small></div>
    </h1>
    <p>
      <input
        class="form-control"
        autofocus
        type="text"
        on_input={|e: &web_sys::InputEvent| {
          if let Some(target) = e.target() {
            let target: web_sys::HtmlInputElement = web_sys::HtmlInputElement::unchecked_from_js(target.into());
            let value = target.value();
            if value.len() > 0 {
              *search_api_call_opt = Some(smithy::unwrapped_promise_from_future(crate::api::search(&value)));
            } else {
              *search_api_call_opt = None;
            }
          }
        }}
      />
    </p>
    {
      search_api_call_opt.as_ref().map(|ref mut search_api_call| {
        match &mut *search_api_call.borrow_mut() {
          PromiseState::Success(search_results) => {
            // TODO why do we have to clone here?
            let search_results = search_results.clone();
            render_search_results(search_results, |track_id| {
              page.go_to_detail_view(track_id);
            })
          },
          PromiseState::Pending => smd!(<p>Loading</p>),
          PromiseState::Error(_) => smd!(<p>Something went wrong</p>),
        }
      })
    }
  )
}

fn render_search_results<'a>(
  search_results: SearchResults,
  go_to_detail_view: impl FnMut(usize) + 'a,
) -> SmithyComponent<'a> {
  let SearchResults {
    result_count,
    mut results,
  } = search_results;
  let mut go_to_detail_view = Rc::new(RefCell::new(go_to_detail_view));

  let mut inner = smd!({
    let results = crate::util::clone_and_zip(results.iter_mut(), &mut go_to_detail_view);
    &mut results
      .map(|(result, mut go_to_detail_view)| {
        smd!(
          <a
            href
            on_click={|e: &web_sys::MouseEvent| {
              (&mut *go_to_detail_view.borrow_mut())(result.track_id.unwrap_or(0));
              e.prevent_default();
            }}
          >
            <li>
              { &mut result.track_name.as_ref().unwrap_or(&"(No track name)".to_string()) }
              {' '}by{' '}
              { &mut result.artist_name }
            </li>
          </a>
        )
      })
      .collect::<Vec<SmithyComponent>>()
  });

  smd!(
    <p class="lead">Found { result_count }{' '}results</p>
    <ul>
      { &mut inner }
    </ul>
  )
}

fn render_detail_view<'a>(
  page: &'a mut Page,
  search_api_call_opt: &'a mut Option<UnwrappedPromise<SearchResults, ()>>,
  detail_api_call_opt: &'a mut Option<UnwrappedPromise<SearchResults, ()>>,
) -> SmithyComponent<'a> {
  smd!(
    {
      match detail_api_call_opt {
        Some(detail_api_call) =>
          smd!(
            {
              match &mut *detail_api_call.borrow_mut() {
                PromiseState::Success(search_results) => {
                  if let Some(item) = search_results.results.get(0) {
                    let mut item = item.clone();
                    render_detail_item(item)
                  } else {
                    smd!(<h1>The search returned no results</h1>)
                  }
                },
                PromiseState::Pending => smd!(<p>Loading</p>),
                PromiseState::Error(_) => smd!(<p>Something went wrong</p>),
              }
            }
          ),
          None => smd!(We have no api call. This should not happen),
      }
    }
    <a href on_click={|e: &web_sys::MouseEvent| {
      e.prevent_default();
      page.go_to_search_view();
      *search_api_call_opt = None;
    }}>Back to search</a>
  )
}

fn render_detail_item<'a>(mut item: SearchItem) -> SmithyComponent<'a> {
  smd!(
    <h1>{
      &mut item.track_name.as_ref().unwrap_or(&"Unknown track".to_string())
    }{' '}by { &mut item.artist_name }</h1>
    {
      item.artwork_url_100.as_ref().map(|s| smd!(
        <img src={s} />
      ))
    }
    {
      item.collection_name.as_ref().map(|s| smd!(
        <div>
          <b>Album:</b>{' '}{ s }
        </div>
      ))
    }
  )
}

fn render_head<'a>() -> SmithyComponent<'a> {
  smd!(
    <title>Smithy iTunes API Search</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
  )
}

fn render_styles<'a>() -> SmithyComponent<'a> {
  smd!(
    <link
      rel="stylesheet"
      href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css"
      integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T"
      crossorigin="anonymous"
    />
    <style type="text/css">{"
      body { padding-top: 40px; }
      .form-control { margin: 20px 0; }
    "}</style>
  )
}