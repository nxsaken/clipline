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
use crate::math::{Math, Num, Point};
use crate::symmetry::{f, vh};
use crate::utils::map;

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
pub struct SignedAxisAligned<const VERT: bool, const FLIP: bool, T> {
    u: T,
    v1: T,
    v2: T,
}

/// Iterator over a directed line segment
/// covered by the given *positive* [signed axis](SignedAxisAligned).
pub type PositiveAxisAligned<const VERT: bool, T> = SignedAxisAligned<VERT, false, T>;
/// Iterator over a directed line segment
/// covered by the given *negative* [signed axis](SignedAxisAligned).
pub type NegativeAxisAligned<const VERT: bool, T> = SignedAxisAligned<VERT, true, T>;

/// Iterator over a directed line segment
/// covered by the given *horizontal* [signed axis](SignedAxisAligned).
pub type SignedHorizontal<const FLIP: bool, T> = SignedAxisAligned<false, FLIP, T>;
/// Iterator over a directed line segment
/// covered by the given *vertical* [signed axis](SignedAxisAligned).
pub type SignedVertical<const FLIP: bool, T> = SignedAxisAligned<true, FLIP, T>;

/// Iterator over a directed line segment
/// covered by the *positive horizontal* [signed axis](SignedAxisAligned).
pub type PositiveHorizontal<T> = SignedHorizontal<false, T>;
/// Iterator over a directed line segment
/// covered by the *negative horizontal* [signed axis](SignedAxisAligned).
pub type NegativeHorizontal<T> = SignedHorizontal<true, T>;
/// Iterator over a directed line segment
/// covered by the *positive vertical* [signed axis](SignedAxisAligned).
pub type PositiveVertical<T> = SignedVertical<false, T>;
/// Iterator over a directed line segment
/// covered by the *negative vertical* [signed axis](SignedAxisAligned).
pub type NegativeVertical<T> = SignedVertical<true, T>;

macro_rules! signed_axis_impl {
    ($T:ty) => {
        impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<VERT, FLIP, $T> {
            #[inline(always)]
            #[must_use]
            const fn new_inner(u: $T, v1: $T, v2: $T) -> Self {
                Self { u, v1, v2 }
            }

            /// Returns an iterator over a directed line segment
            /// if it is covered by the given [signed axis](SignedAxisAligned),
            /// otherwise returns [`None`].
            ///
            /// The line segment is defined by its starting point and its length.
            ///
            /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
            /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
            ///
            /// **Note**: `(u, v2)`/`(v2, u)` is not included.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Option<Self> {
                if f!(v2 <= v1, v1 <= v2) {
                    return None;
                }
                Some(Self::new_inner(u, v1, v2))
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
            ///
            /// **Note**: `(u, v2)`/`(v2, u)` is not included.
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                if f!(v2 <= v1, v1 <= v2) {
                    return None;
                }
                Self::clip_inner(u, v1, v2, clip)
            }

            /// Returns `true` if the iterator has terminated.
            #[inline]
            #[must_use]
            pub const fn is_done(&self) -> bool {
                f!(self.v2 <= self.v1, self.v1 <= self.v2)
            }

            /// Returns the remaining length of this iterator.
            #[inline]
            #[must_use]
            pub const fn length(&self) -> <$T as Num>::U {
                Math::<$T>::delta(f!(self.v2, self.v1), f!(self.v1, self.v2))
            }
        }

        impl<const VERT: bool, const FLIP: bool> Iterator for SignedAxisAligned<VERT, FLIP, $T> {
            type Item = Point<$T>;

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
                match usize::try_from(self.length()) {
                    Ok(length) => (length, Some(length)),
                    Err(_) => (usize::MAX, None),
                }
            }
        }

        impl<const VERT: bool, const FLIP: bool> DoubleEndedIterator
            for SignedAxisAligned<VERT, FLIP, $T>
        {
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

        impl<const VERT: bool, const FLIP: bool> core::iter::FusedIterator
            for SignedAxisAligned<VERT, FLIP, $T>
        {
        }
    };
}

