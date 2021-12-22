use crate::cursor::{Cursor, CursorMut};
use crate::iter::{IntoIter, Iter, Keys, Nodes};
use crate::map::Map;
use crate::node::Node;
use crate::{node_next_mut, node_prev_mut};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Index;

/// A doubly-linked list that stores key-node pairs.
#[derive(Clone)]
pub struct KeyNodeList<K, N, M = HashMap<K, N>> {
  pub(crate) nodes: M,
  pub(crate) head: Option<K>,
  pub(crate) tail: Option<K>,
  phantom: PhantomData<N>,
}

impl<K, N, M> KeyNodeList<K, N, M>
where
  M: Map<K, N>,
{
  /// Creates an linked list with the given hash map `map`.
  #[inline]
  pub fn with_map(map: M) -> Self {
    Self {
      nodes: map,
      head: None,
      tail: None,
      phantom: PhantomData,
    }
  }

  /// Returns a reference to the front key, or `None` if the list is empty.
  ///
  /// This operation should compute in *O*(1) time.
  #[inline]
  pub fn front_key(&self) -> Option<&K> {
    self.head.as_ref()
  }

  /// Returns a reference to the back key, or `None` if the list is empty.
  ///
  /// This operation should compute in *O*(1) time.
  #[inline]
  pub fn back_key(&self) -> Option<&K> {
    self.tail.as_ref()
  }

  /// Returns the number of key-node pairs in the list.
  ///
  /// This operation should compute in *O*(1) time.
  #[inline]
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  /// Returns `true` if the list contains no key-node pairs.
  ///
  /// This operation should compute in *O*(1) time.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  /// Removes all key-node pairs in the list.
  #[inline]
  pub fn clear(&mut self) {
    self.nodes.clear();
    self.head = None;
    self.tail = None;
  }

  /// Creates an iterator from the list.
  #[inline]
  pub fn into_iter(self) -> IntoIter<K, N, M> {
    IntoIter { list: self }
  }

  /// Returns an iterator over all keys and nodes.
  #[inline]
  pub fn iter(&self) -> Iter<K, N, M> {
    Iter {
      list: self,
      key: self.head.as_ref(),
    }
  }

  /// Returns an iterator over all keys.
  #[inline]
  pub fn keys(&self) -> Keys<K, N, M> {
    Keys { iter: self.iter() }
  }

  /// Returns an iterator over all nodes.
  #[inline]
  pub fn nodes(&self) -> Nodes<K, N, M> {
    Nodes { iter: self.iter() }
  }
}

impl<K, N, M> KeyNodeList<K, N, M>
where
  M: Map<K, N> + Default,
{
  /// Creates an empty linked list.
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }
}

impl<K, N, M> KeyNodeList<K, N, M>
where
  K: Hash + Eq,
  M: Map<K, N>,
{
  /// Returns `true` if the linked list contains a node for the specified key.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    self.nodes.contains_key(key)
  }

  /// Returns a reference to the node corresponding to the key,
  /// or `None` if key does not exist.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn node<Q: ?Sized>(&self, key: &Q) -> Option<&N>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    self.nodes.get(key)
  }

  /// Returns a mutable reference to the node corresponding to the key,
  /// or `None` if key does not exist.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn node_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut N>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    self.nodes.get_mut(key)
  }

  /// Returns a reference to the front node, or `None` if the list is empty.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn front_node(&self) -> Option<&N> {
    self.head.as_ref().and_then(|k| self.nodes.get(k))
  }

  /// Returns a mutable reference to the front node,
  /// or `None` if the list is empty.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn front_node_mut(&mut self) -> Option<&mut N> {
    self.head.as_ref().and_then(|k| self.nodes.get_mut(k))
  }

  /// Returns a reference to the back node, or `None` if the list is empty.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn back_node(&self) -> Option<&N> {
    self.tail.as_ref().and_then(|k| self.nodes.get(k))
  }

  /// Returns a mutable reference to the back node,
  /// or `None` if the list is empty.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn back_node_mut(&mut self) -> Option<&mut N> {
    self.tail.as_ref().and_then(|k| self.nodes.get_mut(k))
  }

  /// Provides a cursor at the specific key.
  ///
  /// The cursor is pointing to the null pair if the key does not exist.
  #[inline]
  pub fn cursor(&self, key: K) -> Cursor<K, N, M> {
    Cursor {
      list: self,
      key: self.contains_key(&key).then(|| key),
    }
  }

  /// Provides a cursor with editing operations at the specific key.
  ///
  /// The cursor is pointing to the null pair if the key does not exist.
  #[inline]
  pub fn cursor_mut(&mut self, key: K) -> CursorMut<K, N, M> {
    CursorMut {
      key: self.contains_key(&key).then(|| key),
      list: self,
    }
  }
}

