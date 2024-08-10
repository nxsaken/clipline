//! ## Diagonal iterators

use crate::clip::Clip;
use crate::math::{Math, Num, Point};
use crate::symmetry::{fx, fy};
use crate::utils::{map, reject_if};

pub mod clip;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Diagonal quadrant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a diagonal line segment in the given **quadrant**.
///
/// A quadrant is defined by its symmetries relative to [`Diagonal0`]:
/// - `FX`: flip the `x` axis if `true`.
/// - `FY`: flip the `y` axis if `true`.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Diagonal<const FX: bool, const FY: bool, T> {
    x1: T,
    y1: T,
    x2: T,
}

/// Iterator over a [diagonal](Diagonal) line segment in the
/// [quadrant](Diagonal) where `x` and `y` **both increase**.
///
/// Covers line segments oriented at `45°`.
pub type Diagonal0<T> = Diagonal<false, false, T>;

/// Iterator over a [diagonal](Diagonal) line segment in the
/// [quadrant](Diagonal) where `x` **increases** and `y` **decreases**.
///
/// Covers line segments oriented at `315°`.
pub type Diagonal1<T> = Diagonal<false, true, T>;

/// Iterator over a [diagonal](Diagonal) line segment in the
/// [quadrant](Diagonal) where `x` **decreases** and `y` **increases**.
///
/// Covers line segments oriented at `135°`.
pub type Diagonal2<T> = Diagonal<true, false, T>;

/// Iterator over a [diagonal](Diagonal) line segment in the
/// [quadrant](Diagonal) where `x` and `y` **both decrease**.
///
/// Covers line segments oriented at `225°`.
pub type Diagonal3<T> = Diagonal<true, true, T>;

macro_rules! diagonal_impl {
    ($T:ty) => {
        impl<const FX: bool, const FY: bool> Diagonal<FX, FY, $T> {
            #[inline(always)]
            #[must_use]
            pub(crate) const fn new_inner((x1, y1): Point<$T>, x2: $T) -> Self {
                Self { x1, y1, x2 }
            }

            #[inline(always)]
            #[must_use]
            const fn covers((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> bool {
                let (u1, u2) = fx!((x1, x2), (x2, x1));
                let dx = if u1 < u2 {
                    Math::<$T>::delta(u2, u1)
                } else {
                    return false;
                };
                let (v1, v2) = fy!((y1, y2), (y2, y1));
                let dy = if v1 < v2 {
                    Math::<$T>::delta(v2, v1)
                } else {
                    return false;
                };
                dx == dy
            }

            /// Returns an iterator over a *half-open* line segment
            /// if it is diagonal and covered by the given [quadrant](Diagonal).
            ///
            /// Returns [`None`] if the line segment is not diagonal or covered by the quadrant.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if !Self::covers((x1, y1), (x2, y2)) {
                    return None;
                };
                Some(Self::new_inner((x1, y1), x2))
            }

            /// Clips a *half-open* line segment to a [rectangular region](Clip)
            /// if it is diagonal and covered by the given [quadrant](Diagonal),
            /// and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment is not diagonal or covered by the quadrant,
            /// or if it does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                let (u1, u2) = fx!((x1, x2), (x2, x1));
                reject_if!(u2 <= wx1 || wx2 < u1);
                let (v1, v2) = fx!((y1, y2), (y2, y1));
                reject_if!(v2 <= wy1 || wy2 < v1);
                if !Self::covers((x1, y1), (x2, y2)) {
                    return None;
                };
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

        impl<const FX: bool, const FY: bool> Iterator for Diagonal<FX, FY, $T> {
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

        impl<const FX: bool, const FY: bool> core::iter::FusedIterator for Diagonal<FX, FY, $T> {}
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
        impl<const FX: bool, const FY: bool> ExactSizeIterator for Diagonal<FX, FY, $T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                self.is_done()
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary diagonal iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over any [diagonal](Diagonal) line segment,
/// with the orientation determined at runtime.
///
/// If you know the orientation of the line segment beforehand,
/// use an iterator from the [`Diagonal`] family.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the orientation only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AnyDiagonal<T> {
    /// Diagonal line segment at `45°`, see [`Diagonal0`].
    Diagonal0(Diagonal0<T>),
    /// Diagonal line segment at `135°`, see [`Diagonal1`].
    Diagonal1(Diagonal1<T>),
    /// Diagonal line segment at `225°`, see [`Diagonal2`].
    Diagonal2(Diagonal2<T>),
    /// Diagonal line segment at `315°`, see [`Diagonal3`].
    Diagonal3(Diagonal3<T>),
}

macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::Diagonal0($me) => $call,
            Self::Diagonal1($me) => $call,
            Self::Diagonal2($me) => $call,
            Self::Diagonal3($me) => $call,
        }
    };
}

