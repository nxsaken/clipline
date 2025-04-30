//! ## Diagonal iterators

use crate::clip::Clip;
use crate::macros::control_flow::{map, return_if, unwrap_or_return, variant};
use crate::macros::derive::{all_nums, impl_fwd, impl_iter_fwd, impl_iter_rev, impl_rev};
use crate::macros::symmetry::{fx, fy};
use crate::math::{Math, Num, Point};

mod clip;
mod convert;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Diagonal quadrant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a diagonal line segment in the given **quadrant**.
///
/// A quadrant is defined by its symmetries relative to [`Diagonal0`]:
/// - `FX`: flip the `x` axis if `true`.
/// - `FY`: flip the `y` axis if `true`.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Diagonal<const FX: bool, const FY: bool, T: Num> {
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

impl<const FX: bool, const FY: bool, T: Num> core::fmt::Debug for Diagonal<FX, FY, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(fx!(fy!("Diagonal0", "Diagonal1"), fy!("Diagonal2", "Diagonal3")))
            .field("x1", &self.x1)
            .field("y1", &self.y1)
            .field("x2", &self.x2)
            .finish()
    }
}

macro_rules! diagonal_impl {
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
        impl<const FX: bool, const FY: bool> Diagonal<FX, FY, $T> {
            #[inline(always)]
            #[must_use]
            pub(crate) const fn new_inner((x1, y1): Point<$T>, x2: $T) -> Self {
                Self { x1, y1, x2 }
            }

            #[inline(always)]
            #[must_use]
            const fn covers((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> bool {
                return_if!(fx!(x2 < x1, x1 <= x2), false);
                return_if!(fy!(y2 < y1, y1 <= y2), false);
                let du = Math::<$T>::delta(fx!(x2, x1), fx!(x1, x2));
                let dv = Math::<$T>::delta(fy!(y2, y1), fy!(y1, y2));
                du == dv
            }

            /// Returns an iterator over a *half-open* line segment
            /// if it is diagonal and covered by the given [quadrant](Diagonal).
            ///
            /// Returns [`None`] if the line segment is not diagonal or covered by the quadrant.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                return_if!(!Self::covers((x1, y1), (x2, y2)));
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
                return_if!(!Self::covers((x1, y1), (x2, y2)));
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                let (u1, u2) = fx!((x1, x2), (x2, x1));
                return_if!(u2 <= wx1 || wx2 < u1);
                let (v1, v2) = fx!((y1, y2), (y2, y1));
                return_if!(v2 <= wy1 || wy2 < v1);
                Self::clip_inner((x1, y1), (x2, y2), clip)
            }

            impl_fwd!(
                self,
                $T,
                is_done = fx!(self.x2 <= self.x1, self.x1 <= self.x2),
                length = Math::<$T>::delta(fx!(self.x2, self.x1), fx!(self.x1, self.x2)),
                head = {
                    return_if!(self.is_done());
                    Some((self.x1, self.y1))
                },
                pop_head = {
                    let (x1, y1) = unwrap_or_return!(self.head());
                    self.x1 = fx!(self.x1.wrapping_add(1), self.x1.wrapping_sub(1));
                    self.y1 = fy!(self.y1.wrapping_add(1), self.y1.wrapping_sub(1));
                    Some((x1, y1))
                },
            );

            impl_rev!(
                self,
                $T,
                tail = {
                    return_if!(self.is_done());
                    let x2 = fx!(self.x2.wrapping_sub(1), self.x2.wrapping_add(1));
                    let dx = Math::<$T>::delta(fx!(x2, self.x1), fx!(self.x1, x2));
                    let y2 = fy!(Math::<$T>::add_delta(self.y1, dx), Math::<$T>::sub_delta(self.y1, dx));
                    Some((x2, y2))
                },
                pop_tail = {
                    let (x2, y2) = unwrap_or_return!(self.tail());
                    self.x2 = x2;
                    Some((x2, y2))
                },
            );
        }

        impl_iter_fwd!(
            Diagonal<const FX, const FY, $T>,
            self,
            next = self.pop_head(),
            size_hint = {
                match usize::try_from(self.length()) {
                    Ok(length) => (length, Some(length)),
                    Err(_) => (usize::MAX, None),
                }
            },
            is_empty = self.is_done()
            $(, cfg_esi = $cfg_esi)?
        );

        impl_iter_rev!(
            Diagonal<const FX, const FY, $T>,
            self,
            next_back = self.pop_tail(),
        );
    };
}