impl<K, N, M> KeyNodeList<K, N, M>
where
  K: Hash + Eq + Clone,
  M: Map<K, N>,
{
  /// Provides a cursor at the front key-node pair.
  ///
  /// The cursor is pointing to the null pair if the list is empty.
  #[inline]
  pub fn cursor_front(&self) -> Cursor<K, N, M> {
    Cursor {
      list: self,
      key: self.head.clone(),
    }
  }

  /// Provides a cursor with editing operations at the front key-node pair.
  ///
  /// The cursor is pointing to the null pair if the list is empty.
  #[inline]
  pub fn cursor_front_mut(&mut self) -> CursorMut<K, N, M> {
    CursorMut {
      key: self.head.clone(),
      list: self,
    }
  }

  /// Provides a cursor at the back key-node pair.
  ///
  /// The cursor is pointing to the null pair if the list is empty.
  #[inline]
  pub fn cursor_back(&self) -> Cursor<K, N, M> {
    Cursor {
      list: self,
      key: self.tail.clone(),
    }
  }

  /// Provides a cursor with editing operations at the back key-node pair.
  ///
  /// The cursor is pointing to the null pair if the list is empty.
  #[inline]
  pub fn cursor_back_mut(&mut self) -> CursorMut<K, N, M> {
    CursorMut {
      key: self.tail.clone(),
      list: self,
    }
  }
}

impl<K, N, M> KeyNodeList<K, N, M>
where
  K: Hash + Eq + Clone,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  /// Adds an key-node pair first in the list.
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  ///
  /// This operation should compute in *O*(1) time on average.
  pub fn push_front<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    if self.contains_key(&key) {
      Err((key, node))
    } else {
      match self.head.replace(key.clone()) {
        Some(k) => *node_prev_mut!(self, &k) = Some(key.clone()),
        None => self.tail = Some(key.clone()),
      }
      self.nodes.insert(key, node.into());
      Ok(())
    }
  }

  /// Adds an key-node pair back in the list.
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  ///
  /// This operation should compute in *O*(1) time on average.
  pub fn push_back<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    if self.contains_key(&key) {
      Err((key, node))
    } else {
      match self.tail.replace(key.clone()) {
        Some(k) => *node_next_mut!(self, &k) = Some(key.clone()),
        None => self.head = Some(key.clone()),
      }
      self.nodes.insert(key, node.into());
      Ok(())
    }
  }

  /// Removes the first key-node pair and returns it, or `None` if the list
  /// is empty.
  ///
  /// This operation should compute in *O*(1) time on average.
  pub fn pop_front(&mut self) -> Option<(K, N)> {
    self.head.take().map(|k| {
      let node = self.nodes.remove(&k).unwrap();
      self.head = node.next().cloned();
      (k, node)
    })
  }

  /// Removes the last key-node pair and returns it, or `None` if the list
  /// is empty.
  ///
  /// This operation should compute in *O*(1) time on average.
  pub fn pop_back(&mut self) -> Option<(K, N)> {
    self.tail.take().map(|k| {
      let node = self.nodes.remove(&k).unwrap();
      self.tail = node.prev().cloned();
      (k, node)
    })
  }

  /// Removes the key-node pair at the given key and returns it,
  /// or returns `None` if `key` does not exists.
  pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, N)>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    self.nodes.remove_entry(key).map(|(k, n)| {
      match n.prev() {
        Some(k) => *node_next_mut!(self, k) = n.next().cloned(),
        None => self.head = n.next().cloned(),
      }
      match n.next() {
        Some(k) => *node_prev_mut!(self, k) = n.prev().cloned(),
        None => self.tail = n.prev().cloned(),
      }
      (k, n)
    })
  }
}

impl<K, N, M> fmt::Debug for KeyNodeList<K, N, M>
where
  K: Hash + Eq + fmt::Debug,
  N: Node<Key = K> + fmt::Debug,
  M: Map<K, N>,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_list().entries(self).finish()
  }
}

impl<K, N, M> Default for KeyNodeList<K, N, M>
where
  M: Map<K, N> + Default,
{
  #[inline]
  fn default() -> Self {
    KeyNodeList {
      nodes: M::default(),
      head: None,
      tail: None,
      phantom: PhantomData,
    }
  }
}

impl<'a, K, Q, N, M> Index<&'a Q> for KeyNodeList<K, N, M>
where
  K: Hash + Eq + Borrow<Q>,
  Q: ?Sized + Hash + Eq,
  M: Map<K, N> + Index<&'a Q, Output = N>,
{
  type Output = N;

  #[inline]
  fn index(&self, key: &'a Q) -> &Self::Output {
    self.nodes.index(key)
  }
}

impl<K, N, M> IntoIterator for KeyNodeList<K, N, M>
where
  K: Hash + Eq + Clone,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  type Item = (K, N);
  type IntoIter = IntoIter<K, N, M>;

  fn into_iter(self) -> Self::IntoIter {
    self.into_iter()
  }
}

impl<'a, K, N, M> IntoIterator for &'a KeyNodeList<K, N, M>
where
  K: Hash + Eq,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  type Item = (&'a K, &'a N);
  type IntoIter = Iter<'a, K, N, M>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
