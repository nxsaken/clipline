use crate::bidiagonal::Bidiagonal;
use crate::math::{ops, CxC, SxS, C, S, U};

/// An iterator over the rasterized points of a half-open diagonal line segment.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Diagonal {
    /// The current start coordinate along the `x` axis.
    x0: C,
    /// The current start coordinate along the `y` axis.
    y0: C,
    /// The current end coordinate along the `x` axis.
    x1: C,
    /// The step sign along the `x` axis.
    sx: S,
    /// The step sign along the `y` axis.
    sy: S,
}

impl Diagonal {
    /// Constructs a [`Diagonal`] iterator from its internal parts.
    ///
    /// # Safety
    ///
    /// `sx == sign(x1 - x0)`.
    /// TODO: constrain x1 such that y0 + |x1 - x0| * sy is in bounds.
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new_unchecked((x0, y0): CxC, x1: C, (sx, sy): SxS) -> Self {
        debug_assert!((x0 <= x1) == matches!(sx, S::Pos));
        debug_assert!((x1 < x0) == matches!(sx, S::Neg));
        Self { x0, y0, x1, sx, sy }
    }

    /// Returns a [`Diagonal`] iterator over a half-open line segment
    /// if the line segment is diagonal, otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        let (sx, dx) = ops::abs_diff(x1, x0);
        let (sy, dy) = ops::abs_diff(y1, y0);
        if dx != dy {
            return None;
        }
        // SAFETY: sx == sign(x1 - x0).
        let this = unsafe { Self::new_unchecked((x0, y0), x1, (sx, sy)) };
        Some(this)
    }

    /// Converts this [`Diagonal`] into a [`Bidiagonal`].
    #[inline]
    #[must_use]
    pub const fn double_ended(self) -> Bidiagonal {
        let dx = self.length();
        let y1 = match self.sy {
            // SAFETY: dx = dy => y0 + (y1 - y0) = y1.
            // y1 <= C::MAX from construction => y0 + dx cannot overflow.
            S::Pos => unsafe { ops::unchecked_add_unsigned(self.y0, dx) },
            // SAFETY: dx = dy => y0 - (y0 - y1) = y1.
            // y1 >= C::MIN from construction => y0 - dx cannot underflow.
            S::Neg => unsafe { ops::unchecked_sub_unsigned(self.y0, dx) },
        };
        // SAFETY:
        // * |y0 - y1| = |y0 - (y0 ± dx)| = |±(x0 - x1)| = |x0 - x1|.
        // * sx == sign(x1 - x0).
        // * sy == sign(y1 - y0).
        unsafe { Bidiagonal::new_unchecked((self.x0, self.y0), (self.x1, y1), (self.sx, self.sy)) }
    }

    /// Returns a copy of this [`Diagonal`] iterator.
    #[inline]
    #[must_use]
    pub const fn copy(&self) -> Self {
        Self { ..*self }
    }
}

impl Diagonal {
    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.x0 == self.x1
    }

    /// Returns the remaining length of this iterator.
    #[inline]
    #[must_use]
    pub const fn length(&self) -> U {
        self.x0.abs_diff(self.x1)
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
        Some((self.x0, self.y0))
    }

    /// Consumes and returns the point at the start of the iterator.
    /// This advances the iterator forwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_head(&mut self) -> Option<CxC> {
        let Some((x0, y0)) = self.head() else { return None };
        // SAFETY:
        // * sx > 0 => x0 < x1 => x0 + 1 cannot overflow.
        // * sx < 0 => x1 < x0 => x0 - 1 cannot overflow.
        self.x0 = unsafe { ops::unchecked_add_sign(self.x0, self.sx) };
        // SAFETY:
        // * sy > 0 => y0 < y1 => y0 + 1 cannot overflow.
        // * sy < 0 => y1 < y0 => y0 - 1 cannot overflow.
        self.y0 = unsafe { ops::unchecked_add_sign(self.y0, self.sy) };
        Some((x0, y0))
    }
}

impl Iterator for Diagonal {
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

impl core::iter::FusedIterator for Diagonal {}

impl ExactSizeIterator for Diagonal {
    #[inline]
    fn len(&self) -> usize {
        usize::from(self.length())
    }
}

impl Diagonal {
    /// Returns the point immediately before the end of the iterator.
    /// This does not advance the iterator.
    ///
    /// Returns [`None`] if the iterator has terminated.
    ///
    /// # Performance
    ///
    /// This method performs non-trivial arithmetic to compute the last point.
    /// Use sparingly in performance-critical code. Avoid pairing this with
    /// [`Self::pop_tail`], as it will redo that work.
    ///
    /// See [`Bidiagonal::tail`] for an optimized alternative.
    #[inline]
    #[must_use]
    pub const fn tail(&self) -> Option<CxC> {
        if self.is_done() {
            return None;
        }
        // SAFETY:
        // * sx > 0 => x0 < x1 => x1 - 1 cannot underflow.
        // * sx < 0 => x1 < x0 => x1 + 1 cannot overflow.
        let xt = unsafe { ops::unchecked_sub_sign(self.x1, self.sx) };
        let dxt = match self.sx {
            // SAFETY: x0 < x1, xt = x1 - 1 => x0 <= xt.
            S::Pos => unsafe { ops::unchecked_abs_diff(xt, self.x0) },
            // SAFETY: x1 < x0, xt = x1 + 1 => xt <= x0.
            S::Neg => unsafe { ops::unchecked_abs_diff(self.x0, xt) },
        };
        let yt = match self.sy {
            // SAFETY: dxt = dyt => y0 + (yt - y0) = yt = y1 - 1 => y0 + dxt cannot overflow.
            S::Pos => unsafe { ops::unchecked_add_unsigned(self.y0, dxt) },
            // SAFETY: dxt = dyt => y0 - (y0 - yt) = yt = y1 + 1 => y0 - dxt cannot underflow.
            S::Neg => unsafe { ops::unchecked_sub_unsigned(self.y0, dxt) },
        };
        Some((xt, yt))
    }

    /// Consumes and returns the point immediately before the end of the iterator.
    /// This advances the iterator backwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    ///
    /// # Performance
    ///
    /// This method performs non-trivial arithmetic to compute the last point.
    /// Use sparingly in performance-critical code.
    ///
    /// See [`Bidiagonal::pop_tail`] for an optimized alternative.
    #[inline]
    #[must_use]
    pub const fn pop_tail(&mut self) -> Option<CxC> {
        let Some((xt, yt)) = self.tail() else { return None };
        self.x1 = xt;
        Some((xt, yt))
    }
}

impl DoubleEndedIterator for Diagonal {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}
