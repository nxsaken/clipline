//! ## Axis-aligned iterators

use crate::clip::Clip;
use crate::macros::control_flow::{map, return_if, unwrap_or_return, variant};
use crate::macros::derive::{fwd, iter_esi, iter_fwd, iter_rev, nums, rev};
use crate::macros::symmetry::{f, v};
use crate::math::{Math, Num, Point};

mod clip;
mod convert;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signed-axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over an axis-aligned line segment whose
/// *direction* and *orientation* are known at compile-time.
///
/// - `F` fixes the direction of the covered line segments:
///   * `false` – *increasing*, see [`PositiveAxis`].
///   * `true` – *decreasing*, see [`NegativeAxis`].
///
/// - `V` fixes the orientation:
///   * `false` – *horizontal*, see [`SignedAxis0`].
///   * `true` – *vertical*, see [`SignedAxis1`].
///
/// - If both are fixed, see:
///   * [`PositiveAxis0`], [`NegativeAxis0`].
///   * [`PositiveAxis1`], [`NegativeAxis1`].
///
/// - If the direction is determined at runtime, see [`Axis`].
/// - If the orientation is determined at runtime too, see [`AnyAxis`].
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SignedAxis<const F: bool, const V: bool, T: Num> {
    u: T,
    v1: T,
    v2: T,
}

/// An iterator over a *horizontal* axis-aligned line segment
/// whose *direction* is known at compile-time.
///
/// - `F` fixes the direction of the covered line segments:
///   * `false` – increasing, see [`PositiveAxis0`].
///   * `true` – decreasing, see [`NegativeAxis0`].
///
/// - If the direction is determined at runtime, see [`Axis0`].
pub type SignedAxis0<const F: bool, T> = SignedAxis<F, false, T>;

/// An iterator over a *vertical* axis-aligned line segment
/// whose *direction* is known at compile-time.
///
/// - `F` fixes the direction of the covered line segments:
///   * `false` – increasing, see [`PositiveAxis1`].
///   * `true` – decreasing, see [`NegativeAxis1`].
///
/// - If the direction is determined at runtime, see [`Axis1`].
pub type SignedAxis1<const F: bool, T> = SignedAxis<F, true, T>;

/// An iterator over a *positive* axis-aligned line segment
/// whose *orientation* is known at compile-time.
///
/// - `V` fixes the orientation of the covered line segments:
///   * `false` – horizontal, see [`PositiveAxis0`].
///   * `true` – vertical, see [`PositiveAxis1`].
///
/// - Covers empty line segments.
pub type PositiveAxis<const V: bool, T> = SignedAxis<false, V, T>;

/// An iterator over a *negative* axis-aligned line segment
/// whose *orientation* is known at compile-time.
///
/// - `V` fixes the orientation of the covered line segments:
///   * `false` – horizontal, see [`NegativeAxis0`].
///   * `true` – vertical, see [`NegativeAxis1`].
pub type NegativeAxis<const V: bool, T> = SignedAxis<true, V, T>;

/// An iterator over a *positive*, *horizontal* axis-aligned line segment.
///
/// - Covers line segments oriented at `0°`.
/// - Covers empty line segments.
/// - `y = u`, `v1 <= x < v2`
pub type PositiveAxis0<T> = PositiveAxis<false, T>;

/// An iterator over a *negative*, *horizontal* axis-aligned line segment.
///
/// - Covers line segments oriented at `180°`.
/// - `y = u`, `v1 >= x > v2`
pub type NegativeAxis0<T> = NegativeAxis<false, T>;

/// An iterator over a *positive*, *vertical* axis-aligned line segment.
///
/// - Covers line segments oriented at `90°`.
/// - Covers empty line segments.
/// - `x = u`, `v1 <= y < v2`
pub type PositiveAxis1<T> = PositiveAxis<true, T>;

/// An iterator over a *negative*, *vertical* axis-aligned line segment.
///
/// - Covers line segments oriented at `270°`.
/// - `x = u`, `v1 >= y > v2`
pub type NegativeAxis1<T> = NegativeAxis<true, T>;

impl<const F: bool, const V: bool, T: Num> core::fmt::Debug for SignedAxis<F, V, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(f!(
            v!("PositiveAxis0", "PositiveAxis1"),
            v!("NegativeAxis0", "NegativeAxis1")
        ))
        .field(v!("y", "x"), &self.u)
        .field(v!("x1", "y1"), &self.v1)
        .field(v!("x2", "y2"), &self.v2)
        .finish()
    }
}

