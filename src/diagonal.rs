//! ## Diagonal iterators
//!
//! This module provides a family of iterators for directed diagonal line segments.
//!
//! For any diagonal line segment, use the [general diagonal](Diagonal) iterator.
//! If you know the direction of the diagonal line segment,
//! use one of the [diagonal quadrant](Quadrant) iterators instead.

use crate::Point;
#[cfg(feature = "clip")]
use crate::{map_option, Region};

#[cfg(feature = "clip")]
mod kuzmin;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Diagonal quadrant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed diagonal line segment covered by the given *quadrant*.
///
/// A quadrant is defined by the directions of the line segment it covers along each axis:
/// - Negative along the `y` axis if `FY`, positive otherwise.
/// - Negative along the `x` axis if `FX`, positive otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Quadrant<const FX: bool, const FY: bool> {
    x1: isize,
    y1: isize,
    x2: isize,
    #[cfg(feature = "double_ended")]
    y2: isize,
}

/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` and `y` both increase.
pub type Quadrant0 = Quadrant<false, false>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` increases and `y` decreases.
pub type Quadrant1 = Quadrant<false, true>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` decreases and `y` increases.
pub type Quadrant2 = Quadrant<true, false>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` and `y` both decrease.
pub type Quadrant3 = Quadrant<true, true>;

impl<const FX: bool, const FY: bool> Quadrant<FX, FY> {
    /// Creates an iterator over a diagonal directed line segment
    /// covered by the given [quadrant](Quadrant).
    ///
    /// The line segment is defined by its starting point and an absolute offset along both axes.
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, dx: isize) -> Self {
        let x2 = if FX { x1 - dx } else { x1 + dx };
        let y2 = if FY { y1 - dx } else { y1 + dx };
        Self::new_inner((x1, y1), (x2, y2))
    }

    #[inline]
    #[must_use]
    pub(crate) const fn new_inner((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Self {
        #[cfg(not(feature = "double_ended"))]
        let _ = y2;
        Self {
            x1,
            y1,
            x2,
            #[cfg(feature = "double_ended")]
            y2,
        }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        !FX && self.x2 <= self.x1 || FX && self.x1 <= self.x2
    }
}

#[cfg(feature = "clip")]
impl<const FX: bool, const FY: bool> Quadrant<FX, FY> {
    /// Creates an iterator over a directed line segment
    /// covered by the given [quadrant](Quadrant), clipped to a [rectangular region](Region).
    ///
    /// The line segment is defined by its starting point and an absolute offset along both axes.
    ///
    /// Returns [`None`] if the given line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip((x1, y1): Point<isize>, dx: isize, region: Region<isize>) -> Option<Self> {
        let x2 = if FX { x1 - dx } else { x1 + dx };
        let y2 = if FY { y1 - dx } else { y1 + dx };
        Self::clip_inner((x1, y1), (x2, y2), dx, region)
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn clip_inner(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        dx: isize,
        region: Region<isize>,
    ) -> Option<Self> {
        let Some(((cx1, cy1), cx2)) =
            kuzmin::clip::<FX, FY>((x1, y1), (x2, y2), dx, dx * 2, region)
        else {
            return None;
        };
        Some(Self {
            x1: cx1,
            y1: cy1,
            x2: cx2,
            #[cfg(feature = "double_ended")]
            y2: panic!("double-ended clipped diagonal iterator is not supported"),
        })
    }
}

impl<const FX: bool, const FY: bool> Iterator for Quadrant<FX, FY> {
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

#[cfg(feature = "double_ended")]
impl<const FX: bool, const FY: bool> DoubleEndedIterator for Quadrant<FX, FY> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        self.x2 -= if !FX { 1 } else { -1 };
        self.y2 -= if !FY { 1 } else { -1 };
        let (x, y) = (self.x2, self.y2);
        Some((x, y))
    }
}

impl<const FX: bool, const FY: bool> ExactSizeIterator for Quadrant<FX, FY> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FX: bool, const FY: bool> core::iter::FusedIterator for Quadrant<FX, FY> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary diagonal iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed diagonal line segment,
/// with the [quadrant](Quadrant) of iteration determined at runtime.
///
/// If you know the [quadrant](Quadrant) alignment of the line segment beforehand,
/// consider the more specific [quadrant 0](Quadrant0), [quadrant 1](Quadrant1),
/// [quadrant 2](Quadrant2) and [quadrant 3](Quadrant3) iterators instead.
///
/// **Note**: an optimized implementation of [`Diagonal::fold`] is provided.
/// This makes [`Diagonal::for_each`] faster than a `for` loop, since it checks
/// the quadrant of iteration only once instead of on every call to [`Diagonal::next`].
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
                return Some(Self::Quadrant0(Quadrant::new_inner((x1, y1), (x2, y2))));
            }
            if dx != -dy {
                return None;
            }
            return Some(Self::Quadrant1(Quadrant::new_inner((x1, y1), (x2, y2))));
        }
        if 0 < dy {
            if -dx != dy {
                return None;
            }
            return Some(Self::Quadrant2(Quadrant::new_inner((x1, y1), (x2, y2))));
        }
        if -dx != -dy {
            return None;
        }
        Some(Self::Quadrant3(Quadrant::new_inner((x1, y1), (x2, y2))))
    }

    /// Creates an iterator over a directed line segment,
    /// if it is [diagonal](Diagonal), clipped to a [rectangular region](Region).
    ///
    /// Returns [`None`] if the given line segment is not diagonal,
    /// or if it does not intersect the clipping region.
    #[cfg(feature = "clip")]
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        region: Region<isize>,
    ) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dx {
            if 0 < dy {
                if dx != dy {
                    return None;
                }
                return map_option!(
                    Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
                    me => Self::Quadrant0(me)
                );
            }
            if dx != -dy {
                return None;
            }
            return map_option!(
                Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
                me => Self::Quadrant1(me)
            );
        }
        if 0 < dy {
            if -dx != dy {
                return None;
            }
            return map_option!(
                Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
                me => Self::Quadrant2(me)
            );
        }
        if -dx != -dy {
            return None;
        }
        map_option!(
            Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
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

#[cfg(feature = "double_ended")]
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
