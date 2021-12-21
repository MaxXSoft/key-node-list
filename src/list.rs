use crate::iter::{IntoIter, Iter, Keys, Nodes};
use crate::node::{Node, Token};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

/// A linked list with key-node form.
#[derive(Clone)]
pub struct KeyNodeList<K, N> {
  nodes: HashMap<K, N>,
  head: Option<K>,
  tail: Option<K>,
}

impl<K, N> KeyNodeList<K, N> {
  /// Creates an empty linked list.
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns the head key of the linked list.
  #[inline]
  pub fn head(&self) -> Option<&K> {
    self.head.as_ref()
  }

  /// Returns the tail key of the linked list.
  #[inline]
  pub fn tail(&self) -> Option<&K> {
    self.tail.as_ref()
  }

  /// Returns the number of elements in the linked list.
  #[inline]
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  /// Returns `true` if the linked list contains no elements.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  /// Removes all elements in the linked list.
  #[inline]
  pub fn clear(&mut self) {
    self.nodes.clear();
    self.head = None;
    self.tail = None;
  }

  /// Creates an iterator from a value.
  #[inline]
  pub fn into_iter(self) -> IntoIter<K, N> {
    IntoIter { list: self }
  }

  /// Returns an iterator over all keys and nodes.
  #[inline]
  pub fn iter(&self) -> Iter<K, N> {
    Iter {
      list: self,
      key: self.head.as_ref(),
    }
  }

  /// Returns an iterator over all keys.
  #[inline]
  pub fn keys(&self) -> Keys<K, N> {
    Keys { iter: self.iter() }
  }

  /// Returns an iterator over all nodes.
  #[inline]
  pub fn nodes(&self) -> Nodes<K, N> {
    Nodes { iter: self.iter() }
  }
}

impl<K, N> KeyNodeList<K, N>
where
  K: Hash + Eq,
{
  /// Returns `true` if the linked list contains a node for the specified key.
  #[inline]
  pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    self.nodes.contains_key(key)
  }

  /// Returns a reference to the node corresponding to the key.
  #[inline]
  pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&N>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    self.nodes.get(key)
  }

  /// Returns a mutable reference to the node corresponding to the key.
  #[inline]
  pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut N>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    self.nodes.get_mut(key)
  }

  /// Returns a reference to the first node of the linked list.
  #[inline]
  pub fn head_node(&self) -> Option<&N> {
    self.head.as_ref().and_then(|k| self.nodes.get(k))
  }

  /// Returns a mutable reference to the first node of the linked list.
  #[inline]
  pub fn head_node_mut(&mut self) -> Option<&mut N> {
    self.head.as_ref().and_then(|k| self.nodes.get_mut(k))
  }

  /// Returns a reference to the last node of the linked list.
  #[inline]
  pub fn tail_node(&self) -> Option<&N> {
    self.tail.as_ref().and_then(|k| self.nodes.get(k))
  }

  /// Returns a mutable reference to the last node of the linked list.
  #[inline]
  pub fn tail_node_mut(&mut self) -> Option<&mut N> {
    self.tail.as_ref().and_then(|k| self.nodes.get_mut(k))
  }
}

impl<K, N> KeyNodeList<K, N>
where
  K: Hash + Eq,
  N: Node<Key = K>,
{
  /// Returns a reference of the previous node of `node`,
  /// or `None` if `node` is the first node or not in the linked list.
  #[inline]
  pub fn prev_node(&self, node: &N) -> Option<&N> {
    node.prev().and_then(|k| self.nodes.get(k))
  }

  /// Returns a mutable reference of the previous node of `node`,
  /// or `None` if `node` is the first node or not in the linked list.
  #[inline]
  pub fn prev_node_mut(&mut self, node: &N) -> Option<&mut N> {
    node.prev().and_then(|k| self.nodes.get_mut(k))
  }

  /// Returns a reference of the next node of `node`,
  /// or `None` if `node` is the last node or not in the linked list.
  #[inline]
  pub fn next_node(&self, node: &N) -> Option<&N> {
    node.next().and_then(|k| self.nodes.get(k))
  }

  /// Returns a mutable reference of the next node of `node`,
  /// or `None` if `node` is the last node or not in the linked list.
  #[inline]
  pub fn next_node_mut(&mut self, node: &N) -> Option<&mut N> {
    node.next().and_then(|k| self.nodes.get_mut(k))
  }

  /// Inserts key-node pair `key` and `node` before the node `cur`.
  ///
  /// If `cur` is `None`, `key` and `node` are inserted at the end of the
  /// linked list.
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  pub fn insert_before(&mut self, cur: Option<&N>, key: K, mut node: N) -> Result<(), (K, N)>
  where
    K: Clone,
  {
    if self.nodes.contains_key(&key) {
      Err((key, node))
    } else {
      if let Some(cur) = cur {
        let cur_key = cur.prev().map_or_else(
          || self.head.replace(key.clone()),
          |prev_key| {
            let prev = self.nodes.get_mut(prev_key).unwrap();
            prev.next_mut::<Token>().replace(key.clone())
          },
        );
        *node.prev_mut::<Token>() = self
          .nodes
          .get_mut(cur_key.as_ref().unwrap())
          .unwrap()
          .prev_mut::<Token>()
          .replace(key.clone());
        *node.next_mut::<Token>() = cur_key;
      } else {
        let prev_key = self.tail.replace(key.clone());
        // self.nodes.get_mut(prev_key.as_ref().unwrap())
      }
      self.nodes.insert(key, node);
      Ok(())
    }
  }
}

impl<K, N> Default for KeyNodeList<K, N> {
  #[inline]
  fn default() -> Self {
    KeyNodeList {
      nodes: HashMap::default(),
      head: None,
      tail: None,
    }
  }
}

impl<K, N> Index<&K> for KeyNodeList<K, N>
where
  K: Hash + Eq,
{
  type Output = N;

  #[inline]
  fn index(&self, key: &K) -> &Self::Output {
    self.nodes.index(key)
  }
}

impl<K, N> IntoIterator for KeyNodeList<K, N>
where
  K: Hash + Eq,
  N: Node<Key = K>,
{
  type Item = (K, N);
  type IntoIter = IntoIter<K, N>;

  fn into_iter(self) -> Self::IntoIter {
    self.into_iter()
  }
}

impl<'a, K, N> IntoIterator for &'a KeyNodeList<K, N>
where
  K: Hash + Eq,
  N: Node<Key = K>,
{
  type Item = (&'a K, &'a N);
  type IntoIter = Iter<'a, K, N>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
