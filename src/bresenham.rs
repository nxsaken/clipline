use crate::bresenham_case::{BresenhamCase, BresenhamFast, BresenhamSlow};
use crate::math::{ops, CxC, U};

/// An iterator over the rasterized points of a half-open line segment.
///
/// Selects one of the two cases of Bresenham's algorithm at runtime.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Bresenham {
    /// See [`BresenhamSlow`].
    Slow(BresenhamSlow),
    /// See [`BresenhamFast`].
    Fast(BresenhamFast),
}

impl Bresenham {
    /// Returns a [`Bresenham`] iterator over a half-open line segment.
    #[inline]
    #[must_use]
    pub const fn new((x0, y0): CxC, (x1, y1): CxC) -> Self {
        let (sx, dx) = ops::abs_diff(x1, x0);
        let (sy, dy) = ops::abs_diff(y1, y0);

        if dy <= dx {
            // SAFETY:
            // 1. dx == |x1 - x0|.
            // 2. dy <= dx.
            // 3. sx = sign(x1 - x0).
            let case =
                unsafe { BresenhamCase::new_unchecked_noclip((x0, y0), (dx, dy), x1, (sx, sy)) };
            Self::Slow(case)
        } else {
            // SAFETY:
            // 1. dy == |y1 - y0|.
            // 2. dx < dy.
            // 3. sy = sign(y1 - y0).
            let case =
                unsafe { BresenhamCase::new_unchecked_noclip((y0, x0), (dy, dx), y1, (sy, sx)) };
            Self::Fast(case)
        }
    }

    /// Returns a copy of this [`Bresenham`] iterator.
    #[inline]
    #[must_use]
    pub const fn copy(&self) -> Self {
        match self {
            Self::Slow(case) => Self::Slow(case.copy()),
            Self::Fast(case) => Self::Fast(case.copy()),
        }
    }
}

impl Bresenham {
    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        match self {
            Self::Slow(case) => case.is_done(),
            Self::Fast(case) => case.is_done(),
        }
    }

    /// Returns the remaining length of this iterator.
    #[inline]
    #[must_use]
    pub const fn length(&self) -> U {
        match self {
            Self::Slow(case) => case.length(),
            Self::Fast(case) => case.length(),
        }
    }

    /// Returns the point at the start of the iterator.
    /// This does not advance the iterator.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn head(&self) -> Option<CxC> {
        match self {
            Self::Slow(case) => case.head(),
            Self::Fast(case) => case.head(),
        }
    }

    /// Consumes and returns the point at the start of the iterator.
    /// This advances the iterator forwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_head(&mut self) -> Option<CxC> {
        match self {
            Self::Slow(case) => case.pop_head(),
            Self::Fast(case) => case.pop_head(),
        }
    }
}

impl Iterator for Bresenham {
    type Item = CxC;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Slow(case) => case.next(),
            Self::Fast(case) => case.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Slow(case) => case.size_hint(),
            Self::Fast(case) => case.size_hint(),
        }
    }
}

impl core::iter::FusedIterator for Bresenham {}

impl ExactSizeIterator for Bresenham {
    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Slow(case) => case.len(),
            Self::Fast(case) => case.len(),
        }
    }
}
