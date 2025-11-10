use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// An interface to the hash map operations used by
/// [`KeyNodeList`](crate::KeyNodeList).
///
/// Any data structure that implements this trait can be used as the
/// underlying hash map for [`KeyNodeList`](crate::KeyNodeList).
pub trait Map<K, V> {
  /// Returns the number of elements in the map.
  ///
  /// This operation should compute in *O*(1) time.
  fn len(&self) -> usize;

  /// Returns `true` if the map contains no elements.
  ///
  /// This operation should compute in *O*(1) time.
  #[inline]
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Clears the map, removing all key-value pairs.
  /// Keeps the allocated memory for reuse.
  fn clear(&mut self);

  /// Returns `true` if the map contains a value for the specified key.
  ///
  /// The key may be any borrowed form of the map’s key type, but [`Hash`]
  /// and [`Eq`] on the borrowed form must match those for the key type.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  fn contains_key<Q>(&self, k: &Q) -> bool
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
  {
    self.get(k).is_some()
  }

  /// Returns a reference to the value corresponding to the key.
  ///
  /// The key may be any borrowed form of the map’s key type, but [`Hash`]
  /// and [`Eq`] on the borrowed form must match those for the key type.
  ///
  /// This operation should compute in *O*(1) time on average.
  fn get<Q>(&self, k: &Q) -> Option<&V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq;

  /// Returns a mutable reference to the value corresponding to the key.
  ///
  /// The key may be any borrowed form of the map’s key type, but [`Hash`]
  /// and [`Eq`] on the borrowed form must match those for the key type.
  ///
  /// This operation should compute in *O*(1) time on average.
  fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq;

  /// Inserts a key-value pair into the map.
  ///
  /// If the map did have this key present, returns an error containing the
  /// key and the value.
  ///
  /// If the map did not have this key present, the key-value pair is
  /// inserted, and [`Ok(())`](Ok) is returned.
  ///
  /// This operation should compute in *O*(1) time on average.
  fn insert<T: Into<V>>(&mut self, k: K, v: T) -> Result<(), (K, T)>
  where
    K: Hash + Eq;

  /// Removes a key from the map, returning the value at the key if the key
  /// was previously in the map.
  ///
  /// The key may be any borrowed form of the map’s key type, but [`Hash`]
  /// and [`Eq`] on the borrowed form must match those for the key type.
  ///
  /// This operation should compute in *O*(1) time on average.
  #[inline]
  fn remove<Q>(&mut self, k: &Q) -> Option<V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
  {
    self.remove_entry(k).map(|(_, v)| v)
  }

  /// Removes a key from the map, returning the stored key and value if the
  /// key was previously in the map.
  ///
  /// The key may be any borrowed form of the map’s key type, but [`Hash`]
  /// and [`Eq`] on the borrowed form must match those for the key type.
  ///
  /// This operation should compute in *O*(1) time on average.
  fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq;
}

impl<K, V> Map<K, V> for HashMap<K, V> {
  #[inline]
  fn len(&self) -> usize {
    self.len()
  }

  #[inline]
  fn clear(&mut self) {
    self.clear()
  }

  #[inline]
  fn get<Q>(&self, k: &Q) -> Option<&V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
  {
    self.get(k)
  }

  #[inline]
  fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
  {
    self.get_mut(k)
  }

  #[inline]
  #[allow(clippy::map_entry)]
  fn insert<T: Into<V>>(&mut self, k: K, v: T) -> Result<(), (K, T)>
  where
    K: Hash + Eq,
  {
    if self.contains_key(&k) {
      Err((k, v))
    } else {
      self.insert(k, v.into());
      Ok(())
    }
  }

  #[inline]
  fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
  {
    self.remove_entry(k)
  }
}