signed_axis_impl!(i8);
signed_axis_impl!(u8);
signed_axis_impl!(i16);
signed_axis_impl!(u16);
signed_axis_impl!(i32);
signed_axis_impl!(u32);
signed_axis_impl!(i64);
signed_axis_impl!(u64);
signed_axis_impl!(isize);
signed_axis_impl!(usize);

macro_rules! signed_axis_exact_size_iter_impl {
    ($T:ty) => {
        impl<const VERT: bool, const FLIP: bool> ExactSizeIterator
            for SignedAxisAligned<VERT, FLIP, $T>
        {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                self.is_done()
            }
        }
    };
}

signed_axis_exact_size_iter_impl!(i8);
signed_axis_exact_size_iter_impl!(u8);
signed_axis_exact_size_iter_impl!(i16);
signed_axis_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
signed_axis_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
signed_axis_exact_size_iter_impl!(u32);
#[cfg(target_pointer_width = "64")]
signed_axis_exact_size_iter_impl!(i64);
#[cfg(target_pointer_width = "64")]
signed_axis_exact_size_iter_impl!(u64);
signed_axis_exact_size_iter_impl!(isize);
signed_axis_exact_size_iter_impl!(usize);

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
pub enum AxisAligned<const VERT: bool, T> {
    /// See [`PositiveAxisAligned`].
    Positive(PositiveAxisAligned<VERT, T>),
    /// See [`NegativeAxisAligned`].
    Negative(NegativeAxisAligned<VERT, T>),
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
pub type Horizontal<T> = AxisAligned<false, T>;

/// Iterator over a directed line segment covered by the *vertical axis*,
/// with the direction of iteration determined at runtime.
///
/// If you know the direction of the line segment beforehand, consider the more
/// specific [`PositiveVertical`] and [`NegativeVertical`] iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction of iteration only once instead of on every call to [`Iterator::next`].
pub type Vertical<T> = AxisAligned<true, T>;

/// Delegates calls to directional variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::Positive($me) => $call,
            Self::Negative($me) => $call,
        }
    };
}

macro_rules! axis_impl {
    ($T:ty) => {
        impl<const VERT: bool> AxisAligned<VERT, $T> {
            /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment.
            ///
            /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
            /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
            ///
            /// **Note**: `(u, v2)`/`(v2, u)` is not included.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Self {
                if v1 <= v2 {
                    Self::Positive(PositiveAxisAligned::<VERT, $T>::new_inner(u, v1, v2))
                } else {
                    Self::Negative(NegativeAxisAligned::<VERT, $T>::new_inner(u, v1, v2))
                }
            }

            /// Returns an iterator over an [axis-aligned](AxisAligned) directed line segment
            /// clipped to a [rectangular region](Clip).
            ///
            /// - A [vertical](Vertical) line segment has endpoints `(u, v1)` and `(u, v2)`.
            /// - A [horizontal](Horizontal) line segment has endpoints `(v1, u)` and `(v2, u)`.
            ///
            /// Returns [`None`] if the line segment does not intersect the clipping region.
            ///
            /// **Note**: `(u, v2)`/`(v2, u)` is not included.
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                if v1 <= v2 {
                    map!(
                        PositiveAxisAligned::<VERT, $T>::clip_inner(u, v1, v2, clip),
                        Self::Positive,
                    )
                } else {
                    map!(
                        NegativeAxisAligned::<VERT, $T>::clip_inner(u, v1, v2, clip),
                        Self::Negative,
                    )
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
            pub const fn length(&self) -> <$T as Num>::U {
                delegate!(self, me => me.length())
            }
        }

        impl<const VERT: bool> Iterator for AxisAligned<VERT, $T> {
            type Item = Point<$T>;

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

        impl<const VERT: bool> DoubleEndedIterator for AxisAligned<VERT, $T> {
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

        impl<const VERT: bool> core::iter::FusedIterator for AxisAligned<VERT, $T> {}
    };
}

axis_impl!(i8);
axis_impl!(u8);
axis_impl!(i16);
axis_impl!(u16);
axis_impl!(i32);
axis_impl!(u32);
axis_impl!(i64);
axis_impl!(u64);
axis_impl!(isize);
axis_impl!(usize);

macro_rules! axis_exact_size_iter_impl {
    ($T:ty) => {
        impl<const VERT: bool> ExactSizeIterator for AxisAligned<VERT, $T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                delegate!(self, me => me.is_empty())
            }
        }
    };
}

