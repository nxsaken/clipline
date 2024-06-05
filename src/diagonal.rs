//! ## Diagonal line segment iterators
//!
//! This module provides a family of iterators
//! for directed diagonal line segments.

use crate::Point;

/// Iterator over a directed diagonal line segment in the given quadrant.
///
/// A quadrant is defined by its transformations relative to [`Quadrant0`].
/// - `FY`: flip the `y` axis if `true`.
/// - `FX`: flip the `x` axis if `true`.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Quadrant<const FY: bool, const FX: bool> {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

/// Iterator over a directed diagonal line segment in the [`Quadrant`]
/// where `x` and `y` both increase.
pub type Quadrant0 = Quadrant<false, false>;
/// Iterator over a directed diagonal line segment in the [`Quadrant`]
/// where `x` decreases and `y` increases.
pub type Quadrant1 = Quadrant<false, true>;
/// Iterator over a directed diagonal line segment in the [`Quadrant`]
/// where `x` increases and `y` decreases.
pub type Quadrant2 = Quadrant<true, false>;
/// Iterator over a directed diagonal line segment in the [`Quadrant`]
/// where `x` and `y` both decrease.
pub type Quadrant3 = Quadrant<true, true>;

impl<const FY: bool, const FX: bool> Quadrant<FY, FX> {
    /// Creates a diagonal iterator from `(x1, y1)` to `(x2, y2)`, *exclusive*.
    ///
    /// Returns [`None`] if the given line segment is not diagonal,
    /// or if it is uncovered by the [`Quadrant`].
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if FX && 0 <= dx || !FX && dx <= 0 || FY && 0 <= dy || !FY && dy <= 0 {
            return None;
        }
        let dy = if FY { -dy } else { dy };
        let dx = if FX { -dx } else { dx };
        if dy != dx {
            return None;
        }
        Some(Self::new_unchecked((x1, y1), (x2, y2)))
    }

    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        match FX {
            true => self.x1 <= self.x2,
            false => self.x2 <= self.x1,
        }
    }
}

impl<const FY: bool, const FX: bool> Iterator for Quadrant<FY, FX> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = (self.x1, self.y1);
        self.x1 += if FX { -1 } else { 1 };
        self.y1 += if FY { -1 } else { 1 };
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // slightly optimized over `isize::abs_diff`,
        // see its implementation for the proof that this cast is legal
        #[allow(clippy::cast_sign_loss)]
        let length = match FX {
            true => usize::wrapping_sub(self.x1 as usize, self.x2 as usize),
            false => usize::wrapping_sub(self.x2 as usize, self.x1 as usize),
        };
        (length, Some(length))
    }
}

impl<const FY: bool, const FX: bool> DoubleEndedIterator for Quadrant<FY, FX> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        self.x2 -= if FX { -1 } else { 1 };
        self.y2 -= if FY { -1 } else { 1 };
        let (x, y) = (self.x2, self.y2);
        Some((x, y))
    }
}

impl<const FY: bool, const FX: bool> ExactSizeIterator for Quadrant<FY, FX> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FY: bool, const FX: bool> core::iter::FusedIterator for Quadrant<FY, FX> {}

/// Iterator over a directed diagonal line segment.
/// The quadrant of iteration is determined at runtime.
///
/// **Note**: an optimized implementation of [`Diagonal::fold`] is provided.
/// This makes [`Diagonal::for_each`] faster than a `for` loop, since it checks
/// the iteration quadrant only once instead of on every call to [`Diagonal::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Diagonal {
    /// See [`Quadrant0`].
    Quadrant0(Quadrant0),
    /// See [`Quadrant1`].
    Quadrant1(Quadrant1),
    /// See [`Quadrant2`].
    Quadrant2(Quadrant2),
    /// See [`Quadrant3`].
    Quadrant3(Quadrant3),
}

/// Delegates calls to quadrant variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::Quadrant0($me) => $call,
            Self::Quadrant1($me) => $call,
            Self::Quadrant2($me) => $call,
            Self::Quadrant3($me) => $call,
        }
    };
}

impl Diagonal {
    /// Creates a [`Diagonal`] iterator from `(x1, y1)` to `(x2, y2)`, *exclusive*.
    ///
    /// Returns [`None`] if the given line segment is not diagonal.
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dy {
            if 0 < dx {
                if dx != dy {
                    return None;
                }
                return Some(Self::Quadrant0(Quadrant::new_unchecked((x1, y1), (x2, y2))));
            }
            if -dx != dy {
                return None;
            }
            return Some(Self::Quadrant1(Quadrant::new_unchecked((x1, y1), (x2, y2))));
        }
        if 0 < dx {
            if dx != -dy {
                return None;
            }
            return Some(Self::Quadrant2(Quadrant::new_unchecked((x1, y1), (x2, y2))));
        }
        if -dx != -dy {
            return None;
        }
        Some(Self::Quadrant3(Quadrant::new_unchecked((x1, y1), (x2, y2))))
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }
}

impl Iterator for Diagonal {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        delegate!(self, me => me.next())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        delegate!(self, me => me.size_hint())
    }

    #[cfg(feature = "try_fold")]
    #[inline]
    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: core::ops::Try<Output = B>,
    {
        delegate!(self, me => me.try_fold(init, f))
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        delegate!(self, me => me.fold(init, f))
    }
}

impl DoubleEndedIterator for Diagonal {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        delegate!(self, me => me.next_back())
    }

    #[cfg(feature = "try_fold")]
    #[inline]
    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: core::ops::Try<Output = B>,
    {
        delegate!(self, me => me.try_rfold(init, f))
    }

    #[inline]
    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        delegate!(self, me => me.rfold(init, f))
    }
}

impl ExactSizeIterator for Diagonal {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Diagonal {}
