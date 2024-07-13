//! ## Kuzmin clipping
//!
//! This module provides an implementation of [YP Kuzmin's algorithm][1] for
//! efficient clipping of directed line segments to [rectangular regions](Clip).
//!
//! [1]: https://doi.org/10.1111/1467-8659.1450275

use crate::{Clip, Delta, Delta2, Point};

/// Clips the starting point of a directed line segment covered by the given
/// [octant](crate::BresenhamOctant) against a [rectangular region](Clip).
///
/// Returns the clipped point `(cx1, cy1)` and the initial `error` term,
/// or [`None`] if the line segment does not intersect the clipping region.
///
/// **Note**: this function assumes that the line segment was not trivially rejected.
#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
#[must_use]
#[inline(always)]
pub const fn enter<const FX: bool, const FY: bool, const SWAP: bool>(
    (x1, y1): Point<i8>,
    (dx, dy): Delta<i8>,
    (dx2, dy2): Delta2<i8>,
    w: Clip<i8>,
) -> Option<(Point<i8>, i16)> {
    let (mut cx1, mut cy1) = (x1, y1);
    // will not wrap because dx2 <= u8::MAX * 2 < i16::MAX
    #[allow(clippy::cast_possible_wrap)]
    let mut error = match SWAP {
        false => i16::wrapping_sub(dy2 as _, dx as _),
        true => i16::wrapping_sub(dx2 as _, dy as _),
    };
    // horizontal entry (from below or above)
    if !SWAP && (!FY && y1 < w.y1 || FY && w.y2 < y1)
        || SWAP && (!FX && x1 < w.x1 || FX && w.x2 < x1)
    {
        let tmp = {
            // diff != 0 due to entry condition
            #[allow(clippy::cast_sign_loss)]
            let diff = match (SWAP, FX, FY) {
                (false, _, false) => u8::wrapping_sub(w.y1 as _, y1 as _),
                (false, _, true) => u8::wrapping_sub(y1 as _, w.y2 as _),
                (true, false, _) => u8::wrapping_sub(w.x1 as _, x1 as _),
                (true, true, _) => u8::wrapping_sub(x1 as _, w.x2 as _),
            };
            // FIXME: can overflow even in u16? need to check constraints
            //  ((diff as u16) * 2 - 1) * if !SWAP { dx } else { dy } as u16 <= 65025
            match u16::checked_mul(
                (diff as u16).wrapping_shl(1).wrapping_sub(1),
                if !SWAP { dx } else { dy } as _,
            ) {
                None => panic!("FIXME: overflow"),
                Some(tmp) => tmp,
            }
        };
        let msd = match tmp.checked_div(if !SWAP { dy2 } else { dx2 }) {
            None => {
                // SAFETY: vertical, horizontal and empty lines have been rejected,
                // therefore dx > 0 && dy > 0, and dx2 > 0 && dy2 > 0.
                unsafe { core::hint::unreachable_unchecked() }
            }
            #[allow(clippy::cast_possible_truncation)]
            Some(msd) => {
                debug_assert!(msd <= u8::MAX as u16);
                msd as u8
            }
        };
        match (SWAP, FX, FY) {
            (false, false, _) => cx1 = cx1.wrapping_add_unsigned(msd),
            (false, true, _) => cx1 = cx1.wrapping_sub_unsigned(msd),
            (true, _, false) => cy1 = cy1.wrapping_add_unsigned(msd),
            (true, _, true) => cy1 = cy1.wrapping_sub_unsigned(msd),
        }
        // non-trivial reject
        if !SWAP && (!FX && w.x2 < cx1 || FX && cx1 < w.x1)
            || SWAP && (!FY && w.y2 < cy1 || FY && cy1 < w.y1)
        {
            return None;
        }
        if !SWAP && (!FX && w.x1 <= cx1 || FX && cx1 <= w.x2)
            || SWAP && (!FY && w.y1 <= cy1 || FY && cy1 <= w.y2)
        {
            let tmp_cropped = match u16::checked_mul(msd as _, if !SWAP { dy2 } else { dx2 }) {
                None => panic!("FIXME: overflow"),
                Some(tmp) => tmp,
            };
            let rem = tmp.wrapping_sub(tmp_cropped);
            match SWAP {
                false => cy1 = if !FY { w.y1 } else { w.y2 },
                true => cx1 = if !FX { w.x1 } else { w.x2 },
            }
            error = error.wrapping_sub_unsigned(match SWAP {
                false => rem.wrapping_add(dx as u16),
                true => rem.wrapping_add(dy as u16),
            });
            if 0 < rem {
                match (SWAP, FX, FY) {
                    (false, false, _) => cx1 = cx1.wrapping_add_unsigned(1),
                    (false, true, _) => cx1 = cx1.wrapping_sub_unsigned(1),
                    (true, _, false) => cy1 = cy1.wrapping_add_unsigned(1),
                    (true, _, true) => cy1 = cy1.wrapping_sub_unsigned(1),
                }
                error = error.wrapping_add_unsigned(if !SWAP { dy2 } else { dx2 });
            }
            return Some(((cx1, cy1), error));
        }
    }
    // vertical entry (from left or right)
    if !SWAP && (!FX && x1 < w.x1 || FX && w.x2 < x1)
        || SWAP && (!FY && y1 < w.y1 || FY && w.y2 < y1)
    {
        let tmp = {
            // diff != 0 due to entry condition
            #[allow(clippy::cast_sign_loss)]
            let diff = match (SWAP, FX, FY) {
                (false, false, _) => u8::wrapping_sub(w.x1 as _, x1 as _),
                (false, true, _) => u8::wrapping_sub(x1 as _, w.x2 as _),
                (true, _, false) => u8::wrapping_sub(w.y1 as _, y1 as _),
                (true, _, true) => u8::wrapping_sub(y1 as _, w.y2 as _),
            };
            match u16::checked_mul(diff as _, if !SWAP { dy2 } else { dx2 }) {
                None => panic!("FIXME: overflow"),
                Some(tmp) => tmp,
            }
        };
        let msd = match tmp.checked_div(if !SWAP { dx2 } else { dy2 }) {
            None => {
                // SAFETY: vertical, horizontal and empty lines have been rejected,
                // therefore dx > 0 && dy > 0, and dx2 > 0 && dy2 > 0.
                unsafe { core::hint::unreachable_unchecked() }
            }
            #[allow(clippy::cast_possible_truncation)]
            Some(msd) => {
                debug_assert!(msd <= u8::MAX as u16);
                msd as u8
            }
        };
        let rem = match tmp.checked_rem(if !SWAP { dx2 } else { dy2 }) {
            None => {
                // SAFETY: vertical, horizontal and empty lines have been rejected,
                // therefore dx > 0 && dy > 0, and dx2 > 0 && dy2 > 0.
                unsafe { core::hint::unreachable_unchecked() }
            }
            #[allow(clippy::cast_possible_truncation)]
            Some(rem) => {
                debug_assert!(rem <= u8::MAX as u16);
                rem as u8
            }
        };
        match (SWAP, FX, FY) {
            (false, _, false) => cy1 = cy1.wrapping_add_unsigned(msd),
            (false, _, true) => cy1 = cy1.wrapping_sub_unsigned(msd),
            (true, false, _) => cx1 = cx1.wrapping_add_unsigned(msd),
            (true, true, _) => cx1 = cx1.wrapping_sub_unsigned(msd),
        };
        // non-trivial reject
        if !SWAP
            && (!FY && (w.y2 < cy1 || cy1 == w.y2 && dx <= rem)
                || FY && (cy1 < w.y1 || cy1 == w.y1 && dx <= rem))
            || SWAP
                && (!FX && (w.x2 < cx1 || cx1 == w.x2 && dy <= rem)
                    || FX && (cx1 < w.x1 || cx1 == w.x1 && dy <= rem))
        {
            return None;
        }
        match SWAP {
            false => cx1 = if !FX { w.x1 } else { w.x2 },
            true => cy1 = if !FY { w.y1 } else { w.y2 },
        }
        error = error.wrapping_add_unsigned(rem as u16);
        if !SWAP && dx <= rem || SWAP && dy <= rem {
            match (SWAP, FX, FY) {
                (false, _, false) => cy1 = cy1.wrapping_add_unsigned(1),
                (false, _, true) => cy1 = cy1.wrapping_sub_unsigned(1),
                (true, false, _) => cx1 = cx1.wrapping_add_unsigned(1),
                (true, true, _) => cx1 = cx1.wrapping_sub_unsigned(1),
            };
            error = error.wrapping_sub_unsigned(if !SWAP { dx2 } else { dy2 });
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
    (x1, y1): Point<i8>,
    (x2, y2): Point<i8>,
    (dx, dy): Delta<i8>,
    (dx2, dy2): Delta2<i8>,
    w: Clip<i8>,
) -> i8 {
    let mut end = if !SWAP { x2 } else { y2 };
    if !SWAP && (!FY && w.y2 < y2 || FY && y2 < w.y1)
        || SWAP && (!FX && w.x2 < x2 || FX && x2 < w.x1)
    {
        let tmp = {
            // diff != 0 due to entry condition
            #[allow(clippy::cast_sign_loss)]
            let diff = match (SWAP, FX, FY) {
                (false, _, false) => u8::wrapping_sub(w.y2 as _, y1 as _),
                (false, _, true) => u8::wrapping_sub(y1 as _, w.y1 as _),
                (true, false, _) => u8::wrapping_sub(w.x2 as _, x1 as _),
                (true, true, _) => u8::wrapping_sub(x1 as _, w.x1 as _),
            };
            match SWAP {
                false => match dx2.checked_mul(diff as _) {
                    None => panic!("FIXME: overflow"),
                    Some(tmp) => tmp.wrapping_add(dx as _),
                },
                true => match dy2.checked_mul(diff as _) {
                    None => panic!("FIXME: overflow"),
                    Some(tmp) => tmp.wrapping_add(dy as _),
                },
            }
        };
        let msd = match tmp.checked_div(if !SWAP { dy2 } else { dx2 }) {
            None => {
                // SAFETY: vertical, horizontal and empty lines have been rejected,
                // therefore dx > 0 && dy > 0, and dx2 > 0 && dy2 > 0.
                unsafe { core::hint::unreachable_unchecked() }
            }
            #[allow(clippy::cast_possible_truncation)]
            Some(msd) => {
                debug_assert!(msd <= u8::MAX as u16);
                msd as u8
            }
        };
        end = match (SWAP, FX, FY) {
            (false, false, _) => x1.wrapping_add_unsigned(msd),
            (false, true, _) => x1.wrapping_sub_unsigned(msd),
            (true, _, false) => y1.wrapping_add_unsigned(msd),
            (true, _, true) => y1.wrapping_sub_unsigned(msd),
        };
        let tmp_cropped = match u16::checked_mul(msd as _, if !SWAP { dy2 } else { dx2 }) {
            None => panic!("FIXME: overflow"),
            Some(tmp) => tmp,
        };
        if tmp == tmp_cropped {
            end = end.wrapping_sub(1);
        }
    }
    match (SWAP, FX, FY) {
        (false, false, _) if w.x2 < end => w.x2,
        (false, true, _) if end < w.x1 => w.x1,
        (true, _, false) if w.y2 < end => w.y2,
        (true, _, true) if end < w.y1 => w.y1,
        _ => end,
    }
}
