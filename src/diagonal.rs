//! ## Diagonal iterators
//!
//! This module provides a family of iterators for directed diagonal line segments.
//!
//! For any diagonal line segment, use the [diagonal](Diagonal) iterator.
//! If you know the direction and length of the diagonal line segment, use
//! one of the [diagonal quadrant](Quadrant) iterators instead.

use crate::clip::Clip;
use crate::math::{Math, Num, Point};
use crate::symmetry::{assert_sorted, fx, fy};
use crate::utils::map;

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
pub struct Quadrant<const FX: bool, const FY: bool, T> {
    x1: T,
    y1: T,
    x2: T,
}

/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` and `y` both increase.
pub type Quadrant0<T> = Quadrant<false, false, T>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` increases and `y` decreases.
pub type Quadrant1<T> = Quadrant<false, true, T>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` decreases and `y` increases.
pub type Quadrant2<T> = Quadrant<true, false, T>;
/// Iterator over a directed diagonal line segment covered
/// by the [quadrant](Quadrant) where `x` and `y` both decrease.
pub type Quadrant3<T> = Quadrant<true, true, T>;

macro_rules! quadrant_impl {
    ($T:ty) => {
        impl<const FX: bool, const FY: bool> Quadrant<FX, FY, $T> {
            #[inline(always)]
            #[must_use]
            pub(crate) const fn new_inner((x1, y1): Point<$T>, x2: $T) -> Self {
                Self { x1, y1, x2 }
            }

            #[inline(always)]
            #[must_use]
            const fn covers((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> bool {
                let dx = {
                    let (a, b) = assert_sorted!(FX, x1, x2, false);
                    Math::<$T>::delta(b, a)
                };
                let dy = {
                    let (a, b) = assert_sorted!(FY, y1, y2, false);
                    Math::<$T>::delta(b, a)
                };
                dx == dy
            }

            /// Returns an iterator over a directed line segment
            /// if it is diagonal and covered by the given [quadrant](Quadrant).
            ///
            /// Returns [`None`] if the line segment is not diagonal,
            /// or is not covered by the quadrant.
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
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
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: Clip<$T>,
            ) -> Option<Self> {
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
            #[inline]
            #[must_use]
            pub const fn length(&self) -> <$T as Num>::U {
                Math::<$T>::delta(fx!(self.x2, self.x1), fx!(self.x1, self.x2))
            }
        }

        impl<const FX: bool, const FY: bool> Iterator for Quadrant<FX, FY, $T> {
            type Item = Point<$T>;

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
                match usize::try_from(self.length()) {
                    Ok(length) => (length, Some(length)),
                    Err(_) => (usize::MAX, None),
                }
            }
        }

        impl<const FX: bool, const FY: bool> core::iter::FusedIterator for Quadrant<FX, FY, $T> {}
    };
}

quadrant_impl!(i8);
quadrant_impl!(u8);
quadrant_impl!(i16);
quadrant_impl!(u16);
quadrant_impl!(i32);
quadrant_impl!(u32);
quadrant_impl!(i64);
quadrant_impl!(u64);
quadrant_impl!(isize);
quadrant_impl!(usize);

macro_rules! quadrant_exact_size_iter_impl {
    ($T:ty) => {
        impl<const FX: bool, const FY: bool> ExactSizeIterator for Quadrant<FX, FY, $T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                self.is_done()
            }
        }
    };
}

quadrant_exact_size_iter_impl!(i8);
quadrant_exact_size_iter_impl!(u8);
quadrant_exact_size_iter_impl!(i16);
quadrant_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
quadrant_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
quadrant_exact_size_iter_impl!(u32);
#[cfg(target_pointer_width = "64")]
quadrant_exact_size_iter_impl!(i64);
#[cfg(target_pointer_width = "64")]
quadrant_exact_size_iter_impl!(u64);
quadrant_exact_size_iter_impl!(isize);
quadrant_exact_size_iter_impl!(usize);

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
        $T:ty,
        ($x1:ident, $y1:ident), ($x2:ident, $y2:ident),
        $quadrant_0:expr,
        $quadrant_1:expr,
        $quadrant_2:expr,
        $quadrant_3:expr$(,)?
    ) => {
        #[allow(clippy::cast_sign_loss)]
        {
            if $x1 < $x2 {
                let dx = Math::<$T>::delta($x2, $x1);
                if $y1 < $y2 {
                    let dy = Math::<$T>::delta($y2, $y1);
                    if dx != dy {
                        return None;
                    }
                    return $quadrant_0;
                }
                let dy = Math::<$T>::delta($y1, $y2);
                if dx != dy {
                    return None;
                }
                return $quadrant_1;
            }
            let dx = Math::<$T>::delta($x1, $x2);
            if $y1 < $y2 {
                let dy = Math::<$T>::delta($y2, $y1);
                if dx != dy {
                    return None;
                }
                return $quadrant_2;
            }
            let dy = Math::<$T>::delta($y1, $y2);
            if dx != dy {
                return None;
            }
            return $quadrant_3;
        }
    };
}

macro_rules! diagonal_impl {
    ($T:ty) => {
        impl Diagonal<$T> {
            /// Returns an iterator over a directed line segment
            /// if it is [diagonal](Diagonal), otherwise returns [`None`].
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                quadrants!(
                    $T,
                    (x1, y1),
                    (x2, y2),
                    Some(Self::Quadrant0(Quadrant0::<$T>::new_inner((x1, y1), x2))),
                    Some(Self::Quadrant1(Quadrant1::<$T>::new_inner((x1, y1), x2))),
                    Some(Self::Quadrant2(Quadrant2::<$T>::new_inner((x1, y1), x2))),
                    Some(Self::Quadrant3(Quadrant3::<$T>::new_inner((x1, y1), x2))),
                );
            }

            /// Returns an iterator over a directed line segment,
            /// if it is [diagonal](Diagonal), clipped to a [rectangular region](Clip).
            ///
            /// Returns [`None`] if the given line segment is not diagonal,
            /// or if it does not intersect the clipping region.
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: Clip<$T>
            ) -> Option<Self> {
                quadrants!(
                    $T,
                    (x1, y1),
                    (x2, y2),
                    map!(Quadrant0::<$T>::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant0),
                    map!(Quadrant1::<$T>::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant1),
                    map!(Quadrant2::<$T>::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant2),
                    map!(Quadrant3::<$T>::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant3),
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
            pub const fn length(&self) -> <$T as Num>::U {
                delegate!(self, me => me.length())
            }
        }

        impl Iterator for Diagonal<$T> {
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

        impl core::iter::FusedIterator for Diagonal<$T> {}
    };
}

diagonal_impl!(i8);
diagonal_impl!(u8);
diagonal_impl!(i16);
diagonal_impl!(u16);
diagonal_impl!(i32);
diagonal_impl!(u32);
diagonal_impl!(i64);
diagonal_impl!(u64);
diagonal_impl!(isize);
diagonal_impl!(usize);

macro_rules! diagonal_exact_size_iter_impl {
    ($T:ty) => {
        impl ExactSizeIterator for Diagonal<$T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                delegate!(self, me => me.is_empty())
            }
        }
    };
}

diagonal_exact_size_iter_impl!(i8);
diagonal_exact_size_iter_impl!(u8);
diagonal_exact_size_iter_impl!(i16);
diagonal_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
diagonal_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
diagonal_exact_size_iter_impl!(u32);
#[cfg(target_pointer_width = "64")]
diagonal_exact_size_iter_impl!(i64);
#[cfg(target_pointer_width = "64")]
diagonal_exact_size_iter_impl!(u64);
diagonal_exact_size_iter_impl!(isize);
diagonal_exact_size_iter_impl!(usize);

#[cfg(test)]
mod static_tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[cfg(target_pointer_width = "16")]
    #[test]
    const fn numerics_satisfy_target_pointer_width() {
        use static_assertions::assert_not_impl_any;

        assert_impl_all!(Quadrant0<i8>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u8>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<i16>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u16>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<isize>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<usize>: ExactSizeIterator);
        assert_not_impl_any!(Quadrant0<i32>: ExactSizeIterator);
        assert_not_impl_any!(Quadrant0<u32>: ExactSizeIterator);
        assert_not_impl_any!(Quadrant0<i64>: ExactSizeIterator);
        assert_not_impl_any!(Quadrant0<u64>: ExactSizeIterator);

        assert_impl_all!(Diagonal<i8>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u8>: ExactSizeIterator);
        assert_impl_all!(Diagonal<i16>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u16>: ExactSizeIterator);
        assert_impl_all!(Diagonal<isize>: ExactSizeIterator);
        assert_impl_all!(Diagonal<usize>: ExactSizeIterator);
        assert_not_impl_any!(Diagonal<i32>: ExactSizeIterator);
        assert_not_impl_any!(Diagonal<u32>: ExactSizeIterator);
        assert_not_impl_any!(Diagonal<i64>: ExactSizeIterator);
        assert_not_impl_any!(Diagonal<u64>: ExactSizeIterator);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    const fn numerics_satisfy_target_pointer_width() {
        use static_assertions::assert_not_impl_any;

        assert_impl_all!(Quadrant0<i8>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u8>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<i16>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u16>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<i32>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u32>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<isize>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<usize>: ExactSizeIterator);
        assert_not_impl_any!(Quadrant0<i64>: ExactSizeIterator);
        assert_not_impl_any!(Quadrant0<u64>: ExactSizeIterator);

        assert_impl_all!(Diagonal<i8>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u8>: ExactSizeIterator);
        assert_impl_all!(Diagonal<i16>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u16>: ExactSizeIterator);
        assert_impl_all!(Diagonal<i32>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u32>: ExactSizeIterator);
        assert_impl_all!(Diagonal<isize>: ExactSizeIterator);
        assert_impl_all!(Diagonal<usize>: ExactSizeIterator);
        assert_not_impl_any!(Diagonal<i64>: ExactSizeIterator);
        assert_not_impl_any!(Diagonal<u64>: ExactSizeIterator);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    const fn numerics_satisfy_target_pointer_width() {
        assert_impl_all!(Quadrant0<i8>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u8>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<i16>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u16>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<i32>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u32>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<i64>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<u64>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<isize>: ExactSizeIterator);
        assert_impl_all!(Quadrant0<usize>: ExactSizeIterator);

        assert_impl_all!(Diagonal<i8>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u8>: ExactSizeIterator);
        assert_impl_all!(Diagonal<i16>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u16>: ExactSizeIterator);
        assert_impl_all!(Diagonal<i32>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u32>: ExactSizeIterator);
        assert_impl_all!(Diagonal<i64>: ExactSizeIterator);
        assert_impl_all!(Diagonal<u64>: ExactSizeIterator);
        assert_impl_all!(Diagonal<isize>: ExactSizeIterator);
        assert_impl_all!(Diagonal<usize>: ExactSizeIterator);
    }
}
