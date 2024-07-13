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

/// Primitive numeric type.
trait Num: Copy + Eq + core::hash::Hash + core::fmt::Debug + Default {}

macro_rules! num_impl {
    ($signed:ty, $unsigned:ty) => {
        impl Num for $signed {}
        impl Num for $unsigned {}
    };
}

num_impl!(i8, u8);
num_impl!(i16, u16);
num_impl!(i32, u32);
num_impl!(i64, u64);
num_impl!(i128, u128);
num_impl!(isize, usize);

/// Numeric type for representing coordinates.
trait Coord: Num {
    /// Offset representation.
    type Delta: Num;
    /// Error representation.
    type Error: Num;
    /// Double offset representation.
    type Delta2: Num;
}

macro_rules! coord_impl {
    ($signed:ty, $unsigned:ty, $double_signed:ty, $double_unsigned:ty) => {
        impl Coord for $signed {
            type Delta = $unsigned;
            type Error = $double_signed;
            type Delta2 = $double_unsigned;
        }
        impl Coord for $unsigned {
            type Delta = $unsigned;
            type Error = $double_signed;
            type Delta2 = $double_unsigned;
        }
    };
}

coord_impl!(i8, u8, i16, u16);
coord_impl!(i16, u16, i32, u32);
coord_impl!(i32, u32, i64, u64);
coord_impl!(i64, u64, i128, u128);
#[cfg(target_pointer_width = "16")]
coord_impl!(isize, usize, i32, u32);
#[cfg(target_pointer_width = "32")]
coord_impl!(isize, usize, i64, u64);
#[cfg(target_pointer_width = "64")]
coord_impl!(isize, usize, i128, u128);

// TODO(#12): support all integer types, including unsigned.
/// A generic point on a Cartesian plane.
pub type Point<T> = (T, T);

/// A generic offset between two [points](Point) on a Cartesian plane.
type Delta<T> = (<T as Coord>::Delta, <T as Coord>::Delta);

/// A generic double offset between two [points](Point) on a Cartesian plane.
type Delta2<T> = (<T as Coord>::Delta2, <T as Coord>::Delta2);
