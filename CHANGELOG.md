# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] – 2025-07-28

### Added

- Add `LineA*`, `LineB*`, `LineD*` iterators
- Add `head`, `pop_head` methods (for all iterators)
- Add `tail`, `pop_tail` methods (for all iterators except `LineB*`)
- Add `Clip` and `Viewport` clipping regions
- Add `Clip::line_*` and `Clip::line_*_proj` methods
- Add `Viewport::line_*` and `Viewport::line_*_proj` methods
- Property tests for all line segment types

### Changed

- `Clip` separated into `Clip` (zero-origin) and `Viewport` (any origin)

### Removed

- `SignedAxis`, `Axis`, `AnyAxis` (now `LineA*`)
- `Diagonal`, `AnyDiagonal` (now `LineD*`)
- `Octant`, `AnyOctant` (now `LineB*`)

## [0.3.0] – 2024-08-11

### Added

- Add `SignedAxis`, `Axis` and `AnyAxis` iterators (and aliases)
- Add `Diagonal` and `AnyDiagonal` iterators (and aliases)
- Add `Octant` and `AnyOctant` iterators (and aliases)
- Add `Clip` and a new clipping algorithm
- Add property tests for the clipping algorithm
- Add compile-time tests for `Iterator`-related impl checks

### Removed

- Remove the `clipline` function
- Remove the `Clipline` iterator and its variants
- Remove the `Constant` trait

## [0.2.0] – 2023-12-18

### Fixed

- Fix broken compilation due to private trait bound

## [0.1.3] – 2023-12-07 [YANKED]

### Added

- Implement `DoubleEndedIterator` for vertical and horizontal iterators
- Implement `FusedIterator` for all iterators
- Implement `ExactSizeIterator` for iterators over numeric types that fit into `usize`

### Changed

- Generify the library over the signed numeric types

## [0.1.2] – 2023-11-02

### Added

- Add benchmarks against `bresenham` and `line_drawing`

### Changed

- Inline internal functions

## [0.1.1] – 2023-10-31

### Added

- Add the `Clipline` iterator

### Changed

- Gate the `clipline` function behind `func`
- Gate the `Clipline` iterator behind `iter`

## [0.1.0] – 2023-10-25

### Added

- Add the `clipline` function

[0.4.0]: https://github.com/nxsaken/clipline/releases/tag/0.4.0
[0.3.0]: https://github.com/nxsaken/clipline/releases/tag/0.3.0
[0.2.0]: https://github.com/nxsaken/clipline/releases/tag/0.2.0
