//! ## Diagonal iterators
//!
//! This module provides a family of iterators for directed diagonal line segments.
//!
//! For any diagonal line segment, use the [diagonal](Diagonal) iterator.
//! If you know the direction and length of the diagonal line segment, use
//! one of the [diagonal quadrant](Quadrant) iterators instead.

use crate::{clip, Clip, Point};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Diagonal quadrant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed diagonal line segment covered by the given *quadrant*.
///
/// A quadrant is defined by the directions of the line segment it covers along each axis:
/// - Negative along the `y` axis if `FY`, positive otherwise.
/// - Negative along the `x` axis if `FX`, positive otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Quadrant<T, const FX: bool, const FY: bool> {
    x1: T,
    y1: T,
    x2: T,
}

/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` and `y` both increase.
pub type Quadrant0<T> = Quadrant<T, false, false>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` increases and `y` decreases.
pub type Quadrant1<T> = Quadrant<T, false, true>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` decreases and `y` increases.
pub type Quadrant2<T> = Quadrant<T, true, false>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` and `y` both decrease.
pub type Quadrant3<T> = Quadrant<T, true, true>;

impl<const FX: bool, const FY: bool> Quadrant<isize, FX, FY> {
    /// Creates an iterator over a diagonal directed line segment
    /// covered by the given [quadrant](Quadrant).
    ///
    /// *Assumes that the line segment is covered by the given quadrant.*
    #[inline(always)]
    #[must_use]
    pub(crate) const fn new_unchecked((x1, y1): Point<isize>, x2: isize) -> Self {
        Self { x1, y1, x2 }
    }

    /// Creates an iterator over a diagonal directed line segment
    /// covered by the given [quadrant](Quadrant).
    ///
    /// The line segment is defined by its starting point and its
    /// [Chebyshev length](https://en.wikipedia.org/wiki/Chebyshev_distance),
    /// which is equivalent to the absolute offset along the `x` or `y` coordinate.
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, length: isize) -> Self {
        let x2 = if !FX { x1 + length } else { x1 - length };
        Self::new_unchecked((x1, y1), x2)
    }

    /// Clips a diagonal directed line segment covered by the given
    /// [quadrant](DiagonalQuadrant) against a [rectangular region](Clip).
    ///
    /// Returns a [`DiagonalQuadrant`] iterator over the clipped line segment,
    /// or [`None`] if it does not intersect the clipping region.
    ///
    /// *Assumes that the line segment is covered by the given quadrant.*
    #[must_use]
    #[inline(always)]
    const fn clip_unchecked(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        clip: &Clip<isize>,
    ) -> Option<Self> {
        if clip::diagonal::out_of_bounds::<FX, FY>((x1, y1), (x2, y2), clip) {
            return None;
        }
        let Some((cx1, cy1)) = clip::diagonal::enter::<FX, FY>((x1, y1), clip) else {
            return None;
        };
        let cx2 = clip::diagonal::exit::<FX, FY>((x1, y1), (x2, y2), clip);
        Some(Self::new_unchecked((cx1, cy1), cx2))
    }

    /// Creates an iterator over a diagonal directed line segment
    /// covered by the given [quadrant](Quadrant), clipped to a [rectangular region](Clip).
    ///
    /// The line segment is defined by its starting point and its
    /// [Chebyshev length](https://en.wikipedia.org/wiki/Chebyshev_distance),
    /// which is equivalent to the absolute change in the `x` or `y` coordinate.
    ///
    /// Returns [`None`] if the line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip((x1, y1): Point<isize>, length: isize, clip: &Clip<isize>) -> Option<Self> {
        let x2 = if !FX { x1 + length } else { x1 - length };
        let y2 = if !FY { y1 + length } else { y1 - length };
        Self::clip_unchecked((x1, y1), (x2, y2), clip)
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        !FX && self.x2 <= self.x1 || FX && self.x1 <= self.x2
    }
}

