//! ## Diagonal iterators
//!
//! This module provides a family of iterators for directed diagonal line segments.
//!
//! For any diagonal line segment, use the [diagonal](Diagonal) iterator.
//! If you know the direction and length of the diagonal line segment, use
//! one of the [diagonal quadrant](Quadrant) iterators instead.

use crate::clip::Clip;
use crate::math::{Math, Num, Point};
use crate::symmetry::{fx, fy, sorted};
use crate::utils::map_opt;

pub mod clip;

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

impl<const FX: bool, const FY: bool> Quadrant<i8, FX, FY> {
    #[inline(always)]
    #[must_use]
    pub(crate) const fn new_inner((x1, y1): Point<i8>, x2: i8) -> Self {
        Self { x1, y1, x2 }
    }

    #[inline(always)]
    #[must_use]
    const fn covers((x1, y1): Point<i8>, (x2, y2): Point<i8>) -> bool {
        let dx = {
            let (a, b) = sorted!(FX, x1, x2, false);
            Math::<i8>::delta(b, a)
        };
        let dy = {
            let (a, b) = sorted!(FY, y1, y2, false);
            Math::<i8>::delta(b, a)
        };
        dx == dy
    }

    /// Returns an iterator over a directed line segment
    /// if it is diagonal and covered by the given [quadrant](Quadrant).
    ///
    /// Returns [`None`] if the line segment is not diagonal,
    /// or is not covered by the quadrant.
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<i8>, (x2, y2): Point<i8>) -> Option<Self> {
        if !Self::covers((x1, y1), (x2, y2)) {
            return None;
        }
        Some(Self::new_inner((x1, y1), x2))
    }

    /// Returns an iterator over a directed line segment, if it is diagonal and
    /// covered by the given [quadrant](Quadrant), clipped to a [rectangular region](Clip).
    ///
    /// Returns [`None`] if the line segment is not diagonal,
    /// is not covered by the quadrant, or does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip((x1, y1): Point<i8>, (x2, y2): Point<i8>, clip: Clip<i8>) -> Option<Self> {
        if !Self::covers((x1, y1), (x2, y2)) {
            return None;
        }
        Self::clip_inner((x1, y1), (x2, y2), clip)
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        fx!(self.x2 <= self.x1, self.x1 <= self.x2)
    }

    /// Returns the remaining length of this iterator.
    ///
    /// Optimized over [`i8::abs_diff`].
    #[inline]
    #[must_use]
    pub const fn length(&self) -> <i8 as Num>::U {
        Math::<i8>::delta(fx!(self.x2, self.x1), fx!(self.x1, self.x2))
    }
}

impl<const FX: bool, const FY: bool> Iterator for Quadrant<i8, FX, FY> {
    type Item = Point<i8>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = (self.x1, self.y1);
        self.x1 = fx!(self.x1.wrapping_add(1), self.x1.wrapping_sub(1));
        self.y1 = fy!(self.y1.wrapping_add(1), self.y1.wrapping_sub(1));
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.length() as usize;
        (length, Some(length))
    }
}

impl<const FX: bool, const FY: bool> ExactSizeIterator for Quadrant<i8, FX, FY> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FX: bool, const FY: bool> core::iter::FusedIterator for Quadrant<i8, FX, FY> {}

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

macro_rules! quadrants {
    (
        $num:ty,
        ($x1:ident, $y1:ident), ($x2:ident, $y2:ident),
        $quadrant_0:expr,
        $quadrant_1:expr,
        $quadrant_2:expr,
        $quadrant_3:expr$(,)?
    ) => {
        #[allow(clippy::cast_sign_loss)]
        {
            if $x1 < $x2 {
                let dx = Math::<$num>::delta($x2, $x1);
                if $y1 < $y2 {
                    let dy = Math::<$num>::delta($y2, $y1);
                    if dx != dy {
                        return None;
                    }
                    return $quadrant_0;
                }
                let dy = Math::<$num>::delta($y1, $y2);
                if dx != dy {
                    return None;
                }
                return $quadrant_1;
            }
            let dx = Math::<$num>::delta($x1, $x2);
            if $y1 < $y2 {
                let dy = Math::<$num>::delta($y2, $y1);
                if dx != dy {
                    return None;
                }
                return $quadrant_2;
            }
            let dy = Math::<$num>::delta($y1, $y2);
            if dx != dy {
                return None;
            }
            return $quadrant_3;
        }
    };
}

impl Diagonal<i8> {
    /// Returns an iterator over a directed line segment if it is [diagonal](Diagonal).
    ///
    /// Returns [`None`] if the given line segment is not diagonal.
    #[must_use]
    pub const fn new((x1, y1): Point<i8>, (x2, y2): Point<i8>) -> Option<Self> {
        quadrants!(
            i8,
            (x1, y1),
            (x2, y2),
            Some(Self::Quadrant0(Quadrant::new_inner((x1, y1), x2))),
            Some(Self::Quadrant1(Quadrant::new_inner((x1, y1), x2))),
            Some(Self::Quadrant2(Quadrant::new_inner((x1, y1), x2))),
            Some(Self::Quadrant3(Quadrant::new_inner((x1, y1), x2))),
        );
    }

    /// Returns an iterator over a directed line segment,
    /// if it is [diagonal](Diagonal), clipped to a [rectangular region](Clip).
    ///
    /// Returns [`None`] if the given line segment is not diagonal,
    /// or if it does not intersect the clipping region.
    #[must_use]
    pub const fn clip((x1, y1): Point<i8>, (x2, y2): Point<i8>, clip: Clip<i8>) -> Option<Self> {
        quadrants!(
            i8,
            (x1, y1),
            (x2, y2),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant0),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant1),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant2),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant3),
        );
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }

    /// Returns the remaining length of this iterator.
    #[inline]
    #[must_use]
    pub const fn length(&self) -> <i8 as Num>::U {
        delegate!(self, me => me.length())
    }
}

impl Iterator for Diagonal<i8> {
    type Item = Point<i8>;

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

impl ExactSizeIterator for Diagonal<i8> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Diagonal<i8> {}
