use crate::math::{ops, CxC, C, S, U};

mod axis;
mod bresenham_case;
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
        let clip = Self { x0, y0, x1, y1 };
        Some(clip)
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
        // SAFETY: width > 0 => width - 1 cannot overflow.
        let x1 = unsafe { ops::unchecked_sub_sign(width, S::Pos) };
        // SAFETY: height > 0 => height - 1 cannot overflow.
        let y1 = unsafe { ops::unchecked_sub_sign(height, S::Pos) };
        let clip = Self { x0: 0, y0: 0, x1, y1 };
        Some(clip)
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
    /// Checks if the u-component of the bounding box
    /// of a half-open line segment misses this clipping region.
    ///
    /// # Safety
    ///
    /// * `wu0 <= wu1`.
    /// * `FU == u1 < u0`.
    #[inline]
    const unsafe fn rejects_bbox_u<const FU: bool>(wu0: C, wu1: C, u0: C, u1: C) -> bool {
        match FU {
            false => u1 <= wu0 || wu1 < u0,
            true => u0 < wu0 || wu1 <= u1,
        }
    }

    /// Checks if the bounding box of a half-open line segment misses this clipping region.
    ///
    /// # Safety
    ///
    /// * `FX == x1 < x0`.
    /// * `FY == y1 < y0`.
    #[inline]
    const unsafe fn rejects_bbox<const FX: bool, const FY: bool>(
        &self,
        (x0, y0): CxC,
        (x1, y1): CxC,
    ) -> bool {
        // SAFETY:
        // * self.x0 <= self.x1.
        // * FX == x1 < x0.
        let reject_x = unsafe { Self::rejects_bbox_u::<FX>(self.x0, self.x1, x0, x1) };
        // SAFETY:
        // * self.y0 <= self.y1.
        // * FY == y1 < y0.
        let reject_y = unsafe { Self::rejects_bbox_u::<FY>(self.y0, self.y1, y0, y1) };
        reject_x || reject_y
    }

    /// Checks if `u0` of the line segment lies before the u-entry
    /// of the region, and if `u1` lies after the u-exit.
    ///
    /// # Safety
    ///
    /// * `wu0 <= wu1`.
    /// * `FU == u1 < u0`.
    #[inline]
    const unsafe fn maybe_iou<const FU: bool>(wu0: C, wu1: C, u0: C, u1: C) -> (bool, bool) {
        match FU {
            false => (u0 < wu0, wu1 < u1),
            true => (wu1 < u0, u1 < wu0),
        }
    }

    /// Returns the offset between `u0` of the line segment
    /// and the u-entry of this clipping region.
    ///
    /// # Safety
    ///
    /// * `wu0 <= wu1`.
    /// * `u0` must lie before the u-entry.
    #[inline]
    const unsafe fn du0<const FU: bool>(wu0: C, wu1: C, u0: C) -> U {
        match FU {
            // SAFETY: u0 < wu0 because u0 lies before the u-entry.
            false => unsafe { ops::unchecked_abs_diff(wu0, u0) },
            // SAFETY: wu1 < u0 because u0 lies before the u-entry.
            true => unsafe { ops::unchecked_abs_diff(u0, wu1) },
        }
    }

    /// Returns the offset between `u0` of the line segment
    /// and the u-exit of this clipping region.
    ///
    /// # Safety
    ///
    /// `u0` must lie before the u-exit.
    #[inline]
    const unsafe fn du1<const FU: bool>(wu0: C, wu1: C, u0: C) -> U {
        match FU {
            // SAFETY: u0 < wu1 because u0 lies before the u-exit.
            false => unsafe { ops::unchecked_abs_diff(wu1, u0) },
            // SAFETY: wu0 < u0 because u0 lies before the u-exit.
            true => unsafe { ops::unchecked_abs_diff(u0, wu0) },
        }
    }
}
