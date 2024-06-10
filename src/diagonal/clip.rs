//! ### Diagonal clipping
//!
//! This module provides [clipping](Clip) utilities for
//! [diagonal](crate::Diagonal) directed line segments.

use crate::clip::Clip;
use crate::math::Point;
use crate::symmetry::{fx, fy};

/// Checks if the directed line segment trivially lies outside the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn out_of_bounds<const FX: bool, const FY: bool>(
    (x1, y1): Point<i8>,
    (x2, y2): Point<i8>,
    Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
) -> bool {
    fx!(x2 < wx1 || wx2 <= x1, x1 < wx1 || wx2 <= x2)
        || fy!(y2 < wy1 || wy2 <= y1, y1 < wy1 || wy2 <= y2)
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
    (x1, y1): Point<i8>,
    Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
) -> Option<Point<i8>> {
    let (mut cx1, mut cy1) = (x1, y1);
    if fx!(x1 < wx1, wx2 < x1) {
        #[allow(clippy::cast_sign_loss, non_snake_case)]
        let Dx1 = u8::wrapping_sub(fx!(wx1, x1) as _, fx!(x1, wx1) as _);
        cy1 = fy!(cy1.wrapping_add_unsigned(Dx1), cy1.wrapping_sub_unsigned(Dx1));
        if fy!(wy2 < cy1, cy1 < wy1) {
            return None;
        }
        if fy!(wy1 <= cy1, cy1 <= wy2) {
            cx1 = fx!(wx1, wx2);
            return Some((cx1, cy1));
        }
    }
    if fy!(y1 < wy1, wy2 < y1) {
        #[allow(clippy::cast_sign_loss, non_snake_case)]
        let Dy1 = u8::wrapping_sub(fy!(wy1, y1) as _, fy!(y1, wy2) as _);
        cx1 = fy!(cx1.wrapping_add_unsigned(Dy1), cx1.wrapping_sub_unsigned(Dy1));
        if fy!(wx2 < cx1, cx1 < wx1) {
            return None;
        }
        cy1 = fx!(wy1, wy2);
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
    (x1, y1): Point<i8>,
    (x2, y2): Point<i8>,
    Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
) -> i8 {
    let cx2 = if fy!(wy2 < y2, y2 < wy1) {
        #[allow(clippy::cast_sign_loss, non_snake_case)]
        let Dy2 = u8::wrapping_sub(fy!(wy2, y1) as _, fy!(y1, wy1) as _);
        fx!(x1.wrapping_add_unsigned(Dy2), x1.wrapping_sub_unsigned(Dy2))
    } else {
        x2
    };
    match FX {
        false if wx2 < cx2 => wx2,
        true if cx2 < wx1 => wx1,
        _ => cx2,
    }
}
