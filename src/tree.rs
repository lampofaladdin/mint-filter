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
      root: Node::new('r', false),
    }
  }

  // 插入节点
  pub fn insert() {}

  // 删除节点
  pub fn remove() {}
}
