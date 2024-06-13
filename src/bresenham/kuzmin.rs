//! ## Kuzmin clipping
//!
//! This module provides an implementation of [Kuzmin's algorithm][1] for
//! clipping directed line segments with integer coordinates to [rectangular regions](Region).
//!
//! [1]: https://doi.org/10.1111/1467-8659.1450275

use super::Offset;
use crate::{Point, Region};

/// Clips a directed line segment in the given octant against a rectangular window.
///
/// Returns the clipped point `(cx1, cy1)`, the initial `error` term,
/// and the `end` coordinate along the iteration axis, or [`None`]
/// if the line segment does not intersect the window.
#[must_use]
#[inline(always)]
pub const fn clip<const FX: bool, const FY: bool, const SWAP: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    (dx, dy): Offset<isize>,   // absolute value
    (dx2, dy2): Offset<isize>, // absolute value
    Region { wx1, wy1, wx2, wy2 }: Region<isize>,
) -> Option<(Point<isize>, isize, isize)> {
    let Some(((cx1, cy1), error)) =
        enter::<FX, FY, SWAP>((x1, y1), (dx, dy), (dx2, dy2), (wx1, wy1), (wx2, wy2))
    else {
        return None;
    };
    let end =
        exit::<FX, FY, SWAP>((x1, y1), (x2, y2), (dx, dy), (dx2, dy2), (wx1, wy1), (wx2, wy2));
    Some(((cx1, cy1), error, end))
}

/// Clips the starting point of a line segment in the given octant
/// against a rectangular window.
///
/// Returns the clipped point `(cx1, cy1)` and the initial `error` term.
#[allow(clippy::cognitive_complexity)]
#[must_use]
#[inline(always)]
const fn enter<const FX: bool, const FY: bool, const SWAP: bool>(
    (x1, y1): Point<isize>,
    (dx, dy): Offset<isize>,   // absolute value
    (dx2, dy2): Offset<isize>, // absolute value
    (wx1, wy1): Point<isize>,
    (wx2, wy2): Point<isize>,
) -> Option<(Point<isize>, isize)> {
    let (mut cx1, mut cy1) = (x1, y1);
    let mut error = if !SWAP { dy2 - dx } else { dx2 - dy };
    if !SWAP && (!FX && x1 < wx1 || FX && wx2 < x1) || SWAP && (!FY && y1 < wy1 || FY && wy2 < y1) {
        let tmp = {
            let diff = match (SWAP, FX, FY) {
                (false, false, _) => wx1 - x1,
                (false, true, _) => x1 - wx2,
                (true, _, false) => wy1 - y1,
                (true, _, true) => y1 - wy2,
            };
            (diff * 2 - 1) * if !SWAP { dy } else { dx }
        };
        let msd = tmp / if !SWAP { dx2 } else { dy2 };
        match (SWAP, FX, FY) {
            (false, _, false) => cy1 += msd,
            (false, _, true) => cy1 -= msd,
            (true, false, _) => cx1 += msd,
            (true, true, _) => cx1 -= msd,
        }
        if !SWAP && (!FY && wy2 < cy1 || FY && cy1 < wy1)
            || SWAP && (!FX && wx2 < cx1 || FX && cx1 < wx1)
        {
            return None;
        }
        if !SWAP && (!FY && wy1 <= cy1 || FY && cy1 <= wy2)
            || SWAP && (!FX && wx1 <= cx1 || FX && cx1 <= wx2)
        {
            let rem = tmp - msd * if !SWAP { dx2 } else { dy2 };
            match SWAP {
                false => cx1 = if !FX { wx1 } else { wx2 },
                true => cy1 = if !FY { wy1 } else { wy2 },
            }
            error -= rem + if !SWAP { dy } else { dx };
            if 0 < rem {
                match SWAP {
                    false => cy1 += if !FY { 1 } else { -1 },
                    true => cx1 += if !FX { 1 } else { -1 },
                }
                error += if !SWAP { dx2 } else { dy2 };
            }
            return Some(((cx1, cy1), error));
        }
    }
    if !SWAP && (!FY && y1 < wy1 || FY && wy2 < y1) || SWAP && (!FX && x1 < wx1 || FX && wx2 < x1) {
        let tmp = match SWAP {
            false => (if !FY { wy1 - y1 } else { y1 - wy2 }) * dx2,
            true => (if !FX { wx1 - x1 } else { x1 - wx2 }) * dy2,
        };
        match (SWAP, FX, FY) {
            (false, false, _) => cx1 += tmp / dy2,
            (false, true, _) => cx1 -= tmp / dy2,
            (true, _, false) => cy1 += tmp / dx2,
            (true, _, true) => cy1 -= tmp / dx2,
        }
        let rem = tmp % if !SWAP { dy2 } else { dx2 };
        if !SWAP
            && (!FX && (wx2 < cx1 || cx1 == wx2 && dy <= rem)
                || FX && (cx1 < wx1 || cx1 == wx1 && dy <= rem))
            || SWAP
                && (!FY && (wy2 < cy1 || cy1 == wy2 && dx <= rem)
                    || FY && (cy1 < wy1 || cy1 == wy1 && dx <= rem))
        {
            return None;
        }
        match SWAP {
            false => cy1 = if !FY { wy1 } else { wy2 },
            true => cx1 = if !FX { wx1 } else { wx2 },
        }
        error += rem;
        if !SWAP && dy <= rem || SWAP && dx <= rem {
            match SWAP {
                false => cx1 += if !FX { 1 } else { -1 },
                true => cy1 += if !FY { 1 } else { -1 },
            }
            error -= if !SWAP { dy2 } else { dx2 };
        }
    }
    Some(((cx1, cy1), error))
}

