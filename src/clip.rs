//! ## Clipping
//!
//! This module provides the [`Clip`] type representing a rectangular clipping region, as well as
//! methods for constructing iterators over clipped directed line segments of various types.

use crate::{Bresenham, Point};

pub mod diagonal;
pub mod kuzmin;
pub mod signed_axis;

/// A generic rectangular region defined by its minimum and maximum [corners](Point).
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Default)]
pub struct Clip<T> {
    x1: T,
    y1: T,
    x2: T,
    y2: T,
}

impl Clip<isize> {
    /// Returns a new [`Clip`] if `x1 <= x2 && y1 <= y2`, otherwise returns [`None`].
    #[must_use]
    #[inline]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
        if x2 < x1 || y2 < y1 {
            return None;
        }
        Some(Self { x1, y1, x2, y2 })
    }

    /// Returns a [Bresenham] iterator over a directed line segment clipped to this region.
    ///
    /// Returns [`None`] if the line segment does not intersect this region.
    #[must_use]
    #[inline]
    pub const fn bresenham(
        &self,
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
    ) -> Option<Bresenham<isize>> {
        Bresenham::clip((x1, y1), (x2, y2), self)
    }
}

/// Macro that maps over an [`Option`], for use in const contexts.
macro_rules! map_option_inner {
    ($option:expr, $some:pat => $mapped:expr) => {
        match $option {
            None => None,
            Some($some) => Some($mapped),
        }
    };
}

pub(crate) use map_option_inner as map_option;
