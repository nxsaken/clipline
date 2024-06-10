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
#![forbid(
    clippy::arithmetic_side_effects,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::missing_safety_doc,
    clippy::unnecessary_safety_doc
)]
#![deny(missing_docs)]
#![warn(clippy::nursery, clippy::cargo, clippy::pedantic)]
#![allow(
    clippy::match_bool,
    clippy::module_name_repetitions,
    clippy::inline_always,
    clippy::similar_names,
    clippy::if_not_else,
    clippy::cast_lossless
)]

mod bresenham;
mod clip;
mod diagonal;
mod math;
mod orthogonal;
mod symmetry;
mod utils;

pub use clip::Clip;
pub use math::Point;

pub use bresenham::{
    Bresenham, Octant as BresenhamOctant, Octant0 as BresenhamOctant0, Octant1 as BresenhamOctant1,
    Octant2 as BresenhamOctant2, Octant3 as BresenhamOctant3, Octant4 as BresenhamOctant4,
    Octant5 as BresenhamOctant5, Octant6 as BresenhamOctant6, Octant7 as BresenhamOctant7,
};
pub use diagonal::{
    Diagonal, Quadrant as DiagonalQuadrant, Quadrant0 as DiagonalQuadrant0,
    Quadrant1 as DiagonalQuadrant1, Quadrant2 as DiagonalQuadrant2, Quadrant3 as DiagonalQuadrant3,
};
pub use orthogonal::{
    AxisAligned, Horizontal, NegativeAxisAligned, NegativeHorizontal, NegativeVertical, Orthogonal,
    PositiveAxisAligned, PositiveHorizontal, PositiveVertical, SignedAxisAligned, SignedHorizontal,
    SignedVertical, Vertical,
};

/// ## Project TO-DOs
///
/// ### Optimized internal iterators:
/// - Run-slice Bresenham's algorithm
/// - Scanline on a generic slice
/// - Do not recalculate the array index of the point from scratch
///   - I<i8/u8> -> u16 index, etc.
struct _Todo;
