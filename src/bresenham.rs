use crate::math::{CxC, SxS, UxU, C, I2, S, U};

/// An iterator over the points of a rasterized, half-open line segment,
/// backed by one of the two cases of [Bresenham's algorithm][0].
///
/// The `YX` parameter determines which class of line segments is covered:
/// - `YX = false`: "gentle" segments (`dy <= dx`).
/// - `YX = true`: "steep" segments (`dx < dy`).
///
/// [0]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
pub struct Bresenham<const YX: bool> {
    /// The current coordinate along the fast axis.
    u0: C,
    /// The current coordinate along the slow axis.
    v0: C,
    /// The accumulated error.
    err: I2,
    /// The absolute offset between the start and end coordinates along the fast axis.
    du: U,
    /// The absolute offset between the start and end coordinates along the slow axis.
    dv: U,
    /// The step sign along the fast axis.
    su: S,
    /// The step sign along the slow axis.
    sv: S,
    /// The end coordinate along the fast axis.
    u1: C,
}

impl<const YX: bool> Bresenham<YX> {
    /// Constructs a [`Bresenham`] iterator from its internal parts.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the parameters satisfy the following:
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

    /// Constructs a [`Bresenham`] iterator from the parameters of a normalized half-open line segment.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the parameters satisfy the following:
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

    /// Returns a [`Bresenham`] iterator for a half-open line segment
    /// if the line segment's slope matches `YX`, otherwise returns [`None`].
    ///
    /// - For `YX = false`, accepts gently-sloped segments: `dy <= dx`.
    /// - For `YX = true`, accepts steeply-sloped segments: `dx < dy`.
    #[inline]
    #[must_use]
    pub const fn from_points((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        #[expect(clippy::cast_sign_loss)]
        const fn sign_delta(c0: C, c1: C) -> (S, U) {
            if c0 <= c1 {
                (S::P, U::wrapping_sub(c1 as U, c0 as U))
            } else {
                (S::N, U::wrapping_sub(c0 as U, c1 as U))
            }
        }

        let (sx, dx) = sign_delta(x0, x1);
        let (sy, dy) = sign_delta(y0, y1);

        if YX && dy <= dx || !YX && dx < dy {
            return None;
        }

        let (u0, v0, u1, su, sv, du, dv) =
            if YX { (y0, x0, y1, sy, sx, dy, dx) } else { (x0, y0, x1, sx, sy, dx, dy) };

        // SAFETY: the line segment has been normalized.
        let this = unsafe { Self::from_normalized((u0, v0), (du, dv), (su, sv), u1) };
        Some(this)
    }

    /// Returns `true` if the iterator is exhausted.
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

    /// Returns the current point without checking for exhaustion.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the iterator has not been exhausted,
    /// i.e. [`Bresenham::is_done`] is `false`.
    #[inline]
    #[must_use]
    pub const unsafe fn head_unchecked(&self) -> CxC {
        let (x0, y0) = if YX { (self.v0, self.u0) } else { (self.u0, self.v0) };
        (x0, y0)
    }

    /// Returns the current point, or [`None`] if the iterator is exhausted.
    #[inline]
    #[must_use]
    pub const fn head(&self) -> Option<CxC> {
        if self.is_done() {
            return None;
        }
        // SAFETY: the line segment has not been exhausted.
        let (x0, y0) = unsafe { self.head_unchecked() };
        Some((x0, y0))
    }

    /// Advances the iterator without checking for exhaustion.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the iterator has not been exhausted,
    /// i.e. [`Bresenham::is_done`] is `false`.
    #[inline]
    pub const unsafe fn step_unchecked(&mut self) {
        if 0 <= self.err {
            self.v0 = self.v0.wrapping_add(self.sv as _);
            self.err = self.err.wrapping_sub_unsigned(self.du as _);
        }
        self.u0 = self.u0.wrapping_add(self.su as _);
        self.err = self.err.wrapping_add_unsigned(self.dv as _);
    }

    /// Consumes the current point and advances the iterator.
    ///
    /// Returns the point, or [`None`] if the iterator is exhausted.
    #[inline]
    #[must_use]
    pub const fn pop_head(&mut self) -> Option<CxC> {
        let Some((x0, y0)) = self.head() else { return None };
        // SAFETY: the line segment has not been exhausted.
        unsafe { self.step_unchecked() };
        Some((x0, y0))
    }
}

impl<const YX: bool> Iterator for Bresenham<YX> {
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

impl<const YX: bool> core::iter::FusedIterator for Bresenham<YX> {}

impl<const YX: bool> ExactSizeIterator for Bresenham<YX> {}
