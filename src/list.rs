use crate::cursor::{Cursor, CursorMut};
use crate::iter::{IntoIter, IntoKeys, IntoNodes, Iter, Keys, Nodes};
use crate::map::Map;
use crate::node::Node;
use crate::{node_next_mut, node_prev_mut};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
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
  M: Default,
{
  /// Creates an empty linked list.
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }
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

  /// Returns an iterator over all keys and nodes.
  /// The iterator element type is `(&'a K, &'a N)`.
  #[inline]
  pub fn iter(&self) -> Iter<K, N, M> {
    Iter {
      list: self,
      key: self.head.as_ref(),
    }
  }

  /// Returns an iterator over all keys.
  /// The iterator element type is `&'a K`.
  #[inline]
  pub fn keys(&self) -> Keys<K, N, M> {
    Keys { iter: self.iter() }
  }

  /// Returns an iterator over all nodes.
  /// The iterator element type is `&'a N`.
  #[inline]
  pub fn nodes(&self) -> Nodes<K, N, M> {
    Nodes { iter: self.iter() }
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
  /// Creates a consuming iterator over all keys.
  /// The list cannot be used after calling this.
  /// The iterator element type is `K`.
  #[inline]
  pub fn into_keys(self) -> IntoKeys<K, N, M> {
    IntoKeys {
      iter: self.into_iter(),
    }
  }

  /// Creates a consuming iterator over all nodes.
  /// The list cannot be used after calling this.
  /// The iterator element type is `N`.
  #[inline]
  pub fn into_nodes(self) -> IntoNodes<K, N, M> {
    IntoNodes {
      iter: self.into_iter(),
    }
  }

  /// Adds a key-node pair first in the list.
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  ///
  /// This operation should compute in *O*(1) time on average.
  pub fn push_front<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    self.nodes.insert(key.clone(), node).map(|_| {
      let next = self.head.replace(key.clone());
      match &next {
        Some(k) => *node_prev_mut!(self, k) = Some(key.clone()),
        None => self.tail = Some(key.clone()),
      }
      let node = self.node_mut(&key).unwrap();
      *node_prev_mut!(node) = None;
      *node_next_mut!(node) = next;
    })
  }

  /// Adds a key-node pair back in the list.
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  ///
  /// This operation should compute in *O*(1) time on average.
  pub fn push_back<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    self.nodes.insert(key.clone(), node).map(|_| {
      let prev = self.tail.replace(key.clone());
      match &prev {
        Some(k) => *node_next_mut!(self, k) = Some(key.clone()),
        None => self.head = Some(key.clone()),
      }
      let node = self.node_mut(&key).unwrap();
      *node_prev_mut!(node) = prev;
      *node_next_mut!(node) = None;
    })
  }

  /// Adds a key first in the list.
  ///
  /// If `key` already exists, returns an error containing `key`.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn push_key_front(&mut self, key: K) -> Result<(), K>
  where
    (): Into<N>,
  {
    self.push_front(key, ()).map_err(|(k, _)| k)
  }

  /// Adds a key back in the list.
  ///
  /// If `key` already exists, returns an error containing `key`.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn push_key_back(&mut self, key: K) -> Result<(), K>
  where
    (): Into<N>,
  {
    self.push_back(key, ()).map_err(|(k, _)| k)
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
  M: Default,
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

impl<'a, K, T, N, M> Extend<(&'a K, &'a T)> for KeyNodeList<K, N, M>
where
  K: Eq + Hash + Copy,
  T: Into<N> + Copy,
  N: Node<Key = K> + Copy,
  M: Map<K, N>,
{
  fn extend<I: IntoIterator<Item = (&'a K, &'a T)>>(&mut self, iter: I) {
    self.extend(iter.into_iter().map(|(k, n)| (*k, *n)))
  }
}

impl<'a, K: 'a, N, M> Extend<&'a K> for KeyNodeList<K, N, M>
where
  K: Eq + Hash + Copy,
  (): Into<N>,
  N: Node<Key = K> + Copy,
  M: Map<K, N>,
{
  /// Extends a `KeyNodeList` with an key iterator
  /// if the node can be built with a `()`.
  ///
  /// # Example
  ///
  /// ```
  /// use key_node_list::KeyValueList;
  ///
  /// let mut list: KeyValueList<i32, ()> = KeyValueList::new();
  /// list.extend(&[1, 2, 3]);
  /// assert_eq!(list.front_key(), Some(&1));
  /// assert_eq!(list.back_key(), Some(&3));
  /// ```
  fn extend<I: IntoIterator<Item = &'a K>>(&mut self, iter: I) {
    self.extend(iter.into_iter().copied())
  }
}

