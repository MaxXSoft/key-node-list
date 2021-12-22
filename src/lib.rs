//! TODO
//!
//! * This doc.
//! * Examples.
//! * More trait impls.

mod cursor;
mod iter;
mod list;
mod node;

pub use cursor::*;
pub use iter::*;
pub use list::*;
pub use node::*;

/// A [`KeyNodeList`] that uses [`GenericNode<K, V>`] as its node type.
///
/// `KeyValueList` can store key-value pairs and organize them in the form
/// of a doubly-linked list.
pub type KeyValueList<K, V> = KeyNodeList<K, GenericNode<K, V>>;

/// Gets a mutable reference of the previous pointer of the specific node.
macro_rules! node_prev_mut {
  ($list:expr, $key:expr) => {
    $list
      .node_mut::<K>($key)
      .unwrap()
      .prev_mut::<$crate::node::Token>()
  };
}
pub(crate) use node_prev_mut;

/// Gets a mutable reference of the next pointer of the specific node.
macro_rules! node_next_mut {
  ($list:expr, $key:expr) => {
    $list
      .node_mut::<K>($key)
      .unwrap()
      .next_mut::<$crate::node::Token>()
  };
}
pub(crate) use node_next_mut;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_push_into_iter() {
    let mut list = KeyValueList::new();
    for i in 0..10 {
      list.push_back(i, i).unwrap();
    }
    for (i, (k, n)) in list.into_iter().enumerate() {
      assert_eq!(i as i32, k);
      assert_eq!(i as i32, n.into_value());
    }
  }

  #[test]
  fn test_push_pop() {
    let mut list = KeyValueList::new();
    for i in 0..10 {
      list.push_back(i, i).unwrap();
    }
    let mut cur = 9;
    while let Some((k, n)) = list.pop_back() {
      assert_eq!(cur, k);
      assert_eq!(cur, n.into_value());
      cur -= 1;
    }
  }
}
