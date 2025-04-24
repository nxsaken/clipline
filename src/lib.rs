//! # clipline
//!
//! Efficient rasterization of line segments with pixel-perfect [clipping][clip].
//!
//! ## Overview
//!
//! - Provides iterators for clipped and unclipped rasterized line segments.
//!   - Eliminates bounds checking: clipped line segments are guaranteed to be within the region.
//!   - Guarantees clipped line segments match the unclipped versions of themselves.
//! - Supports signed and unsigned integer coordinates of most sizes.
//!   - Uses integer arithmetic only.
//!   - Prevents overflow and division by zero, forbids `clippy::arithmetic_side_effects`.
//!   - Defines the iterators on the entire domains of the underlying numeric types.
//! - Usable in `const` contexts and `#![no_std]` environments.
//!
//! ## Usage
//!
//! - **Unclipped** iterators are created using constructors: [`AnyOctant::<i8>::new`].
//! - For **clipped** iterators:
//!   - Define a rectangular clipping region using the [`Clip`] type.
//!   - Construct the desired iterator, e.g. [`AnyOctant::<i8>`]:
//!     - **Builder style**: using one of the methods on [`Clip`], e.g. [`Clip::<i8>::any_octant`].
//!       Should be preferred, as it avoids specifying the numeric type again.
//!     - **Constructor style**: [`AnyOctant::<i8>::clip`].
//!
//! ### Octant iterators
//!
//! For an arbitrary line segment, use the [`AnyOctant`] iterator,
//! which determines the type of the line segment at runtime
//! and handles it with a specialized iterator.
//!
//! If you know more about the line segment, you can use an iterator
//! from the [axis-aligned](Axis) or [diagonal](Diagonal) families (more below),
//! or the generic [`Octant`] backed by one of the eight cases of [Bresenham's algorithm][bres]:
//!
//! - [`Octant0`]: `x` and `y` both increase, `x` changes faster than `y`.
//! - [`Octant1`]: `x` increases and `y` decreases, `y` changes faster than `x`.
//! - [`Octant2`]: `x` decreases and `y` increases, `x` changes faster than `y`.
//! - [`Octant3`]: `x` and `y` both decrease, `y` changes faster than `x`.
//! - [`Octant4`]: `x` and `y` both increase, `x` changes faster than `y`.
//! - [`Octant5`]: `x` increases and `y` decreases, `y` changes faster than `x`.
//! - [`Octant6`]: `x` decreases and `y` increases, `x` changes faster than `y`.
//! - [`Octant7`]: `x` and `y` both decrease, `y` changes faster than `x`.
//!
//! ### Axis-aligned iterators
//!
//! For an arbitrary axis-aligned line segment, use the [`AnyAxis`] iterator,
//! which determines both the axis-alignment and direction at runtime.
//!
//! If you know the axis-alignment of the line segment but not the direction,
//! use the generic [`Axis`] iterator, or one of its type aliases:
//!
//! - [`Axis0`]: horizontal, runtime direction.
//! - [`Axis1`]: vertical, runtime direction.
//!
//! If you also know the direction, use the generic [`SignedAxis`] iterator,
//! or one of its type aliases:
//!
//! - [`PositiveAxis`]/[`NegativeAxis`]: fixed direction, generic orientation.
//! - [`SignedAxis0`]/[`SignedAxis1`]: fixed orientation, generic direction.
//! - [`PositiveAxis0`]/[`NegativeAxis0`]/[`PositiveAxis1`]/[`NegativeAxis1`]: both fixed.
//!
//! ### Diagonal iterators
//!
//! For an arbitrary diagonal line segment, use the [`AnyDiagonal`] iterator,
//! which determines the orientation at runtime.
//!
//! If you know the orientation, use the generic [`Diagonal`] iterator,
//! or one of its type aliases:
//!
//! - [`Diagonal0`]: `x` and `y` both increase.
//! - [`Diagonal1`]: `x` increases and `y` decreases.
//! - [`Diagonal2`]: `x` decreases and `y` increases.
//! - [`Diagonal3`]: `x` and `y` both decrease.
//!
//! ## Example
//!
//! ```
//! use clipline::{AnyOctant, Clip, Diagonal0, Point};
//!
//! /// Width of the pixel buffer.
//! const WIDTH: usize = 64;
//! /// Height of the pixel buffer.
//! const HEIGHT: usize = 48;
//!
//! /// Pixel color value.
//! const RGBA: u32 = 0xFFFFFFFF;
//!
//! /// A function that operates on a single pixel in a pixel buffer.
//! ///
//! /// ## Safety
//! /// `(x, y)` must be inside the `buffer`.
//! unsafe fn draw(buffer: &mut [u32], (x, y): Point<i8>, rgba: u32) {
//!     let index = y as usize * WIDTH + x as usize;
//!     debug_assert!(index < buffer.len());
//!     *buffer.get_unchecked_mut(index) = rgba;
//! }
//!
//! fn main() {
//!     let mut buffer = [0_u32; WIDTH * HEIGHT];
//!
//!     // The clipping region is closed/inclusive, thus 1 needs to be subtracted from the size.
//!     let clip = Clip::<i8>::new((0, 0), (WIDTH as i8 - 1, HEIGHT as i8 - 1)).unwrap();
//!
//!     // `Clip` has convenience methods for the general iterators.
//!     clip.any_octant((-128, -100), (100, 80))
//!         // None if the line segment is completely invisible.
//!         // You might want to handle that case differently.
//!         .unwrap()
//!         // clipped to [(0, 1), ..., (58, 47)]
//!         .for_each(|xy| {
//!             // SAFETY: (x, y) has been clipped to the buffer.
//!             unsafe { draw(&mut buffer, xy, RGBA) }
//!         });
//!
//!     // Alternatively, use the iterator constructors.
//!     AnyOctant::<i8>::clip((12, 0), (87, 23), &clip)
//!         .into_iter()
//!         .flatten()
//!         // clipped to [(12, 0), ..., (63, 16)]
//!         .for_each(|xy| {
//!             // SAFETY: (x, y) has been clipped to the buffer.
//!             unsafe { draw(&mut buffer, xy, RGBA) }
//!         });
//!
//!     // Horizontal and vertical line segments.
//!     clip.axis_0(32, 76, -23)
//!         .unwrap()
//!         // clipped to [(63, 32), ..., (0, 32)]
//!         .for_each(|xy| {
//!             // SAFETY: (x, y) has been clipped to the buffer.
//!             unsafe { draw(&mut buffer, xy, RGBA) }
//!         });
//!
//!     clip.axis_1(32, -23, 76)
//!         .unwrap()
//!         // clipped to [(32, 0), ..., (32, 47)]
//!         .for_each(|xy| {
//!             // SAFETY: (x, y) has been clipped to the buffer.
//!             unsafe { draw(&mut buffer, xy, RGBA) }
//!         });
//!
//!     // Unclipped iterators are also available.
//!     // (-2, -2) -> (12, 12) is covered by Diagonal0, we can construct it directly.
//!     Diagonal0::<i8>::new((-2, -2), (12, 12))
//!         .unwrap()
//!         // Need to check every pixel to avoid going out of bounds.
//!         .filter(|&xy| clip.point(xy))
//!         .for_each(|xy| {
//!             // SAFETY: (x, y) is inside the buffer.
//!             unsafe { draw(&mut buffer, xy, RGBA) }
//!         });
//! }
//! ```
//!
//! ## Limitations
//!
//! * To support usage in `const` contexts, types must have an inherent implementation for every
//!   supported numeric type instead of relying on a trait. This and Rust's lack of support for
//!   function overloading means that the numeric type parameter must always be specified.
//! * Currently, only half-open line segments can be iterated. This allows [`ExactSizeIterator`]
//!   to be implemented for all types. Inclusive iterators are tracked in [#1].
//!
//! ## Feature flags
//!
//! - `octant_64`
//!   * Enables [`Octant`] and [`AnyOctant`] over [`i64`]/[`u64`] for all targets,
//!     and over [`isize`]/[`usize`] for 64-bit targets.
//!   * Use this only if you need the full 64-bit range, as [`Octant`] will use
//!     [`u128`] and [`i128`] for some calculations.
//! - `try_fold`, `is_empty` *(nightly-only)*
//!   * Enable optimized [`Iterator::try_fold`] and [`ExactSizeIterator::is_empty`] implementations.
//!
//! ## References
//!
//! `clipline` is inspired by the following papers:
//!
//! * [A fast two-dimensional line clipping algorithm via line encoding][spy],
//!   Mark S. Sobkow, Paul Pospisil, Yee-Hong Yang, 1987.
//! * [A new approach to parametric line clipping][dorr],
//!   Michael Dörr, 1990.
//! * [Bresenham's Line Generation Algorithm with Built-in Clipping][kuzmin],
//!   Yevgeny P. Kuzmin, 1995.
//!
//! [clip]: https://en.wikipedia.org/wiki/Line_clipping
//! [bres]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
//! [spy]: https://doi.org/10.1016/0097-8493(87)90061-6
//! [dorr]: https://doi.org/10.1016/0097-8493(90)90067-8
//! [kuzmin]: https://doi.org/10.1111/1467-8659.1450275
//! [#1]: https://github.com/nxsaken/clipline/issues/1

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
    clippy::module_name_repetitions,
    clippy::inline_always,
    clippy::similar_names,
    clippy::if_not_else,
    clippy::cast_lossless
)]