axis_exact_size_iter_impl!(i8);
axis_exact_size_iter_impl!(u8);
axis_exact_size_iter_impl!(i16);
axis_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
axis_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
axis_exact_size_iter_impl!(u32);
#[cfg(target_pointer_width = "64")]
axis_exact_size_iter_impl!(i64);
#[cfg(target_pointer_width = "64")]
axis_exact_size_iter_impl!(u64);
axis_exact_size_iter_impl!(isize);
axis_exact_size_iter_impl!(usize);

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

macro_rules! orthogonal_impl {
    ($T:ty) => {
        impl Orthogonal<$T> {
            /// Returns an iterator over a directed line segment
            /// if it is [orthogonal](Orthogonal), otherwise returns [`None`].
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if y1 == y2 {
                    return match Horizontal::<$T>::new(y1, x1, x2) {
                        AxisAligned::Positive(me) => Some(Self::SignedAxis0(me)),
                        AxisAligned::Negative(me) => Some(Self::SignedAxis1(me)),
                    };
                }
                if x1 == x2 {
                    return match Vertical::<$T>::new(x1, y1, y2) {
                        AxisAligned::Positive(me) => Some(Self::SignedAxis2(me)),
                        AxisAligned::Negative(me) => Some(Self::SignedAxis3(me)),
                    };
                }
                None
            }

            /// Returns an iterator over a directed line segment,
            /// if it is [orthogonal](Orthogonal), clipped to the [rectangular region](Clip).
            ///
            /// Returns [`None`] if the line segment is not orthogonal,
            /// or if it does not intersect the clipping region.
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn clip((x1, y1): Point<$T>, (x2, y2): Point<$T>, clip: &Clip<$T>) -> Option<Self> {
                if y1 == y2 {
                    return map!(
                        Horizontal::<$T>::clip(y1, x1, x2, clip),
                        me => match me {
                            AxisAligned::Positive(me) => Self::SignedAxis0(me),
                            AxisAligned::Negative(me) => Self::SignedAxis1(me),
                        }
                    );
                }
                if x1 == x2 {
                    return map!(
                        Vertical::<$T>::clip(x1, y1, y2, clip),
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
            pub const fn length(&self) -> <$T as Num>::U {
                delegate!(self, me => me.length())
            }
        }

        impl Iterator for Orthogonal<$T> {
            type Item = Point<$T>;

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

        impl DoubleEndedIterator for Orthogonal<$T> {
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

        impl core::iter::FusedIterator for Orthogonal<$T> {}
    };
}

orthogonal_impl!(i8);
orthogonal_impl!(u8);
orthogonal_impl!(i16);
orthogonal_impl!(u16);
orthogonal_impl!(i32);
orthogonal_impl!(u32);
orthogonal_impl!(i64);
orthogonal_impl!(u64);
orthogonal_impl!(isize);
orthogonal_impl!(usize);

macro_rules! orthogonal_exact_size_iter_impl {
    ($T:ty) => {
        impl ExactSizeIterator for Orthogonal<$T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                delegate!(self, me => me.is_empty())
            }
        }
    };
}

orthogonal_exact_size_iter_impl!(i8);
orthogonal_exact_size_iter_impl!(u8);
orthogonal_exact_size_iter_impl!(i16);
orthogonal_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
orthogonal_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
orthogonal_exact_size_iter_impl!(u32);
#[cfg(target_pointer_width = "64")]
orthogonal_exact_size_iter_impl!(i64);
#[cfg(target_pointer_width = "64")]
orthogonal_exact_size_iter_impl!(u64);
orthogonal_exact_size_iter_impl!(isize);
orthogonal_exact_size_iter_impl!(usize);

#[cfg(test)]
mod static_tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    const fn iterator_8() {
        assert_impl_all!(PositiveVertical<i8>: ExactSizeIterator);
        assert_impl_all!(PositiveVertical<u8>: ExactSizeIterator);
        assert_impl_all!(Vertical<i8>: ExactSizeIterator);
        assert_impl_all!(Vertical<u8>: ExactSizeIterator);
        assert_impl_all!(Orthogonal<i8>: ExactSizeIterator);
        assert_impl_all!(Orthogonal<u8>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_16() {
        assert_impl_all!(PositiveVertical<i16>: ExactSizeIterator);
        assert_impl_all!(PositiveVertical<u16>: ExactSizeIterator);
        assert_impl_all!(Vertical<i16>: ExactSizeIterator);
        assert_impl_all!(Vertical<u16>: ExactSizeIterator);
        assert_impl_all!(Orthogonal<i16>: ExactSizeIterator);
        assert_impl_all!(Orthogonal<u16>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_32() {
        #[cfg(target_pointer_width = "16")]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(PositiveVertical<i32>: Iterator);
            assert_impl_all!(PositiveVertical<u32>: Iterator);
            assert_impl_all!(Vertical<i32>: Iterator);
            assert_impl_all!(Vertical<u32>: Iterator);
            assert_impl_all!(Orthogonal<i32>: Iterator);
            assert_impl_all!(Orthogonal<u32>: Iterator);
            assert_not_impl_any!(PositiveVertical<i32>: ExactSizeIterator);
            assert_not_impl_any!(PositiveVertical<u32>: ExactSizeIterator);
            assert_not_impl_any!(Vertical<i32>: ExactSizeIterator);
            assert_not_impl_any!(Vertical<u32>: ExactSizeIterator);
            assert_not_impl_any!(Orthogonal<i32>: ExactSizeIterator);
            assert_not_impl_any!(Orthogonal<u32>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            assert_impl_all!(PositiveVertical<i32>: ExactSizeIterator);
            assert_impl_all!(PositiveVertical<u32>: ExactSizeIterator);
            assert_impl_all!(Vertical<i32>: ExactSizeIterator);
            assert_impl_all!(Vertical<u32>: ExactSizeIterator);
            assert_impl_all!(Orthogonal<i32>: ExactSizeIterator);
            assert_impl_all!(Orthogonal<u32>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_64() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_impl_all!(PositiveVertical<i64>: ExactSizeIterator);
            assert_impl_all!(PositiveVertical<u64>: ExactSizeIterator);
            assert_impl_all!(Vertical<i64>: ExactSizeIterator);
            assert_impl_all!(Vertical<u64>: ExactSizeIterator);
            assert_impl_all!(Orthogonal<i64>: ExactSizeIterator);
            assert_impl_all!(Orthogonal<u64>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(PositiveVertical<i64>: Iterator);
            assert_impl_all!(PositiveVertical<u64>: Iterator);
            assert_impl_all!(Vertical<i64>: Iterator);
            assert_impl_all!(Vertical<u64>: Iterator);
            assert_impl_all!(Orthogonal<i64>: Iterator);
            assert_impl_all!(Orthogonal<u64>: Iterator);
            assert_not_impl_any!(PositiveVertical<i64>: ExactSizeIterator);
            assert_not_impl_any!(PositiveVertical<u64>: ExactSizeIterator);
            assert_not_impl_any!(Vertical<i64>: ExactSizeIterator);
            assert_not_impl_any!(Vertical<u64>: ExactSizeIterator);
            assert_not_impl_any!(Orthogonal<i64>: ExactSizeIterator);
            assert_not_impl_any!(Orthogonal<u64>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_pointer_size() {
        assert_impl_all!(PositiveVertical<isize>: ExactSizeIterator);
        assert_impl_all!(PositiveVertical<usize>: ExactSizeIterator);
        assert_impl_all!(Vertical<isize>: ExactSizeIterator);
        assert_impl_all!(Vertical<usize>: ExactSizeIterator);
        assert_impl_all!(Orthogonal<isize>: ExactSizeIterator);
        assert_impl_all!(Orthogonal<usize>: ExactSizeIterator);
    }
}
