use crate::list::KeyNodeList;
use crate::node::Node;
use std::fmt;
use std::hash::Hash;

/// A cursor over a [`KeyNodeList`].
#[derive(Clone)]
pub struct Cursor<'a, K, N> {
  pub(crate) list: &'a KeyNodeList<K, N>,
  pub(crate) key: Option<&'a K>,
}

impl<'a, K, N> Cursor<'a, K, N> {
  /// Checks if the cursor is currently pointing to the null pair.
  #[inline]
  pub fn is_null(&self) -> bool {
    self.key.is_none()
  }

  /// Returns a reference to the key that the cursor is currently pointing to.
  ///
  /// Returns `None` if the cursor is currently pointing to the null pair.
  #[inline]
  pub fn key(&self) -> Option<&K> {
    self.key
  }

  /// Provides a reference to the front key of the cursor’s parent list,
  /// or `None` if the list is empty.
  #[inline]
  pub fn front_key(&self) -> Option<&K> {
    self.list.head.as_ref()
  }

  /// Provides a reference to the back key of the cursor’s parent list,
  /// or `None` if the list is empty.
  #[inline]
  pub fn back_key(&self) -> Option<&K> {
    self.list.tail.as_ref()
  }
}

impl<'a, K, N> Cursor<'a, K, N>
where
  K: Hash + Eq,
{
  /// Returns a reference to the node that the cursor is currently pointing to.
  ///
  /// Returns `None` if the cursor is currently pointing to the null pair.
  #[inline]
  pub fn node(&self) -> Option<&N> {
    self.key.and_then(|k| self.list.nodes.get(k))
  }

  /// Provides a reference to the front node of the cursor’s parent list,
  /// or `None` if the list is empty.
  #[inline]
  pub fn front_node(&self) -> Option<&N> {
    self.front_key().and_then(|k| self.list.nodes.get(k))
  }

  /// Provides a reference to the back node of the cursor’s parent list,
  /// or `None` if the list is empty.
  #[inline]
  pub fn back_node(&self) -> Option<&N> {
    self.back_key().and_then(|k| self.list.nodes.get(k))
  }
}

impl<'a, K, N> Cursor<'a, K, N>
where
  K: Hash + Eq,
  N: Node<Key = K>,
{
  /// Returns a reference to the next key.
  ///
  /// If the cursor is pointing to the null pair then this returns the first
  /// key of the [`KeyNodeList`]. If it is pointing to the last key of the
  /// [`KeyNodeList`] then this returns `None`.
  #[inline]
  pub fn next_key(&self) -> Option<&K> {
    self.key.map_or_else(
      || self.list.head.as_ref(),
      |k| self.list.get(k).and_then(|n| n.next()),
    )
  }

  /// Returns a reference to the previous key.
  ///
  /// If the cursor is pointing to the null pair then this returns the last
  /// key of the [`KeyNodeList`]. If it is pointing to the first key of the
  /// [`KeyNodeList`] then this returns `None`.
  #[inline]
  pub fn prev_key(&self) -> Option<&K> {
    self.key.map_or_else(
      || self.list.tail.as_ref(),
      |k| self.list.get(k).and_then(|n| n.prev()),
    )
  }

  /// Returns a reference to the next node.
  ///
  /// If the cursor is pointing to the null pair then this returns the first
  /// node of the [`KeyNodeList`]. If it is pointing to the last node of the
  /// [`KeyNodeList`] then this returns `None`.
  #[inline]
  pub fn next_node(&self) -> Option<&N> {
    self.next_key().and_then(|k| self.list.get(k))
  }

  /// Returns a reference to the previous node.
  ///
  /// If the cursor is pointing to the null pair then this returns the last
  /// node of the [`KeyNodeList`]. If it is pointing to the first node of the
  /// [`KeyNodeList`] then this returns `None`.
  #[inline]
  pub fn prev_node(&self) -> Option<&N> {
    self.prev_key().and_then(|k| self.list.get(k))
  }

  /// Moves the cursor to the next key-node pair of the [`KeyNodeList`].
  ///
  /// If the cursor is pointing to the null pair then this will move it to
  /// the first key-node pair of the [`KeyNodeList`]. If it is pointing to
  /// the last key-node pair of the [`KeyNodeList`] then this will move it
  /// to the null pair.
  #[inline]
  pub fn move_next(&mut self) {
    self.key = self.key.map_or_else(
      || self.list.head.as_ref(),
      |k| self.list.get(k).and_then(|n| n.next()),
    );
  }

  /// Moves the cursor to the previous key-node pair of the [`KeyNodeList`].
  ///
  /// If the cursor is pointing to the null pair then this will move it to
  /// the last key-node pair of the [`KeyNodeList`]. If it is pointing to
  /// the first key-node pair of the [`KeyNodeList`] then this will move it
  /// to the null pair.
  #[inline]
  pub fn move_prev(&mut self) {
    self.key = self.key.map_or_else(
      || self.list.tail.as_ref(),
      |k| self.list.get(k).and_then(|n| n.prev()),
    );
  }
}

impl<'a, K, N> fmt::Debug for Cursor<'a, K, N>
where
  K: Hash + Eq + fmt::Debug,
  N: Node<Key = K> + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_tuple("Cursor")
      .field(self.list)
      .field(&self.key)
      .finish()
  }
}