mod axis_aligned;
mod clip;
mod diagonal;
mod macros;
mod math;
mod octant;

pub use clip::Clip;
pub use math::Point;

pub use octant::AnyOctant;
pub use octant::Octant;
pub use octant::Octant0;
pub use octant::Octant1;
pub use octant::Octant2;
pub use octant::Octant3;
pub use octant::Octant4;
pub use octant::Octant5;
pub use octant::Octant6;
pub use octant::Octant7;

pub use diagonal::AnyDiagonal;
pub use diagonal::Diagonal;
pub use diagonal::Diagonal0;
pub use diagonal::Diagonal1;
pub use diagonal::Diagonal2;
pub use diagonal::Diagonal3;

pub use axis_aligned::AnyAxis;
pub use axis_aligned::Axis;
pub use axis_aligned::Axis0;
pub use axis_aligned::Axis1;
pub use axis_aligned::NegativeAxis;
pub use axis_aligned::NegativeAxis0;
pub use axis_aligned::NegativeAxis1;
pub use axis_aligned::PositiveAxis;
pub use axis_aligned::PositiveAxis0;
pub use axis_aligned::PositiveAxis1;
pub use axis_aligned::SignedAxis;
pub use axis_aligned::SignedAxis0;
pub use axis_aligned::SignedAxis1;
