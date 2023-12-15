//! Efficient scan conversion (rasterization) of line segments with clipping to a rectangular window.
//!
//! The key advantage of `clipline` over vanilla Bresenham is that it eliminates the need for
//! bounds checking on every pixel, which speeds up line drawing. Furthermore, the clipping uses
//! integer arithmetic, producing pixel-perfect endpoints. This sets it apart from floating-point
//! clipping algorithms like Cohen-Sutherland, which may distort the line due to rounding errors.
//!
//! ## Usage
//! This crate provides two ways of performing scan conversion: the [`clipline`] function, and the
//! [`Clipline`] iterator. The former is slightly more optimized, the latter allows external iteration.
//! Both methods can be toggled with the `func` and `iter` features (both enabled by default).

#![no_std]

#[cfg(feature = "func")]
mod func;
#[cfg(feature = "iter")]
mod iter;
#[cfg(any(feature = "func", feature = "iter"))]
mod util;

#[cfg(feature = "func")]
pub use func::clipline;
#[cfg(feature = "iter")]
pub use iter::{AbsDiff, Clipline, Gentleham, Hlipline, Steepnham, Vlipline};
#[cfg(any(feature = "func", feature = "iter"))]
pub use util::Constant;
