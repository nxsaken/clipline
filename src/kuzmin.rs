//! ## Kuzmin clipping
//!
//! This module provides an implementation of [Kuzmin's algorithm][1]
//! for clipping line segments with integer coordinates to rectangular windows.
//!
//! [1]: https://doi.org/10.1111/1467-8659.1450275

use crate::Point;

/// Clips the starting point of a line segment against a rectangular window.
///
/// Returns the clipped point `(x1, y1)` and the initial `error` term.
#[allow(unused)]
pub const fn enter<const FY: bool, const FX: bool, const SWAP: bool>(
    (x1, y1): Point<isize>,
    (dx, dy): (isize, isize),
    (dx2, dy2): (isize, isize),
    (wx1, wy1): Point<isize>,
    (wx2, wy2): Point<isize>,
) -> Option<(Point<isize>, isize)> {
    todo!()
}

/// Clips the ending point of a line segment against a rectangular window.
///
/// Returns the clipped `end` coordinate along the axis of iteration
/// (`end_x` for gentle slopes, `end_y` for steep slopes).
#[allow(unused)]
pub const fn exit<const FY: bool, const FX: bool, const SWAP: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    (dx, dy): (isize, isize),
    (dx2, dy2): (isize, isize),
    (wx2, wy2): Point<isize>,
) -> isize {
    todo!()
}
