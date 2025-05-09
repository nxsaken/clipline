use crate::math::{ops, CxC, SxS, C, S, U};

/// An iterator over the rasterized points of a half-open diagonal line segment.
///
/// This is a variant of the [`Diagonal`] iterator optimized
/// for [`DoubleEndedIterator`] by storing both end coordinates.
///
/// If you do not need double-ended iteration, prefer [`Diagonal`].
///
/// [`Diagonal`]: crate::Diagonal
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
    /// 1. `|x0 - x1|` must be equal to `|y0 - y1|`.
    /// 2. `sx` must match the direction from `x0` to `x1`.
    /// 3. `sy` must match the direction from `y0` to `y1`.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked((x0, y0): CxC, (x1, y1): CxC, (sx, sy): SxS) -> Self {
        debug_assert!(x0.abs_diff(x1) == y0.abs_diff(y1));
        debug_assert!((x0 <= x1) == matches!(sx, S::P));
        debug_assert!((x1 < x0) == matches!(sx, S::N));
        debug_assert!((y0 <= y1) == matches!(sy, S::P));
        debug_assert!((y1 < y0) == matches!(sy, S::N));
        Self { x0, y0, x1, y1, sx, sy }
    }

    /// Returns a [`Bidiagonal`] iterator over a half-open line segment
    /// if the line segment is diagonal, otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        let (sx, dx) = ops::sd(x0, x1);
        let (sy, dy) = ops::sd(y0, y1);

        if dx != dy {
            return None;
        }

        // SAFETY:
        // 1. |x0 - x1| is equal to |y0 - y1|.
        // 2. sx matches the direction from x0 to x1.
        // 3. sy matches the direction from y0 to y1.
        let this = unsafe { Self::new_unchecked((x0, y0), (x1, y1), (sx, sy)) };
        Some(this)
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
        self.x0 = self.x0.wrapping_add(self.sx as C);
        self.y0 = self.y0.wrapping_add(self.sy as C);
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
        let x1 = self.x1.wrapping_sub(self.sx as C);
        let y1 = self.y1.wrapping_sub(self.sy as C);
        Some((x1, y1))
    }

    /// Consumes and returns the point immediately before the end of the iterator.
    /// This advances the iterator backwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_tail(&mut self) -> Option<CxC> {
        let Some((x1, y1)) = self.tail() else { return None };
        self.x1 = x1;
        self.y1 = y1;
        Some((x1, y1))
    }
}

impl DoubleEndedIterator for Bidiagonal {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}
