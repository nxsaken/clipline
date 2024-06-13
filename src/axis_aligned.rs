//! ## Axis-aligned line segment iterators
//!
//! This module provides a family of iterators for horizontal and vertical directed line segments.
//!
//! For a line segment that is either vertical or horizontal, use the [orthogonal](Orthogonal)
//! iterator, which determines the orientation at runtime. If you know the orientation beforehand,
//! use an [axis-aligned](AxisAligned) iterator: [vertical](Vertical) or [horizontal](Horizontal).
//!
//! If you also know the direction, you can specialize further and pick a [signed-axis-aligned]
//! iterator: [positive](PositiveAxisAligned) and [negative](NegativeAxisAligned), or
//! [signed horizontal](SignedHorizontal) and [signed vertical](SignedVertical). Even more specific
//! [positive horizontal](PositiveHorizontal), [negative horizontal](NegativeHorizontal),
//! [positive vertical](PositiveVertical) and [negative vertical](NegativeVertical)
//! type aliases are available for convenience.

use crate::Point;
#[cfg(feature = "clip")]
use crate::{map_option, Region};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signed-axis-aligned line segment iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment aligned to the given *signed axis*.
///
/// A signed axis is defined by the *orientation* and *direction* of the line segments it covers:
/// - [vertical](SignedVertical) if `VERT`, [horizontal](SignedHorizontal) otherwise.
/// - [negative](NegativeAxisAligned) if `FLIP`, [positive](PositiveAxisAligned) otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SignedAxisAligned<const VERT: bool, const FLIP: bool> {
    u: isize,
    v1: isize,
    v2: isize,
}

/// Iterator over a directed line segment
/// aligned to the given *positive* [signed axis](SignedAxisAligned).
pub type PositiveAxisAligned<const VERT: bool> = SignedAxisAligned<VERT, false>;
/// Iterator over a directed line segment
/// aligned to the given *negative* [signed axis](SignedAxisAligned).
pub type NegativeAxisAligned<const VERT: bool> = SignedAxisAligned<VERT, true>;

/// Iterator over a directed line segment
/// aligned to the given *horizontal* [signed axis](SignedAxisAligned).
pub type SignedHorizontal<const FLIP: bool> = SignedAxisAligned<false, FLIP>;
/// Iterator over a directed line segment
/// aligned to the given *vertical* [signed axis](SignedAxisAligned).
pub type SignedVertical<const FLIP: bool> = SignedAxisAligned<true, FLIP>;

/// Iterator over a directed line segment
/// aligned to the given *positive horizontal* [signed axis](SignedAxisAligned).
pub type PositiveHorizontal = SignedHorizontal<false>;
/// Iterator over a directed line segment
/// aligned to the given *negative horizontal* [signed axis](SignedAxisAligned).
pub type NegativeHorizontal = SignedHorizontal<true>;
/// Iterator over a directed line segment
/// aligned to the given *positive vertical* [signed axis](SignedAxisAligned).
pub type PositiveVertical = SignedVertical<false>;
/// Iterator over a directed line segment
/// aligned to the given *negative vertical* [signed axis](SignedAxisAligned).
pub type NegativeVertical = SignedVertical<true>;

impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<VERT, FLIP> {
    /// Creates an iterator over a directed line segment
    /// if it is aligned to the given [signed axis](AxisAligned).
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment is not aligned to the signed axis.
    #[inline]
    #[must_use]
    pub const fn new(u: isize, v1: isize, v2: isize) -> Option<Self> {
        if !FLIP && v2 <= v1 || FLIP && v1 <= v2 {
            return None;
        }
        Some(Self::new_inner(u, v1, v2))
    }

    #[inline(always)]
    #[must_use]
    const fn new_inner(u: isize, v1: isize, v2: isize) -> Self {
        Self { u, v1, v2 }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        !FLIP && self.v2 <= self.v1 || FLIP && self.v1 <= self.v2
    }
}

