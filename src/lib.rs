//! # Clipline
//!
//! This crate provides efficient iterators for directed line segments:
//!
//! - [Bresenham] and [octant](BresenhamOctant) iterators.
//! - [Diagonal] and [quadrant](DiagonalQuadrant) iterators.
//! - [Orthogonal], [unsigned](AxisAligned) and [signed axis-aligned](SignedAxisAligned) iterators.
//!
//! All iterators support clipping to [rectangular regions](Clip).

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

mod bresenham;
mod clip;
mod diagonal;
mod orthogonal;

pub use clip::Clip;

pub use orthogonal::{
    AxisAligned, Horizontal, NegativeAxisAligned, NegativeHorizontal, NegativeVertical, Orthogonal,
    PositiveAxisAligned, PositiveHorizontal, PositiveVertical, SignedAxisAligned, SignedHorizontal,
    SignedVertical, Vertical,
};

pub use bresenham::{
    Bresenham, Octant as BresenhamOctant, Octant0 as BresenhamOctant0, Octant1 as BresenhamOctant1,
    Octant2 as BresenhamOctant2, Octant3 as BresenhamOctant3, Octant4 as BresenhamOctant4,
    Octant5 as BresenhamOctant5, Octant6 as BresenhamOctant6, Octant7 as BresenhamOctant7,
};

pub use diagonal::{
    Diagonal, Quadrant as DiagonalQuadrant, Quadrant0 as DiagonalQuadrant0,
    Quadrant1 as DiagonalQuadrant1, Quadrant2 as DiagonalQuadrant2, Quadrant3 as DiagonalQuadrant3,
};

// TODO(#12): support all integer types, including unsigned.
/// A generic point on a Cartesian plane.
pub type Point<T> = (T, T);

/// A generic offset between two [points](Point) on a Cartesian plane.
pub type Offset<T> = (T, T);
