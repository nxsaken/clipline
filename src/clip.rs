use crate::math::{ops, CxC, C, S, U};

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

impl Clip {
    /// Checks if a half-open line segment trivially misses this clipping region.
    ///
    /// # Safety
    ///
    /// - `sx` must match the direction from `x0` to `x1`.
    /// - `sy` must match the direction from `y0` to `y1`.
    const unsafe fn rejects_trivial(&self, x0: C, y0: C, x1: C, y1: C, sx: S, sy: S) -> bool {
        let miss_x = match sx {
            S::P => x1 <= self.x0 || self.x1 < x0,
            S::N => x0 < self.x0 || self.x1 <= x1,
        };
        let miss_y = match sy {
            S::P => y1 <= self.y0 || self.y1 < y0,
            S::N => y0 < self.y0 || self.y1 <= y1,
        };
        miss_x || miss_y
    }

    /// Checks if `u0` of the line segment lies before the u-entry
    /// of the region, and if `u1` lies after the u-exit.
    ///
    /// # Safety
    ///
    /// `su` must match the direction from `u0` to `u1`.
    const unsafe fn iou<const YX: bool>(&self, u0: C, u1: C, su: S) -> (bool, bool) {
        let (wu0, wu1) = if YX { (self.y0, self.y1) } else { (self.x0, self.x1) };
        let (iu, ou) = match su {
            S::P => (u0 < wu0, wu1 < u1),
            S::N => (wu1 < u0, u1 < wu0),
        };
        (iu, ou)
    }

    /// Alias for [`Self::iou::<false>`], with `u = x`.
    const unsafe fn iox(&self, x0: C, x1: C, sx: S) -> (bool, bool) {
        #[expect(unsafe_op_in_unsafe_fn)]
        self.iou::<false>(x0, x1, sx)
    }

    /// Alias for [`Self::iov::<false>`], with `v = y`.
    const unsafe fn ioy(&self, y0: C, y1: C, sy: S) -> (bool, bool) {
        #[expect(unsafe_op_in_unsafe_fn)]
        self.iou::<true>(y0, y1, sy)
    }

    /// Returns the offset between `u0` of the line segment
    /// and the u-entry of this clipping region.
    ///
    /// # Safety
    ///
    /// `u0` must lie before the u-entry.
    const unsafe fn du0<const YX: bool>(&self, u0: C, su: S) -> U {
        let (wu0, wu1) = if YX { (self.y0, self.y1) } else { (self.x0, self.x1) };
        match su {
            // SAFETY: u0 < wu0 because u0 lies before the u-entry.
            S::P => unsafe { ops::d_unchecked(wu0, u0) },
            // SAFETY: wu1 < u0 because u0 lies before the u-entry.
            S::N => unsafe { ops::d_unchecked(u0, wu1) },
        }
    }

    /// Alias for [`Self::du0::<false>`], with `u = x`.
    const unsafe fn dx0(&self, x0: C, sx: S) -> U {
        #[expect(unsafe_op_in_unsafe_fn)]
        self.du0::<false>(x0, sx)
    }

    /// Alias for [`Self::du0::<true>`], with `u = y`.
    const unsafe fn dy0(&self, y0: C, sy: S) -> U {
        #[expect(unsafe_op_in_unsafe_fn)]
        self.du0::<true>(y0, sy)
    }

    /// Returns the offset between `u0` of the line segment
    /// and the u-exit of this clipping region.
    ///
    /// # Safety
    ///
    /// `u0` must lie before the u-exit.
    const unsafe fn du1<const YX: bool>(&self, u0: C, su: S) -> U {
        let (wu0, wu1) = if YX { (self.y0, self.y1) } else { (self.x0, self.x1) };
        match su {
            // SAFETY: u0 < wu1 because u0 lies before the u-exit.
            S::P => unsafe { ops::d_unchecked(wu1, u0) },
            // SAFETY: wu0 < u0 because u0 lies before the u-exit.
            S::N => unsafe { ops::d_unchecked(u0, wu0) },
        }
    }

    /// Alias for [`Self::du1::<false>`], with `u = x`.
    const unsafe fn dx1(&self, x0: C, sx: S) -> U {
        #[expect(unsafe_op_in_unsafe_fn)]
        self.du1::<false>(x0, sx)
    }

    /// Alias for [`Self::du1::<true>`], with `u = y`.
    const unsafe fn dy1(&self, y0: C, sy: S) -> U {
        #[expect(unsafe_op_in_unsafe_fn)]
        self.du1::<true>(y0, sy)
    }
}
