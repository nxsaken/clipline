//! ## Clipping
//!
//! This module provides the [`Clip`] type representing a rectangular clipping region, as well as
//! methods for constructing iterators over clipped directed line segments of various types.

use crate::math::Point;
use crate::{Bresenham, Diagonal, Horizontal, Orthogonal, Vertical};

/// A rectangular region defined by its minimum and maximum [corners](Point).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Clip<T> {
    pub(crate) wx1: T,
    pub(crate) wy1: T,
    pub(crate) wx2: T,
    pub(crate) wy2: T,
}

macro_rules! clip_impl {
    ($T:ty) => {
        impl Clip<$T> {
            /// Returns a new [`Clip`] if `x1 <= x2 && y1 <= y2`, otherwise returns [`None`].
            #[inline]
            #[must_use]
            pub const fn new((wx1, wy1): Point<$T>, (wx2, wy2): Point<$T>) -> Option<Self> {
                if wx2 < wx1 || wy2 < wy1 {
                    return None;
                }
                Some(Self { wx1, wy1, wx2, wy2 })
            }

            /// Checks if this region contains a [point](Point).
            #[inline]
            #[must_use]
            pub const fn contains(&self, (x, y): Point<$T>) -> bool {
                self.wx1 <= x && x <= self.wx2 && self.wy1 <= y && y <= self.wy2
            }

            /// Returns an iterator over a [horizontal](Horizontal)
            /// directed line segment clipped to this region.
            ///
            /// Returns [`None`] if the line segment does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn horizontal(self, y: $T, x1: $T, x2: $T) -> Option<Horizontal<$T>> {
                Horizontal::<$T>::clip(y, x1, x2, self)
            }

            /// Returns an iterator over a [vertical](Vertical)
            /// directed line segment clipped to this region.
            ///
            /// Returns [`None`] if the line segment does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn vertical(self, x: $T, y1: $T, y2: $T) -> Option<Vertical<$T>> {
                Vertical::<$T>::clip(x, y1, y2, self)
            }

            /// Returns an iterator over a directed line segment,
            /// if it is [orthogonal](Orthogonal), clipped to this region.
            ///
            /// Returns [`None`] if the line segment is not orthogonal,
            /// or if it does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn orthogonal(self, p1: Point<$T>, p2: Point<$T>) -> Option<Orthogonal<$T>> {
                Orthogonal::<$T>::clip(p1, p2, self)
            }

            /// Returns an iterator over a directed line segment,
            /// if it is [diagonal](Diagonal), clipped to this region.
            ///
            /// Returns [`None`] if the line segment is not diagonal,
            /// or if it does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn diagonal(self, p1: Point<$T>, p2: Point<$T>) -> Option<Diagonal<$T>> {
                Diagonal::<$T>::clip(p1, p2, self)
            }

            /// Returns a [Bresenham] iterator over an arbitrary
            /// directed line segment clipped to this region.
            ///
            /// Returns [`None`] if the line segment does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn bresenham(self, p1: Point<$T>, p2: Point<$T>) -> Option<Bresenham<$T>> {
                Bresenham::<$T>::clip(p1, p2, self)
            }
        }
    };
}

clip_impl!(i8);
clip_impl!(u8);
clip_impl!(i16);
clip_impl!(u16);
clip_impl!(i32);
clip_impl!(u32);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
clip_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
clip_impl!(usize);