/// Clips the ending point of a line segment against a rectangular window.
///
/// Returns the clipped `end` coordinate along the axis of iteration
/// (`end_x` for gentle slopes, `end_y` for steep slopes).
#[must_use]
#[inline(always)]
const fn exit<const FX: bool, const FY: bool, const SWAP: bool>(
    (x1, y1): Point<isize>,
    (x2, y2): Point<isize>,
    (dx, dy): Offset<isize>,   // absolute value
    (dx2, dy2): Offset<isize>, // absolute value
    (wx1, wy1): Point<isize>,
    (wx2, wy2): Point<isize>,
) -> isize {
    let mut end = if !SWAP { x2 } else { y2 };
    if !SWAP && (!FY && wy2 < y2 || FY && y2 < wy1) || SWAP && (!FX && wx2 < x2 || FX && x2 < wx1) {
        let tmp = match SWAP {
            false => dx2 * if !FY { wy2 - y1 } else { y1 - wy1 } + dx,
            true => dy2 * if !FX { wx2 - x1 } else { x1 - wx1 } + dy,
        };
        let msd = tmp / if !SWAP { dy2 } else { dx2 };
        end = match SWAP {
            false => match FX {
                false => msd + x1,
                true => msd - x1,
            },
            true => match FY {
                false => msd + y1,
                true => msd - y1,
            },
        };
        if tmp - msd * if !SWAP { dy2 } else { dx2 } == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const REGION: Region<isize> = Region::new((0, 0), (320, 240));

    #[test]
    fn octant_0_vertical_entry() {
        let (x1, y1) = (-160, 0);
        let (x2, y2) = (320, 360);
        assert_eq!(
            clip::<false, false, false>(
                (x1, y1),
                (x2, y2),
                (x2 - x1, y2 - y1),
                (2 * (x2 - x1), 2 * (y2 - y1)),
                REGION
            ),
            Some(((0, 120), 240, 160))
        );
    }

    #[test]
    fn octant_0_horizontal_entry() {
        let (x1, y1) = (80, -60);
        let (x2, y2) = (320 + 80, 240 - 60);
        assert_eq!(
            clip::<false, false, false>(
                (x1, y1),
                (x2, y2),
                (x2 - x1, y2 - y1),
                (2 * (x2 - x1), 2 * (y2 - y1)),
                REGION
            ),
            Some(((160, 0), 160, 320))
        );
    }
}
