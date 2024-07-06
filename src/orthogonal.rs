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

use crate::{clip, Clip, Point};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signed-axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment aligned to the given *signed axis*.
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
/// aligned to the given *positive* [signed axis](SignedAxisAligned).
pub type PositiveAxisAligned<T, const VERT: bool> = SignedAxisAligned<T, VERT, false>;
/// Iterator over a directed line segment
/// aligned to the given *negative* [signed axis](SignedAxisAligned).
pub type NegativeAxisAligned<T, const VERT: bool> = SignedAxisAligned<T, VERT, true>;

/// Iterator over a directed line segment
/// aligned to the given *horizontal* [signed axis](SignedAxisAligned).
pub type SignedHorizontal<T, const FLIP: bool> = SignedAxisAligned<T, false, FLIP>;
/// Iterator over a directed line segment
/// aligned to the given *vertical* [signed axis](SignedAxisAligned).
pub type SignedVertical<T, const FLIP: bool> = SignedAxisAligned<T, true, FLIP>;

/// Iterator over a directed line segment
/// aligned to the *positive horizontal* [signed axis](SignedAxisAligned).
pub type PositiveHorizontal<T> = SignedHorizontal<T, false>;
/// Iterator over a directed line segment
/// aligned to the *negative horizontal* [signed axis](SignedAxisAligned).
pub type NegativeHorizontal<T> = SignedHorizontal<T, true>;
/// Iterator over a directed line segment
/// aligned to the *positive vertical* [signed axis](SignedAxisAligned).
pub type PositiveVertical<T> = SignedVertical<T, false>;
/// Iterator over a directed line segment
/// aligned to the *negative vertical* [signed axis](SignedAxisAligned).
pub type NegativeVertical<T> = SignedVertical<T, true>;

impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<isize, VERT, FLIP> {
    /// Creates an iterator over an axis-aligned directed line segment
    /// aligned to the given [signed axis](SignedAxisAligned).
    ///
    /// *Assumes that the line segment is aligned to the given signed axis.*
    #[inline(always)]
    #[must_use]
    const fn new_unchecked(u: isize, v1: isize, v2: isize) -> Self {
        Self { u, v1, v2 }
    }

    /// Creates an iterator over a directed line segment
    /// aligned to the given [signed axis](AxisAligned).
    ///
    /// The line segment is defined by its starting point and its length.
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    #[inline]
    #[must_use]
    pub const fn new(u: isize, v1: isize, length: isize) -> Self {
        let v2 = if !FLIP { v1 + length } else { v1 - length };
        Self::new_unchecked(u, v1, v2)
    }

    /// Creates an iterator over a directed line segment
    /// aligned to the given [signed axis](AxisAligned),
    /// clipped to a [rectangular region](Clip).
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if it does not intersect the clipping region.
    ///
    /// *Assumes that the line segment is aligned to the given signed axis.*
    #[inline(always)]
    #[must_use]
    const fn clip_unchecked(u: isize, v1: isize, v2: isize, clip: &Clip<isize>) -> Option<Self> {
        if clip::signed_axis::out_of_bounds::<VERT, FLIP>(u, v1, v2, clip) {
            return None;
        }
        Some(Self::new_unchecked(
            u,
            clip::signed_axis::enter::<VERT, FLIP>(v1, clip),
            clip::signed_axis::exit::<VERT, FLIP>(v2, clip),
        ))
    }

    /// Creates an iterator over a directed line segment
    /// aligned to the given [signed axis](AxisAligned),
    /// clipped to a [rectangular region](Clip).
    ///
    /// The line segment is defined by its starting point and its length.
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(u: isize, v1: isize, length: isize, clip: &Clip<isize>) -> Option<Self> {
        let v2 = if !FLIP { v1 + length } else { v1 - length };
        Self::clip_unchecked(u, v1, v2, clip)
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        !FLIP && self.v2 <= self.v1 || FLIP && self.v1 <= self.v2
    }
}

