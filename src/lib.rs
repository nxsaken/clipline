//! # Clipline
//!
//! Efficient rasterization of line segments with clipping.

#![no_std]
#![cfg_attr(feature = "try_fold", feature(try_trait_v2))]
#![cfg_attr(feature = "is_empty", feature(exact_size_is_empty))]
#![forbid(unsafe_code)]
// TODO(#10): #![forbid(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]
#![warn(clippy::nursery, clippy::cargo, clippy::pedantic)]
#![allow(
    clippy::match_bool,
    clippy::module_name_repetitions,
    clippy::inline_always,
    clippy::similar_names,
    clippy::if_not_else
)]

#[cfg(feature = "axis_aligned")]
mod axis_aligned;
#[cfg(feature = "bresenham")]
mod bresenham;
#[cfg(feature = "diagonal")]
mod diagonal;

#[cfg(feature = "axis_aligned")]
pub use axis_aligned::{
    AxisAligned, Horizontal, NegativeAxisAligned, NegativeHorizontal, NegativeVertical, Orthogonal,
    PositiveAxisAligned, PositiveHorizontal, PositiveVertical, SignedAxisAligned, SignedHorizontal,
    SignedVertical, Vertical,
};

#[cfg(feature = "bresenham")]
pub use bresenham::{
    Bresenham, Octant as BresenhamOctant, Octant0 as BresenhamOctant0, Octant1 as BresenhamOctant1,
    Octant2 as BresenhamOctant2, Octant3 as BresenhamOctant3, Octant4 as BresenhamOctant4,
    Octant5 as BresenhamOctant5, Octant6 as BresenhamOctant6, Octant7 as BresenhamOctant7,
};

#[cfg(feature = "diagonal")]
pub use diagonal::{
    Diagonal, Quadrant as DiagonalQuadrant, Quadrant0 as DiagonalQuadrant0,
    Quadrant1 as DiagonalQuadrant1, Quadrant2 as DiagonalQuadrant2, Quadrant3 as DiagonalQuadrant3,
};

// TODO(#12): support all integer types, including unsigned.
/// A generic point on a Cartesian plane.
#[cfg(any(feature = "axis_aligned", feature = "bresenham", feature = "diagonal"))]
type Point<T> = (T, T);

/// A generic rectangular region defined by its minimum and maximum [corners](Point).
#[cfg(feature = "clip")]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Default)]
pub struct Region<T> {
    wx1: T,
    wy1: T,
    wx2: T,
    wy2: T,
}

#[cfg(feature = "clip")]
impl Region<isize> {
    /// Returns a new [`Region`] if `x1 <= x2 && y1 <= y2`, otherwise returns [`None`].
    #[must_use]
    #[inline]
    pub const fn try_new((wx1, wy1): Point<isize>, (wx2, wy2): Point<isize>) -> Option<Self> {
        if wx2 < wx1 || wy2 < wy1 {
            return None;
        }
        Some(Self { wx1, wy1, wx2, wy2 })
    }

    /// Returns a new [`Region`], sorting the coordinates such that `x1 <= x2 && y1 <= y2`.
    #[must_use]
    #[inline]
    pub const fn new((wx1, wy1): Point<isize>, (wx2, wy2): Point<isize>) -> Self {
        let (wx1, wx2) = if wx1 <= wx2 { (wx1, wx2) } else { (wx2, wx1) };
        let (wy1, wy2) = if wy1 <= wy2 { (wy1, wy2) } else { (wy2, wy1) };
        Self { wx1, wy1, wx2, wy2 }
    }
}

/// Macro that maps over an [`Option`], for use in const contexts.
#[cfg(feature = "clip")]
macro_rules! map_option {
    ($option:expr, $some:ident => $mapped:expr) => {
        match $option {
            None => None,
            Some($some) => Some($mapped),
        }
    };
}

#[cfg(feature = "clip")]
pub(crate) use map_option;