impl<K, T, N, M> Extend<(K, T)> for KeyNodeList<K, N, M>
where
  K: Eq + Hash + Clone,
  T: Into<N>,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  fn extend<I: IntoIterator<Item = (K, T)>>(&mut self, iter: I) {
    iter.into_iter().for_each(|(k, n)| {
      let _ = self.push_back(k, n);
    });
  }
}

impl<K, N, M> Extend<K> for KeyNodeList<K, N, M>
where
  K: Eq + Hash + Clone,
  (): Into<N>,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  /// Extends a `KeyNodeList` with an key iterator
  /// if the node can be built with a `()`.
  ///
  /// # Example
  ///
  /// ```
  /// use key_node_list::KeyValueList;
  ///
  /// let mut list: KeyValueList<i32, ()> = KeyValueList::new();
  /// list.extend([1, 2, 3]);
  /// assert_eq!(list.front_key(), Some(&1));
  /// assert_eq!(list.back_key(), Some(&3));
  /// ```
  fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I) {
    iter.into_iter().for_each(|k| {
      let _ = self.push_key_back(k);
    });
  }
}

impl<K, T, N, M, const LEN: usize> From<[(K, T); LEN]> for KeyNodeList<K, N, M>
where
  K: Eq + Hash + Clone,
  T: Into<N>,
  N: Node<Key = K>,
  M: Map<K, N> + Default,
{
  fn from(arr: [(K, T); LEN]) -> Self {
    std::array::IntoIter::new(arr).collect()
  }
}

impl<K, T, N, M> FromIterator<(K, T)> for KeyNodeList<K, N, M>
where
  K: Eq + Hash + Clone,
  T: Into<N>,
  N: Node<Key = K>,
  M: Map<K, N> + Default,
{
  fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
    let mut list = Self::new();
    list.extend(iter);
    list
  }
}

impl<K, N, M> FromIterator<K> for KeyNodeList<K, N, M>
where
  K: Eq + Hash + Clone,
  (): Into<N>,
  N: Node<Key = K>,
  M: Map<K, N> + Default,
{
  /// Creates a `KeyNodeList` from an key iterator
  /// if the node can be built with a `()`.
  ///
  /// # Example
  ///
  /// ```
  /// use key_node_list::KeyValueList;
  ///
  /// let list: KeyValueList<i32, ()> = [1, 2, 3].into_iter().collect();
  /// assert_eq!(list.front_key(), Some(&1));
  /// assert_eq!(list.back_key(), Some(&3));
  /// ```
  fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
    let mut list = Self::new();
    list.extend(iter);
    list
  }
}

impl<'a, K, Q, N, M> Index<&'a Q> for KeyNodeList<K, N, M>
where
  K: Hash + Eq + Borrow<Q>,
  Q: ?Sized + Hash + Eq,
  M: Map<K, N>,
{
  type Output = N;

  /// Returns a reference to the value corresponding to the supplied key.
  ///
  /// # Panics
  ///
  /// Panics if the key is not present in the [`KeyNodeList`].
  #[inline]
  fn index(&self, key: &'a Q) -> &Self::Output {
    self.nodes.get(key).expect("no entry found for key")
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
    IntoIter { list: self }
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

impl<K, N, M> PartialEq<KeyNodeList<K, N, M>> for KeyNodeList<K, N, M>
where
  M: PartialEq,
{
  fn eq(&self, other: &KeyNodeList<K, N, M>) -> bool {
    self.nodes == other.nodes
  }
}

impl<K, N, M> Eq for KeyNodeList<K, N, M> where M: PartialEq {}
