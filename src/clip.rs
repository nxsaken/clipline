//! ## Clipping
//!
//! This module provides the [`Clip`] type representing a rectangular clipping region,
//! as well as methods for constructing iterators over clipped line segments of common types.

use crate::axis::{AnyAxis, Axis0, Axis1};
use crate::diagonal::AnyDiagonal;
use crate::macros::control_flow::return_if;
use crate::macros::derive::nums;
use crate::math::{ops, Point};
use crate::octant::AnyOctant;

/// A closed[^1] rectangular clipping region defined by its minimum and maximum corners.
///
/// [^1]: Both corners are included.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Clip<T> {
    pub(crate) wx1: T,
    pub(crate) wy1: T,
    pub(crate) wx2: T,
    pub(crate) wy2: T,
}

macro_rules! impl_clip {
    ($T:ty) => {
        impl Clip<$T> {
            /// Constructs a new [`Clip`] from minimum and maximum corners.
            ///
            /// Returns [`None`] if `wx2 < wx1` or `wy2 < wy1`.
            #[inline]
            #[must_use]
            pub const fn from_min_max(
                (wx1, wy1): Point<$T>,
                (wx2, wy2): Point<$T>,
            ) -> Option<Self> {
                return_if!(wx2 < wx1 || wy2 < wy1);
                Some(Self { wx1, wy1, wx2, wy2 })
            }

            /// Constructs a new [`Clip`] with corners `(0, 0)` and `(width - 1, height - 1)`.
            ///
            /// Returns [`None`] if `width <= 0` or `height <= 0`.
            #[inline]
            #[must_use]
            pub const fn from_size(width: $T, height: $T) -> Option<Self> {
                return_if!(width <= 0 || height <= 0);
                Some(Self {
                    wx1: 0,
                    wy1: 0,
                    wx2: ops::<$T>::t_sub_1(width),
                    wy2: ops::<$T>::t_sub_1(height),
                })
            }

            /// Returns the minimum corner of this [`Clip`].
            #[inline]
            #[must_use]
            pub const fn min(&self) -> Point<$T> {
                (self.wx1, self.wy1)
            }

            /// Returns the maximum corner of this [`Clip`].
            #[inline]
            #[must_use]
            pub const fn max(&self) -> Point<$T> {
                (self.wx2, self.wy2)
            }

            /// Checks if this [`Clip`] contains a point.
            #[inline]
            #[must_use]
            pub const fn point(&self, (x, y): Point<$T>) -> bool {
                self.wx1 <= x && x <= self.wx2 && self.wy1 <= y && y <= self.wy2
            }

            /// Clips a half-open horizontal line segment to this [`Clip`]
            /// and constructs an [`Axis0`] over the portion inside.
            ///
            /// Returns [`None`] if the line segment lies outside this region.
            #[inline]
            #[must_use]
            pub const fn axis_0(&self, y: $T, x1: $T, x2: $T) -> Option<Axis0<$T>> {
                Axis0::<$T>::clip(y, x1, x2, self)
            }

            /// Clips a half-open vertical line segment to this [`Clip`]
            /// and constructs an [`Axis1`] over the portion inside.
            ///
            /// Returns [`None`] if the line segment lies outside this region.
            #[inline]
            #[must_use]
            pub const fn axis_1(&self, x: $T, y1: $T, y2: $T) -> Option<Axis1<$T>> {
                Axis1::<$T>::clip(x, y1, y2, self)
            }

            /// Clips a half-open line segment to this [`Clip`]
            /// and constructs an [`AnyAxis`] over the portion inside.
            ///
            /// Returns [`None`] if the line segment is not axis-aligned,
            /// or lies outside this region.
            #[inline]
            #[must_use]
            pub const fn any_axis(&self, p1: Point<$T>, p2: Point<$T>) -> Option<AnyAxis<$T>> {
                AnyAxis::<$T>::clip(p1, p2, self)
            }

            /// Clips a half-open line segment to this [`Clip`]
            /// and constructs an [`AnyDiagonal`] over the portion inside.
            ///
            /// Returns [`None`] if the line segment is not diagonal,
            /// or lies outside this region.
            #[inline]
            #[must_use]
            pub const fn any_diagonal(
                &self,
                p1: Point<$T>,
                p2: Point<$T>,
            ) -> Option<AnyDiagonal<$T>> {
                AnyDiagonal::<$T>::clip(p1, p2, self)
            }
        }
    };
}

macro_rules! impl_clip_octant {
    ($T:ty) => {
        impl Clip<$T> {
            /// Clips a half-open line segment to this [`Clip`]
            /// and constructs an [`AnyOctant`] over the portion inside.
            ///
            /// Returns [`None`] if the line segment lies outside the clipping region.
            #[inline]
            #[must_use]
            pub const fn any_octant(&self, p1: Point<$T>, p2: Point<$T>) -> Option<AnyOctant<$T>> {
                AnyOctant::<$T>::clip(p1, p2, self)
            }
        }
    };
}

nums!(impl_clip);
nums!(impl_clip_octant, cfg_octant_64);
