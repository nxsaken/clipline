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
    /// `sx` must match the direction from `x0` to `x1`.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked((x0, y0): CxC, x1: C, (sx, sy): SxS) -> Self {
        Self { x0, y0, x1, sx, sy }
    }

    /// Returns a [`Diagonal`] iterator over a half-open line segment
    /// if the line segment is diagonal, otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((x0, y0): CxC, (x1, y1): CxC) -> Option<Self> {
        let (sx, dx) = ops::sd(x0, x1);
        let (sy, dy) = ops::sd(y0, y1);

        if dx != dy {
            return None;
        }

        // SAFETY: sx matches the direction from x0 to x1.
        let this = unsafe { Self::new_unchecked((x0, y0), x1, (sx, sy)) };
        Some(this)
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
        self.x0 = self.x0.wrapping_add(self.sx as C);
        self.y0 = self.y0.wrapping_add(self.sy as C);
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
        // TODO: fallible version.
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
    /// See [`Bidiagonal`](crate::Bidiagonal) for an optimized alternative.
    #[inline]
    #[must_use]
    pub const fn tail(&self) -> Option<CxC> {
        if self.is_done() {
            return None;
        }
        let x1 = self.x1.wrapping_sub(self.sx as C);
        let dx = x1.abs_diff(self.x0);
        let y1 = match self.sy {
            S::P => self.y0.wrapping_add_unsigned(dx),
            S::N => self.y0.wrapping_sub_unsigned(dx),
        };
        Some((x1, y1))
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
    /// See [`Bidiagonal`](crate::Bidiagonal) for an optimized alternative.
    #[inline]
    #[must_use]
    pub const fn pop_tail(&mut self) -> Option<CxC> {
        let Some((x1, y1)) = self.tail() else { return None };
        self.x1 = x1;
        Some((x1, y1))
    }
}

impl DoubleEndedIterator for Diagonal {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}
