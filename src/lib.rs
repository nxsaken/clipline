//! Line segment rasterization with pixel-perfect clipping.

#![no_std]
//
#![forbid(clippy::arithmetic_side_effects)]
#![forbid(clippy::undocumented_unsafe_blocks)]
#![forbid(clippy::unnecessary_safety_comment)]
#![forbid(clippy::missing_safety_doc)]
#![forbid(clippy::unnecessary_safety_doc)]
//
#![deny(missing_docs)]
//
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]

mod axis;
mod bidiagonal;
mod bresenham;
mod bresenham_case;
mod diagonal;
mod math;

pub use axis::{Axis, Axis0, Axis1};
pub use bidiagonal::Bidiagonal;
pub use bresenham::Bresenham;
pub use bresenham_case::{BresenhamCase, BresenhamFast, BresenhamSlow};
pub use diagonal::Diagonal;
pub use math::{CxC, SxS, UxU, C, S, U};
