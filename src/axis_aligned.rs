//! ## Axis-aligned iterators

use crate::clip::Clip;
use crate::macros::{all_nums, f, hv, impl_iters, impl_methods, map, return_if, variant};
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
///   * `false` – *increasing*, see [`PositiveAxis`]
///   * `true` – *decreasing*, see [`NegativeAxis`]
///
/// - `V` fixes the orientation of the covered line segments:
///   * `false` – *horizontal*, see [`SignedAxis0`]
///   * `true` – *vertical*, see [`SignedAxis1`]
///
/// - If both are fixed, see:
///   * [`PositiveAxis0`], [`NegativeAxis0`]
///   * [`PositiveAxis1`], [`NegativeAxis1`]
///
/// - If the direction is determined at runtime, see [`Axis`].
/// - If the orientation is determined at runtime too, see [`AnyAxis`].
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SignedAxis<const F: bool, const V: bool, T: Num> {
    u: T,
    v1: T,
    v2: T,
}

/// An iterator over a *horizontal* axis-aligned line segment
/// whose *direction* is known at compile-time.
///
/// - `F` fixes the direction of the covered line segments:
///   * `false` – increasing, see [`PositiveAxis0`]
///   * `true` – decreasing, see [`NegativeAxis0`]
///
/// - If the direction is determined at runtime, see [`Axis0`].
pub type SignedAxis0<const F: bool, T> = SignedAxis<F, false, T>;

/// An iterator over a *vertical* axis-aligned line segment
/// whose *direction* is known at compile-time.
///
/// - `F` fixes the direction of the covered line segments:
///   * `false` – increasing, see [`PositiveAxis1`]
///   * `true` – decreasing, see [`NegativeAxis1`]
///
/// - If the direction is determined at runtime, see [`Axis1`].
pub type SignedAxis1<const F: bool, T> = SignedAxis<F, true, T>;

/// An iterator over a *positive* axis-aligned line segment
/// whose *orientation* is known at compile-time.
///
/// - `V` fixes the orientation of the covered line segments:
///   * `false` – horizontal, see [`PositiveAxis0`]
///   * `true` – vertical, see [`PositiveAxis1`]
pub type PositiveAxis<const V: bool, T> = SignedAxis<false, V, T>;

/// An iterator over a *negative* axis-aligned line segment
/// whose *orientation* is known at compile-time.
///
/// - `V` fixes the orientation of the covered line segments:
///   * `false` – horizontal, see [`NegativeAxis0`]
///   * `true` – vertical, see [`NegativeAxis1`]
pub type NegativeAxis<const V: bool, T> = SignedAxis<true, V, T>;

/// An iterator over a *positive*, *horizontal* axis-aligned line segment.
///
/// - Covers line segments oriented at `0°`.
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
            hv!("PositiveAxis0", "PositiveAxis1"),
            hv!("NegativeAxis0", "NegativeAxis1")
        ))
        .field(hv!("y", "x"), &self.u)
        .field(hv!("x1", "y1"), &self.v1)
        .field(hv!("x2", "y2"), &self.v2)
        .finish()
    }
}

