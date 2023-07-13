#[derive(Debug)]
pub struct Node {
  pub key: char,
  pub word: bool,
  pub children: Option<Vec<Node>>,
}

impl Node {
  pub fn new(key: char, word: bool)-> Node {
    Node {
      key,
      word,
      children: None,
    }
  }
}