impl<const FX: bool, const FY: bool> Iterator for Quadrant<isize, FX, FY> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = (self.x1, self.y1);
        self.x1 += if !FX { 1 } else { -1 };
        self.y1 += if !FY { 1 } else { -1 };
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // slightly optimized over `isize::abs_diff`,
        // see its implementation for the proof that this cast is legal
        #[allow(clippy::cast_sign_loss)]
        let length = match FX {
            false => usize::wrapping_sub(self.x2 as usize, self.x1 as usize),
            true => usize::wrapping_sub(self.x1 as usize, self.x2 as usize),
        };
        (length, Some(length))
    }
}

impl<const FX: bool, const FY: bool> ExactSizeIterator for Quadrant<isize, FX, FY> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FX: bool, const FY: bool> core::iter::FusedIterator for Quadrant<isize, FX, FY> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary diagonal iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed diagonal line segment,
/// with the [quadrant](Quadrant) of iteration determined at runtime.
///
/// If you know the [quadrant](Quadrant) alignment of the line segment beforehand, consider the
/// more specific [`Quadrant0`], [`Quadrant1`], [`Quadrant2`] and [`Quadrant3`] iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the quadrant of iteration only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Diagonal<T> {
    /// Diagonal line segment at `45°`, see [`Quadrant0`].
    Quadrant0(Quadrant0<T>),
    /// Diagonal line segment at `135°`, see [`Quadrant1`].
    Quadrant1(Quadrant1<T>),
    /// Diagonal line segment at `225°`, see [`Quadrant2`].
    Quadrant2(Quadrant2<T>),
    /// Diagonal line segment at `315°`, see [`Quadrant3`].
    Quadrant3(Quadrant3<T>),
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

impl Diagonal<isize> {
    /// Creates an iterator over a directed line segment if it is [diagonal](Diagonal).
    ///
    /// Returns [`None`] if the given line segment is not diagonal.
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dx {
            if 0 < dy {
                if dx != dy {
                    return None;
                }
                return Some(Self::Quadrant0(Quadrant::new_unchecked((x1, y1), x2)));
            }
            if dx != -dy {
                return None;
            }
            return Some(Self::Quadrant1(Quadrant::new_unchecked((x1, y1), x2)));
        }
        if 0 < dy {
            if -dx != dy {
                return None;
            }
            return Some(Self::Quadrant2(Quadrant::new_unchecked((x1, y1), x2)));
        }
        if -dx != -dy {
            return None;
        }
        Some(Self::Quadrant3(Quadrant::new_unchecked((x1, y1), x2)))
    }

    /// Creates an iterator over a directed line segment,
    /// if it is [diagonal](Diagonal), clipped to a [rectangular region](Clip).
    ///
    /// Returns [`None`] if the given line segment is not diagonal,
    /// or if it does not intersect the clipping region.
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        clip: &Clip<isize>,
    ) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dx {
            if 0 < dy {
                if dx != dy {
                    return None;
                }
                return clip::map_option!(
                    Quadrant::clip_unchecked((x1, y1), (x2, y2), clip),
                    me => Self::Quadrant0(me)
                );
            }
            if dx != -dy {
                return None;
            }
            return clip::map_option!(
                Quadrant::clip_unchecked((x1, y1), (x2, y2), clip),
                me => Self::Quadrant1(me)
            );
        }
        if 0 < dy {
            if -dx != dy {
                return None;
            }
            return clip::map_option!(
                Quadrant::clip_unchecked((x1, y1), (x2, y2), clip),
                me => Self::Quadrant2(me)
            );
        }
        if -dx != -dy {
            return None;
        }
        clip::map_option!(
            Quadrant::clip_unchecked((x1, y1), (x2, y2), clip),
            me => Self::Quadrant3(me)
        )
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }
}

impl Iterator for Diagonal<isize> {
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

impl ExactSizeIterator for Diagonal<isize> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Diagonal<isize> {}
