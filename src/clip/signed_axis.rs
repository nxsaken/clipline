//! ### Signed axis clipping
//!
//! This module provides [clipping](Clip) utilities for
//! [signed-axis-aligned](SignedAxisAligned) directed line segments.

use crate::Clip;

/// Checks if the [axis-aligned](crate::AxisAligned)
/// directed line segment lies outside the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn out_of_bounds<const VERT: bool, const FLIP: bool>(
    u: i8,
    v1: i8,
    v2: i8,
    w: Clip<i8>,
) -> bool {
    !VERT
        && ((u < w.y1 || w.y2 <= u)
            || (!FLIP && (v2 < w.x1 || w.x2 <= v1) || FLIP && (v1 < w.x1 || w.x2 <= v2)))
        || VERT
            && ((u < w.x1 || w.x2 <= u)
                || (!FLIP && (v2 < w.y1 || w.y2 <= v1) || FLIP && (v1 < w.y1 || w.y2 <= v2)))
}

/// Clips the starting point of the [axis-aligned](crate::AxisAligned)
/// directed line segment to the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn enter<const VERT: bool, const FLIP: bool>(v1: i8, w: Clip<i8>) -> i8 {
    match (VERT, FLIP) {
        (false, false) if v1 < w.x1 => w.x1,
        (false, true) if w.x2 < v1 => w.x2,
        (true, false) if v1 < w.y1 => w.y1,
        (true, true) if w.y2 < v1 => w.y2,
        _ => v1,
    }
}

/// Clips the ending point of the [axis-aligned](crate::AxisAligned)
/// directed line segment to the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn exit<const VERT: bool, const FLIP: bool>(v2: i8, w: Clip<i8>) -> i8 {
    match (VERT, FLIP) {
        (false, false) if w.x2 < v2 => w.x2,
        (false, true) if v2 < w.x1 => w.x1,
        (true, false) if w.y2 < v2 => w.y2,
        (true, true) if v2 < w.y1 => w.y1,
        _ => v2,
    }
}
