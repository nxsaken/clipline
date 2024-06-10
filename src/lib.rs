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
    clippy::wildcard_imports
)]

mod axis_aligned;
#[cfg(feature = "bresenham")]
mod bresenham;
mod diagonal;
#[cfg(feature = "kuzmin")]
mod kuzmin;

pub use axis_aligned::{
    AxisAligned, Horizontal, NegativeAxisAligned, NegativeHorizontal, NegativeVertical,
    PositiveAxisAligned, PositiveHorizontal, PositiveVertical, SignedAxisAligned, SignedHorizontal,
    SignedVertical, Vertical,
};

#[cfg(feature = "bresenham")]
pub use bresenham::{
    Bresenham, Octant as BresenhamOctant, Octant0 as BresenhamOctant0, Octant1 as BresenhamOctant1,
    Octant2 as BresenhamOctant2, Octant3 as BresenhamOctant3, Octant4 as BresenhamOctant4,
    Octant5 as BresenhamOctant5, Octant6 as BresenhamOctant6, Octant7 as BresenhamOctant7,
};

pub use diagonal::{
    Diagonal, Quadrant as DiagonalQuadrant, Quadrant0 as DiagonalQuadrant0,
    Quadrant1 as DiagonalQuadrant1, Quadrant2 as DiagonalQuadrant2, Quadrant3 as DiagonalQuadrant3,
};

// TODO(#12): support all integer types.
/// Generic point on a Cartesian plane.
pub type Point<T> = (T, T);
