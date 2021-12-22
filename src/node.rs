/// Trait for nodes that holds its previous and next key in [`KeyNodeList`].
pub trait Node {
  /// Key type of the current `Node`.
  type Key;

  /// Returns a reference to the previous key of the current node,
  /// or returns `None` if the current node is the first node in the list.
  fn prev(&self) -> Option<&Self::Key>;

  /// Returns a reference to the next key of the current node,
  /// or returns `None` if the current node is the last node in the list.
  fn next(&self) -> Option<&Self::Key>;

  /// Returns a mutable reference to the previous key of the current node
  /// so that [`KeyNodeList`] can update the order of the nodes.
  fn prev_mut<T: NodeToken>(&mut self) -> &mut Option<Self::Key>;

  /// Returns a mutable reference to the next key of the current node
  /// so that [`KeyNodeList`] can update the order of the nodes.
  fn next_mut<T: NodeToken>(&mut self) -> &mut Option<Self::Key>;
}

/// Implements [`Node`] trait for the specific structure.
#[macro_export]
macro_rules! impl_node {
  (
    $node:ident$(<$($g:ident),* $(,)?>)?
    { Key = $key:ty, prev = $prev:ident, next = $next:ident $(,)? }
  ) => {
    impl$(<$($g),*>)? $crate::Node for $node$(<$($g),*>)? {
      type Key = $key;

      #[inline]
      fn prev(&self) -> Option<&Self::Key> {
        self.$prev.as_ref()
      }

      #[inline]
      fn next(&self) -> Option<&Self::Key> {
        self.$next.as_ref()
      }

      #[inline]
      fn prev_mut<__: NodeToken>(&mut self) -> &mut Option<Self::Key> {
        &mut self.$prev
      }

      #[inline]
      fn next_mut<__: NodeToken>(&mut self) -> &mut Option<Self::Key> {
        &mut self.$next
      }
    }
  };
}

/// A generic node for the [`KeyNodeList`].
///
/// `GenericNode` can hold any kind of value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenericNode<K, V> {
  value: V,
  prev: Option<K>,
  next: Option<K>,
}

impl_node!(GenericNode<K, V> { Key = K, prev = prev, next = next});

impl<K, V> GenericNode<K, V> {
  /// Creates a new node with `value`.
  pub fn new(value: V) -> Self {
    Self {
      value,
      prev: None,
      next: None,
    }
  }

  /// Consumes this [`GenericNode`], returning the underlying value.
  pub fn into_value(self) -> V {
    self.value
  }

  /// Returns a reference to the node value.
  pub fn value(&self) -> &V {
    &self.value
  }

  /// Returns a mutable reference to the node value.
  pub fn value_mut(&mut self) -> &mut V {
    &mut self.value
  }
}

impl<K, V> Default for GenericNode<K, V>
where
  V: Default,
{
  fn default() -> Self {
    Self::new(V::default())
  }
}

impl<K, V> From<V> for GenericNode<K, V> {
  fn from(value: V) -> Self {
    Self::new(value)
  }
}

/// Token that used to update the keys in the `Node`.
///
/// Only the `key_node_list` crate holds the actual token.
pub trait NodeToken: private::Sealed {}

pub(crate) struct Token;
impl NodeToken for Token {}
impl private::Sealed for Token {}

mod private {
  pub trait Sealed {}
}
