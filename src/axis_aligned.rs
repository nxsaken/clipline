//! ## Axis-aligned iterators

use crate::clip::Clip;
use crate::math::{Math, Num, Point};
use crate::symmetry::{f, vh};
use crate::utils::map;

mod clip;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signed-axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a line segment aligned to the given **signed axis**.
///
/// A signed axis is defined by the direction and axis-alignment of the line segments aligned to it:
/// - [negative](NegativeAxis) if `FLIP`, [positive](PositiveAxis) otherwise.
/// - [vertical](SignedAxis1) if `VERT`, [horizontal](SignedAxis0) otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SignedAxis<const FLIP: bool, const VERT: bool, T> {
    u: T,
    v1: T,
    v2: T,
}

/// Iterator over a line segment aligned to the given
/// **positive** [signed axis](SignedAxis).
pub type PositiveAxis<const VERT: bool, T> = SignedAxis<false, VERT, T>;

/// Iterator over a line segment aligned to the given
/// **negative** [signed axis](SignedAxis).
pub type NegativeAxis<const VERT: bool, T> = SignedAxis<true, VERT, T>;

/// Iterator over a line segment aligned to the given
/// **horizontal** [signed axis](SignedAxis).
pub type SignedAxis0<const FLIP: bool, T> = SignedAxis<FLIP, false, T>;

/// Iterator over a line segment aligned to the given
/// **vertical** [signed axis](SignedAxis).
pub type SignedAxis1<const FLIP: bool, T> = SignedAxis<FLIP, true, T>;

/// Iterator over a line segment aligned to the
/// **positive horizontal** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `0°`.
pub type PositiveAxis0<T> = SignedAxis0<false, T>;

/// Iterator over a line segment aligned to the
/// **negative horizontal** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `180°`.
pub type NegativeAxis0<T> = SignedAxis0<true, T>;

/// Iterator over a line segment aligned to the
/// **positive vertical** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `90°`.
pub type PositiveAxis1<T> = SignedAxis1<false, T>;

/// Iterator over a line segment aligned to the
/// **negative vertical** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `270°`.
pub type NegativeAxis1<T> = SignedAxis1<true, T>;

