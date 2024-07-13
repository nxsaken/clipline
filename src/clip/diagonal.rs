//! ## Diagonal clipping
//!
//! This module provides [clipping](Clip) utilities for
//! [diagonal](crate::Diagonal) directed line segments.

use crate::{Clip, Point};

/// Checks if the directed line segment trivially lies outside the [clipping region](Clip).
#[must_use]
#[inline(always)]
pub const fn out_of_bounds<const FX: bool, const FY: bool>(
    (x1, y1): Point<i8>,
    (x2, y2): Point<i8>,
    w: Clip<i8>,
) -> bool {
    !FX && (x2 < w.x1 || w.x2 <= x1)
        || FX && (x1 < w.x1 || w.x2 <= x2)
        || !FY && (y2 < w.y1 || w.y2 <= y1)
        || FY && (y1 < w.y1 || w.y2 <= y2)
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
    w: Clip<i8>,
) -> Option<Point<i8>> {
    let (mut cx1, mut cy1) = (x1, y1);
    if !FX && x1 < w.x1 || FX && w.x2 < x1 {
        #[allow(clippy::cast_sign_loss)]
        let diff = match FX {
            false => u8::wrapping_sub(w.x1 as _, x1 as _),
            true => u8::wrapping_sub(x1 as _, w.x2 as _),
        };
        cy1 = match FY {
            false => cy1.wrapping_add_unsigned(diff),
            true => cy1.wrapping_sub_unsigned(diff),
        };
        if !FY && w.y2 < cy1 || FY && cy1 < w.y1 {
            return None;
        }
        if !FY && w.y1 <= cy1 || FY && cy1 <= w.y2 {
            cx1 = if !FX { w.x1 } else { w.x2 };
            return Some((cx1, cy1));
        }
    }
    if !FY && y1 < w.y1 || FY && w.y2 < y1 {
        #[allow(clippy::cast_sign_loss)]
        let diff = match FY {
            false => u8::wrapping_sub(w.y1 as _, y1 as _),
            true => u8::wrapping_sub(y1 as _, w.y2 as _),
        };
        cx1 = match FX {
            false => cx1.wrapping_add_unsigned(diff),
            true => cx1.wrapping_sub_unsigned(diff),
        };
        if !FX && w.x2 < cx1 || FX && cx1 < w.x1 {
            return None;
        }
        cy1 = if !FY { w.y1 } else { w.y2 };
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
    w: Clip<i8>,
) -> i8 {
    let cx2 = if !FY && w.y2 < y2 || FY && y2 < w.y1 {
        #[allow(clippy::cast_sign_loss)]
        let diff = match FY {
            false => u8::wrapping_sub(w.y2 as _, y1 as _),
            true => u8::wrapping_sub(y1 as _, w.y1 as _),
        };
        match FX {
            false => x1.wrapping_add_unsigned(diff),
            true => x1.wrapping_sub_unsigned(diff),
        }
    } else {
        x2
    };
    match FX {
        false if w.x2 < cx2 => w.x2,
        true if cx2 < w.x1 => w.x1,
        _ => cx2,
    }
}