all_nums!(diagonal_impl);

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
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum AnyDiagonal<T: Num> {
    /// Diagonal line segment at `45°`, see [`Diagonal0`].
    Diagonal0(Diagonal0<T>),
    /// Diagonal line segment at `135°`, see [`Diagonal1`].
    Diagonal1(Diagonal1<T>),
    /// Diagonal line segment at `225°`, see [`Diagonal2`].
    Diagonal2(Diagonal2<T>),
    /// Diagonal line segment at `315°`, see [`Diagonal3`].
    Diagonal3(Diagonal3<T>),
}

impl<T: Num> core::fmt::Debug for AnyDiagonal<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AnyDiagonal::")?;
        variant!(Self::{Diagonal0, Diagonal1, Diagonal2, Diagonal3}, self, me => me.fmt(f))
    }
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

macro_rules! impl_any_diagonal {
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
        impl AnyDiagonal<$T> {
            /// Returns an iterator over a *half-open* line segment
            /// if it is diagonal, otherwise returns [`None`].
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if x1 <= x2 {
                    let dx = Math::<$T>::delta(x2, x1);
                    if y1 <= y2 {
                        let dy = Math::<$T>::delta(y2, y1);
                        return_if!(dx != dy);
                        return Some(quadrant!(Diagonal0, $T, (x1, y1), x2));
                    }
                    let dy = Math::<$T>::delta(y1, y2);
                    return_if!(dx != dy);
                    return Some(quadrant!(Diagonal1, $T, (x1, y1), x2));
                }
                let dx = Math::<$T>::delta(x1, x2);
                if y1 <= y2 {
                    let dy = Math::<$T>::delta(y2, y1);
                    return_if!(dx != dy);
                    return Some(quadrant!(Diagonal2, $T, (x1, y1), x2));
                }
                let dy = Math::<$T>::delta(y1, y2);
                return_if!(dx != dy);
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
                if x1 <= x2 {
                    // TODO: strict comparison for closed line segments
                    return_if!(x2 <= wx1 || wx2 < x1);
                    let dx = Math::<$T>::delta(x2, x1);
                    if y1 <= y2 {
                        return_if!(y2 <= wy1 || wy2 < y1);
                        let dy = Math::<$T>::delta(y2, y1);
                        return_if!(dx != dy);
                        return quadrant!(Diagonal0, $T, (x1, y1), (x2, y2), clip);
                    }
                    return_if!(y1 < wy1 || wy2 <= y2);
                    let dy = Math::<$T>::delta(y1, y2);
                    return_if!(dx != dy);
                    return quadrant!(Diagonal1, $T, (x1, y1), (x2, y2), clip);
                }
                return_if!(x1 < wx1 || wx2 <= x2);
                let dx = Math::<$T>::delta(x1, x2);
                if y1 <= y2 {
                    return_if!(y2 <= wy1 || wy2 < y1);
                    let dy = Math::<$T>::delta(y2, y1);
                    return_if!(dx != dy);
                    return quadrant!(Diagonal2, $T, (x1, y1), (x2, y2), clip);
                }
                return_if!(y1 < wy1 || wy2 <= y2);
                let dy = Math::<$T>::delta(y1, y2);
                return_if!(dx != dy);
                return quadrant!(Diagonal3, $T, (x1, y1), (x2, y2), clip);
            }

            impl_fwd!($T, Self::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
            impl_rev!($T, Self::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
        }

        impl_iter_fwd!(AnyDiagonal<$T>::{Diagonal0, Diagonal1, Diagonal2, Diagonal3} $(, cfg_esi = $cfg_esi)?);
        impl_iter_rev!(AnyDiagonal<$T>::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
    };
}

all_nums!(impl_any_diagonal);

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