macro_rules! impl_signed_axis {
    ($T:ty, cfg_size = $cfg_size:meta) => {
        impl<const F: bool, const V: bool> SignedAxis<F, V, $T> {
            #[inline(always)]
            #[must_use]
            const fn new_inner(u: $T, v1: $T, v2: $T) -> Self {
                Self { u, v1, v2 }
            }

            #[inline(always)]
            #[must_use]
            const fn covers(v1: $T, v2: $T) -> bool {
                f!(v1 <= v2, v2 < v1)
            }

            /// Constructs a [`SignedAxis`] over a half-open line segment.
            ///
            /// Returns [`None`] if the line segment is not covered by the signed axis.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Option<Self> {
                return_if!(!Self::covers(v1, v2));
                Some(Self::new_inner(u, v1, v2))
            }

            /// Clips a half-open line segment to a rectangular region
            /// and constructs a [`SignedAxis`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment is not covered by the signed axis,
            /// or lies outside the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                return_if!(!Self::covers(v1, v2));
                Self::clip_inner(u, v1, v2, clip)
            }

            fwd!(
                self,
                $T,
                is_done = f!(self.v2 <= self.v1, self.v1 <= self.v2),
                length = Math::<$T>::sub_tt(f!(self.v2, self.v1), f!(self.v1, self.v2)),
                head = {
                    return_if!(self.is_done());
                    let (x1, y1) = v!((self.v1, self.u), (self.u, self.v1));
                    Some((x1, y1))
                },
                pop_head = {
                    let (x1, y1) = unwrap_or_return!(self.head());
                    self.v1 = f!(self.v1.wrapping_add(1), self.v1.wrapping_sub(1));
                    Some((x1, y1))
                },
            );

            rev!(
                self,
                $T,
                tail = {
                    return_if!(self.is_done());
                    let v2 = f!(self.v2.wrapping_sub(1), self.v2.wrapping_add(1));
                    let (x2, y2) = v!((v2, self.u), (self.u, v2));
                    Some((x2, y2))
                },
                pop_tail = {
                    let (x2, y2) = unwrap_or_return!(self.tail());
                    self.v2 = v!(x2, y2);
                    Some((x2, y2))
                },
            );
        }

        iter_fwd!(
            SignedAxis<const F, const V, $T>,
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
            SignedAxis<const F, const V, $T>,
            self,
            is_empty = self.is_done(),
        );

        iter_rev!(
            SignedAxis<const F, const V, $T>,
            self,
            next_back = self.pop_tail(),
        );
    };
}

nums!(impl_signed_axis, cfg_size);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over an axis-aligned line segment whose *direction*
/// is determined at runtime, and *orientation* is known at compile-time.
///
/// - `V` fixes the orientation of the covered line segments:
///   * `false` – *horizontal*, see [`Axis0`].
///   * `true` – *vertical*, see [`Axis1`].
/// - If the direction is known at compile-time, see [`SignedAxis`].
/// - If the orientation is determined at runtime, see [`AnyAxis`].
///
/// **Note**: optimized [`Iterator::fold`] checks the direction once, not on every call
/// to [`Iterator::next`]. This makes [`Iterator::for_each`] faster than a `for` loop.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Axis<const V: bool, T: Num> {
    /// See [`PositiveAxis`].
    Positive(PositiveAxis<V, T>),
    /// See [`NegativeAxis`].
    Negative(NegativeAxis<V, T>),
}

/// An iterator over a *horizontal* axis-aligned line segment
/// whose *direction* is determined at runtime.
///
/// If the direction is known at compile-time, see [`SignedAxis0`].
///
/// **Note**: optimized [`Iterator::fold`] checks the direction once, not on every call
/// to [`Iterator::next`]. This makes [`Iterator::for_each`] faster than a `for` loop.
pub type Axis0<T> = Axis<false, T>;

/// An iterator over a *vertical* axis-aligned line segment
/// whose *direction* is determined at runtime.
///
/// If the direction is known at compile-time, see [`SignedAxis1`].
///
/// **Note**: optimized [`Iterator::fold`] checks the direction once, not on every call
/// to [`Iterator::next`]. This makes [`Iterator::for_each`] faster than a `for` loop.
pub type Axis1<T> = Axis<true, T>;

