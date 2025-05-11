use crate::diagonal::Diagonal;
use crate::math::{ops, CxC, SxS, C, S, U};

/// An iterator over the rasterized points of a half-open
/// diagonal line segment with fast double-ended traversal.
///
/// If you do not need reversed iteration,
/// prefer [`Diagonal`], as it takes up less space.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Bidiagonal {
    /// The current start coordinate along the `x` axis.
    x0: C,
    /// The current start coordinate along the `y` axis.
    y0: C,
    /// The current end coordinate along the `x` axis.
    x1: C,
    /// The current end coordinate along the `y` axis.
    y1: C,
    /// The step sign along the `x` axis.
    sx: S,
    /// The step sign along the `y` axis.
    sy: S,
}

impl Bidiagonal {
    /// Constructs a [`Bidiagonal`] iterator from its internal parts.
    ///
    /// # Safety
    ///
    /// 1. `|x0 - x1| == |y0 - y1|`.
    /// 2. `sx == sign(x1 - x0)`.
    /// 3. `sy == sign(y1 - y0)`.
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new_unchecked((x0, y0): CxC, (x1, y1): CxC, (sx, sy): SxS) -> Self {
        debug_assert!(x0.abs_diff(x1) == y0.abs_diff(y1));
        debug_assert!((x0 <= x1) == matches!(sx, S::Pos));
        debug_assert!((x1 < x0) == matches!(sx, S::Neg));
        debug_assert!((y0 <= y1) == matches!(sy, S::Pos));
        debug_assert!((y1 < y0) == matches!(sy, S::Neg));
        Self { x0, y0, x1, y1, sx, sy }
    }

    /// Returns a [`Bidiagonal`] iterator over a half-open line segment
    /// if the line segment is diagonal, otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        let (sx, dx) = ops::abs_diff(x1, x0);
        let (sy, dy) = ops::abs_diff(y1, y0);
        if dx != dy {
            return None;
        }
        // SAFETY:
        // 1. |x0 - x1| == |y0 - y1|.
        // 2. sx == sign(x1 - x0).
        // 3. sy == sign(y1 - y0).
        let this = unsafe { Self::new_unchecked((x0, y0), (x1, y1), (sx, sy)) };
        Some(this)
    }

    /// Converts this [`Bidiagonal`] into a [`Diagonal`].
    ///
    /// Note that [`Diagonal`] implements [`DoubleEndedIterator`]
    /// as well, but its tail point is more expensive to compute.
    #[inline]
    #[must_use]
    pub const fn single_ended(self) -> Diagonal {
        // SAFETY: sx == sign(x1 - x0).
        unsafe { Diagonal::new_unchecked((self.x0, self.y0), self.x1, (self.sx, self.sy)) }
    }

    /// Returns a copy of this [`Bidiagonal`] iterator.
    #[inline]
    #[must_use]
    pub const fn copy(&self) -> Self {
        Self { ..*self }
    }
}

impl Bidiagonal {
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
        match self.sx {
            // SAFETY: x0 <= x1.
            S::Pos => unsafe { ops::unchecked_abs_diff(self.x1, self.x0) },
            // SAFETY: x1 <= x0.
            S::Neg => unsafe { ops::unchecked_abs_diff(self.x0, self.x1) },
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

impl Iterator for Bidiagonal {
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

impl core::iter::FusedIterator for Bidiagonal {}

impl ExactSizeIterator for Bidiagonal {
    #[inline]
    fn len(&self) -> usize {
        usize::from(self.length())
    }
}

impl Bidiagonal {
    /// Returns the point immediately before the end of the iterator.
    /// This does not advance the iterator.
    ///
    /// Returns [`None`] if the iterator has terminated.
    ///
    /// # Performance
    ///
    /// This method performs trivial arithmetic to compute the last point.
    /// Avoid pairing this with [`Self::pop_tail`], as it will redo that work.
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
        // SAFETY:
        // * sy > 0 => y0 < y1 => y1 - 1 cannot underflow.
        // * sy < 0 => y1 < y0 => y1 + 1 cannot overflow.
        let yt = unsafe { ops::unchecked_sub_sign(self.y1, self.sy) };
        Some((xt, yt))
    }

    /// Consumes and returns the point immediately before the end of the iterator.
    /// This advances the iterator backwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_tail(&mut self) -> Option<CxC> {
        let Some((xt, yt)) = self.tail() else { return None };
        self.x1 = xt;
        self.y1 = yt;
        Some((xt, yt))
    }
}

impl DoubleEndedIterator for Bidiagonal {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}
