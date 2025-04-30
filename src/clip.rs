//! ## Clipping
//!
//! This module provides the [`Clip`] type representing a rectangular clipping region,
//! as well as methods for constructing iterators over clipped line segments of common types.

use crate::axis::{AnyAxis, Axis0, Axis1};
use crate::diagonal::AnyDiagonal;
use crate::macros::control_flow::return_if;
use crate::math::Point;
use crate::octant::AnyOctant;

/// A rectangular region defined by its minimum and maximum [corners](Point).
///
/// *Both corners are included in the region.*
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
                return_if!(wx2 < wx1 || wy2 < wy1);
                Some(Self { wx1, wy1, wx2, wy2 })
            }

            /// Returns the minimum corner of this clipping region.
            #[inline]
            #[must_use]
            pub const fn min(&self) -> Point<$T> {
                (self.wx1, self.wy1)
            }

            /// Returns the maximum corner of this clipping region.
            #[inline]
            #[must_use]
            pub const fn max(&self) -> Point<$T> {
                (self.wx2, self.wy2)
            }

            /// Checks if this region contains a [point](Point).
            #[inline]
            #[must_use]
            pub const fn point(&self, (x, y): Point<$T>) -> bool {
                self.wx1 <= x && x <= self.wx2 && self.wy1 <= y && y <= self.wy2
            }

            /// Clips a *half-open* [horizontal](Axis0) line segment
            /// to this region, and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn axis_0(&self, y: $T, x1: $T, x2: $T) -> Option<Axis0<$T>> {
                Axis0::<$T>::clip(y, x1, x2, self)
            }

            /// Clips a *half-open* [vertical](Axis1) line segment
            /// to this region, and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn axis_1(&self, x: $T, y1: $T, y2: $T) -> Option<Axis1<$T>> {
                Axis1::<$T>::clip(x, y1, y2, self)
            }

            /// Clips a *half-open* line segment to this region
            /// if it is aligned to [any axis](AnyAxis), and returns an iterator over it,
            /// .
            ///
            /// Returns [`None`] if the line segment is not axis-aligned,
            /// or if it does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn any_axis(&self, p1: Point<$T>, p2: Point<$T>) -> Option<AnyAxis<$T>> {
                AnyAxis::<$T>::clip(p1, p2, self)
            }

            /// Clips a *half-open* line segment to this region
            /// if it is [diagonal](AnyDiagonal), and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment is not diagonal,
            /// or if it does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn any_diagonal(
                &self,
                p1: Point<$T>,
                p2: Point<$T>,
            ) -> Option<AnyDiagonal<$T>> {
                AnyDiagonal::<$T>::clip(p1, p2, self)
            }

            /// Clips a *half-open* [arbitrary](AnyOctant) line segment
            /// to this region, and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment does not intersect this clipping region.
            #[inline]
            #[must_use]
            pub const fn any_octant(&self, p1: Point<$T>, p2: Point<$T>) -> Option<AnyOctant<$T>> {
                AnyOctant::<$T>::clip(p1, p2, self)
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
