# clipline

[![CI](https://github.com/nxsaken/clipline/actions/workflows/ci.yml/badge.svg)](https://github.com/nxsaken/clipline/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/clipline.svg)](https://crates.io/crates/clipline)
[![docs.rs](https://img.shields.io/docsrs/clipline)](https://docs.rs/clipline/latest/clipline)
[![downloads](https://img.shields.io/crates/d/clipline.svg)](https://crates.io/crates/clipline)

This crate provides iterators over the rasterized points of directed, half-open line segments with pixel-perfect [clipping][clip] to rectangular regions.

See the [documentation](https://docs.rs/clipline/latest/clipline) for details,
or the example below.

## Features

- Supports unsigned and signed coordinates of most sizes.
  - Defines the iterators on the entire domains of the underlying numeric types.
  - Avoids integer overflow without overhead.
- Guarantees that clipped segments match the unclipped versions of themselves.
- Usable in `const` contexts and `#![no_std]` environments.

![`clipline` in action](img/clip.gif)

## Usage

```rust
use clipline::*;

/// Width of the pixel buffer.
const WIDTH: u16 = 256;
/// Height of the pixel buffer.
const HEIGHT: u16 = 240;

/// A function that operates on a single pixel in a pixel buffer.
///
/// # Safety
///
/// `i < WIDTH`, `j < HEIGHT`.
unsafe fn draw(pixels: &mut [bool], i: u16, j: u16, pixel: bool) {
    let index = j as usize * WIDTH as usize + i as usize;
    debug_assert!(index < pixels.len());
    unsafe { *pixels.get_unchecked_mut(index) = pixel; }
}

fn main() {
    let mut pixels = [false; WIDTH as usize * HEIGHT as usize];
  
    // This defines a clipping region (0, 0, WIDTH-1, HEIGHT-1),
    // which covers all valid indices of the pixel buffer.
    let clip = Clip::<i16>::from_size(WIDTH, HEIGHT).unwrap();
  
    // For Clip, *_proj involves a simple cast from i16 to u16.
    clip.line_b_proj(-32, -64, 320, 256)
        // This will panic if the line segment is completely outside the region.
        .unwrap()
        // This iterates over all points inside the region, relative to that region.
        // Effectively this allows to safely index into the underlying buffer.
        .for_each(|(i, j)| {
            // SAFETY: i < WIDTH, j < HEIGHT.
            unsafe { draw(&mut pixels, i, j, true) }
        });
  
    // This is how you can construct an unclipped line segment iterator.
    LineD::<u16>::new(1, 2, 31, 32)
        // This will panic if the segment is not diagonal.
        .unwrap()
        // By construction, all points of this line segment lie inside the region, thus
        // clipping can be skipped. Do this if you are sure your line segments are inside.
        .for_each(|(i, j)| {
            // SAFETY: i < WIDTH, j < HEIGHT.
            unsafe { draw(&mut pixels, i, j, true) }
        });
  
    // (-32, 16) -> (64, 16)
    LineAx::<i16>::new(16, -32, 64)
        // This is a naive pointwise clip-projection.
        // It's much slower than clipping the segment as a whole.
        .filter_map(|(x, y)| clip.point_proj(x, y))
        // But it gets the job done.
        .for_each(|(i, j)| {
            // SAFETY: i < WIDTH, j < HEIGHT.
            unsafe { draw(&mut pixels, i, j, true) }
        });
  
    // This defines a clipping region (16, 32, 16 + WIDTH - 1, 32 + HEIGHT - 1),
    // which covers all valid indices of the pixel buffer *after projection*.
    let clip = Viewport::<i16>::from_min_size(16, 32, WIDTH, HEIGHT).unwrap();
  
    // For Viewport, *_proj involves subtracting the minimum corner of the Viewport
    // from all the clipped coordinates.
    clip.line_b_proj(-16, -32, 336, 288)
        // This is equivalent to the first example (we just shifted the original line segment).
        .unwrap()
        // This iterates over all points inside the region, relative to that region.
        // Effectively this allows to safely index into the underlying buffer.
        .for_each(|(i, j)| {
            // SAFETY: i < WIDTH, j < HEIGHT.
            unsafe { draw(&mut pixels, i, j, true) }
        });
  
    fn do_at_world_pos(x: i16, y: i16) {
        println!("doing something at world position {x}, {y}")
    }
  
    // Both Clip and Viewport support clipping without projection.
    // This could be useful if you want to iterate over a line segment
    // in "world-space" (represented by signed or unsigned coordinates),
    // restricted to a region. It's not safe to use this to index into a grid.
    clip.line_b(-16, -32, 336, 288)
        .unwrap()
        .for_each(|(x, y)| do_at_world_pos(x, y));
  
    // Unsigned Clips have infallible from_max constructors
    // and do not provide *_proj methods (no need).
    let mut line = Clip::<u16>::from_max(WIDTH - 1, HEIGHT - 1)
        .line_b(1, 2, 320, 256)
        .unwrap();
  
    // custom iteration APIs are available in const contexts
    while let Some((i, j)) = line.pop_head() {
        // SAFETY: i < WIDTH, j < HEIGHT.
        unsafe { draw(&mut pixels, i, j, true) }
    }
}
```

## References

`clipline` synthesizes the algorithms from the following papers:

* [A fast two-dimensional line clipping algorithm via line encoding][spy],
  Mark S. Sobkow, Paul Pospisil, Yee-Hong Yang, 1987.
* [A new approach to parametric line clipping][dorr],
  Michael Dörr, 1990.
* [Bresenham's Line Generation Algorithm with Built-in Clipping][kuzmin],
  Yevgeny P. Kuzmin, 1995.

[clip]: https://en.wikipedia.org/wiki/Line_clipping
[bres]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
[spy]: https://doi.org/10.1016/0097-8493(87)90061-6
[dorr]: https://doi.org/10.1016/0097-8493(90)90067-8
[kuzmin]: https://doi.org/10.1111/1467-8659.1450275