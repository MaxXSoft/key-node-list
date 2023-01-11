# Changelog

All notable changes to the Koopa will be documented in this file.

## Unreleased

### Fixed

* Some deprecated and non-recommended uses in the source code.

## 0.0.4 - 2022-01-09

### Added

* Implemented `Clone` trait for all list iterators.

## 0.0.3 - 2022-01-05

### Added

* Implemented `Extend<&K>`, `Extend<K>`, `FromIterator<K>` trait for `KeyNodeList`.

## 0.0.2 - 2021-12-24

### Added

* More provided methods to `Map` trait.
* Method `KeyNodeList::push_key_front` and `KeyNodeList::push_key_back`.
* Method `CursorMut::insert_key_after` and `CursorMut::insert_key_before`.

### Changed

* Signature of trait method `Map::insert`.
* Relexed generic type bound of `KeyNodeList::new`.

### Fixed

* Problem about generic type bound of `Index` trait of `KeyNodeList`.

## 0.0.1 - 2021-12-22