impl<const V: bool, T: Num> core::fmt::Debug for Axis<V, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(v!("Axis0::", "Axis1::"))?;
        variant!(Self::{Positive, Negative}, self, me => me.fmt(f))
    }
}

macro_rules! impl_axis {
    ($T:ty, cfg_size = $cfg_size:meta) => {
        impl<const V: bool> Axis<V, $T> {
            /// Constructs an [`Axis`] over a half-open line segment.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Self {
                if v1 <= v2 {
                    return Self::Positive(PositiveAxis::<V, $T>::new_inner(u, v1, v2))
                }
                Self::Negative(NegativeAxis::<V, $T>::new_inner(u, v1, v2))
            }

            /// Clips a half-open line segment to a rectangular region
            /// and constructs an [`Axis`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment lies outside the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                if v1 <= v2 {
                    return map!(PositiveAxis::<V, $T>::clip_inner(u, v1, v2, clip), Self::Positive)
                }
                return map!(NegativeAxis::<V, $T>::clip_inner(u, v1, v2, clip), Self::Negative)
            }

            fwd!($T, Self::{Positive, Negative});
            rev!($T, Self::{Positive, Negative});
        }

        iter_fwd!(Axis<const V, $T>::{Positive, Negative});
        #[$cfg_size]
        iter_esi!(Axis<const V, $T>::{Positive, Negative});
        iter_rev!(Axis<const V, $T>::{Positive, Negative});
    };
}

nums!(impl_axis, cfg_size);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary axis-aligned iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over an axis-aligned line segment whose
/// *direction* and *orientation* are determined at runtime.
///
/// - If the orientation is known at compile-time, see [`Axis`].
/// - If the direction is known at compile-time too, see [`SignedAxis`].
///
/// **Note**: optimized [`Iterator::fold`] checks the direction once, not on every call
/// to [`Iterator::next`]. This makes [`Iterator::for_each`] faster than a `for` loop.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AnyAxis<T: Num> {
    /// See [`PositiveAxis0`].
    PositiveAxis0(PositiveAxis0<T>),
    /// See [`PositiveAxis1`].
    PositiveAxis1(PositiveAxis1<T>),
    /// See [`NegativeAxis0`].
    NegativeAxis0(NegativeAxis0<T>),
    /// See [`NegativeAxis1`].
    NegativeAxis1(NegativeAxis1<T>),
}

impl<T: Num> core::fmt::Debug for AnyAxis<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AnyAxis::")?;
        variant!(Self::{PositiveAxis0, PositiveAxis1, NegativeAxis0, NegativeAxis1}, self, me => me.fmt(f))
    }
}

macro_rules! impl_any_axis {
    ($T:ty, cfg_size = $cfg_size:meta) => {
        impl AnyAxis<$T> {
            /// Constructs an [`AnyAxis`] over a half-open line segment.
            ///
            /// Returns [`None`] if the line segment is not axis-aligned.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if y1 == y2 {
                    return Some(Axis0::<$T>::new(y1, x1, x2).into_any_axis());
                }
                if x1 == x2 {
                    return Some(Axis1::<$T>::new(x1, y1, y2).into_any_axis());
                }
                None
            }

            /// Clips a half-open line segment to a rectangular region
            /// and constructs an [`AnyAxis`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment is not axis-aligned,
            /// or lies outside the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip((x1, y1): Point<$T>, (x2, y2): Point<$T>, clip: &Clip<$T>) -> Option<Self> {
                if y1 == y2 {
                    return map!(Axis0::<$T>::clip(y1, x1, x2, clip), Self::from_axis);
                }
                if x1 == x2 {
                    return map!(Axis1::<$T>::clip(x1, y1, y2, clip), Self::from_axis);
                }
                None
            }

            fwd!($T, Self::{PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1});
            rev!($T, Self::{PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1});
        }

        iter_fwd!(AnyAxis<$T>::{PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1});
        #[$cfg_size]
        iter_esi!(AnyAxis<$T>::{PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1});
        iter_rev!(AnyAxis<$T>::{PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1});
    };
}

nums!(impl_any_axis, cfg_size);

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
