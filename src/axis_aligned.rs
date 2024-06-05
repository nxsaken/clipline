//! ## Axis-aligned line segment iterators
//!
//! This module provides a family of iterators
//! for directed axis-aligned line segments.

use crate::Point;

/// Iterator over a directed line segment aligned to the given signed axis.
///
/// A signed axis is defined by the orientation and direction
/// of the line segments covered by it:
/// - `VERT`: vertical if `true`, horizontal otherwise.
/// - `FLIP`: negative if `true`, positive otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SignedAxisAligned<const VERT: bool, const FLIP: bool> {
    u: isize,
    v1: isize,
    v2: isize,
}

/// Iterator over a positive [`SignedAxisAligned`] line segment with the given orientation.
pub type PositiveAxisAligned<const VERT: bool> = SignedAxisAligned<VERT, false>;
/// Iterator over a negative [`SignedAxisAligned`] line segment with the given orientation.
pub type NegativeAxisAligned<const VERT: bool> = SignedAxisAligned<VERT, true>;

/// Iterator over a horizontal [`SignedAxisAligned`] line segment with the given direction.
pub type SignedHorizontal<const FLIP: bool> = SignedAxisAligned<false, FLIP>;
/// Iterator over a vertical [`SignedAxisAligned`] line segment with the given direction.
pub type SignedVertical<const FLIP: bool> = SignedAxisAligned<true, FLIP>;

/// Iterator over a positive horizontal [`SignedAxisAligned`] line segment.
pub type PositiveHorizontal = SignedHorizontal<false>;
/// Iterator over a negative horizontal [`SignedAxisAligned`] line segment.
pub type NegativeHorizontal = SignedHorizontal<true>;
/// Iterator over a positive vertical [`SignedAxisAligned`] line segment.
pub type PositiveVertical = SignedVertical<false>;
/// Iterator over a negative vertical [`SignedAxisAligned`] line segment.
pub type NegativeVertical = SignedVertical<true>;

impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<VERT, FLIP> {
    /// Creates a [`SignedAxisAligned`] iterator over a directed line segment.
    ///
    /// ### Arguments
    /// * `u` - fixed coordinate (`x` for vertical lines, `y` for horizontal).
    /// * `v1`, `v2` - start and end *(exclusive)* coordinates along the variable axis.
    ///
    /// Returns [`None`] if the line segment is not aligned to the signed axis.
    #[inline]
    #[must_use]
    pub const fn new(u: isize, v1: isize, v2: isize) -> Option<Self> {
        if FLIP && v1 <= v2 || !FLIP && v2 <= v1 {
            return None;
        }
        Some(Self { u, v1, v2 })
    }

    #[inline]
    #[must_use]
    const fn new_unchecked(u: isize, v1: isize, v2: isize) -> Self {
        Self { u, v1, v2 }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        match FLIP {
            true => self.v1 <= self.v2,
            false => self.v2 <= self.v1,
        }
    }
}

impl<const VERT: bool, const FLIP: bool> Iterator for SignedAxisAligned<VERT, FLIP> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = match VERT {
            true => (self.u, self.v1),
            false => (self.v1, self.u),
        };
        self.v1 += if FLIP { -1 } else { 1 };
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // slightly optimized over `isize::abs_diff`,
        // see its implementation for the proof that this cast is legal
        #[allow(clippy::cast_sign_loss)]
        let length = match FLIP {
            true => usize::wrapping_sub(self.v1 as usize, self.v2 as usize),
            false => usize::wrapping_sub(self.v2 as usize, self.v1 as usize),
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
        self.v2 -= if FLIP { -1 } else { 1 };
        let (x, y) = match VERT {
            true => (self.u, self.v2),
            false => (self.v2, self.u),
        };
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

/// Iterator over a directed line segment aligned to the given axis.
///
/// Picks the direction of iteration at runtime.
///
/// **Note**: an optimized implementation of [`AxisAligned::fold`] is provided.
/// This makes [`AxisAligned::for_each`] faster than a `for` loop, since it checks
/// the iteration direction only once instead of on every call to [`AxisAligned::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AxisAligned<const VERT: bool> {
    /// See [`PositiveAxisAligned`].
    Positive(PositiveAxisAligned<VERT>),
    /// See [`NegativeAxisAligned`].
    Negative(NegativeAxisAligned<VERT>),
}

/// Iterator over a horizontal [`AxisAligned`] directed line segment.
///
/// Picks the direction of iteration at runtime.
///
/// **Note**: an optimized implementation of [`Horizontal::fold`] is provided.
/// This makes [`Horizontal::for_each`] faster than a `for` loop, since it checks
/// the iteration direction only once instead of on every call to [`Horizontal::next`].
pub type Horizontal = AxisAligned<false>;

/// Iterator over a vertical [`AxisAligned`] directed line segment.
///
/// Picks the direction of iteration at runtime.
///
/// **Note**: an optimized implementation of [`Vertical::fold`] is provided.
/// This makes [`Vertical::for_each`] faster than a `for` loop, since it checks
/// the iteration direction only once instead of on every call to [`Vertical::next`].
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
    /// Creates an [`AxisAligned`] iterator over a directed line segment.
    ///
    /// ### Arguments
    /// * `u` - fixed coordinate (`x` for vertical lines, `y` for horizontal).
    /// * `v1`, `v2` - start and end *(exclusive)* coordinates along the variable axis.
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