macro_rules! signed_axis_impl {
    ($T:ty) => {
        impl<const FLIP: bool, const VERT: bool> SignedAxis<FLIP, VERT, $T> {
            #[inline(always)]
            #[must_use]
            const fn new_inner(u: $T, v1: $T, v2: $T) -> Self {
                Self { u, v1, v2 }
            }

            /// Returns an iterator over a *half-open* line segment if it is aligned to
            /// the given [signed axis](SignedAxis), otherwise returns [`None`].
            ///
            /// - A [horizontal](SignedAxis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](SignedAxis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Option<Self> {
                if f!(v2 <= v1, v1 <= v2) {
                    return None;
                }
                Some(Self::new_inner(u, v1, v2))
            }

            /// Clips a *half-open* line segment to a [rectangular region](Clip)
            /// if it aligned to the given [signed axis](SignedAxis),
            /// and returns an iterator over it.
            ///
            /// - A [horizontal](SignedAxis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](SignedAxis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            ///
            /// Returns [`None`] if the line segment is not aligned to the signed axis,
            /// or if it does not intersect the clipping region.
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

        impl<const FLIP: bool, const VERT: bool> Iterator for SignedAxis<FLIP, VERT, $T> {
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

        impl<const FLIP: bool, const VERT: bool> DoubleEndedIterator
            for SignedAxis<FLIP, VERT, $T>
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

        impl<const FLIP: bool, const VERT: bool> core::iter::FusedIterator
            for SignedAxis<FLIP, VERT, $T>
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
        impl<const FLIP: bool, const VERT: bool> ExactSizeIterator for SignedAxis<FLIP, VERT, $T> {
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

/// Iterator over a line segment aligned to the given **axis**,
/// with the direction determined at runtime.
///
/// An axis is defined by the orientation of the line segments it covers:
/// [vertical](Axis1) if `VERT`, [horizontal](Axis0) otherwise.
///
/// If you know the [direction](SignedAxis) of the line segment,
/// consider [`PositiveAxis`] and [`NegativeAxis`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Axis<const VERT: bool, T> {
    /// See [`PositiveAxis`].
    Positive(PositiveAxis<VERT, T>),
    /// See [`NegativeAxis`].
    Negative(NegativeAxis<VERT, T>),
}

/// Iterator over a line segment aligned to the **horizontal** [axis](Axis),
/// with the direction determined at runtime.
///
/// If you know the [direction](SignedAxis) of the line segment,
/// consider [`PositiveAxis0`] and [`NegativeAxis0`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction only once instead of on every call to [`Iterator::next`].
pub type Axis0<T> = Axis<false, T>;

/// Iterator over a line segment aligned to the **vertical** [axis](Axis),
/// with the direction determined at runtime.
///
/// If you know the [direction](SignedAxis) of the line segment,
/// consider [`PositiveAxis1`] and [`NegativeAxis1`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction only once instead of on every call to [`Iterator::next`].
pub type Axis1<T> = Axis<true, T>;

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
        impl<const VERT: bool> Axis<VERT, $T> {
            /// Returns an iterator over a *half-open* line segment aligned to the given [axis](Axis).
            ///
            /// - A [horizontal](Axis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](Axis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Self {
                if v1 <= v2 {
                    Self::Positive(PositiveAxis::<VERT, $T>::new_inner(u, v1, v2))
                } else {
                    Self::Negative(NegativeAxis::<VERT, $T>::new_inner(u, v1, v2))
                }
            }

            /// Clips a *half-open* line segment aligned to the given [axis](Axis)
            /// to a [rectangular region](Clip), and returns an iterator over it.
            ///
            /// - A [horizontal](Axis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](Axis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            ///
            /// Returns [`None`] if the line segment does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                if v1 <= v2 {
                    map!(
                        PositiveAxis::<VERT, $T>::clip_inner(u, v1, v2, clip),
                        Self::Positive,
                    )
                } else {
                    map!(
                        NegativeAxis::<VERT, $T>::clip_inner(u, v1, v2, clip),
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

        impl<const VERT: bool> Iterator for Axis<VERT, $T> {
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

        impl<const VERT: bool> DoubleEndedIterator for Axis<VERT, $T> {
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

        impl<const VERT: bool> core::iter::FusedIterator for Axis<VERT, $T> {}
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
        impl<const VERT: bool> ExactSizeIterator for Axis<VERT, $T> {
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
// Arbitrary axis-aligned iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a [horizontal](Axis0) or [vertical](Axis1) line segment,
/// with the axis-alignment and direction determined at runtime.
///
/// If you know the [axis-alignment](Axis) of the line segment, use [`Axis0`] or [`Axis1`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the signed axis only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AnyAxis<T> {
    /// Horizontal line segment at `0°`, see [`PositiveAxis0`].
    PositiveAxis0(PositiveAxis0<T>),
    /// Vertical line segment at `90°`, see [`PositiveAxis1`].
    PositiveAxis1(PositiveAxis1<T>),
    /// Horizontal line segment at `180°`, see [`NegativeAxis0`].
    NegativeAxis0(NegativeAxis0<T>),
    /// Vertical line segment at `270°`, see [`NegativeAxis1`].
    NegativeAxis1(NegativeAxis1<T>),
}

/// Delegates calls to signed-axis variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::PositiveAxis0($me) => $call,
            Self::NegativeAxis0($me) => $call,
            Self::PositiveAxis1($me) => $call,
            Self::NegativeAxis1($me) => $call,
        }
    };
}

macro_rules! any_axis_impl {
    ($T:ty) => {
        impl AnyAxis<$T> {
            /// Returns an iterator over a *half-open* line segment
            /// if it is aligned to any [axis](Axis), otherwise returns [`None`].
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if y1 == y2 {
                    return match Axis0::<$T>::new(y1, x1, x2) {
                        Axis::Positive(me) => Some(Self::PositiveAxis0(me)),
                        Axis::Negative(me) => Some(Self::NegativeAxis0(me)),
                    };
                }
                if x1 == x2 {
                    return match Axis1::<$T>::new(x1, y1, y2) {
                        Axis::Positive(me) => Some(Self::PositiveAxis1(me)),
                        Axis::Negative(me) => Some(Self::NegativeAxis1(me)),
                    };
                }
                None
            }

            /// Clips a *half-open* line segment to a [rectangular region](Clip)
            /// if it is aligned to any [axis](Axis), and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment is not axis-aligned,
            /// or if it does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip((x1, y1): Point<$T>, (x2, y2): Point<$T>, clip: &Clip<$T>) -> Option<Self> {
                if y1 == y2 {
                    return map!(
                        Axis0::<$T>::clip(y1, x1, x2, clip),
                        me => match me {
                            Axis::Positive(me) => Self::PositiveAxis0(me),
                            Axis::Negative(me) => Self::NegativeAxis0(me),
                        }
                    );
                }
                if x1 == x2 {
                    return map!(
                        Axis1::<$T>::clip(x1, y1, y2, clip),
                        me => match me {
                            Axis::Positive(me) => Self::PositiveAxis1(me),
                            Axis::Negative(me) => Self::NegativeAxis1(me),
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

        impl Iterator for AnyAxis<$T> {
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

        impl DoubleEndedIterator for AnyAxis<$T> {
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

        impl core::iter::FusedIterator for AnyAxis<$T> {}
    };
}

any_axis_impl!(i8);
any_axis_impl!(u8);
any_axis_impl!(i16);
any_axis_impl!(u16);
any_axis_impl!(i32);
any_axis_impl!(u32);
any_axis_impl!(i64);
any_axis_impl!(u64);
any_axis_impl!(isize);
any_axis_impl!(usize);

macro_rules! any_axis_exact_size_iter_impl {
    ($T:ty) => {
        impl ExactSizeIterator for AnyAxis<$T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                delegate!(self, me => me.is_empty())
            }
        }
    };
}

any_axis_exact_size_iter_impl!(i8);
any_axis_exact_size_iter_impl!(u8);
any_axis_exact_size_iter_impl!(i16);
any_axis_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
any_axis_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
any_axis_exact_size_iter_impl!(u32);
#[cfg(target_pointer_width = "64")]
any_axis_exact_size_iter_impl!(i64);
#[cfg(target_pointer_width = "64")]
any_axis_exact_size_iter_impl!(u64);
any_axis_exact_size_iter_impl!(isize);
any_axis_exact_size_iter_impl!(usize);

#[cfg(test)]
mod static_tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    const fn iterator_8() {
        assert_impl_all!(PositiveAxis0<i8>: ExactSizeIterator);
        assert_impl_all!(PositiveAxis0<u8>: ExactSizeIterator);
        assert_impl_all!(Axis0<i8>: ExactSizeIterator);
        assert_impl_all!(Axis0<u8>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<i8>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<u8>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_16() {
        assert_impl_all!(PositiveAxis0<i16>: ExactSizeIterator);
        assert_impl_all!(PositiveAxis0<u16>: ExactSizeIterator);
        assert_impl_all!(Axis0<i16>: ExactSizeIterator);
        assert_impl_all!(Axis0<u16>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<i16>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<u16>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_32() {
        #[cfg(target_pointer_width = "16")]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(PositiveAxis0<i32>: Iterator);
            assert_impl_all!(PositiveAxis0<u32>: Iterator);
            assert_impl_all!(Axis0<i32>: Iterator);
            assert_impl_all!(Axis0<u32>: Iterator);
            assert_impl_all!(AnyAxis<i32>: Iterator);
            assert_impl_all!(AnyAxis<u32>: Iterator);
            assert_not_impl_any!(PositiveAxis0<i32>: ExactSizeIterator);
            assert_not_impl_any!(PositiveAxis0<u32>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<i32>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<u32>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<i32>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<u32>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            assert_impl_all!(PositiveAxis0<i32>: ExactSizeIterator);
            assert_impl_all!(PositiveAxis0<u32>: ExactSizeIterator);
            assert_impl_all!(Axis0<i32>: ExactSizeIterator);
            assert_impl_all!(Axis0<u32>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<i32>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<u32>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_64() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_impl_all!(PositiveAxis0<i64>: ExactSizeIterator);
            assert_impl_all!(PositiveAxis0<u64>: ExactSizeIterator);
            assert_impl_all!(Axis0<i64>: ExactSizeIterator);
            assert_impl_all!(Axis0<u64>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<i64>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<u64>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(PositiveAxis0<i64>: Iterator);
            assert_impl_all!(PositiveAxis0<u64>: Iterator);
            assert_impl_all!(Axis0<i64>: Iterator);
            assert_impl_all!(Axis0<u64>: Iterator);
            assert_impl_all!(AnyAxis<i64>: Iterator);
            assert_impl_all!(AnyAxis<u64>: Iterator);
            assert_not_impl_any!(PositiveAxis0<i64>: ExactSizeIterator);
            assert_not_impl_any!(PositiveAxis0<u64>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<i64>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<u64>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<i64>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<u64>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_pointer_size() {
        assert_impl_all!(PositiveAxis0<isize>: ExactSizeIterator);
        assert_impl_all!(PositiveAxis0<usize>: ExactSizeIterator);
        assert_impl_all!(Axis0<isize>: ExactSizeIterator);
        assert_impl_all!(Axis0<usize>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<isize>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<usize>: ExactSizeIterator);
    }
}
