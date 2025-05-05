use crate::math::{ops, CxC, SxS, UxU, C, I2, S, U, U2};

/// An iterator over the rasterized points of a half-open line segment,
/// using one of the two cases of Bresenham's algorithm.
///
/// The `YX` parameter determines which class of line segments is covered:
/// - `false`: segments with a "slow" slope (`dy <= dx`).
/// - `true`: segments with a "fast" slope (`dx < dy`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BresenhamCase<const YX: bool> {
    /// The current start coordinate along the major axis.
    u0: C,
    /// The current start coordinate along the minor axis.
    v0: C,
    /// The accumulated error.
    err: I2,
    /// The offset between the initial start and end coordinates along the major axis.
    du: U,
    /// The offset between the initial start and end coordinates along the minor axis.
    dv: U,
    /// The step sign along the major axis.
    su: S,
    /// The step sign along the minor axis.
    sv: S,
    /// The current end coordinate along the major axis.
    u1: C,
}

/// A [`BresenhamCase`] iterator over a half-open line segment with a "slow" slope (`dy <= dx`).
pub type BresenhamSlow = BresenhamCase<false>;

/// A [`BresenhamCase`] iterator over a half-open line segment with a "fast" slope (`dx < dy`).
pub type BresenhamFast = BresenhamCase<true>;

impl<const YX: bool> BresenhamCase<YX> {
    /// Constructs a [`BresenhamCase`] iterator from its internal parts.
    ///
    /// # Safety
    ///
    /// 1. `du` must match the offset from `u0` to `u1`.
    /// 2. `dv <= du` (normalized line segments are gently sloped).
    /// 3. `su` must match the direction from `u0` to `u1`.
    /// 4. `err` must be initialized to `dv - (du / 2) + (du % 2)`.
    #[inline]
    #[must_use]
    pub const unsafe fn from_parts(
        (u0, v0): CxC,
        (du, dv): UxU,
        (su, sv): SxS,
        err: I2,
        u1: C,
    ) -> Self {
        Self { u0, v0, err, du, dv, su, sv, u1 }
    }

    /// Constructs a [`BresenhamCase`] iterator from the parameters of a normalized half-open line segment.
    ///
    /// # Safety
    ///
    /// 1. `du` must match the offset from `u0` to `u1`.
    /// 2. `dv` must be less or equal to `du` (normalized line segments are gently sloped).
    /// 3. `su` must match the direction from `u0` to `u1`.
    #[inline]
    #[must_use]
    pub const unsafe fn from_normalized(
        (u0, v0): CxC,
        (du, dv): UxU,
        (su, sv): SxS,
        u1: C,
    ) -> Self {
        debug_assert!(du == u0.abs_diff(u1));
        debug_assert!(dv <= du);
        debug_assert!((u0 <= u1) == matches!(su, S::P));
        debug_assert!((u1 < u0) == matches!(su, S::N));

        let err = {
            let (half_du, rem) = (du.wrapping_shr(2), du & 1);
            let half_du_adj = half_du.wrapping_add(rem);
            I2::wrapping_sub(dv as I2, half_du_adj as I2)
        };

        // SAFETY:
        // 1. du matches the offset from u0 to u1.
        // 2. dv <= du.
        // 3. su matches the direction from u0 to u1.
        // 4. err is initialized to dv - (du / 2) + (du % 2).
        unsafe { Self::from_parts((u0, v0), (du, dv), (su, sv), err, u1) }
    }

    /// Returns a [`BresenhamCase`] iterator over a half-open line segment
    /// if the line segment's slope matches `YX`, otherwise returns [`None`].
    ///
    /// The `YX` parameter determines which class of line segments is covered:
    /// - `false`: segments with a "slow" slope (`dy <= dx`).
    /// - `true`: segments with a "fast" slope (`dx < dy`).
    #[inline]
    #[must_use]
    pub const fn from_points((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        let (sx, dx) = ops::sd(x0, x1);
        let (sy, dy) = ops::sd(y0, y1);

        if YX && dy <= dx || !YX && dx < dy {
            return None;
        }

        let (u0, v0, u1, su, sv, du, dv) =
            if YX { (y0, x0, y1, sy, sx, dy, dx) } else { (x0, y0, x1, sx, sy, dx, dy) };

        // SAFETY: the line segment has been normalized.
        let this = unsafe { Self::from_normalized((u0, v0), (du, dv), (su, sv), u1) };
        Some(this)
    }
}

impl<const YX: bool> BresenhamCase<YX> {
    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.u0 == self.u1
    }

    /// Returns the remaining length of this iterator.
    #[inline]
    #[must_use]
    pub const fn length(&self) -> U {
        self.u0.abs_diff(self.u1)
    }

    /// Returns the point at the start of the iterator.
    /// This does not advance the iterator.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn head(&self) -> Option<CxC> {
        if self.is_done() {
            return None;
        }
        let (x0, y0) = if YX { (self.v0, self.u0) } else { (self.u0, self.v0) };
        Some((x0, y0))
    }

    /// Consumes and returns the point at the start of the iterator.
    /// This advances the iterator forwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_head(&mut self) -> Option<CxC> {
        let Some((x0, y0)) = self.head() else { return None };
        if 0 <= self.err {
            self.v0 = self.v0.wrapping_add(self.sv as C);
            self.err = self.err.wrapping_sub_unsigned(self.du as U2);
        }
        self.u0 = self.u0.wrapping_add(self.su as C);
        self.err = self.err.wrapping_add_unsigned(self.dv as U2);
        Some((x0, y0))
    }
}

impl<const YX: bool> Iterator for BresenhamCase<YX> {
    type Item = CxC;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_head()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO: fallible version.
        let len = usize::from(self.length());
        (len, Some(len))
    }
}

impl<const YX: bool> core::iter::FusedIterator for BresenhamCase<YX> {}

impl<const YX: bool> ExactSizeIterator for BresenhamCase<YX> {}
