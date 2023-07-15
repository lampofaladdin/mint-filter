use std::collections::HashMap;

use crate::node::Node;

#[derive(Debug)]
pub struct Tree {
  pub root: Node,
}

// Trie 树结构
impl Tree {
  /// 初始化Trie树
  ///
  /// # Examples
  ///
  /// ```
  /// let trie = Tree::new();
  ///
  /// ```
  pub fn new() -> Tree {
    Tree {
      root: Node::new('💡', false),
    }
  }

  // 插入节点
  pub fn insert(&mut self, key: String) -> bool {
    if key.len() == 0 {
      return false;
    }
    let mut key_array: Vec<char> = key.chars().collect();
    let first_child = key_array[0];

    if self.root.children.is_none() {
      self.root.children = Some(HashMap::new());
    }

    // TODO 如何将TS的逻辑变更成rust的逻辑
    // Some vec 如何处理
    // 尝试修复bug
    if let Some(self.root.children) = None{

    }

    return true;
  }

  // 删除节点
  pub fn remove() {}
}