macro_rules! quadrant {
    ($Quadrant:ident, $T:ty, $p1:expr, $x2:expr) => {
        Self::$Quadrant($Quadrant::<$T>::new_inner($p1, $x2))
    };
    ($Quadrant:ident, $T:ty, $p1:expr, $p2:expr, $clip:expr) => {
        map!($Quadrant::<$T>::clip_inner($p1, $p2, $clip), Self::$Quadrant)
    };
}

pub(crate) use quadrant;

macro_rules! any_diagonal_impl {
    ($T:ty) => {
        impl AnyDiagonal<$T> {
            /// Returns an iterator over a *half-open* line segment
            /// if it is diagonal, otherwise returns [`None`].
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if x1 < x2 {
                    let dx = Math::<$T>::delta(x2, x1);
                    if y1 < y2 {
                        let dy = Math::<$T>::delta(y2, y1);
                        reject_if!(dx != dy);
                        return Some(quadrant!(Diagonal0, $T, (x1, y1), x2));
                    }
                    let dy = Math::<$T>::delta(y1, y2);
                    reject_if!(dx != dy);
                    return Some(quadrant!(Diagonal1, $T, (x1, y1), x2));
                }
                let dx = Math::<$T>::delta(x1, x2);
                if y1 < y2 {
                    let dy = Math::<$T>::delta(y2, y1);
                    reject_if!(dx != dy);
                    return Some(quadrant!(Diagonal2, $T, (x1, y1), x2));
                }
                let dy = Math::<$T>::delta(y1, y2);
                reject_if!(dx != dy);
                return Some(quadrant!(Diagonal3, $T, (x1, y1), x2));
            }

            /// Clips a *half-open* line segment to a [rectangular region](Clip)
            /// if it is diagonal, and returns an iterator over it.
            ///
            /// Returns [`None`] if the given line segment is not diagonal,
            /// or if it does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>
            ) -> Option<Self> {
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                if x1 < x2 {
                    // TODO: strict comparison for closed line segments
                    reject_if!(x2 <= wx1 || wx2 < x1);
                    let dx = Math::<$T>::delta(x2, x1);
                    if y1 < y2 {
                        reject_if!(y2 <= wy1 || wy2 < y1);
                        let dy = Math::<$T>::delta(y2, y1);
                        reject_if!(dx != dy);
                        return quadrant!(Diagonal0, $T, (x1, y1), (x2, y2), clip);
                    }
                    reject_if!(y1 < wy1 || wy2 <= y2);
                    let dy = Math::<$T>::delta(y1, y2);
                    reject_if!(dx != dy);
                    return quadrant!(Diagonal1, $T, (x1, y1), (x2, y2), clip);
                }
                reject_if!(x1 < wx1 || wx2 <= x2);
                let dx = Math::<$T>::delta(x1, x2);
                if y1 < y2 {
                    reject_if!(y2 <= wy1 || wy2 < y1);
                    let dy = Math::<$T>::delta(y2, y1);
                    reject_if!(dx != dy);
                    return quadrant!(Diagonal2, $T, (x1, y1), (x2, y2), clip);
                }
                reject_if!(y1 < wy1 || wy2 <= y2);
                let dy = Math::<$T>::delta(y1, y2);
                reject_if!(dx != dy);
                return quadrant!(Diagonal3, $T, (x1, y1), (x2, y2), clip);
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

        impl Iterator for AnyDiagonal<$T> {
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

        impl core::iter::FusedIterator for AnyDiagonal<$T> {}
    };
}

any_diagonal_impl!(i8);
any_diagonal_impl!(u8);
any_diagonal_impl!(i16);
any_diagonal_impl!(u16);
any_diagonal_impl!(i32);
any_diagonal_impl!(u32);
any_diagonal_impl!(i64);
any_diagonal_impl!(u64);
any_diagonal_impl!(isize);
any_diagonal_impl!(usize);

macro_rules! any_diagonal_exact_size_iter_impl {
    ($T:ty) => {
        impl ExactSizeIterator for AnyDiagonal<$T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                delegate!(self, me => me.is_empty())
            }
        }
    };
}

