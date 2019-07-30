use web_sys::{
  window,
  Document,
  Location,
};

pub fn get_location() -> Location {
  window().unwrap().location()
}

pub fn get_current_hash() -> Option<String> {
  get_location()
    .hash()
    .ok()
    .map(|hash_with_hash| hash_with_hash.chars().skip(1).collect::<String>())
}

pub fn document() -> Document {
  window().unwrap().document().unwrap()
}

pub fn clone_and_zip<'a, T, U>(
  iter: impl Iterator<Item = T> + 'a,
  cloneable: &'a U,
) -> impl Iterator<Item = (T, U)> + 'a
where
  U: Clone,
{
  iter.map(move |item| (item, cloneable.clone()))
}