impl<const VERT: bool, const FLIP: bool> Iterator for SignedAxisAligned<isize, VERT, FLIP> {
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

impl<const VERT: bool, const FLIP: bool> DoubleEndedIterator
    for SignedAxisAligned<isize, VERT, FLIP>
{
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

impl<const VERT: bool, const FLIP: bool> ExactSizeIterator
    for SignedAxisAligned<isize, VERT, FLIP>
{
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const VERT: bool, const FLIP: bool> core::iter::FusedIterator
    for SignedAxisAligned<isize, VERT, FLIP>
{
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment aligned to the given *axis*,
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

/// Iterator over a directed line segment aligned to the *horizontal axis*,
/// with the direction of iteration determined at runtime.
///
/// If you know the direction of the line segment beforehand, consider the more
/// specific [`PositiveHorizontal`] and [`NegativeHorizontal`] iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`Iterator::next`].
pub type Horizontal<T> = AxisAligned<T, false>;

/// Iterator over a directed line segment aligned to the *vertical axis*,
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

impl<const VERT: bool> AxisAligned<isize, VERT> {
    /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment.
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    #[inline]
    #[must_use]
    pub const fn new(u: isize, v1: isize, v2: isize) -> Self {
        if v1 <= v2 {
            Self::Positive(SignedAxisAligned::new_unchecked(u, v1, v2))
        } else {
            Self::Negative(SignedAxisAligned::new_unchecked(u, v1, v2))
        }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }
}

impl<const VERT: bool> AxisAligned<isize, VERT> {
    /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment
    /// clipped to a [rectangular region](Clip).
    ///
    /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
    /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
    ///
    /// Returns [`None`] if the line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(u: isize, v1: isize, v2: isize, clip: &Clip<isize>) -> Option<Self> {
        if v1 <= v2 {
            clip::map_option!(
                SignedAxisAligned::clip_unchecked(u, v1, v2, clip),
                me => Self::Positive(me)
            )
        } else {
            clip::map_option!(
                SignedAxisAligned::clip_unchecked(u, v1, v2, clip),
                me => Self::Negative(me)
            )
        }
    }
}

impl<const VERT: bool> Iterator for AxisAligned<isize, VERT> {
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

impl<const VERT: bool> DoubleEndedIterator for AxisAligned<isize, VERT> {
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

impl<const VERT: bool> ExactSizeIterator for AxisAligned<isize, VERT> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const VERT: bool> core::iter::FusedIterator for AxisAligned<isize, VERT> {}

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

impl Orthogonal<isize> {
    /// Returns an iterator over a directed line segment
    /// if it is [orthogonal](Orthogonal), otherwise returns [`None`].
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
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

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }
}

impl Orthogonal<isize> {
    /// Returns an iterator over a directed line segment,
    /// if it is [orthogonal](Orthogonal), clipped to the [rectangular region](Clip).
    ///
    /// Returns [`None`] if the line segment is not orthogonal
    /// or if it does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        clip: &Clip<isize>,
    ) -> Option<Self> {
        if y1 == y2 {
            return clip::map_option!(
                Horizontal::clip(x1, y1, y2, clip),
                me => match me {
                    AxisAligned::Positive(me) => Self::SignedAxis0(me),
                    AxisAligned::Negative(me) => Self::SignedAxis1(me),
                }
            );
        }
        if x1 == x2 {
            return clip::map_option!(
                Vertical::clip(x1, y1, y2, clip),
                me => match me {
                    AxisAligned::Positive(me) => Self::SignedAxis2(me),
                    AxisAligned::Negative(me) => Self::SignedAxis3(me),
                }
            );
        }
        None
    }
}

impl Iterator for Orthogonal<isize> {
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

impl DoubleEndedIterator for Orthogonal<isize> {
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

impl ExactSizeIterator for Orthogonal<isize> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Orthogonal<isize> {}