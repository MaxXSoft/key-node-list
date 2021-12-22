use crate::list::KeyNodeList;
use crate::map::Map;
use crate::node::Node;
use std::hash::Hash;

/// An owning iterator over the key-node paris of a [`KeyNodeList`].
pub struct IntoIter<K, N, M> {
  pub(crate) list: KeyNodeList<K, N, M>,
}

impl<K, N, M> Iterator for IntoIter<K, N, M>
where
  K: Hash + Eq + Clone,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  type Item = (K, N);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop_front()
  }
}

/// An iterator over the key-node pairs of a [`KeyNodeList`].
pub struct Iter<'a, K, N, M> {
  pub(crate) list: &'a KeyNodeList<K, N, M>,
  pub(crate) key: Option<&'a K>,
}

impl<'a, K, N, M> Iterator for Iter<'a, K, N, M>
where
  K: Hash + Eq,
  N: Node<Key = K>,
  M: Map<K, N>,
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
pub struct Keys<'a, K, N, M> {
  pub(crate) iter: Iter<'a, K, N, M>,
}

impl<'a, K, N, M> Iterator for Keys<'a, K, N, M>
where
  K: Hash + Eq,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  type Item = &'a K;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|(k, _)| k)
  }
}

/// An iterator over the nodes of a [`KeyNodeList`].
pub struct Nodes<'a, K, N, M> {
  pub(crate) iter: Iter<'a, K, N, M>,
}

impl<'a, K, N, M> Iterator for Nodes<'a, K, N, M>
where
  K: Hash + Eq,
  N: Node<Key = K>,
  M: Map<K, N>,
{
  type Item = &'a N;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|(_, n)| n)
  }
}
