# key-node-list

Doubly-linked list that stores key-node pairs.

`KeyNodeList` is a doubly-linked list, it uses a hash map to maintain correspondence between keys and nodes, and records the previous key and the next key of the current node in the node itself. There is no pointer operations during this process, so `key_node_list` is all implemented in safe Rust.

You can complete key lookups, key-node pair updates, key-node pair deletions and other operations of `KeyNodeList` in *O*(1)~ time. You can also use cursor-based interface to traverse or edit the linked list.

## Usage

[`key_node_list` is available on crates.io](https://crates.io/crates/key-node-list), so:

```toml
[dependencies]
key-node-list = "0.0.1"
```

## Example

```rust
use key_node_list::KeyValueList;

// construct key-value list from tuple array
let mut list = KeyValueList::from([(1, "Reimu"), (2, "Marisa")]);

// or pushing other key-value pairs to the front/back of list
list.push_front(0, "Alice").unwrap();
list.push_back(3, "Patchouli").unwrap();

// query nodes by key
assert_eq!(list[&1].value(), &"Reimu");
assert_eq!(list[&0].value(), &"Alice");

// also you can update nodes by key
*list.node_mut(&3).unwrap().value_mut() = "Youmu";
*list.front_node_mut().unwrap().value_mut() = "Mokou";
assert_eq!(list[&3].value(), &"Youmu");
assert_eq!(list[&0].value(), &"Mokou");
assert_eq!(list.front_node().unwrap().value(), &"Mokou");

// remove some key-node pairs
assert!(list.pop_front().is_some());
assert!(list.remove(&2).is_some());

// all key-node pairs are in order
list.push_back(5, "Yuyuko");
let vec: Vec<_> = list.into_iter().map(|(k, n)| (k, n.into_value())).collect();
assert_eq!(vec, [(1, "Reimu"), (3, "Youmu"), (5, "Yuyuko")]);
```

For more details, visit [`key_node_list` on docs.rs](https://docs.rs/key-node-list).

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Copyright (C) 2010-2021 MaxXing. License GPLv3.
