pub trait Node {
  type Key;

  fn prev(&self) -> Option<&Self::Key>;
  fn next(&self) -> Option<&Self::Key>;
  fn prev_mut<T: NodeToken>(&mut self) -> &mut Option<Self::Key>;
  fn next_mut<T: NodeToken>(&mut self) -> &mut Option<Self::Key>;
}

#[macro_export]
macro_rules! impl_node {
  ($node:ident { Key = $k:ty, prev = $p:ident, next = $n:ident $(,)? }) => {
    impl $crate::linkedlist::Node for $node {
      type Key = $k;

      fn prev(&self) -> Option<&Self::Key> {
        self.$p.as_ref()
      }

      fn next(&self) -> Option<&Self::Key> {
        self.$n.as_ref()
      }

      fn prev_mut<T: NodeToken>(&mut self) -> &mut Option<Self::Key> {
        &mut self.$p
      }

      fn next_mut<T: NodeToken>(&mut self) -> &mut Option<Self::Key> {
        &mut self.$n
      }
    }
  };
}

pub trait NodeToken: private::Sealed {}

pub(crate) struct Token;
impl NodeToken for Token {}
impl private::Sealed for Token {}

mod private {
  pub trait Sealed {}
}
