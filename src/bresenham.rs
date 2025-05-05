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
    pub const fn from_points((x0, y0): CxC, (x1, y1): CxC) -> Self {
        let (sx, dx) = ops::sd(x0, x1);
        let (sy, dy) = ops::sd(y0, y1);

        if dy <= dx {
            // SAFETY:
            // 1. dx matches the offset between x0 and x1.
            // 2. dy <= dx.
            // 3. sx matches the direction from x0 to x1.
            let case = unsafe { BresenhamCase::from_normalized((x0, y0), (dx, dy), (sx, sy), x1) };
            Self::Slow(case)
        } else {
            // SAFETY:
            // 1. dy matches the offset between y0 and y1.
            // 2. dx < dy.
            // 3. sy matches the direction from y0 to y1.
            let case = unsafe { BresenhamCase::from_normalized((y0, x0), (dy, dx), (sy, sx), y1) };
            Self::Fast(case)
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

impl ExactSizeIterator for Bresenham {}
