//! ## Clipping
//!
//! This module provides the [`Clip`] type representing a rectangular clipping region, as well as
//! methods for constructing iterators over clipped directed line segments of various types.

use crate::math::Point;
use crate::Bresenham;

/// A generic rectangular region defined by its minimum and maximum [corners](Point).
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Default)]
pub struct Clip<T> {
    pub(crate) wx1: T,
    pub(crate) wy1: T,
    pub(crate) wx2: T,
    pub(crate) wy2: T,
}

impl Clip<i8> {
    /// Returns a new [`Clip`] if `x1 <= x2 && y1 <= y2`, otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((wx1, wy1): Point<i8>, (wx2, wy2): Point<i8>) -> Option<Self> {
        if wx2 < wx1 || wy2 < wy1 {
            return None;
        }
        Some(Self { wx1, wy1, wx2, wy2 })
    }

    /// Returns a [Bresenham] iterator over an arbitrary directed line segment
    /// clipped to this clipping region.
    ///
    /// Returns [`None`] if the line segment does not intersect this clipping region.
    #[inline]
    #[must_use]
    pub const fn bresenham(self, p1: Point<i8>, p2: Point<i8>) -> Option<Bresenham<i8>> {
        Bresenham::clip(p1, p2, self)
    }
}
