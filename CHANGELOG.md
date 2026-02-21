# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) and this
project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

### Changed

- Fix the metadata converter for the Python bindings.

## [0.1.2] - 2026-02-19

### Changed

- Fix the Python signature file.

## [0.1.1] - 2026-01-11

### Changed

- [#12](https://github.com/scott-wilson/openpathresolver/issues/12) Fix documentation and added examples.
- [#14](https://github.com/scott-wilson/openpathresolver/issues/14) Fix the `get_workspace`/`create_workspace` path generation so that path parts from an item are not included if later placeholders cannot be resolved.
- [#14](https://github.com/scott-wilson/openpathresolver/issues/14) Fix the deferred logic to treat all paths as deferred unless explicitly set to not.
- [#14](https://github.com/scott-wilson/openpathresolver/issues/14) Fix the `get_workspace`/`create_workspace` logic so that paths will properly inherit the metadata from their ancestors.

## [0.1.0] - 2026-01-09

### Added

- Initial release

[0.1.2]: https://github.com/scott-wilson/openpathresolver/compare/v0.1.1...v0.1.2

[0.1.1]: https://github.com/scott-wilson/openpathresolver/compare/v0.1.0...v0.1.1

[0.1.0]: https://github.com/scott-wilson/openpathresolver/releases/tag/v0.1.0
