//! ### Signed axis clipping
//!
//! This module provides [clipping](Clip) for
//! [signed-axis-aligned](SignedAxisAligned) directed line segments.

use super::{f, vh, SignedAxisAligned};
use crate::clip::Clip;

impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<i8, VERT, FLIP> {
    #[inline(always)]
    #[must_use]
    const fn reject(u: i8, v1: i8, v2: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        vh!(
            (u < wy1 || wy2 <= u) || f!(v2 < wx1 || wx2 <= v1, v1 < wx1 || wx2 <= v2),
            (u < wx1 || wx2 <= u) || f!(v2 < wy1 || wy2 <= v1, v1 < wy1 || wy2 <= v2)
        )
    }

    #[inline(always)]
    #[must_use]
    const fn cv1(v1: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> i8 {
        match (VERT, FLIP) {
            (false, false) if v1 < wx1 => wx1,
            (false, true) if wx2 < v1 => wx2,
            (true, false) if v1 < wy1 => wy1,
            (true, true) if wy2 < v1 => wy2,
            _ => v1,
        }
    }

    #[inline(always)]
    #[must_use]
    const fn cv2(v2: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> i8 {
        match (VERT, FLIP) {
            (false, false) if wx2 < v2 => wx2,
            (false, true) if v2 < wx1 => wx1,
            (true, false) if wy2 < v2 => wy2,
            (true, true) if v2 < wy1 => wy1,
            _ => v2,
        }
    }

    #[inline(always)]
    #[must_use]
    pub(super) const fn clip_inner(u: i8, v1: i8, v2: i8, clip: Clip<i8>) -> Option<Self> {
        if Self::reject(u, v1, v2, clip) {
            return None;
        }
        Some(Self::new_unchecked(u, Self::cv1(v1, clip), Self::cv2(v2, clip)))
    }
}
