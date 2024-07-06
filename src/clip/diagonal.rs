//! ## Diagonal clipping
//!
//! This module provides [clipping](Clip) utilities for
//! [diagonal](crate::Diagonal) directed line segments.

use crate::{Clip, Point};

/// Checks if the directed line segment trivially lies outside the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn out_of_bounds<const FX: bool, const FY: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    &Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: &Clip<isize>,
) -> bool {
    !FX && (x2 < wx1 || wx2 < x1)
        || FX && (x1 < wx1 || wx2 < x2)
        || !FY && (y2 < wy1 || wy2 < y1)
        || FY && (y1 < wy1 || wy2 < y2)
}

/// Clips the starting point of a diagonal directed line segment covered by
/// the given [quadrant](crate::DiagonalQuadrant) against a [rectangular region](Clip).
///
/// Returns the clipped point `(cx1, cy1)`, or [`None`]
/// if the line segment does not intersect the clipping region.
///
/// **Note**: this function assumes that the line segment was not trivially rejected.
#[must_use]
#[inline(always)]
pub const fn enter<const FX: bool, const FY: bool>(
    (x1, y1): Point<isize>,
    &Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: &Clip<isize>,
) -> Option<Point<isize>> {
    let (mut cx1, mut cy1) = (x1, y1);
    if !FX && x1 < wx1 || FX && wx2 < x1 {
        let diff = if !FX { wx1 - x1 } else { x1 - wx2 };
        match FY {
            false => cy1 += diff,
            true => cy1 -= diff,
        }
        if !FY && wy2 < cy1 || FY && cy1 < wy1 {
            return None;
        }
        if !FY && wy1 <= cy1 || FY && cy1 <= wy2 {
            cx1 = if !FX { wx1 } else { wx2 };
            return Some((cx1, cy1));
        }
    }
    if !FY && y1 < wy1 || FY && wy2 < y1 {
        let diff = if !FY { wy1 - y1 } else { y1 - wy2 };
        match FX {
            false => cx1 += diff,
            true => cx1 -= diff,
        }
        if !FX && wx2 < cx1 || FX && cx1 < wx1 {
            return None;
        }
        cy1 = if !FY { wy1 } else { wy2 };
    }
    Some((cx1, cy1))
}

/// Clips the ending point of a diagonal directed line segment covered by the given
/// [quadrant](crate::DiagonalQuadrant) against a [rectangular region](Clip).
///
/// Returns the clipped `cx2` coordinate.
///
/// **Note**: this function assumes that the line segment was not trivially rejected.
#[must_use]
#[inline(always)]
pub const fn exit<const FX: bool, const FY: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    &Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: &Clip<isize>,
) -> isize {
    let cx2 = if !FY && wy2 < y2 || FY && y2 < wy1 {
        let diff = if !FY { wy2 - y1 } else { y1 - wy1 };
        match FX {
            false => diff + x1,
            true => diff - x1,
        }
    } else {
        x2
    };
    match FX {
        false if wx2 < cx2 => wx2,
        true if cx2 < wx1 => wx1,
        _ => cx2,
    }
}
