# ✂️ clipline 📏

[![Rust](https://github.com/nxsaken/clipline/actions/workflows/rust.yml/badge.svg)](https://github.com/nxsaken/clipline/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/clipline.svg)](https://crates.io/crates/clipline)
[![docs.rs](https://img.shields.io/docsrs/clipline)](https://docs.rs/clipline/latest/clipline/)

`clipline` is a Rust crate for efficient scan conversion (rasterization) of line segments with [clipping](https://en.wikipedia.org/wiki/Line_clipping) to a rectangular window. It is an implementation of
[the 1995 paper by YP Kuzmin](https://doi.org/10.1111/1467-8659.1450275).

The key advantage of `clipline` over vanilla Bresenham is that it eliminates the need for bounds checking on every pixel, which speeds up line drawing. Furthermore, the clipping uses integer arithmetic, producing pixel-perfect endpoints. This sets it apart from floating-point clipping algorithms like [Cohen-Sutherland](https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm), which [may distort the line](https://www.virtualdub.org/blog2/entry_341.html) due to rounding errors.

![`clipline` in action](img/clip_anim.gif)

## Benchmarks

[Benchmarks are available here](BENCHMARKS.md). I used [criterion.rs](https://github.com/bheisler/criterion.rs) to compare `clipline` to two popular line drawing crates – `bresenham` and `line_drawing`, by "drawing" 256 lines of varying clipping window sizes and line orientations.

In practice, `bresenham` and `line_drawing` will require bounds checks when indexing into a frame buffer, hence the difference between the `draw_pixel_checked` and `draw_pixel_unchecked` functions.

## Installation

To use `clipline`, add it to your `Cargo.toml` file:

```toml
[dependencies]
clipline = "0.2.0"
```

## Usage

This crate provides two ways of performing scan conversion: the `clipline` function, and the
`Clipline` iterator. The former is slightly more optimized, the latter allows external iteration. Both methods can be toggled with the `func` and `iter` features (both enabled by default).

```rust
use clipline::{clipline, Clipline, Clipline::*};

let draw_pixel = |x, y| {
    // Your custom pixel logic
    // No bounds checks necessary here!
};

let line = ((0, 0), (10, 10));
let clip_rect = ((2, 2), (8, 8));

// A. Use the `clipline` function for slightly faster operations
// `(start, end)` represents the visible portion of the line.
let (start, end) = clipline(line, clip_rect, draw_pixel)
    .expect("line intersects clip_rect");

// B. Iterate over `Clipline` with indirection
// `Clipline::new` returns None if `line` is fully outside `clip_rect`.
for (x, y) in Clipline::new(line, clip_rect).unwrap() {
    draw_pixel(x, y);
}

// C. Iterate over each `Clipline` case directly
match Clipline::new(line, clip_rect).unwrap() {
    Vlipline(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
    Hlipline(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
    Gentleham(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
    Steepnham(pixels) => {
        for (x, y) in pixels {
            draw_pixel(x, y);
        }
    }
}
```