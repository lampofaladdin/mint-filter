#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Mint {
  pub  keywords: Vec<String>,
}

#[napi]
impl Mint {
  #[napi(constructor)]
  pub fn new(keywords:Vec<String>) -> Mint {
    Self {
      keywords: keywords,
    }
  }

  #[napi]
  pub fn get_data(&self)->napi::Result<Vec<String>>{
    Ok(self.keywords.clone()[0..1].to_vec())
    // Ok(self.keywords.as_ref()[0..1].to_vec())
  }
}

