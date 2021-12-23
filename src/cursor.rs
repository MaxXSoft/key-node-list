use crate::list::KeyNodeList;
use crate::map::Map;
use crate::node::Node;
use crate::{node_next_mut, node_prev_mut};
use std::fmt;
use std::hash::Hash;

macro_rules! impl_cursor {
  ($name:ident<$a:lifetime, $k:ident, $n:ident, $m:ident>($list:ident, $key:ident)) => {
    impl<$a, $k, $n, $m> $name<$a, $k, $n, $m> {
      /// Checks if the cursor is currently pointing to the null pair.
      #[inline]
      pub fn is_null(&self) -> bool {
        self.$key.is_none()
      }

      /// Returns a reference to the key that the cursor is currently pointing to.
      ///
      /// Returns `None` if the cursor is currently pointing to the null pair.
      #[inline]
      pub fn key(&self) -> Option<&$k> {
        self.$key.as_ref()
      }

      /// Provides a reference to the front key of the cursor’s parent list,
      /// or `None` if the list is empty.
      #[inline]
      pub fn front_key(&self) -> Option<&$k> {
        self.$list.head.as_ref()
      }

      /// Provides a reference to the back key of the cursor’s parent list,
      /// or `None` if the list is empty.
      #[inline]
      pub fn back_key(&self) -> Option<&$k> {
        self.$list.tail.as_ref()
      }
    }

    impl<$a, $k, $n, $m> $name<$a, $k, $n, $m>
    where
      $k: Hash + Eq,
      $m: Map<K, N>,
    {
      /// Returns a reference to the node that the cursor is currently pointing to.
      ///
      /// Returns `None` if the cursor is currently pointing to the null pair.
      #[inline]
      pub fn node(&self) -> Option<&$n> {
        self.$key.as_ref().and_then(|k| self.$list.nodes.get(k))
      }

      /// Provides a reference to the front node of the cursor’s parent list,
      /// or `None` if the list is empty.
      #[inline]
      pub fn front_node(&self) -> Option<&$n> {
        self.front_key().and_then(|k| self.$list.nodes.get(k))
      }

      /// Provides a reference to the back node of the cursor’s parent list,
      /// or `None` if the list is empty.
      #[inline]
      pub fn back_node(&self) -> Option<&$n> {
        self.back_key().and_then(|k| self.$list.nodes.get(k))
      }
    }

    impl<$a, $k, $n, $m> $name<$a, $k, $n, $m>
    where
      $k: Hash + Eq,
      $n: Node<Key = $k>,
      $m: Map<K, N>,
    {
      /// Returns a reference to the next key.
      ///
      /// If the cursor is pointing to the null pair then this returns the first
      /// key of the [`KeyNodeList`]. If it is pointing to the last key of the
      /// [`KeyNodeList`] then this returns `None`.
      #[inline]
      pub fn next_key(&self) -> Option<&$k> {
        self.$key.as_ref().map_or_else(
          || self.$list.head.as_ref(),
          |k| self.$list.node(k).and_then(|n| n.next()),
        )
      }

      /// Returns a reference to the previous key.
      ///
      /// If the cursor is pointing to the null pair then this returns the last
      /// key of the [`KeyNodeList`]. If it is pointing to the first key of the
      /// [`KeyNodeList`] then this returns `None`.
      #[inline]
      pub fn prev_key(&self) -> Option<&$k> {
        self.$key.as_ref().map_or_else(
          || self.$list.tail.as_ref(),
          |k| self.$list.node(k).and_then(|n| n.prev()),
        )
      }

      /// Returns a reference to the next node.
      ///
      /// If the cursor is pointing to the null pair then this returns the first
      /// node of the [`KeyNodeList`]. If it is pointing to the last node of the
      /// [`KeyNodeList`] then this returns `None`.
      #[inline]
      pub fn next_node(&self) -> Option<&$n> {
        self.next_key().and_then(|k| self.$list.node(k))
      }

      /// Returns a reference to the previous node.
      ///
      /// If the cursor is pointing to the null pair then this returns the last
      /// node of the [`KeyNodeList`]. If it is pointing to the first node of the
      /// [`KeyNodeList`] then this returns `None`.
      #[inline]
      pub fn prev_node(&self) -> Option<&$n> {
        self.prev_key().and_then(|k| self.$list.node(k))
      }
    }

    impl<$a, $k, $n, $m> $name<$a, $k, $n, $m>
    where
      $k: Hash + Eq + Clone,
      $n: Node<Key = $k>,
      $m: Map<K, N>,
    {
      /// Moves the cursor to the next key-node pair of the [`KeyNodeList`].
      ///
      /// If the cursor is pointing to the null pair then this will move it to
      /// the first key-node pair of the [`KeyNodeList`]. If it is pointing to
      /// the last key-node pair of the [`KeyNodeList`] then this will move it
      /// to the null pair.
      #[inline]
      pub fn move_next(&mut self) {
        self.$key = self.$key.as_ref().map_or_else(
          || self.$list.head.clone(),
          |k| self.$list.node(k).and_then(|n| n.next().cloned()),
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
        self.$key = self.$key.as_ref().map_or_else(
          || self.$list.tail.clone(),
          |k| self.$list.node(k).and_then(|n| n.prev().cloned()),
        );
      }
    }

    impl<$a, $k, $n, $m> fmt::Debug for $name<$a, $k, $n, $m>
    where
      $k: Hash + Eq + fmt::Debug,
      $n: Node<Key = $k> + fmt::Debug,
      $m: Map<K, N>,
    {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple(stringify!($name))
          .field(self.$list)
          .field(&self.$key)
          .finish()
      }
    }
  };
}

