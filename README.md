# Smithy iTunes search

> A demonstration of using Smithy v0.0.6 to hit an API and render different pages.
>
> [See it live!](https://www.smithy.rs/examples/itunes/)

## Commands and the like

### Installing dependencies

`npm install`

Also, install `wasm-pack` by visiting https://rustwasm.github.io/wasm-pack/installer/

### Running locally

`npm start`

N.B. if this takes forever, and says `ℹ️  Installing wasm-pack`, then I would recommend installing `wasm-pack` separately.

### Building for production

`npm run build:prod`

### Serving locally, as if in prod

`npm run serve:prod`

### Uploading to S3

`npm run upload`

## What does this example project demonstrate?

### State management

The `smithy` runtime does not know about the state. The state is entirely managed by you, the developer, exactly how you would expect it to work. First, a mutable variable is declared, and then it is used within the `smd!` block. See, for example, [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/app.rs#L24) and [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/app.rs#L63).

### API calls

Using Smithy's `unwrapped_promise_from_future` method, we are able to make API calls and declaratively render them. See [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/app.rs#L72-L82) for its usage in the codebase. A more concise example is as follows:

```rs
let track_id = 299608205;
let search_api_call = smithy::unwrapped_promise_from_future(
  crate::api::fetch_details(&track_id),
);
smd!({
  match search_api_call {
    PromiseState::Success(search_results) => smd!(<p>Api call succeeded!</p>),
    PromiseState::Pending => smd!(<p>Loading</p>),
    PromiseState::Error(_) => smd!(<p>Something went wrong</p>),
  }
})
```

### Subcomponents

Subcomponents in Smithy are just interpolated components. There is nothing magical and no special syntax. See [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/app.rs#L37-L38) and [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/app.rs#L76-L78).

For example:

```rs
fn render_foo<'a>() -> SmithyComponent<'a> {
  smd!(<div>FOO!</div>)
}

// elsewhere
smd!(
  { render_foo() }
);

// or
let foo = render_foo();
smd!(
  { foo }
);
```
### Routing

Routing is entirely within Rust-land, using a `Page` enum and an `on_hash_change` event handler. There are no anchor tags to get around the borrow checker, though the app works just as well with anchor tags.

See [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/app.rs#L23), [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/app.rs#L28-L31) and [here](https://github.com/rbalicki2/smithy_itunes_search/blob/master/src/types.rs#L10-L50).

Rolling your own routing in Smithy can be as simple as:

```rs
enum Page {
  Home,
  DetailView(usize),
}

impl Page {
  pub fn handle_hash_change(&mut self) {
    match crate::util::get_current_hash().and_then(|hash| hash.parse::<usize>().ok()) {
      Some(id) => self.go_to_detail_view(),
      None => self.go_to_home_view(),
    };
  }

  pub fn go_to_detail_view(&mut self, id: usize) {
    *self = Page::DetailView(id);
    let _ = crate::util::get_location().set_hash(&id.to_string());
  }

  pub fn go_to_home_view(&mut self) {
    *self = Page::Home;
    let _ = crate::util::get_location().set_hash("");
  }
}

// somewhere else
smd!(
  on_hash_change={|_| page.handle_hash_change()};
  {
    match Page {
      Page::Home => {
        <a on_click={|_| page.go_to_detail_view(1)}>Go to detail 1</a>
      },
      // etc
    }
  }
);
```
