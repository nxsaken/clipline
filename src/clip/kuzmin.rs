//! ## Kuzmin clipping
//!
//! This module provides an implementation of [YP Kuzmin's algorithm][1] for
//! efficient clipping of directed line segments to [rectangular regions](Clip).
//!
//! [1]: https://doi.org/10.1111/1467-8659.1450275

use crate::{Clip, Offset, Point};

/// Clips the starting point of a directed line segment covered by the given
/// [octant](crate::BresenhamOctant) against a [rectangular region](Clip).
///
/// Returns the clipped point `(cx1, cy1)` and the initial `error` term,
/// or [`None`] if the line segment does not intersect the clipping region.
///
/// **Note**: this function assumes that the line segment was not trivially rejected.
#[allow(clippy::cognitive_complexity)]
#[must_use]
#[inline(always)]
pub const fn enter<const FX: bool, const FY: bool, const SWAP: bool>(
    (x1, y1): Point<isize>,
    (dx, dy): Offset<isize>,   // absolute value
    (dx2, dy2): Offset<isize>, // absolute value
    &Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: &Clip<isize>,
) -> Option<(Point<isize>, isize)> {
    let (mut cx1, mut cy1) = (x1, y1);
    let mut error = if !SWAP { dy2 - dx } else { dx2 - dy };
    // horizontal entry (from below or above)
    if !SWAP && (!FY && y1 < wy1 || FY && wy2 < y1) || SWAP && (!FX && x1 < wx1 || FX && wx2 < x1) {
        let tmp = {
            let diff = match (SWAP, FX, FY) {
                (false, _, false) => wy1 - y1,
                (false, _, true) => y1 - wy2,
                (true, false, _) => wx1 - x1,
                (true, true, _) => x1 - wx2,
            };
            (diff * 2 - 1) * if !SWAP { dx } else { dy }
        };
        let msd = tmp / if !SWAP { dy2 } else { dx2 };
        match (SWAP, FX, FY) {
            (false, false, _) => cx1 += msd,
            (false, true, _) => cx1 -= msd,
            (true, _, false) => cy1 += msd,
            (true, _, true) => cy1 -= msd,
        }
        // non-trivial reject
        if !SWAP && (!FX && wx2 < cx1 || FX && cx1 < wx1)
            || SWAP && (!FY && wy2 < cy1 || FY && cy1 < wy1)
        {
            return None;
        }
        if !SWAP && (!FX && wx1 <= cx1 || FX && cx1 <= wx2)
            || SWAP && (!FY && wy1 <= cy1 || FY && cy1 <= wy2)
        {
            let rem = tmp - msd * if !SWAP { dy2 } else { dx2 };
            match SWAP {
                false => cy1 = if !FY { wy1 } else { wy2 },
                true => cx1 = if !FX { wx1 } else { wx2 },
            }
            error -= rem + if !SWAP { dx } else { dy };
            if 0 < rem {
                match SWAP {
                    false => cx1 += if !FX { 1 } else { -1 },
                    true => cy1 += if !FY { 1 } else { -1 },
                }
                error += if !SWAP { dy2 } else { dx2 };
            }
            return Some(((cx1, cy1), error));
        }
    }
    // vertical entry (from left or right)
    if !SWAP && (!FX && x1 < wx1 || FX && wx2 < x1) || SWAP && (!FY && y1 < wy1 || FY && wy2 < y1) {
        let tmp = match SWAP {
            false => (if !FX { wx1 - x1 } else { x1 - wx2 }) * dy2,
            true => (if !FY { wy1 - y1 } else { y1 - wy2 }) * dx2,
        };
        match (SWAP, FX, FY) {
            (false, _, false) => cy1 += tmp / dx2,
            (false, _, true) => cy1 -= tmp / dx2,
            (true, false, _) => cx1 += tmp / dy2,
            (true, true, _) => cx1 -= tmp / dy2,
        }
        let rem = tmp % if !SWAP { dx2 } else { dy2 };
        // non-trivial reject
        if !SWAP
            && (!FY && (wy2 < cy1 || cy1 == wy2 && dx <= rem)
                || FY && (cy1 < wy1 || cy1 == wy1 && dx <= rem))
            || SWAP
                && (!FX && (wx2 < cx1 || cx1 == wx2 && dy <= rem)
                    || FX && (cx1 < wx1 || cx1 == wx1 && dy <= rem))
        {
            return None;
        }
        match SWAP {
            false => cx1 = if !FX { wx1 } else { wx2 },
            true => cy1 = if !FY { wy1 } else { wy2 },
        }
        error += rem;
        if !SWAP && dx <= rem || SWAP && dy <= rem {
            match SWAP {
                false => cy1 += if !FY { 1 } else { -1 },
                true => cx1 += if !FX { 1 } else { -1 },
            }
            error -= if !SWAP { dx2 } else { dy2 };
        }
    }
    Some(((cx1, cy1), error))
}

/// Clips the ending point of a directed line segment covered by the given
/// [octant](crate::BresenhamOctant) against a [rectangular region](Clip).
///
/// Returns the clipped `end` coordinate along the axis of iteration
/// (`x` for gentle slopes, `y` for steep slopes).
///
/// **Note**: this function assumes that the line segment intersects the clipping region.
#[must_use]
#[inline(always)]
pub const fn exit<const FX: bool, const FY: bool, const SWAP: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    (dx, dy): Offset<isize>,   // absolute value
    (dx2, dy2): Offset<isize>, // absolute value
    &Clip { x1: wx1, y1: wy1, x2: wx2, y2: wy2 }: &Clip<isize>,
) -> isize {
    let mut end = if !SWAP { x2 } else { y2 };
    if !SWAP && (!FY && wy2 < y2 || FY && y2 < wy1) || SWAP && (!FX && wx2 < x2 || FX && x2 < wx1) {
        let tmp = match SWAP {
            false => dx2 * if !FY { wy2 - y1 } else { y1 - wy1 } + dx,
            true => dy2 * if !FX { wx2 - x1 } else { x1 - wx1 } + dy,
        };
        let msd = tmp / if !SWAP { dy2 } else { dx2 };
        end = match (SWAP, FX, FY) {
            (false, false, _) => msd + x1,
            (false, true, _) => msd - x1,
            (true, _, false) => msd + y1,
            (true, _, true) => msd - y1,
        };
        if tmp == msd * if !SWAP { dy2 } else { dx2 } {
            end -= 1;
        }
    }
    match (SWAP, FX, FY) {
        (false, false, _) if wx2 < end => wx2,
        (false, true, _) if end < wx1 => wx1,
        (true, _, false) if wy2 < end => wy2,
        (true, _, true) if end < wy1 => wy1,
        _ => end,
    }
}
