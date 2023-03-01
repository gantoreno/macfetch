# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2023-03-01

### Changed

- The GPU segment finally has a non-shell spawning implementation (huge thanks to [@k-ronny](https://github.com/k-ronny)'s suggestion on #1) and started using the [iron-oxide](https://docs.rs/iron-oxide/latest/iron_oxide/) crate.

## [2.0.0] - 2023-02-24

### Changed

- Everything, all project files & functions have been ported out to [Rust](https://www.rust-lang.org/).

## [1.1.1] - 2023-02-19

### Changed

- Changed cache implementations with new utils (`get_cache`, `set_cache`, `clear_cache`).

## [1.1.0] - 2023-02-18

### Added

- Add `wm`segment.

### Updated

- Use mutex locks for shell commands.

## [1.0.2] - 2023-01-03

### Updated

- Update `packages`command.

## [1.0.0] - 2022-05-15

### Added

- Initial release.
