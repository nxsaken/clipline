//! ## Clipping
//!
//! This module provides the [`Clip`] type representing a rectangular clipping region, as well as
//! methods for constructing iterators over clipped directed line segments of various types.

pub mod diagonal;
pub mod kuzmin;
pub mod signed_axis;

/// A generic rectangular region defined by its minimum and maximum [corners](crate::Point).
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Default)]
pub struct Clip<T> {
    x1: T,
    y1: T,
    x2: T,
    y2: T,
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
