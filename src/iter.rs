use crate::list::KeyNodeList;
use crate::node::Node;
use std::hash::Hash;

/// An owning iterator over the key-node paris of a [`KeyNodeList`].
pub struct IntoIter<K, N> {
  pub(crate) list: KeyNodeList<K, N>,
}

impl<K, N> Iterator for IntoIter<K, N>
where
  K: Hash + Eq + Clone,
  N: Node<Key = K>,
{
  type Item = (K, N);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop_front()
  }
}

/// An iterator over the key-node pairs of a [`KeyNodeList`].
pub struct Iter<'a, K, N> {
  pub(crate) list: &'a KeyNodeList<K, N>,
  pub(crate) key: Option<&'a K>,
}

impl<'a, K, N> Iterator for Iter<'a, K, N>
where
  K: Hash + Eq,
  N: Node<Key = K>,
{
  type Item = (&'a K, &'a N);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.key.and_then(|k| {
      self.list.node(k).map(|n| {
        self.key = n.next();
        (k, n)
      })
    })
  }
}

/// An iterator over the keys of a [`KeyNodeList`].
pub struct Keys<'a, K, N> {
  pub(crate) iter: Iter<'a, K, N>,
}

impl<'a, K, N> Iterator for Keys<'a, K, N>
where
  K: Hash + Eq,
  N: Node<Key = K>,
{
  type Item = &'a K;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|(k, _)| k)
  }
}

/// An iterator over the nodes of a [`KeyNodeList`].
pub struct Nodes<'a, K, N> {
  pub(crate) iter: Iter<'a, K, N>,
}

impl<'a, K, N> Iterator for Nodes<'a, K, N>
where
  K: Hash + Eq,
  N: Node<Key = K>,
{
  type Item = &'a N;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|(_, n)| n)
  }
}
