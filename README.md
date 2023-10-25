# ✂️ clipline 📏

[![Rust](https://github.com/nxsaken/clipline/actions/workflows/rust.yml/badge.svg)](https://github.com/nxsaken/clipline/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/clipline.svg)](https://crates.io/crates/clipline)
[![docs.rs](https://img.shields.io/docsrs/clipline)](https://docs.rs/clipline/latest/clipline/)

`clipline` is a Rust crate for efficient scan conversion of a line 
segment with [clipping](https://en.wikipedia.org/wiki/Line_clipping) to a rectangular window. It is an implementation of 
[the 1995 paper by YP Kuzmin](https://doi.org/10.1111/1467-8659.1450275). 

The key advantage of this algorithm over vanilla Bresenham is that it 
eliminates the need for bounds checking on every pixel, which speeds up line drawing. 
Furthermore, the clipping is done via integer arithmetic, producing accurate 
clipped endpoints. This sets it apart from floating-point clipping algorithms 
like [Cohen-Sutherland](https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm), which [may distort the line](https://www.virtualdub.org/blog2/entry_341.html) due to rounding errors.

![`clipline` in action](img/clip_anim.gif)

## Installation

To use `clipline`, add it to your `Cargo.toml` file:

```toml
[dependencies]
clipline = "0.1.0"
```

## Usage

```rust
use clipline::clipline;

fn draw_pixel(x: isize, y: isize) {
    // Your custom pixel drawing logic
    // No bounds checks necessary here
}

let line = ((0, 0), (10, 10));
let clip_rect = ((2, 2), (8, 8));

let (start, end) = clipline(line, clip_rect, draw_pixel)
    .expect("line intersects the clipping rectangle");

// Perform any additional logic with the clipped line segment.
// `start` and `end` represent the visible portion of the line.
```

In this example, `clipline` takes a line defined by two endpoints, `line`, 
and corners of a clipping rectangle, `clip_rect`. The closure `draw_pixel` 
handles each pixel within the clipped region, and the visible portion of the line is represented by the `start` and `end` variables.
