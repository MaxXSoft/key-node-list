//! Doubly-linked list that stores key-node pairs.
//!
//! [`KeyNodeList`] is a doubly-linked list, it uses a hash map to maintain
//! correspondence between keys and nodes, and records the previous key and
//! the next key of the current node in the node itself. There is no pointer
//! operations during this process, so `key_node_list` is all implemented in
//! safe Rust.
//!
//! You can complete key lookups, key-node pair updates, key-node pair
//! deletions and other operations of [`KeyNodeList`] in *O*(1)~ time. You
//! can also use cursor-based interface to traverse or edit the linked list.

mod cursor;
mod iter;
mod list;
mod map;
mod node;

pub use cursor::*;
pub use iter::*;
pub use list::*;
pub use map::*;
pub use node::*;

/// A [`KeyNodeList`] that uses [`GenericNode<K, V>`] as its node type and
/// [`HashMap`](std::collections::HashMap) as its underlying hash map.
///
/// `KeyValueList` stores key-value pairs and organize them in the form of
/// a doubly-linked list.
pub type KeyValueList<K, V> = KeyNodeList<K, GenericNode<K, V>>;

/// Gets a mutable reference of the previous pointer of the specific node.
macro_rules! node_prev_mut {
  ($list:expr, $key:expr) => {
    $list
      .node_mut::<K>($key)
      .unwrap()
      .prev_mut::<$crate::node::Token>()
  };
  ($node:expr) => {
    $node.prev_mut::<$crate::node::Token>()
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
  ($node:expr) => {
    $node.next_mut::<$crate::node::Token>()
  };
}
pub(crate) use node_next_mut;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_capacity() {
    let mut list = KeyValueList::new();
    assert_eq!(list.front_key(), None);
    assert_eq!(list.back_key(), None);
    assert_eq!(list.len(), 0);
    assert_eq!(list.is_empty(), true);
    list.push_back(1, 1).unwrap();
    assert_eq!(list.front_key(), Some(&1));
    assert_eq!(list.back_key(), Some(&1));
    assert_eq!(list.len(), 1);
    assert_eq!(list.is_empty(), false);
    list.push_back(2, 2).unwrap();
    assert_eq!(list.front_key(), Some(&1));
    assert_eq!(list.back_key(), Some(&2));
    assert_eq!(list.len(), 2);
    assert_eq!(list.is_empty(), false);
    list.clear();
    assert_eq!(list.front_key(), None);
    assert_eq!(list.back_key(), None);
    assert_eq!(list.len(), 0);
    assert_eq!(list.is_empty(), true);
  }

  #[test]
  fn test_push_into_iter() {
    let mut list = KeyValueList::new();
    for i in 0..10 {
      list.push_back(i, i).unwrap();
    }
    let vec = list.into_iter().collect::<Vec<_>>();
    assert_eq!(vec.len(), 10);
    for (i, (k, n)) in vec.into_iter().enumerate() {
      assert_eq!(i as i32, k);
      assert_eq!(i as i32, n.into_value());
    }
  }

  #[test]
  fn test_push_pop() {
    let mut list = KeyValueList::new();
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_front(), None);
    for i in 0..10 {
      list.push_back(i, i).unwrap();
    }
    let mut cur = 9;
    while let Some((k, n)) = list.pop_back() {
      assert_eq!(cur, k);
      assert_eq!(cur, n.into_value());
      cur -= 1;
    }
    assert_eq!(cur, -1);
  }

  #[test]
  fn test_push_front_iter() {
    let mut list = KeyValueList::new();
    for i in 0..10 {
      list.push_front(i, i * 2).unwrap();
    }
    for (i, (k, n)) in list.iter().enumerate() {
      assert_eq!(9 - i, *k);
      assert_eq!((9 - i) * 2, *n.value());
    }
    for (i, k) in list.keys().enumerate() {
      assert_eq!(9 - i, *k);
    }
    for (i, n) in list.nodes().enumerate() {
      assert_eq!((9 - i) * 2, *n.value());
    }
  }

  #[test]
  fn test_cursor_move() {
    let mut list = KeyValueList::new();
    for i in 0..10 {
      list.push_front(i, i * 2).unwrap();
    }
    let mut cur = list.cursor_back();
    assert_eq!(cur.key(), Some(&0));
    cur.move_prev();
    assert_eq!(cur.key(), Some(&1));
    dbg!(cur.key());
    dbg!(cur.node());
    assert_eq!(cur.next_key(), Some(&0));
    cur.move_next();
    assert_eq!(cur.key(), Some(&0));
    cur.move_next();
    assert_eq!(cur.key(), None);
    cur.move_next();
    assert_eq!(cur.key(), Some(&9));
  }

  #[test]
  fn test_cursor_insert_remove() {
    let mut list = KeyValueList::new();
    for i in 0..10 {
      list.push_back(i * 10, i * 10).unwrap();
    }
    let mut cur = list.cursor_front_mut();
    while cur.key() != Some(&40) {
      cur.move_next();
    }
    cur.insert_before(37, 37).unwrap();
    cur.insert_before(38, 38).unwrap();
    cur.insert_before(39, 39).unwrap();
    cur.insert_after(42, 42).unwrap();
    cur.insert_after(41, 41).unwrap();
    for i in 0..3 {
      assert_eq!(
        cur.remove_current().map(|(k, n)| (k, n.into_value())),
        Some((40 + i, 40 + i))
      );
    }
    assert_eq!(cur.key(), Some(&50));
    for i in 0..3 {
      cur.move_prev();
      assert_eq!(
        cur.remove_current().map(|(k, n)| (k, n.into_value())),
        Some((39 - i, 39 - i))
      );
    }
    assert_eq!(cur.prev_key(), Some(&30));
  }
}
