//! ## Kuzmin clipping (diagonal)
//!
//! This module provides a simplified implementation of [Kuzmin's algorithm][1] for clipping
//! diagonal directed line segments with integer coordinates to [rectangular regions](Region).
//!
//! [1]: https://doi.org/10.1111/1467-8659.1450275

use crate::{Point, Region};

/// Clips a diagonal directed line segment
/// covered by the given [quadrant](super::Quadrant) against a [rectangular region](Region).
///
/// Returns the clipped point `(cx1, cy1)` and the `end` coordinate along the iteration axis,
/// or [`None`] if the line segment does not intersect the clipping region.
#[must_use]
#[inline(always)]
pub const fn clip<const FX: bool, const FY: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    dx: isize,  // absolute value
    dx2: isize, // absolute value
    Region { wx1, wy1, wx2, wy2 }: Region<isize>,
) -> Option<(Point<isize>, isize)> {
    let Some((cx1, cy1)) = enter::<FX, FY>((x1, y1), dx, dx2, (wx1, wy1), (wx2, wy2)) else {
        return None;
    };
    let end = exit::<FX, FY>((x1, y1), (x2, y2), dx, dx2, (wx1, wy1), (wx2, wy2));
    Some(((cx1, cy1), end))
}

/// Clips the starting point of a diagonal directed line segment
/// covered by the given [quadrant](super::Quadrant) against a [rectangular region](Region).
///
/// Returns the clipped point `(cx1, cy1)`, or [`None`]
/// if the line segment does not intersect the clipping region.
#[allow(clippy::cognitive_complexity)]
#[must_use]
#[inline(always)]
const fn enter<const FX: bool, const FY: bool>(
    (x1, y1): Point<isize>,
    dx: isize,  // absolute value
    dx2: isize, // absolute value
    (wx1, wy1): Point<isize>,
    (wx2, wy2): Point<isize>,
) -> Option<Point<isize>> {
    let (mut cx1, mut cy1) = (x1, y1);
    if !FX && x1 < wx1 || FX && wx2 < x1 {
        let tmp = (if !FX { wx1 - x1 } else { x1 - wx2 } * 2 - 1) * dx;
        let msd = tmp / dx2;
        match FY {
            false => cy1 += msd,
            true => cy1 -= msd,
        }
        if !FY && wy2 < cy1 || FY && cy1 < wy1 {
            return None;
        }
        if !FY && wy1 <= cy1 || FY && cy1 <= wy2 {
            let rem = tmp - msd * dx2;
            cx1 = if !FX { wx1 } else { wx2 };
            if 0 < rem {
                cy1 += if !FY { 1 } else { -1 };
            }
            return Some((cx1, cy1));
        }
    }
    if !FY && y1 < wy1 || FY && wy2 < y1 {
        let tmp = (if !FY { wy1 - y1 } else { y1 - wy2 }) * dx2;
        match FX {
            false => cx1 += tmp / dx2,
            true => cx1 -= tmp / dx2,
        }
        let rem = tmp % dx2;
        if !FX && (wx2 < cx1 || cx1 == wx2 && dx <= rem)
            || FX && (cx1 < wx1 || cx1 == wx1 && dx <= rem)
        {
            return None;
        }
        cy1 = if !FY { wy1 } else { wy2 };
        if dx <= rem {
            cx1 += if !FX { 1 } else { -1 };
        }
    }
    Some((cx1, cy1))
}

/// Clips the ending point of a diagonal directed line segment
/// covered by the given [quadrant](super::Quadrant) against a [rectangular region](Region).
///
/// Returns the clipped `end` coordinate along the axis of iteration.
#[must_use]
#[inline(always)]
const fn exit<const FX: bool, const FY: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    dx: isize,  // absolute value
    dx2: isize, // absolute value
    (wx1, wy1): Point<isize>,
    (wx2, wy2): Point<isize>,
) -> isize {
    let mut end = x2;
    if !FY && wy2 < y2 || FY && y2 < wy1 {
        let tmp = dx2 * if !FY { wy2 - y1 } else { y1 - wy1 } + dx;
        let msd = tmp / dx2;
        end = if !FX { msd + x1 } else { msd - x1 };
        if tmp - msd * dx2 == 0 {
            end -= 1;
        }
    }
    if !FX && wx2 < end {
        wx2
    } else if FX && end < wx1 {
        wx1
    } else {
        end
    }
}
