use crate::math::{ops, CxC, SxS, UxU, C, I2, S, U};

/// An iterator over the rasterized points of a half-open line segment,
/// using one of the two cases of Bresenham's algorithm.
///
/// The `YX` parameter determines which class of line segments is covered:
/// * `false`: segments with a "slow" slope (`dy <= dx`).
/// * `true`: segments with a "fast" slope (`dx < dy`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BresenhamCase<const YX: bool> {
    /// The current start coordinate along the major axis.
    u0: C,
    /// The current start coordinate along the minor axis.
    v0: C,
    /// The offset between the initial start and end coordinates along the major axis.
    du: U,
    /// The offset between the initial start and end coordinates along the minor axis.
    dv: U,
    /// The accumulated error.
    err: I2,
    /// The current end coordinate along the major axis.
    u1: C,
    /// The step sign along the major axis.
    su: S,
    /// The step sign along the minor axis.
    sv: S,
}

/// A [`BresenhamCase<false>`] iterator over a half-open line segment with a "slow" slope (`dy <= dx`).
pub type BresenhamSlow = BresenhamCase<false>;

/// A [`BresenhamCase<true>`] iterator over a half-open line segment with a "fast" slope (`dx < dy`).
pub type BresenhamFast = BresenhamCase<true>;

impl<const YX: bool> BresenhamCase<YX> {
    /// Constructs a [`BresenhamCase<YX>`] iterator from
    /// the parameters of a normalized half-open line segment.
    ///
    /// # Safety
    ///
    /// * `du == |u0 - u1|`.
    /// * `dv <= du` (normalized line segments are slow).
    /// * `su == sign(u1 - u0)`.
    /// * `err` has been initialized correctly. TODO: what exactly does this mean?
    /// * TODO: constrain u1 such that v0 + |u1 - u0| * sv is in bounds?
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new_unchecked(
        (u0, v0): CxC,
        (du, dv): UxU,
        err: I2,
        u1: C,
        (su, sv): SxS,
    ) -> Self {
        debug_assert!(du == u0.abs_diff(u1));
        debug_assert!(dv <= du);
        debug_assert!((u0 <= u1) == matches!(su, S::Pos));
        debug_assert!((u1 < u0) == matches!(su, S::Neg));

        Self { u0, v0, du, dv, err, u1, su, sv }
    }

    #[inline]
    #[must_use]
    const fn err_default(du: U, dv: U) -> I2 {
        let (half_du, rem) = ops::half_rem(du);
        // SAFETY: 0 <= rem <= 1, half_du <= U::MAX / 2 => half_du + rem cannot overflow.
        let half_du_adj = unsafe { ops::unchecked_unsigned_add(half_du, rem) };
        // SAFETY: dv - half_du_adj >= 0 - U::MAX > I2::MIN => dv - half_du_adj cannot underflow.
        unsafe { ops::unchecked_wide_sub_unsigned(dv as I2, half_du_adj) }
    }

    /// Constructs a [`BresenhamCase<YX>`] iterator from the parameters
    /// of an unclipped normalized half-open line segment.
    ///
    /// # Safety
    ///
    /// * `du == |u0 - u1|`.
    /// * `dv <= du` (normalized line segments are slow).
    /// * `su == sign(u1 - u0)`.
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new_unchecked_noclip(
        (u0, v0): CxC,
        (du, dv): UxU,
        u1: C,
        (su, sv): SxS,
    ) -> Self {
        let err = Self::err_default(du, dv);
        // SAFETY:
        // * du == |u0 - u1|.
        // * dv <= du.
        // * su == sign(u1 - u0).
        // * err has been initialized correctly.
        unsafe { Self::new_unchecked((u0, v0), (du, dv), err, u1, (su, sv)) }
    }

