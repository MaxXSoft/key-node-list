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
//!
//! # Examples
//!
//! ## Organizing key-value pairs using `KeyValueList`
//!
//! ```
//! use key_node_list::KeyValueList;
//!
//! // construct key-value list from tuple array
//! let mut list = KeyValueList::from([(1, "Reimu"), (2, "Marisa")]);
//!
//! // or pushing other key-value pairs to the front/back of list
//! list.push_front(0, "Alice").unwrap();
//! list.push_back(3, "Patchouli").unwrap();
//!
//! // query nodes by key
//! assert_eq!(list[&1].value(), &"Reimu");
//! assert_eq!(list[&0].value(), &"Alice");
//!
//! // also you can update nodes by key
//! *list.node_mut(&3).unwrap().value_mut() = "Youmu";
//! *list.front_node_mut().unwrap().value_mut() = "Mokou";
//! assert_eq!(list[&3].value(), &"Youmu");
//! assert_eq!(list[&0].value(), &"Mokou");
//! assert_eq!(list.front_node().unwrap().value(), &"Mokou");
//!
//! // remove some key-node pairs
//! assert!(list.pop_front().is_some());
//! assert!(list.remove(&2).is_some());
//!
//! // all key-node pairs are in order
//! list.push_back(5, "Yuyuko");
//! let vec: Vec<_> = list.into_iter().map(|(k, n)| (k, n.into_value())).collect();
//! assert_eq!(vec, [(1, "Reimu"), (3, "Youmu"), (5, "Yuyuko")]);
//! ```
//!
//! ## Editing list using cursors
//!
//! ```
//! use key_node_list::KeyValueList;
//!
//! let mut first_name = KeyValueList::from([
//!   (1, "Reimu".to_string()),
//!   (2, "Marisa".to_string()),
//! ]);
//!
//! let last_name = KeyValueList::from([(1, "Hakurei"), (2, "Kirisame")]);
//!
//! // append last names after first names
//! let mut first_cur = first_name.cursor_front_mut();
//! let mut last_cur = last_name.cursor_front();
//! while let Some(node) = first_cur.node_mut() {
//!   node.value_mut().push(' ');
//!   node.value_mut().push_str(last_cur.node().unwrap().value());
//!   first_cur.move_next();
//!   last_cur.move_next();
//! }
//!
//! let vec: Vec<_> = first_name.iter().map(|(k, n)| (k, n.value().as_str())).collect();
//! assert_eq!(vec, [(&1, "Reimu Hakurei"), (&2, "Marisa Kirisame")]);
//! ```
//!
//! ## Customizing your own nodes
//!
//! ```
//! use key_node_list::{Node, impl_node};
//! use key_node_list::KeyNodeList;
//! 
//! struct NameNode<'a> {
//!   first: &'a str,
//!   last: &'a str,
//!   prev: Option<i32>,
//!   next: Option<i32>,
//! }
//! 
//! impl<'a> From<(&'a str, &'a str)> for NameNode<'a> {
//!   fn from(name: (&'a str, &'a str)) -> Self {
//!     let (first, last) = name;
//!     Self {
//!       first,
//!       last,
//!       prev: None,
//!       next: None,
//!     }
//!   }
//! }
//! 
//! // implements `Node` trait for `NameNode`
//! impl_node!(NameNode<'a> { Key = i32, prev = prev, next = next });
//! 
//! // create a `KeyNodeList` with node type `NameNode`
//! let mut names: KeyNodeList<_, NameNode> = KeyNodeList::new();
//! names.push_back(1, ("Reimu", "Hakurei"));
//! names.push_back(2, ("Marisa", "Kirisame"));
//! assert_eq!(names[&1].first, "Reimu");
//! assert_eq!(names[&2].last, "Kirisame");
//! ```

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

/// A [`KeyNodeList`] that uses [`ValueNode<K, V>`] as its node type and
/// [`HashMap`](std::collections::HashMap) as its underlying hash map.
///
/// `KeyValueList` stores key-value pairs and organize them in the form of
/// a doubly-linked list.
pub type KeyValueList<K, V> = KeyNodeList<K, ValueNode<K, V>>;

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
    assert!(list.is_empty());
    list.push_back(1, 1).unwrap();
    assert_eq!(list.front_key(), Some(&1));
    assert_eq!(list.back_key(), Some(&1));
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    list.push_back(2, 2).unwrap();
    assert_eq!(list.front_key(), Some(&1));
    assert_eq!(list.back_key(), Some(&2));
    assert_eq!(list.len(), 2);
    assert!(!list.is_empty());
    list.clear();
    assert_eq!(list.front_key(), None);
    assert_eq!(list.back_key(), None);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
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

  #[test]
  fn test_from_eq() {
    let list1 = KeyValueList::from([(1, 1), (2, 2), (3, 3)]);
    let list2 = KeyValueList::from([(1, 1), (2, 2), (3, 3)]);
    let list3 = KeyValueList::from([(1, 1), (2, 2), (3, 4)]);
    assert_eq!(list1, list2);
    assert_ne!(list1, list3);
  }
}
