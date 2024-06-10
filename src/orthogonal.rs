//! ## Orthogonal & axis-aligned iterators
//!
//! This module provides iterators for orthogonal and axis-aligned directed line segments.
//!
//! For an arbitrary orthogonal line segment, use the [orthogonal](Orthogonal) iterator, which
//! determines the orientation and direction at runtime. If you know the orientation beforehand,
//! use an [axis-aligned](AxisAligned) iterator: [vertical](Vertical) or [horizontal](Horizontal).
//!
//! If you also know the direction, you can specialize further and pick a [signed-axis-aligned]
//! iterator: [positive](PositiveAxisAligned) and [negative](NegativeAxisAligned), or
//! [signed horizontal](SignedHorizontal) and [signed vertical](SignedVertical). Even more specific
//! [positive horizontal](PositiveHorizontal), [negative horizontal](NegativeHorizontal),
//! [positive vertical](PositiveVertical) and [negative vertical](NegativeVertical)
//! type aliases are available for convenience.

use crate::clip::Clip;
use crate::math::{Math, Point};
use crate::symmetry::{f, vh};
use crate::utils::map_opt;

mod clip;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signed-axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment covered by the given *signed axis*.
///
/// A signed axis is defined by the *orientation* and *direction* of the line segments it covers:
/// - [vertical](SignedVertical) if `VERT`, [horizontal](SignedHorizontal) otherwise.
/// - [negative](NegativeAxisAligned) if `FLIP`, [positive](PositiveAxisAligned) otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SignedAxisAligned<T, const VERT: bool, const FLIP: bool> {
    u: T,
    v1: T,
    v2: T,
}

/// Iterator over a directed line segment
/// covered by the given *positive* [signed axis](SignedAxisAligned).
pub type PositiveAxisAligned<T, const VERT: bool> = SignedAxisAligned<T, VERT, false>;
/// Iterator over a directed line segment
/// covered by the given *negative* [signed axis](SignedAxisAligned).
pub type NegativeAxisAligned<T, const VERT: bool> = SignedAxisAligned<T, VERT, true>;

/// Iterator over a directed line segment
/// covered by the given *horizontal* [signed axis](SignedAxisAligned).
pub type SignedHorizontal<T, const FLIP: bool> = SignedAxisAligned<T, false, FLIP>;
/// Iterator over a directed line segment
/// covered by the given *vertical* [signed axis](SignedAxisAligned).
pub type SignedVertical<T, const FLIP: bool> = SignedAxisAligned<T, true, FLIP>;

/// Iterator over a directed line segment
/// covered by the *positive horizontal* [signed axis](SignedAxisAligned).
pub type PositiveHorizontal<T> = SignedHorizontal<T, false>;
/// Iterator over a directed line segment
/// covered by the *negative horizontal* [signed axis](SignedAxisAligned).
pub type NegativeHorizontal<T> = SignedHorizontal<T, true>;
/// Iterator over a directed line segment
/// covered by the *positive vertical* [signed axis](SignedAxisAligned).
pub type PositiveVertical<T> = SignedVertical<T, false>;
/// Iterator over a directed line segment
/// covered by the *negative vertical* [signed axis](SignedAxisAligned).
pub type NegativeVertical<T> = SignedVertical<T, true>;

impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<i8, VERT, FLIP> {
    /// Returns an iterator over an axis-aligned directed line segment
    /// covered by the given [signed axis](SignedAxisAligned).
    ///
    /// *Assumes that the line segment is covered by the given signed axis.*
    #[inline(always)]
    #[must_use]
    const fn new_unchecked(u: i8, v1: i8, v2: i8) -> Self {
        Self { u, v1, v2 }
    }

    /// Returns an iterator over a directed line segment
    /// covered by the given [signed axis](AxisAligned).
    ///
    /// The line segment is defined by its starting point and its length.
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment is not covered by the signed axis.
    #[inline]
    #[must_use]
    pub const fn new(u: i8, v1: i8, v2: i8) -> Option<Self> {
        if f!(v2 <= v1, v1 <= v2) {
            return None;
        }
        Some(Self::new_unchecked(u, v1, v2))
    }

    /// Returns an iterator over a directed line segment
    /// covered by the given [signed axis](AxisAligned),
    /// clipped to a [rectangular region](Clip).
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if it does not intersect the clipping region.
    ///
    /// *Assumes that the line segment is covered by the given signed axis.*
    #[inline(always)]
    #[must_use]
    const fn clip_unchecked(u: i8, v1: i8, v2: i8, clip: Clip<i8>) -> Option<Self> {
        if clip::out_of_bounds::<VERT, FLIP>(u, v1, v2, clip) {
            return None;
        }
        Some(Self::new_unchecked(
            u,
            clip::enter::<VERT, FLIP>(v1, clip),
            clip::exit::<VERT, FLIP>(v2, clip),
        ))
    }

    /// Returns an iterator over a directed line segment
    /// covered by the given [signed axis](AxisAligned),
    /// clipped to a [rectangular region](Clip).
    ///
    /// The line segment is defined by its starting point and its length.
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment is not covered by the signed axis,
    /// or does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(u: i8, v1: i8, v2: i8, clip: Clip<i8>) -> Option<Self> {
        if f!(v2 <= v1, v1 <= v2) {
            return None;
        }
        Self::clip_unchecked(u, v1, v2, clip)
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        f!(self.v2 <= self.v1, self.v1 <= self.v2)
    }

    /// Returns the remaining length of this iterator.
    ///
    /// Optimized over [`i8::abs_diff`].
    #[inline]
    #[must_use]
    pub const fn length(&self) -> u8 {
        Math::delta(f!(self.v2, self.v1), f!(self.v1, self.v2))
    }
}

impl<const VERT: bool, const FLIP: bool> Iterator for SignedAxisAligned<i8, VERT, FLIP> {
    type Item = Point<i8>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = vh!((self.v1, self.u), (self.u, self.v1));
        self.v1 = f!(self.v1.wrapping_add(1), self.v1.wrapping_sub(1));
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.length() as usize;
        (length, Some(length))
    }
}

impl<const VERT: bool, const FLIP: bool> DoubleEndedIterator for SignedAxisAligned<i8, VERT, FLIP> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        self.v2 = f!(self.v2.wrapping_sub(1), self.v2.wrapping_add(1));
        let (x, y) = vh!((self.v2, self.u), (self.u, self.v2));
        Some((x, y))
    }
}

impl<const VERT: bool, const FLIP: bool> ExactSizeIterator for SignedAxisAligned<i8, VERT, FLIP> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const VERT: bool, const FLIP: bool> core::iter::FusedIterator
    for SignedAxisAligned<i8, VERT, FLIP>
{
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment covered by the given *axis*,
/// with the direction of iteration determined at runtime.
///
/// An axis is defined by the *orientation* of the line segments it covers:
/// [vertical](Vertical) if `VERT`, [horizontal](Horizontal) otherwise.
///
/// If you know the [direction](SignedAxisAligned) of the line segment beforehand, consider
/// the more specific [`PositiveAxisAligned`] and [`NegativeAxisAligned`] iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AxisAligned<T, const VERT: bool> {
    /// See [`PositiveAxisAligned`].
    Positive(PositiveAxisAligned<T, VERT>),
    /// See [`NegativeAxisAligned`].
    Negative(NegativeAxisAligned<T, VERT>),
}

/// Iterator over a directed line segment covered by the *horizontal axis*,
/// with the direction of iteration determined at runtime.
///
/// If you know the direction of the line segment beforehand, consider the more
/// specific [`PositiveHorizontal`] and [`NegativeHorizontal`] iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`Iterator::next`].
pub type Horizontal<T> = AxisAligned<T, false>;

/// Iterator over a directed line segment covered by the *vertical axis*,
/// with the direction of iteration determined at runtime.
///
/// If you know the direction of the line segment beforehand, consider the more
/// specific [`PositiveVertical`] and [`NegativeVertical`] iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`Iterator::next`].
pub type Vertical<T> = AxisAligned<T, true>;

/// Delegates calls to directional variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::Positive($me) => $call,
            Self::Negative($me) => $call,
        }
    };
}

impl<const VERT: bool> AxisAligned<i8, VERT> {
    /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment.
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    #[inline]
    #[must_use]
    pub const fn new(u: i8, v1: i8, v2: i8) -> Self {
        if v1 <= v2 {
            Self::Positive(SignedAxisAligned::new_unchecked(u, v1, v2))
        } else {
            Self::Negative(SignedAxisAligned::new_unchecked(u, v1, v2))
        }
    }