macro_rules! impl_signed_axis {
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
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

            /// Constructs a [`SignedAxis`] over a *half-open* line segment.
            ///
            /// Returns [`None`] if the line segment lies in the opposite direction.
            ///
            /// **Note**: empty line segments are covered by [`PositiveAxis`].
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Option<Self> {
                return_if!(!Self::covers(v1, v2));
                Some(Self::new_inner(u, v1, v2))
            }

            /// Clips a *half-open* line segment against a rectangular region
            /// and constructs a [`SignedAxis`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment lies in the opposite direction,
            /// or outside the clipping region.
            ///
            /// **Note**: empty line segments are covered by [`PositiveAxis`].
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                return_if!(!Self::covers(v1, v2));
                Self::clip_inner(u, v1, v2, clip)
            }

            impl_methods!(
                self,
                $T,
                is_done = f!(self.v2 <= self.v1, self.v1 <= self.v2),
                length = Math::<$T>::delta(f!(self.v2, self.v1), f!(self.v1, self.v2)),
                head = {
                    return_if!(self.is_done());
                    let (x, y) = hv!((self.v1, self.u), (self.u, self.v1));
                    Some((x, y))
                },
                tail = {
                    return_if!(self.is_done());
                    let v2 = f!(self.v2.wrapping_sub(1), self.v2.wrapping_add(1));
                    let (x, y) = hv!((v2, self.u), (self.u, v2));
                    Some((x, y))
                },
                pop_head = {
                    let Some((x, y)) = self.head() else {
                        return None;
                    };
                    self.v1 = f!(self.v1.wrapping_add(1), self.v1.wrapping_sub(1));
                    Some((x, y))
                },
                pop_tail = {
                    let Some((x, y)) = self.tail() else {
                        return None;
                    };
                    self.v2 = hv!(x, y);
                    Some((x, y))
                }
            );
        }

        impl_iters!(
            SignedAxis<const F, const V, $T>,
            self,
            next = self.pop_head(),
            next_back = self.pop_tail(),
            size_hint = {
                match usize::try_from(self.length()) {
                    Ok(length) => (length, Some(length)),
                    Err(_) => (usize::MAX, None),
                }
            },
            is_empty = self.is_done()
            $(, cfg_esi = $cfg_esi)?
        );
    };
}

all_nums!(impl_signed_axis);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over an axis-aligned line segment whose *direction*
/// is determined at runtime, and *orientation* is known at compile-time.
///
/// - `V` fixes the orientation of the covered line segments:
///   * `false` – *horizontal*, see [`Axis0`]
///   * `true` – *vertical*, see [`Axis1`]
/// - If the direction is known at compile-time, see [`SignedAxis`].
/// - If the orientation is determined at runtime, see [`AnyAxis`].
///
/// **Note**: optimized [`Iterator::fold`] checks the direction once, not on every call
/// to [`Iterator::next`]. This makes [`Iterator::for_each`] faster than a `for` loop.
#[derive(Clone, Eq, PartialEq, Hash)]
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
        f.write_str(hv!("Axis0::", "Axis1::"))?;
        variant!(Self::{Positive, Negative}, self, me => me.fmt(f))
    }
}

macro_rules! impl_axis {
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
        impl<const V: bool> Axis<V, $T> {
            /// Constructs an [`Axis`] over a *half-open* line segment.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Self {
                if v1 <= v2 {
                    return Self::Positive(PositiveAxis::<V, $T>::new_inner(u, v1, v2))
                }
                Self::Negative(NegativeAxis::<V, $T>::new_inner(u, v1, v2))
            }

            /// Clips a *half-open* line segment against a rectangular region
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

            impl_methods!(
                $T,
                Self::{Positive, Negative}
            );
        }

        impl_iters!(
            Axis<const V, $T>::{Positive, Negative}
            $(, cfg_esi = $cfg_esi)?
        );
    };
}

all_nums!(impl_axis);

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
#[derive(Clone, Eq, PartialEq, Hash)]
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
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
        impl AnyAxis<$T> {
            /// Constructs an [`AnyAxis`] over a *half-open* line segment.
            ///
            /// Returns [`None`] if the line segment is not axis-aligned.
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

            /// Clips a *half-open* line segment against a rectangular region
            /// and constructs an [`AnyAxis`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment is not axis-aligned,
            /// or lies outside the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip((x1, y1): Point<$T>, (x2, y2): Point<$T>, clip: &Clip<$T>) -> Option<Self> {
                if y1 == y2 {
                    return map!(
                        Axis0::<$T>::clip(y1, x1, x2, clip),
                        |me| match me {
                            Axis::Positive(me) => Self::PositiveAxis0(me),
                            Axis::Negative(me) => Self::NegativeAxis0(me),
                        }
                    );
                }
                if x1 == x2 {
                    return map!(
                        Axis1::<$T>::clip(x1, y1, y2, clip),
                        |me| match me {
                            Axis::Positive(me) => Self::PositiveAxis1(me),
                            Axis::Negative(me) => Self::NegativeAxis1(me),
                        }
                    );
                }
                None
            }

            impl_methods!(
                $T,
                Self::{PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1}
            );
        }

        impl_iters!(
            AnyAxis<$T>::{PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1}
            $(, cfg_esi = $cfg_esi)?
        );
    };
}

all_nums!(impl_any_axis);

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
