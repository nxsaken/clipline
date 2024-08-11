# clipline

[![CI](https://github.com/nxsaken/clipline/actions/workflows/rust.yml/badge.svg)](https://github.com/nxsaken/clipline/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/clipline.svg)](https://crates.io/crates/clipline)
[![docs.rs](https://img.shields.io/docsrs/clipline)](https://docs.rs/clipline/latest/clipline/)
[![downloads](https://img.shields.io/crates/d/clipline.svg)](https://crates.io/crates/clipline)

Efficient rasterization of line segments with pixel-perfect [clipping][clip].

## Overview
  
- Provides iterators for clipped and unclipped rasterized line segments.
  - Eliminates bounds checking: clipped line segments are guaranteed to be within the region.
  - Guarantees clipped line segments match the unclipped versions of themselves.
- Supports signed and unsigned integer coordinates of most sizes.
  - Uses integer arithmetic only.
  - Prevents overflow and division by zero, forbids `clippy::arithmetic_side_effects`.
  - Defines the iterators on the entire domains of the underlying numeric types.
- Usable in `const` contexts and `#![no_std]` environments.

![`clipline` in action](img/clip.gif)

## Usage

Add `clipline` to `Cargo.toml`:

```toml
[dependencies]
clipline = "0.3.0"
```

### Feature flags

- `octant_64`
  * Enables `Octant` and `AnyOctant` over `i64`/`u64` for all targets, and over `isize`/`usize` for 64-bit targets.
  * Use this only if you need the full 64-bit range, as `Octant` will use `u128` and `i128` for some calculations.
- `try_fold`, `is_empty` *(nightly-only)*
  * Enable optimized `Iterator::try_fold` and `ExactSizeIterator::is_empty` implementations.

### Example

```rust
use clipline::{AnyOctant, Clip, Diagonal0, Point};

/// Width of the pixel buffer.
const WIDTH: usize = 64;
/// Height of the pixel buffer.
const HEIGHT: usize = 48;

/// Pixel color value.
const RGBA: u32 = 0xFFFFFFFF;

/// A function that operates on a single pixel in a pixel buffer.
///
/// ## Safety
/// `(x, y)` must be inside the `buffer`.
unsafe fn draw(buffer: &mut [u32], (x, y): Point<i8>, rgba: u32) {
    let index = y as usize * WIDTH + x as usize;
    debug_assert!(index < buffer.len());
    *buffer.get_unchecked_mut(index) = rgba;
}

fn main() {
    let mut buffer = [0_u32; WIDTH * HEIGHT];

    // The clipping region is closed/inclusive, thus 1 needs to be subtracted from the size.
    let clip = Clip::<i8>::new((0, 0), (WIDTH as i8 - 1, HEIGHT as i8 - 1)).unwrap();

    // `Clip` has convenience methods for the general iterators.
    clip.any_octant((-128, -100), (100, 80))
        // None if the line segment is completely invisible.
        // You might want to handle that case differently.
        .unwrap()
        // clipped to [(0, 1), ..., (58, 47)]
        .for_each(|xy| {
            // SAFETY: (x, y) has been clipped to the buffer.
            unsafe { draw(&mut buffer, xy, RGBA) }
        });

    // Alternatively, use the iterator constructors.
    AnyOctant::<i8>::clip((12, 0), (87, 23), &clip)
        .into_iter()
        .flatten()
        // clipped to [(12, 0), ..., (63, 16)]
        .for_each(|xy| {
            // SAFETY: (x, y) has been clipped to the buffer.
            unsafe { draw(&mut buffer, xy, RGBA) }
        });

    // Horizontal and vertical line segments.
    clip.axis_0(32, 76, -23)
        .unwrap()
        // clipped to [(63, 32), ..., (0, 32)]
        .for_each(|xy| {
            // SAFETY: (x, y) has been clipped to the buffer.
            unsafe { draw(&mut buffer, xy, RGBA) }
        });

    clip.axis_1(32, -23, 76)
        .unwrap()
        // clipped to [(32, 0), ..., (32, 47)]
        .for_each(|xy| {
            // SAFETY: (x, y) has been clipped to the buffer.
            unsafe { draw(&mut buffer, xy, RGBA) }
        });

    // Unclipped iterators are also available.
    // (-2, -2) -> (12, 12) is covered by Diagonal0, we can construct it directly.
    Diagonal0::<i8>::new((-2, -2), (12, 12))
        .unwrap()
        // Need to check every pixel to avoid going out of bounds.
        .filter(|&xy| clip.point(xy))
        .for_each(|xy| {
            // SAFETY: (x, y) is inside the buffer.
            unsafe { draw(&mut buffer, xy, RGBA) }
        });
}
```

## Limitations

* To support usage in `const` contexts, types must have an inherent implementation for every supported numeric type instead of relying on a trait. This and Rust's lack of support for function overloading means that the numeric type parameter must always be specified.
* Currently, only half-open line segments can be iterated. This allows `ExactSizeIterator` to be implemented for all types. Inclusive iterators are tracked in [#1](https://github.com/nxsaken/clipline/issues/1).

## Benchmarks

- [`divan`](https://crates.io/crates/divan) is used to [benchmark](benches/comparison.rs) different versions of `clipline`, as well as [`line_drawing`](https://crates.io/crates/line_drawing). Use `cargo bench` to run the benchmarks.
- In practice, with unclipped line segments, bounds checks are required when indexing into a grid, hence the difference between the `draw_pixel_checked` and `draw_pixel_unchecked` functions.

## References

`clipline` is inspired by the following papers:

* [A fast two-dimensional line clipping algorithm via line encoding][spy], Mark S. Sobkow, Paul Pospisil, Yee-Hong Yang, 1987.
* [A new approach to parametric line clipping][dorr], Michael Dörr, 1990.
* [Bresenham's Line Generation Algorithm with Built-in Clipping][kuzmin], Yevgeny P. Kuzmin, 1995.

[clip]: https://en.wikipedia.org/wiki/Line_clipping
[bres]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
[spy]: https://doi.org/10.1016/0097-8493(87)90061-6
[dorr]: https://doi.org/10.1016/0097-8493(90)90067-8
[kuzmin]: https://doi.org/10.1111/1467-8659.1450275