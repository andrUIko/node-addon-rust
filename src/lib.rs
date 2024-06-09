#![deny(clippy::all)]

use napi::{bindgen_prelude::*, JsUndefined};
use napi_derive::napi;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn fibonacci(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

#[napi(ts_args_type = "callback: (s: string) => void")]
pub fn run_callback(callback: JsFunction) {
  let s = "Hi there!".to_string();
  let _ = callback.call1::<String, JsUndefined>(s);
}

#[napi]
pub struct QueryEngine {
  query_name: String
}

#[napi]
impl QueryEngine {
  #[napi(constructor)]
  pub fn new(query_name: String) -> Self {
    Self {
      query_name
    }
  }

  #[napi(getter)]
  pub fn get_query_name(&self) -> &str  {
    self.query_name.as_str()
  }

  #[napi(setter)]
  pub fn set_query_name(&mut self, query_name: String) {
    self.query_name = query_name
  }
}
