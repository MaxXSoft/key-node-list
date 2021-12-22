# key-node-list

[<img alt="github" src="https://img.shields.io/badge/github-MaxXSoft/key--node--list-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MaxXSoft/key-node-list)
[<img alt="crates.io" src="https://img.shields.io/crates/v/key--node--list.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/key-node-list)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-key--node--list-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/key-node-list)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/MaxXSoft/key-node-list/Build%20and%20Test/master?style=for-the-badge" height="20">](https://github.com/MaxXSoft/key-node-list/actions?query=branch%3Amaster)

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
