use crate::math::{CxC, C};

mod axis;
mod diagonal;

/// A closed[^1] rectangular clipping region defined by its minimum and maximum corners.
///
/// [^1]: Both corners are included.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Clip {
    x0: C,
    y0: C,
    x1: C,
    y1: C,
}

impl Clip {
    /// Constructs a [`Clip`] with corners `(x0, y0)` and `(x1, y1)`.
    ///
    /// Returns [`None`] if `x1 < x0` or `y1 < y0`.
    #[inline]
    #[must_use]
    pub const fn with_min_max((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        if x1 < x0 || y1 < y0 {
            return None;
        }
        Some(Self { x0, y0, x1, y1 })
    }

    /// Constructs a [`Clip`] with corners `(0, 0)` and `(width - 1, height - 1)`.
    ///
    /// Returns [`None`] if `width <= 0` or `height <= 0`.
    #[inline]
    #[must_use]
    pub const fn with_size(width: C, height: C) -> Option<Self> {
        if width <= 0 || height <= 0 {
            return None;
        }
        Some(Self { x0: 0, y0: 0, x1: width.wrapping_sub(1), y1: height.wrapping_sub(1) })
    }

    /// Returns a copy of this [`Clip`].
    #[inline]
    #[must_use]
    pub const fn copy(&self) -> Self {
        Self { ..*self }
    }

    /// Returns the minimum corner.
    #[inline]
    #[must_use]
    pub const fn min(&self) -> CxC {
        (self.x0, self.y0)
    }

    /// Returns the maximum corner.
    #[inline]
    #[must_use]
    pub const fn max(&self) -> CxC {
        (self.x1, self.y1)
    }

    /// Checks whether the given point is inside the region.
    #[inline]
    #[must_use]
    pub const fn point(&self, (x, y): CxC) -> bool {
        self.x0 <= x && x <= self.x1 && self.y0 <= y && y <= self.y1
    }
}