any_diagonal_exact_size_iter_impl!(i8);
any_diagonal_exact_size_iter_impl!(u8);
any_diagonal_exact_size_iter_impl!(i16);
any_diagonal_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
any_diagonal_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
any_diagonal_exact_size_iter_impl!(u32);
#[cfg(target_pointer_width = "64")]
any_diagonal_exact_size_iter_impl!(i64);
#[cfg(target_pointer_width = "64")]
any_diagonal_exact_size_iter_impl!(u64);
any_diagonal_exact_size_iter_impl!(isize);
any_diagonal_exact_size_iter_impl!(usize);

#[cfg(test)]
mod static_tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    const fn iterator_8() {
        assert_impl_all!(Diagonal0<i8>: ExactSizeIterator);
        assert_impl_all!(Diagonal0<u8>: ExactSizeIterator);
        assert_impl_all!(AnyDiagonal<i8>: ExactSizeIterator);
        assert_impl_all!(AnyDiagonal<u8>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_16() {
        assert_impl_all!(Diagonal0<i16>: ExactSizeIterator);
        assert_impl_all!(Diagonal0<u16>: ExactSizeIterator);
        assert_impl_all!(AnyDiagonal<i16>: ExactSizeIterator);
        assert_impl_all!(AnyDiagonal<u16>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_32() {
        #[cfg(target_pointer_width = "16")]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(Diagonal0<i32>: Iterator);
            assert_impl_all!(Diagonal0<u32>: Iterator);
            assert_impl_all!(AnyDiagonal<i32>: Iterator);
            assert_impl_all!(AnyDiagonal<u32>: Iterator);
            assert_not_impl_any!(Diagonal0<i32>: ExactSizeIterator);
            assert_not_impl_any!(Diagonal0<u32>: ExactSizeIterator);
            assert_not_impl_any!(AnyDiagonal<i32>: ExactSizeIterator);
            assert_not_impl_any!(AnyDiagonal<u32>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            assert_impl_all!(Diagonal0<i32>: ExactSizeIterator);
            assert_impl_all!(Diagonal0<u32>: ExactSizeIterator);
            assert_impl_all!(AnyDiagonal<i32>: ExactSizeIterator);
            assert_impl_all!(AnyDiagonal<u32>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_64() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_impl_all!(Diagonal0<i64>: ExactSizeIterator);
            assert_impl_all!(Diagonal0<u64>: ExactSizeIterator);
            assert_impl_all!(AnyDiagonal<i64>: ExactSizeIterator);
            assert_impl_all!(AnyDiagonal<u64>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(Diagonal0<i64>: Iterator);
            assert_impl_all!(Diagonal0<u64>: Iterator);
            assert_impl_all!(AnyDiagonal<i64>: Iterator);
            assert_impl_all!(AnyDiagonal<u64>: Iterator);
            assert_not_impl_any!(Diagonal0<i64>: ExactSizeIterator);
            assert_not_impl_any!(Diagonal0<u64>: ExactSizeIterator);
            assert_not_impl_any!(AnyDiagonal<i64>: ExactSizeIterator);
            assert_not_impl_any!(AnyDiagonal<u64>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_pointer_size() {
        assert_impl_all!(Diagonal0<isize>: ExactSizeIterator);
        assert_impl_all!(Diagonal0<usize>: ExactSizeIterator);
        assert_impl_all!(AnyDiagonal<isize>: ExactSizeIterator);
        assert_impl_all!(AnyDiagonal<usize>: ExactSizeIterator);
    }
}
