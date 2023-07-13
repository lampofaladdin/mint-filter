#![deny(clippy::all)]

use crate::tree::Tree;

mod node;
mod tree;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Mint {
  pub keywords: Vec<String>,
}

#[napi]
impl Mint {
  #[napi(constructor)]
  pub fn new(keywords: Vec<String>) -> Mint {
    // println!("input keywords: {:?}", keywords);
    demo();
    Self { keywords: keywords }
  }
  
  #[napi]
  pub fn filter(&self, text: String) -> napi::Result<Vec<String>> {
    println!("input text: {:?}", text);
    Ok(self.keywords.clone()[0..1].to_vec())
  }
}

fn demo() {
  let b = Tree::new();
  println!("b is {:?}", b);
}
