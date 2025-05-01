//! ## Diagonal iterators

use crate::clip::Clip;
use crate::macros::control_flow::{map, return_if, unwrap_or_return, variant};
use crate::macros::derive::{fwd, iter_esi, iter_fwd, iter_rev, nums, rev};
use crate::macros::symmetry::{fx, fy};
use crate::math::{Math, Num, Point};

mod clip;
mod convert;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Diagonal quadrant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over a diagonal line segment whose
/// *directions* along `x` and `y` are known at compile-time.
///
/// - `FX` fixes the direction of the covered line segments along `x`:
///   * `false` – *increasing*, see [`Diagonal0`] and [`Diagonal1`].
///   * `true` – *decreasing*, see [`Diagonal2`] and [`Diagonal3`].
///
/// - `FY` fixes the direction along `y`:
///   * `false` – *increasing*, see [`Diagonal0`] and [`Diagonal2`].
///   * `true` – *decreasing*, see [`Diagonal1`] and [`Diagonal3`].
///
/// - If the directions are determined at runtime, see [`AnyDiagonal`].
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Diagonal<const FX: bool, const FY: bool, T: Num> {
    x1: T,
    y1: T,
    x2: T,
}

/// An iterator over a diagonal line segment
/// in the quadrant where `x` and `y` *both increase*.
///
/// - Covers line segments oriented at `45°`.
/// - Covers empty line segments.
/// - `x1 < x2`, `y1 < y2`
pub type Diagonal0<T> = Diagonal<false, false, T>;

/// An iterator over a diagonal line segment
/// in the quadrant where `x` *increases* and `y` *decreases*.
///
/// - Covers line segments oriented at `315°`.
/// - `x1 < x2`, `y2 < y1`
pub type Diagonal1<T> = Diagonal<false, true, T>;

/// An iterator over a diagonal line segment
/// in the quadrant where `x` *decreases* and `y` *increases*.
///
/// - Covers line segments oriented at `135°`.
/// - `x2 < x1`, `y1 < y2`
pub type Diagonal2<T> = Diagonal<true, false, T>;

/// An iterator over a diagonal line segment
/// in the quadrant where `x` and `y` *both decrease*.
///
/// - Covers line segments oriented at `225°`.
/// - `x2 < x1`, `y2 < y1`
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

macro_rules! impl_diagonal {
    ($T:ty, cfg_size = $cfg_size:meta) => {
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
                let du = Math::<$T>::sub_tt(fx!(x2, x1), fx!(x1, x2));
                let dv = Math::<$T>::sub_tt(fy!(y2, y1), fy!(y1, y2));
                du == dv
            }

            /// Constructs a [`Diagonal`] over a half-open line segment.
            ///
            /// Returns [`None`] if the line segment is not diagonal,
            /// or not covered by the quadrant.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                return_if!(!Self::covers((x1, y1), (x2, y2)));
                Some(Self::new_inner((x1, y1), x2))
            }

            /// Clips a half-open line segment to a rectangular region
            /// and constructs a [`Diagonal`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment is not diagonal,
            /// not covered by the quadrant, or lies outside the clipping region.
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

            fwd!(
                self,
                $T,
                is_done = fx!(self.x2 <= self.x1, self.x1 <= self.x2),
                length = Math::<$T>::sub_tt(fx!(self.x2, self.x1), fx!(self.x1, self.x2)),
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

            rev!(
                self,
                $T,
                tail = {
                    return_if!(self.is_done());
                    let x2 = fx!(self.x2.wrapping_sub(1), self.x2.wrapping_add(1));
                    let dx = Math::<$T>::sub_tt(fx!(x2, self.x1), fx!(self.x1, x2));
                    let y2 = fy!(Math::<$T>::add_tu(self.y1, dx), Math::<$T>::sub_tu(self.y1, dx));
                    Some((x2, y2))
                },
                pop_tail = {
                    let (x2, y2) = unwrap_or_return!(self.tail());
                    self.x2 = x2;
                    Some((x2, y2))
                },
            );
        }

        iter_fwd!(
            Diagonal<const FX, const FY, $T>,
            self,
            next = self.pop_head(),
            size_hint = {
                match usize::try_from(self.length()) {
                    Ok(length) => (length, Some(length)),
                    Err(_) => (usize::MAX, None),
                }
            },
        );

        #[$cfg_size]
        iter_esi!(
            Diagonal<const FX, const FY, $T>,
            self,
            is_empty = self.is_done(),
        );

        iter_rev!(
            Diagonal<const FX, const FY, $T>,
            self,
            next_back = self.pop_tail(),
        );
    };
}

