//! TODO
//!
//! * This doc.
//! * Examples.
//! * More trait impls.

mod cursor;
mod iter;
mod list;
mod node;

pub use cursor::*;
pub use iter::*;
pub use list::*;
pub use node::*;

/// Gets a mutable reference of the previous pointer of the specific node.
macro_rules! node_prev_mut {
  ($list:expr, $key:expr) => {
    $list
      .node_mut::<K>($key)
      .unwrap()
      .prev_mut::<$crate::node::Token>()
  };
}
pub(crate) use node_prev_mut;

/// Gets a mutable reference of the next pointer of the specific node.
macro_rules! node_next_mut {
  ($list:expr, $key:expr) => {
    $list
      .node_mut::<K>($key)
      .unwrap()
      .next_mut::<$crate::node::Token>()
  };
}
pub(crate) use node_next_mut;