/// A cursor over a [`KeyNodeList`].
#[derive(Clone)]
pub struct Cursor<'a, K, N, M> {
  pub(crate) list: &'a KeyNodeList<K, N, M>,
  pub(crate) key: Option<K>,
}

impl_cursor!(Cursor<'a, K, N, M>(list, key));

/// A cursor over a [`KeyNodeList`] with editing operations.
pub struct CursorMut<'a, K, N, M> {
  pub(crate) list: &'a mut KeyNodeList<K, N, M>,
  pub(crate) key: Option<K>,
}

impl_cursor!(CursorMut<'a, K, N, M>(list, key));

impl<'a, K, N, M> CursorMut<'a, K, N, M>
where
  K: Clone,
{
  /// Returns a read-only cursor pointing to the current pair.
  ///
  /// The lifetime of the returned [`Cursor`] is bound to that of the
  /// [`CursorMut`], which means it cannot outlive the [`CursorMut`] and that
  /// the [`CursorMut`] is frozen for the lifetime of the [`Cursor`].
  #[inline]
  pub fn as_cursor(&self) -> Cursor<K, N, M> {
    Cursor {
      list: self.list,
      key: self.key.clone(),
    }
  }
}

impl<'a, K, N, M> CursorMut<'a, K, N, M>
where
  K: Hash + Eq,
  M: Map<K, N>,
{
  /// Returns a mutable reference to the node that the cursor is currently
  /// pointing to.
  ///
  /// Returns `None` if the cursor is currently pointing to the null pair.
  #[inline]
  pub fn node_mut(&mut self) -> Option<&mut N> {
    self.key.as_ref().and_then(|k| self.list.nodes.get_mut(k))
  }

  /// Provides a mutable reference to the front node of the cursor’s parent
  /// list, or `None` if the list is empty.
  #[inline]
  pub fn front_node_mut(&mut self) -> Option<&mut N> {
    self
      .list
      .head
      .as_ref()
      .and_then(|k| self.list.nodes.get_mut(k))
  }

  /// Provides a mutable reference to the back node of the cursor’s parent
  /// list, or `None` if the list is empty.
  #[inline]
  pub fn back_node_mut(&mut self) -> Option<&mut N> {
    self
      .list
      .tail
      .as_ref()
      .and_then(|k| self.list.nodes.get_mut(k))
  }
}

impl<'a, K, N, M> CursorMut<'a, K, N, M>
where
  K: Hash + Eq + Clone,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  /// Inserts a new key-node pair into the [`KeyNodeList`] after the current one.
  ///
  /// If the cursor is pointing at the null pair then the new pair is inserted
  /// at the front of the [`KeyNodeList`].
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  pub fn insert_after<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    self.list.nodes.insert(key.clone(), node).map(|_| {
      // get the `next` pointer of the node pointed by the cursor
      let next = match &self.key {
        // cursor points to the key `k`
        // update the `next` pointer of the `k` node
        Some(k) => node_next_mut!(self.list, k).replace(key.clone()),
        // cursor points to the null pair
        // insert at front of the list, update the head pointer
        None => self.list.head.replace(key.clone()),
      };
      // update the next node at the insertion position
      match &next {
        // next node has key `k`, update its `prev` pointer
        Some(k) => *node_prev_mut!(self.list, k) = Some(key.clone()),
        // next node is the null pair, update the tail pointer
        None => self.list.tail = Some(key.clone()),
      }
      // update node's previous pointer and next pointer
      let node = self.list.node_mut(&key).unwrap();
      *node_prev_mut!(node) = self.key.clone();
      *node_next_mut!(node) = next;
    })
  }

  /// Inserts a new key-node pair into the [`KeyNodeList`] before the current one.
  ///
  /// If the cursor is pointing at the null pair then the new pair is inserted
  /// at the end of the [`KeyNodeList`].
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  pub fn insert_before<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    self.list.nodes.insert(key.clone(), node).map(|_| {
      // get the `prev` pointer of the node pointed by the cursor
      let prev = match &self.key {
        // cursor points to the key `k`
        // update the `prev` pointer of the `k` node
        Some(k) => node_prev_mut!(self.list, k).replace(key.clone()),
        // cursor points to the null pair
        // insert at end of the list, update the tail pointer
        None => self.list.tail.replace(key.clone()),
      };
      // update the previous node at the insertion position
      match &prev {
        // previous node has key `k`, update its `next` pointer
        Some(k) => *node_next_mut!(self.list, k) = Some(key.clone()),
        // previous node is the null pair, update the head pointer
        None => self.list.head = Some(key.clone()),
      }
      // update node's previous pointer and next pointer
      let node = self.list.node_mut(&key).unwrap();
      *node_prev_mut!(node) = prev;
      *node_next_mut!(node) = self.key.clone();
    })
  }

  /// Inserts a key into the [`KeyNodeList`] after the current one.
  ///
  /// If the cursor is pointing at the null pair then the key is inserted
  /// at the front of the [`KeyNodeList`].
  ///
  /// If `key` already exists, returns an error containing `key`.
  pub fn insert_key_after(&mut self, key: K) -> Result<(), K>
  where
    (): Into<N>,
  {
    self.insert_after(key, ()).map_err(|(k, _)| k)
  }

  /// Inserts a key into the [`KeyNodeList`] before the current one.
  ///
  /// If the cursor is pointing at the null pair then the key is inserted
  /// at the front of the [`KeyNodeList`].
  ///
  /// If `key` already exists, returns an error containing `key`.
  pub fn insert_key_before(&mut self, key: K) -> Result<(), K>
  where
    (): Into<N>,
  {
    self.insert_before(key, ()).map_err(|(k, _)| k)
  }

  /// Removes the current pair from the [`KeyNodeList`].
  ///
  /// The pair that was removed is returned, and the cursor is moved to point
  /// to the next pair in the [`KeyNodeList`].
  ///
  /// If the cursor is currently pointing to the null pair then no pair is
  /// removed and `None` is returned.
  #[inline]
  pub fn remove_current(&mut self) -> Option<(K, N)> {
    self.key.take().map(|k| {
      let pair = self.list.remove(&k).unwrap();
      self.key = pair.1.next().cloned();
      pair
    })
  }

  /// Appends an pair to the front of the cursor’s parent list. The pair that
  /// the cursor points to is unchanged, even if it is the null pair.
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn push_front<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    self.list.push_front(key, node)
  }

  /// Appends an pair to the back of the cursor’s parent list. The pair that
  /// the cursor points to is unchanged, even if it is the null pair.
  ///
  /// If `key` already exists, returns an error containing `key` and `node`.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn push_back<T: Into<N>>(&mut self, key: K, node: T) -> Result<(), (K, T)> {
    self.list.push_back(key, node)
  }

  /// Removes the first pair from the cursor’s parent list and returns it, or
  /// `None` if the list is empty. The pair the cursor points to remains
  /// unchanged, unless it was pointing to the front pair. In that case, it
  /// points to the new front pair.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn pop_front(&mut self) -> Option<(K, N)> {
    if self.list.head == self.key {
      self.move_next();
    }
    self.list.pop_front()
  }

  /// Removes the last pair from the cursor’s parent list and returns it, or
  /// `None` if the list is empty. The pair the cursor points to remains
  /// unchanged, unless it was pointing to the back pair. In that case, it
  /// points to the null pair.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  pub fn pop_back(&mut self) -> Option<(K, N)> {
    if self.list.tail == self.key {
      self.key = None;
    }
    self.list.pop_back()
  }
}
