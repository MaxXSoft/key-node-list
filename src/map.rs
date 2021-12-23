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
  fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
  {
    self.get(k).is_some()
  }

  /// Returns a reference to the value corresponding to the key.
  ///
  /// The key may be any borrowed form of the map’s key type, but [`Hash`]
  /// and [`Eq`] on the borrowed form must match those for the key type.
  ///
  /// This operation should compute in *O*(1) time on average.
  fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq;

  /// Returns a mutable reference to the value corresponding to the key.
  ///
  /// The key may be any borrowed form of the map’s key type, but [`Hash`]
  /// and [`Eq`] on the borrowed form must match those for the key type.
  ///
  /// This operation should compute in *O*(1) time on average.
  fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq;

  /// Inserts a key-value pair into the map.
  ///
  /// If the map did not have this key present, [`None`] is returned.
  ///
  /// If the map did have this key present, the value is updated, and the
  /// old value is returned. The key is not updated, though; this matters
  /// for types that can be `==` without being identical.
  ///
  /// This operation should compute in *O*(1) time on average.
  fn insert(&mut self, k: K, v: V)
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
  fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
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
  fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq;
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
  fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
  {
    self.get(k)
  }

  #[inline]
  fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
  {
    self.get_mut(k)
  }

  #[inline]
  fn insert(&mut self, k: K, v: V)
  where
    K: Hash + Eq,
  {
    self.insert(k, v);
  }

  #[inline]
  fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
  where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
  {
    self.remove_entry(k)
  }
}