#[cfg(feature = "clip")]
impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<VERT, FLIP> {
    /// Creates an iterator over a directed line segment,
    /// if it is aligned to the given [signed axis](AxisAligned),
    /// clipped to a [rectangular region](Region).
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment is not aligned to the signed axis,
    /// or if it does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(u: isize, v1: isize, v2: isize, region: Region<isize>) -> Option<Self> {
        if !FLIP && v2 <= v1 || FLIP && v1 <= v2 {
            return None;
        }
        Self::clip_inner(u, v1, v2, region)
    }

    #[inline(always)]
    #[must_use]
    const fn clip_inner(
        u: isize,
        v1: isize,
        v2: isize,
        Region { wx1, wy1, wx2, wy2 }: Region<isize>,
    ) -> Option<Self> {
        if !VERT
            && ((u < wy1 || wy2 < u)
                || (!FLIP && (v2 < wx1 || wx2 < v1) || FLIP && (v1 < wx1 || wx2 < v2)))
            || VERT
                && ((u < wx1 || wx2 < u)
                    || (!FLIP && (v2 < wy1 || wy2 < v1) || FLIP && (v1 < wy1 || wy2 < v2)))
        {
            return None;
        }
        Some(Self {
            u,
            v1: match (VERT, FLIP) {
                (false, false) if v1 < wx1 => wx1,
                (false, true) if wx2 < v1 => wx2,
                (true, false) if v1 < wy1 => wy1,
                (true, true) if wy2 < v1 => wy2,
                _ => v1,
            },
            v2: match (VERT, FLIP) {
                (false, false) if wx2 < v2 => wx2,
                (false, true) if v2 < wx1 => wx1,
                (true, false) if wy2 < v2 => wy2,
                (true, true) if v2 < wy1 => wy1,
                _ => v2,
            },
        })
    }
}

impl<const VERT: bool, const FLIP: bool> Iterator for SignedAxisAligned<VERT, FLIP> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = if !VERT { (self.v1, self.u) } else { (self.u, self.v1) };
        self.v1 += if !FLIP { 1 } else { -1 };
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // slightly optimized over `isize::abs_diff`,
        // see its implementation for the proof that this cast is legal
        #[allow(clippy::cast_sign_loss)]
        let length = match FLIP {
            false => usize::wrapping_sub(self.v2 as usize, self.v1 as usize),
            true => usize::wrapping_sub(self.v1 as usize, self.v2 as usize),
        };
        (length, Some(length))
    }
}

impl<const VERT: bool, const FLIP: bool> DoubleEndedIterator for SignedAxisAligned<VERT, FLIP> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        self.v2 -= if !FLIP { 1 } else { -1 };
        let (x, y) = if !VERT { (self.v2, self.u) } else { (self.u, self.v2) };
        Some((x, y))
    }
}

impl<const VERT: bool, const FLIP: bool> ExactSizeIterator for SignedAxisAligned<VERT, FLIP> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const VERT: bool, const FLIP: bool> core::iter::FusedIterator
    for SignedAxisAligned<VERT, FLIP>
{
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Axis-aligned line segment iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment aligned to the given *axis*,
/// with the direction of iteration determined at runtime.
///
/// An axis is defined by the *orientation* of the line segments it covers:
/// [vertical](Vertical) if `VERT`, [horizontal](Horizontal) otherwise.
///
/// If you know the [direction](SignedAxisAligned) of the line segment beforehand, consider
/// the more specific [positive](PositiveAxisAligned) and [negative](NegativeAxisAligned)
/// signed-axis-aligned iterators instead.
///
/// **Note**: an optimized implementation of [`AxisAligned::fold`] is provided.
/// This makes [`AxisAligned::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`AxisAligned::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AxisAligned<const VERT: bool> {
    /// See [`PositiveAxisAligned`].
    Positive(PositiveAxisAligned<VERT>),
    /// See [`NegativeAxisAligned`].
    Negative(NegativeAxisAligned<VERT>),
}

/// Iterator over a directed line segment aligned to the *horizontal axis*,
/// with the direction of iteration determined at runtime.
///
/// If you know the direction of the line segment beforehand, consider the more specific
/// [positive](PositiveHorizontal) and [negative horizontal](NegativeHorizontal) iterators instead.
///
/// **Note**: an optimized implementation of [`Horizontal::fold`] is provided.
/// This makes [`Horizontal::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`Horizontal::next`].
pub type Horizontal = AxisAligned<false>;

/// Iterator over a directed line segment aligned to the *vertical axis*,
/// with the direction of iteration determined at runtime.
///
/// If you know the direction of the line segment beforehand, consider the more specific
/// [positive](PositiveVertical) and [negative vertical](NegativeVertical) iterators instead.
///
/// **Note**: an optimized implementation of [`Vertical::fold`] is provided.
/// This makes [`Vertical::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`Vertical::next`].
pub type Vertical = AxisAligned<true>;

