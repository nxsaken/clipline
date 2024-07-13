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
    Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: Clip<i8>,
) -> bool {
    !VERT
        && ((u < wy1 || wy2 < u)
            || (!FLIP && (v2 < wx1 || wx2 < v1) || FLIP && (v1 < wx1 || wx2 < v2)))
        || VERT
            && ((u < wx1 || wx2 < u)
                || (!FLIP && (v2 < wy1 || wy2 < v1) || FLIP && (v1 < wy1 || wy2 < v2)))
}

/// Clips the starting point of the [axis-aligned](crate::AxisAligned)
/// directed line segment to the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn enter<const VERT: bool, const FLIP: bool>(
    v1: i8,
    Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: Clip<i8>,
) -> i8 {
    match (VERT, FLIP) {
        (false, false) if v1 < wx1 => wx1,
        (false, true) if wx2 < v1 => wx2,
        (true, false) if v1 < wy1 => wy1,
        (true, true) if wy2 < v1 => wy2,
        _ => v1,
    }
}

/// Clips the ending point of the [axis-aligned](crate::AxisAligned)
/// directed line segment to the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn exit<const VERT: bool, const FLIP: bool>(
    v2: i8,
    Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: Clip<i8>,
) -> i8 {
    match (VERT, FLIP) {
        (false, false) if wx2 < v2 => wx2,
        (false, true) if v2 < wx1 => wx1,
        (true, false) if wy2 < v2 => wy2,
        (true, true) if v2 < wy1 => wy1,
        _ => v2,
    }
}