nums!(impl_diagonal, cfg_size);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary diagonal iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over a diagonal line segment whose
/// *directions* along `x` and `y` are determined at runtime.
///
/// If the directions are known at compile-time, see [`Diagonal`].
///
/// **Note**: optimized [`Iterator::fold`] checks the quadrant once, not on every call
/// to [`Iterator::next`]. This makes [`Iterator::for_each`] faster than a `for` loop.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum AnyDiagonal<T: Num> {
    /// See [`Diagonal0`].
    Diagonal0(Diagonal0<T>),
    /// See [`Diagonal1`].
    Diagonal1(Diagonal1<T>),
    /// See [`Diagonal2`].
    Diagonal2(Diagonal2<T>),
    /// See [`Diagonal3`].
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
    ($T:ty, cfg_size = $cfg_size:meta) => {
        impl AnyDiagonal<$T> {
            /// Constructs an [`AnyDiagonal`] over a half-open line segment.
            ///
            /// Returns [`None`] if the line segment is not diagonal.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if x1 <= x2 {
                    let dx = Math::<$T>::sub_tt(x2, x1);
                    if y1 <= y2 {
                        let dy = Math::<$T>::sub_tt(y2, y1);
                        return_if!(dx != dy);
                        return Some(quadrant!(Diagonal0, $T, (x1, y1), x2));
                    }
                    let dy = Math::<$T>::sub_tt(y1, y2);
                    return_if!(dx != dy);
                    return Some(quadrant!(Diagonal1, $T, (x1, y1), x2));
                }
                let dx = Math::<$T>::sub_tt(x1, x2);
                if y1 <= y2 {
                    let dy = Math::<$T>::sub_tt(y2, y1);
                    return_if!(dx != dy);
                    return Some(quadrant!(Diagonal2, $T, (x1, y1), x2));
                }
                let dy = Math::<$T>::sub_tt(y1, y2);
                return_if!(dx != dy);
                return Some(quadrant!(Diagonal3, $T, (x1, y1), x2));
            }

            /// Clips a half-open line segment to a rectangular region
            /// and constructs an [`AnyDiagonal`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment is not diagonal,
            /// or lies outside the clipping region.
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
                    let dx = Math::<$T>::sub_tt(x2, x1);
                    if y1 <= y2 {
                        return_if!(y2 <= wy1 || wy2 < y1);
                        let dy = Math::<$T>::sub_tt(y2, y1);
                        return_if!(dx != dy);
                        return quadrant!(Diagonal0, $T, (x1, y1), (x2, y2), clip);
                    }
                    return_if!(y1 < wy1 || wy2 <= y2);
                    let dy = Math::<$T>::sub_tt(y1, y2);
                    return_if!(dx != dy);
                    return quadrant!(Diagonal1, $T, (x1, y1), (x2, y2), clip);
                }
                return_if!(x1 < wx1 || wx2 <= x2);
                let dx = Math::<$T>::sub_tt(x1, x2);
                if y1 <= y2 {
                    return_if!(y2 <= wy1 || wy2 < y1);
                    let dy = Math::<$T>::sub_tt(y2, y1);
                    return_if!(dx != dy);
                    return quadrant!(Diagonal2, $T, (x1, y1), (x2, y2), clip);
                }
                return_if!(y1 < wy1 || wy2 <= y2);
                let dy = Math::<$T>::sub_tt(y1, y2);
                return_if!(dx != dy);
                return quadrant!(Diagonal3, $T, (x1, y1), (x2, y2), clip);
            }

            fwd!($T, Self::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
            rev!($T, Self::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
        }

        iter_fwd!(AnyDiagonal<$T>::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
        #[$cfg_size]
        iter_esi!(AnyDiagonal<$T>::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
        iter_rev!(AnyDiagonal<$T>::{Diagonal0, Diagonal1, Diagonal2, Diagonal3});
    };
}

nums!(impl_any_diagonal, cfg_size);

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
