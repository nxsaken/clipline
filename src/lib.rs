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

pub use bresenham::Bresenham;
pub use bresenham::Octant as BresenhamOctant;
pub use bresenham::Octant0 as BresenhamOctant0;
pub use bresenham::Octant1 as BresenhamOctant1;
pub use bresenham::Octant2 as BresenhamOctant2;
pub use bresenham::Octant3 as BresenhamOctant3;
pub use bresenham::Octant4 as BresenhamOctant4;
pub use bresenham::Octant5 as BresenhamOctant5;
pub use bresenham::Octant6 as BresenhamOctant6;
pub use bresenham::Octant7 as BresenhamOctant7;

pub use diagonal::Diagonal;
pub use diagonal::Quadrant as DiagonalQuadrant;
pub use diagonal::Quadrant0 as DiagonalQuadrant0;
pub use diagonal::Quadrant1 as DiagonalQuadrant1;
pub use diagonal::Quadrant2 as DiagonalQuadrant2;
pub use diagonal::Quadrant3 as DiagonalQuadrant3;

pub use orthogonal::AxisAligned;
pub use orthogonal::Horizontal;
pub use orthogonal::NegativeAxisAligned;
pub use orthogonal::NegativeHorizontal;
pub use orthogonal::NegativeVertical;
pub use orthogonal::Orthogonal;
pub use orthogonal::PositiveAxisAligned;
pub use orthogonal::PositiveHorizontal;
pub use orthogonal::PositiveVertical;
pub use orthogonal::SignedAxisAligned;
pub use orthogonal::SignedHorizontal;
pub use orthogonal::SignedVertical;
pub use orthogonal::Vertical;