    /// Returns a [`BresenhamCase<YX>`] iterator over a half-open line segment
    /// if the line segment's slope matches `YX`, otherwise returns [`None`].
    ///
    /// The `YX` parameter determines which class of line segments is covered:
    /// * `false`: segments with a "slow" slope (`dy <= dx`).
    /// * `true`: segments with a "fast" slope (`dx < dy`).
    #[inline]
    #[must_use]
    pub const fn new((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        let (sx, dx) = ops::abs_diff(x1, x0);
        let (sy, dy) = ops::abs_diff(y1, y0);

        if YX && dy <= dx || !YX && dx < dy {
            return None;
        }

        let (u0, v0, u1, du, dv, su, sv) =
            if YX { (y0, x0, y1, dy, dx, sy, sx) } else { (x0, y0, x1, dx, dy, sx, sy) };

        // SAFETY:
        // * du == |u0 - u1|.
        // * dv <= du.
        // * su == sign(u1 - u0).
        let case = unsafe { Self::new_unchecked_noclip((u0, v0), (du, dv), u1, (su, sv)) };
        Some(case)
    }

    /// Returns a copy of this [`BresenhamCase<YX>`] iterator.
    #[inline]
    #[must_use]
    pub const fn copy(&self) -> Self {
        Self { ..*self }
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
        match self.su {
            // SAFETY: u0 <= u1.
            S::Pos => unsafe { ops::unchecked_abs_diff(self.u1, self.u0) },
            // SAFETY: u1 <= u0.
            S::Neg => unsafe { ops::unchecked_abs_diff(self.u0, self.u1) },
        }
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
            // SAFETY:
            // First, we prove that before the i-th iteration (0 <= i < du), A and B hold:
            // A. v0_i = v0_0 + m_i * sv, 0 <= m_i <= dv,
            //    where m_i = number of times (0 <= err_j) for j < i.
            // B. err_i = E0 + i * dv - m_i * du,
            //    where E0 = dv − ⌈du/2⌉ is the initial error.
            //
            // Base (i = 0):
            // * m_0 = 0.
            // * v0_0 = v0_0.
            // * err_0 = E0.
            //
            // Induction (i -> i + 1):
            // - If 0 <= err_i:
            //   * m_{i+1} = m_i + 1.
            //   * v0_{i+1} = v0_i + sv = v0_0 + (m_i + 1) * sv = v0_0 + m_{i+1} * sv.
            //   * err_{i+1} = err_i - du + dv = E0 + (i + 1) * dv - (m_i + 1) * du =
            //       = E0 + (i + 1) * dv - m_{i+1} * du.
            // - Else:
            //   * m_{i+1} = m_i.
            //   * v0_{i+1} = v0_i = v0 + m_i * sv = v0_0 + m_{i+1} * sv.
            //   * err_{i+1} = err_i + dv = E0 + (i + 1) * dv - m_i * du =
            //       = E0 + (i + 1) * dv - m_{i+1} * du.
            //   Therefore, A and B hold for i + 1.
            //
            // Next, we only increment m_j on steps where 0 <= err_j:
            //   0 <= E0 + j * dv - m_j * du
            //     => m_j <= ⌊(E0 + j * dv) / du⌋.
            //
            // Now, find the upper bound on m_i:
            //   E0 = dv - ⌈du/2⌉ => E0 <= dv − 1.
            //   i < du => i <= du - 1
            //     => i * dv <= (du - 1) * dv (multiply both sides by dv)
            //     => E0 + i * dv <= (dv - 1) + (du - 1) * dv (combine with E0 <= dv - 1)
            //     => E0 + i * dv <= du * dv - 1
            //     => E0 + i * dv < du * dv
            //     => (E0 + i * dv) / du < dv (divide both sides by du)
            //     => ⌊(E0 + i * dv) / du⌋ < dv (flooring keeps the inequality)
            //     => m_i <= ⌊(E0 + i * dv) / du⌋ < dv
            //     => m_i <= dv - 1
            //
            // Recall that before the i-th iteration, v0_i = v0_0 + m_i * sv.
            // We've shown that m_i <= dv - 1. Combining the two:
            // - If sv > 0:
            //   * v0_i <= v0_0 + (dv - 1)
            //   * v0_i <= v0_0 + (v1 - v0_0 - 1)
            //   * v0_i <= v1 - 1
            // - If sv < 0:
            //   * v0_0 - (dv - 1) <= v0_i
            //   * v0_0 - (v0_0 - v1 - 1) <= v0_i
            //   * v1 + 1 <= v0_i
            // Therefore, v0_{i < du} is at least one step away from v1 before the i-th iteration.
            // The i-th iteration adds one more sv:
            //   v0_{i+1} = v0_0 + (m_i + 1) * sv
            //     => v0_{i+1} <= v0_0 + dv * sv
            //
            // Therefore, v0 + sv cannot overflow.
            self.v0 = unsafe { ops::unchecked_add_sign(self.v0, self.sv) };
            // SAFETY:
            // du: uN => 0 <= du <= 2^N - 1;
            // err: i{2N} => -2^{2N-1} <= err <= 2^{2N-1} - 1.
            // UNDERFLOW = err_min - du_max < i{2N}::MIN = 0 - (2^N - 1) < -2^{2N-1}.
            // 2^N - 1 < 2^N; for N > 0, 2^N <= 2^{2N-1} => 2^N - 1 <= 2^{2N-1}.
            // Negate both sides: -(2^N - 1) >= -2^{2N-1} = !UNDERFLOW.
            // Therefore, err - du cannot underflow.
            self.err = unsafe { ops::unchecked_wide_sub_unsigned(self.err, self.du) };
        }
        // SAFETY:
        // The iterator has not terminated => u0 != u1.
        // * su > 0 => u0 < u1 => u0 + 1 cannot overflow.
        // * su < 0 => u1 < u0 => u0 - 1 cannot underflow.
        self.u0 = unsafe { ops::unchecked_add_sign(self.u0, self.su) };
        // SAFETY:
        // Let err_0 be the error at the start of the iteration.
        // * 0 <= err_0 => err = err_0 - du + dv.
        //   dv <= du => (err_0 - du) + dv cannot overflow.
        // * err_0 < 0 => err = err_0 + dv.
        //   0 <= dv <= 2^N - 1 => err_0 + dv < 2^N - 1
        //   For N > 0, 2^N - 1 <= 2^{2n-1} - 1.
        //   Therefore, err_0 + dv < 2^{2N-1} and cannot overflow.
        self.err = unsafe { ops::unchecked_wide_add_unsigned(self.err, self.dv) };
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
        let len = usize::from(self.length());
        (len, Some(len))
    }
}

impl<const YX: bool> core::iter::FusedIterator for BresenhamCase<YX> {}

impl<const YX: bool> ExactSizeIterator for BresenhamCase<YX> {
    #[inline]
    fn len(&self) -> usize {
        usize::from(self.length())
    }
}