/// Delegates calls to directional variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::Positive($me) => $call,
            Self::Negative($me) => $call,
        }
    };
}

impl<const VERT: bool> AxisAligned<VERT> {
    /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment.
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    #[inline]
    #[must_use]
    pub const fn new(u: isize, v1: isize, v2: isize) -> Self {
        if v1 <= v2 {
            Self::Positive(SignedAxisAligned::new_inner(u, v1, v2))
        } else {
            Self::Negative(SignedAxisAligned::new_inner(u, v1, v2))
        }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }
}

#[cfg(feature = "clip")]
impl<const VERT: bool> AxisAligned<VERT> {
    /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment
    /// clipped to a [rectangular region](Region).
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(u: isize, v1: isize, v2: isize, region: Region<isize>) -> Option<Self> {
        if v1 <= v2 {
            map_option!(
                SignedAxisAligned::clip_inner(u, v1, v2, region),
                me => Self::Positive(me)
            )
        } else {
            map_option!(
                SignedAxisAligned::clip_inner(u, v1, v2, region),
                me => Self::Negative(me)
            )
        }
    }
}

impl<const VERT: bool> Iterator for AxisAligned<VERT> {
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

impl<const VERT: bool> DoubleEndedIterator for AxisAligned<VERT> {
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

impl<const VERT: bool> ExactSizeIterator for AxisAligned<VERT> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const VERT: bool> core::iter::FusedIterator for AxisAligned<VERT> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Orthogonal line segment iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment
/// aligned to a [signed axis](SignedAxisAligned) determined at runtime.
///
/// If you know the [axis alignment](AxisAligned) of the line segment beforehand, consider
/// the more specific [vertical](Vertical) and [horizontal](Horizontal) iterators instead.
///
/// **Note**: an optimized implementation of [`Orthogonal::fold`] is provided.
/// This makes [`Orthogonal::for_each`] faster than a `for` loop, since it checks
/// the signed axis of iteration only once instead of on every call to [`Orthogonal::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Orthogonal {
    /// See [`PositiveHorizontal`].
    PositiveHorizontal(PositiveHorizontal),
    /// See [`NegativeHorizontal`].
    NegativeHorizontal(NegativeHorizontal),
    /// See [`PositiveVertical`].
    PositiveVertical(PositiveVertical),
    /// See [`NegativeVertical`].
    NegativeVertical(NegativeVertical),
}

/// Delegates calls to signed-axis variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::PositiveHorizontal($me) => $call,
            Self::NegativeHorizontal($me) => $call,
            Self::PositiveVertical($me) => $call,
            Self::NegativeVertical($me) => $call,
        }
    };
}

impl Orthogonal {
    /// Returns an iterator over a directed line segment
    /// if it is [orthogonal](Orthogonal), otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
        if x1 == x2 {
            match Vertical::new(x1, y1, y2) {
                Vertical::Positive(me) => Some(Self::PositiveVertical(me)),
                Vertical::Negative(me) => Some(Self::NegativeVertical(me)),
            }
        } else if y1 == y2 {
            match Horizontal::new(x1, y1, y2) {
                Horizontal::Positive(me) => Some(Self::PositiveHorizontal(me)),
                Horizontal::Negative(me) => Some(Self::NegativeHorizontal(me)),
            }
        } else {
            None
        }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }
}

#[cfg(feature = "clip")]
impl Orthogonal {
    /// Returns an iterator over a directed line segment,
    /// if it is [orthogonal](Orthogonal), clipped to the [rectangular region](Region).
    ///
    /// Returns [`None`] if the line segment is not orthogonal
    /// or if it does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        region: Region<isize>,
    ) -> Option<Self> {
        if x1 == x2 {
            map_option!(
                AxisAligned::clip(x1, y1, y2, region),
                me => match me {
                    Vertical::Positive(me) => Self::PositiveVertical(me),
                    Vertical::Negative(me) => Self::NegativeVertical(me),
                }
            )
        } else if y1 == y2 {
            map_option!(
                AxisAligned::clip(x1, y1, y2, region),
                me => match me {
                    Horizontal::Positive(me) => Self::PositiveHorizontal(me),
                    Horizontal::Negative(me) => Self::NegativeHorizontal(me),
                }
            )
        } else {
            None
        }
    }
}

impl Iterator for Orthogonal {
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

impl DoubleEndedIterator for Orthogonal {
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

impl ExactSizeIterator for Orthogonal {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Orthogonal {}