    /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment
    /// clipped to a [rectangular region](Clip).
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(u: i8, v1: i8, v2: i8, clip: Clip<i8>) -> Option<Self> {
        if v1 <= v2 {
            map_opt!(SignedAxisAligned::clip_unchecked(u, v1, v2, clip), Self::Positive)
        } else {
            map_opt!(SignedAxisAligned::clip_unchecked(u, v1, v2, clip), Self::Negative)
        }
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
    pub const fn length(&self) -> u8 {
        delegate!(self, me => me.length())
    }
}

impl<const VERT: bool> Iterator for AxisAligned<i8, VERT> {
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

impl<const VERT: bool> DoubleEndedIterator for AxisAligned<i8, VERT> {
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

impl<const VERT: bool> ExactSizeIterator for AxisAligned<i8, VERT> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const VERT: bool> core::iter::FusedIterator for AxisAligned<i8, VERT> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Orthogonal iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed [vertical](Vertical) or [horizontal](Horizontal) line segment,
/// with the [signed axis](SignedAxisAligned) of iteration determined at runtime.
///
/// If you know the [axis-alignment](AxisAligned) of the line segment beforehand,
/// consider the more specific [`Vertical`] and [`Horizontal`] iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the signed axis of iteration only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Orthogonal<T> {
    /// Horizontal line segment at `0°`, see [`PositiveHorizontal`].
    SignedAxis0(PositiveHorizontal<T>),
    /// Horizontal line segment at `180°`, see [`NegativeHorizontal`].
    SignedAxis1(NegativeHorizontal<T>),
    /// Vertical line segment at `90°`, see [`PositiveVertical`].
    SignedAxis2(PositiveVertical<T>),
    /// Vertical line segment at `270°`, see [`NegativeVertical`].
    SignedAxis3(NegativeVertical<T>),
}

/// Delegates calls to signed-axis variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::SignedAxis0($me) => $call,
            Self::SignedAxis1($me) => $call,
            Self::SignedAxis2($me) => $call,
            Self::SignedAxis3($me) => $call,
        }
    };
}

impl Orthogonal<i8> {
    /// Returns an iterator over a directed line segment
    /// if it is [orthogonal](Orthogonal), otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<i8>, (x2, y2): Point<i8>) -> Option<Self> {
        if y1 == y2 {
            return match Horizontal::new(x1, y1, y2) {
                AxisAligned::Positive(me) => Some(Self::SignedAxis0(me)),
                AxisAligned::Negative(me) => Some(Self::SignedAxis1(me)),
            };
        }
        if x1 == x2 {
            return match Vertical::new(x1, y1, y2) {
                AxisAligned::Positive(me) => Some(Self::SignedAxis2(me)),
                AxisAligned::Negative(me) => Some(Self::SignedAxis3(me)),
            };
        }
        None
    }

    /// Returns an iterator over a directed line segment,
    /// if it is [orthogonal](Orthogonal), clipped to the [rectangular region](Clip).
    ///
    /// Returns [`None`] if the line segment is not orthogonal
    /// or if it does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip((x1, y1): Point<i8>, (x2, y2): Point<i8>, clip: Clip<i8>) -> Option<Self> {
        if y1 == y2 {
            return map_opt!(
                Horizontal::clip(x1, y1, y2, clip),
                me => match me {
                    AxisAligned::Positive(me) => Self::SignedAxis0(me),
                    AxisAligned::Negative(me) => Self::SignedAxis1(me),
                }
            );
        }
        if x1 == x2 {
            return map_opt!(
                Vertical::clip(x1, y1, y2, clip),
                me => match me {
                    AxisAligned::Positive(me) => Self::SignedAxis2(me),
                    AxisAligned::Negative(me) => Self::SignedAxis3(me),
                }
            );
        }
        None
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
    pub const fn length(&self) -> u8 {
        delegate!(self, me => me.length())
    }
}

impl Iterator for Orthogonal<i8> {
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

impl DoubleEndedIterator for Orthogonal<i8> {
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

impl ExactSizeIterator for Orthogonal<i8> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Orthogonal<i8> {}
