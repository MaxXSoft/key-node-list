# key-node-list

Doubly-linked list that stores key-node pairs.

`KeyNodeList` is a doubly-linked list, it uses a hash map to maintain correspondence between keys and nodes, and records the previous key and the next key of the current node in the node itself. There is no pointer operations during this process, so `key_node_list` is all implemented in safe Rust.

You can complete key lookups, key-node pair updates, key-node pair deletions and other operations of `KeyNodeList` in *O*(1)~ time. You can also use cursor-based interface to traverse or edit the linked list.

## Building from Source

Make sure the Rust toolchain is installed on your computer, and then run:

```sh
git clone --recursive https://github.com/MaxXSoft/key-node-list.git
cd key-node-list
cargo build --release
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Copyright (C) 2010-2021 MaxXing. License GPLv3.
