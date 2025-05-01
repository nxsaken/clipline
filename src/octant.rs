//! ## Octant iterators

use crate::axis::{Axis0, Axis1, NegativeAxis0, NegativeAxis1, PositiveAxis0, PositiveAxis1};
use crate::clip::Clip;
use crate::diagonal::{quadrant, Diagonal0, Diagonal1, Diagonal2, Diagonal3};
use crate::macros::control_flow::{map, return_if, unwrap_or_return, variant};
use crate::macros::derive::{fwd, iter_esi, iter_fwd, nums};
use crate::macros::symmetry::{fx, fy, yx};
use crate::math::{Delta, Math, Num, Point};

mod clip;
mod convert;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Octant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over an oblique line segment whose *directions* along `x` and `y`,
/// and *orientation* are known at compile-time.
///
/// - `FX` fixes the direction of the covered line segments along `x`:
///   * `false` – *increasing*, see [`Octant0`], [`Octant1`], [`Octant2`], [`Octant3`].
///   * `true` – *decreasing*, see [`Octant4`], [`Octant5`], [`Octant6`], [`Octant7`].
///
/// - `FY` fixes the direction of the covered line segments along `y`:
///   * `false` – *increasing*, see [`Octant0`], [`Octant1`], [`Octant4`], [`Octant5`].
///   * `true` – *decreasing*, see [`Octant2`], [`Octant3`], [`Octant6`], [`Octant7`].
///
/// - `YX` fixes the orientation of the covered line segments:
///   * `false` – *gentle*, `dy < dx`, see [`Octant0`], [`Octant2`], [`Octant4`], [`Octant6`].
///   * `true` – *steep*, `dx < dy`, see [`Octant1`], [`Octant3`], [`Octant5`], [`Octant7`].
///
/// - If the directions and orientation are determined at runtime, see [`AnyOctant`].
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Octant<const FX: bool, const FY: bool, const YX: bool, T: Num> {
    x: T,
    y: T,
    error: T::I2,
    dx: T::U,
    dy: T::U,
    end: T,
}

/// An iterator over an oblique line segment in the octant
/// where `x` and `y` *both increase*, and `x` changes faster than `y`.
///
/// - Covers line segments spanning the `(0°, 45°)` sector.
/// - `x1 < x2`, `y1 < y2`, `dy < dx`
pub type Octant0<T> = Octant<false, false, false, T>;

/// An iterator over an oblique line segment in the octant
/// where `x` and `y` *both increase*, and `y` changes faster than `x`.
///
/// - Covers line segments spanning the `(45°, 90°)` sector.
/// - `x1 < x2`, `y1 < y2`, `dx < dy`
pub type Octant1<T> = Octant<false, false, true, T>;

/// An iterator over an oblique line segment in the octant
/// where `x` *increases*, `y` *decreases*, and `x` changes faster than `y`.
///
/// - Covers line segments spanning the `(315°, 360°)` sector.
/// - `x1 < x2`, `y2 < y1`, `dy < dx`
pub type Octant2<T> = Octant<false, true, false, T>;

/// An iterator over an oblique line segment in the octant
/// where `x` *increases*, `y` *decreases*, and `y` changes faster than `x`.
///
/// - Covers line segments spanning the `(270°, 315°)` sector.
/// - `x1 < x2`, `y2 < y1`, `dx < dy`
pub type Octant3<T> = Octant<false, true, true, T>;

/// An iterator over an oblique line segment in the octant
/// where `x` *decreases*, `y` *increases*, and `x` changes faster than `y`.
///
/// - Covers line segments spanning the `(135°, 180°)` sector.
/// - `x2 < x1`, `y1 < y2`, `dy < dx`
pub type Octant4<T> = Octant<true, false, false, T>;

/// An iterator over an oblique line segment in the octant
/// where `x` *decreases*, `y` *increases*, and `y` changes faster than `x`.
///
/// - Covers line segments spanning the `(90°, 135°)` sector.
/// - `x2 < x1`, `y1 < y2`, `dx < dy`
pub type Octant5<T> = Octant<true, false, true, T>;

/// An iterator over an oblique line segment in the octant
/// where `x` and `y` *both decrease*, and `x` changes faster than `y`.
///
/// - Covers line segments spanning the `(180°, 225°)` sector.
/// - `x2 < x1`, `y2 < y1`, `dy < dx`
pub type Octant6<T> = Octant<true, true, false, T>;

/// An iterator over an oblique line segment in the octant
/// where `x` and `y` *both decrease*, and `y` changes faster than `x`.
///
/// - Covers line segments spanning the `(225°, 270°)` sector.
/// - `x2 < x1`, `y2 < y1`, `dx < dy`
pub type Octant7<T> = Octant<true, true, true, T>;

impl<const FX: bool, const FY: bool, const YX: bool, T: Num> core::fmt::Debug
    for Octant<FX, FY, YX, T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(fx!(
            fy!(yx!("Octant0", "Octant1"), yx!("Octant2", "Octant3"),),
            fy!(yx!("Octant4", "Octant5"), yx!("Octant6", "Octant7"),),
        ))
        .field("x", &self.x)
        .field("y", &self.y)
        .field("error", &self.error)
        .field("dx", &self.dx)
        .field("dy", &self.dy)
        .field("end", &self.end)
        .finish()
    }
}

macro_rules! impl_octant {
    ($T:ty, cfg_size = $cfg_size:meta) => {
        impl<const FX: bool, const FY: bool, const YX: bool> Octant<FX, FY, YX, $T> {
            #[inline(always)]
            #[must_use]
            const fn new_inner(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                (dx, dy): Delta<$T>,
            ) -> Self {
                let (half_du, r) = Math::<$T>::div_2u(yx!(dx, dy));
                let error = Math::<$T>::sub_uu(yx!(dy, dx), half_du.wrapping_add(r));
                let end = yx!(x2, y2);
                Self { x: x1, y: y1, error, dx, dy, end }
            }

            #[inline(always)]
            #[must_use]
            const fn covers((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Delta<$T>> {
                return_if!(fx!(x2 <= x1, x1 <= x2));
                return_if!(fy!(y2 <= y1, y1 <= y2));
                let du = Math::<$T>::sub_tt(fx!(x2, x1), fx!(x1, x2));
                let dv = Math::<$T>::sub_tt(fy!(y2, y1), fy!(y1, y2));
                return_if!(yx!(du <= dv, dv <= du));
                Some((du, dv))
            }

            /// Constructs an [`Octant`] over a half-open line segment.
            ///
            /// Returns [`None`] if the line segment is not covered by the octant.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                let delta = unwrap_or_return!(Self::covers((x1, y1), (x2, y2)));
                Some(Self::new_inner((x1, y1), (x2, y2), delta))
            }

            /// Clips a half-open line segment to a rectangular region
            /// and constructs an [`Octant`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment is not covered by the octant,
            /// or lies outside the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                let (u1, u2) = fx!((x1, x2), (x2, x1));
                // TODO: strict comparison for closed line segments
                return_if!(yx!(u2 <= wx1, u2 < wx1) || wx2 < u1);
                let (v1, v2) = fy!((y1, y2), (y2, y1));
                return_if!(yx!(v2 < wy1, v2 <= wy1) || wy2 < v1);
                let delta = unwrap_or_return!(Self::covers((x1, y1), (x2, y2)));
                Self::clip_inner((x1, y1), (x2, y2), delta, clip)
            }

            fwd!(
                self,
                $T,
                is_done = {
                    let (a, b) = yx!(
                        fx!((self.end, self.x), (self.x, self.end)),
                        fy!((self.end, self.y), (self.y, self.end))
                    );
                    a <= b
                },
                length = {
                    let (a, b) = yx!(
                        fx!((self.end, self.x), (self.x, self.end)),
                        fy!((self.end, self.y), (self.y, self.end))
                    );
                    Math::<$T>::sub_tt(a, b)
                },
                head = {
                    return_if!(self.is_done());
                    Some((self.x, self.y))
                },
                pop_head = {
                    let (x1, y1) = unwrap_or_return!(self.head());
                    if 0 <= self.error {
                        yx!(
                            self.y = fy!(self.y.wrapping_add(1), self.y.wrapping_sub(1)),
                            self.x = fx!(self.x.wrapping_add(1), self.x.wrapping_sub(1)),
                        );
                        self.error = self.error.wrapping_sub_unsigned(yx!(self.dx, self.dy) as _);
                    }
                    yx!(
                        self.x = fx!(self.x.wrapping_add(1), self.x.wrapping_sub(1)),
                        self.y = fy!(self.y.wrapping_add(1), self.y.wrapping_sub(1)),
                    );
                    self.error = self.error.wrapping_add_unsigned(yx!(self.dy, self.dx) as _);
                    Some((x1, y1))
                },
            );
        }

        iter_fwd!(
            Octant<const FX, const FY, const YX, $T>,
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
            Octant<const FX, const FY, const YX, $T>,
            self,
            is_empty = self.is_done(),
        );
    };
}

nums!(impl_octant, cfg_size, cfg_octant_64);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// An iterator over a line segment whose type is determined at runtime.
///
/// - If the line segment is axis-aligned, see [`Axis`](crate::Axis) or [`AnyAxis`](crate::AnyAxis).
/// - If the line segment is diagonal, see [`AnyDiagonal`](crate::AnyDiagonal).
/// - If the line segment is oblique, and its octant is known at compile-time, see [`Octant`].
///
/// **Note**: optimized [`Iterator::fold`] checks the type once, not on every call
/// to [`Iterator::next`]. This makes [`Iterator::for_each`] faster than a `for` loop.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AnyOctant<T: Num> {
    /// See [`PositiveAxis0`].
    PositiveAxis0(PositiveAxis0<T>),
    /// See [`NegativeAxis0`].
    NegativeAxis0(NegativeAxis0<T>),
    /// See [`PositiveAxis1`].
    PositiveAxis1(PositiveAxis1<T>),
    /// See [`NegativeAxis1`].
    NegativeAxis1(NegativeAxis1<T>),
    /// See [`Diagonal0`].
    Diagonal0(Diagonal0<T>),
    /// See [`Diagonal1`].
    Diagonal1(Diagonal1<T>),
    /// See[`Diagonal2`].
    Diagonal2(Diagonal2<T>),
    /// See [`Diagonal3`].
    Diagonal3(Diagonal3<T>),
    /// See [`Octant0`].
    Octant0(Octant0<T>),
    /// See [`Octant1`].
    Octant1(Octant1<T>),
    /// See [`Octant2`].
    Octant2(Octant2<T>),
    /// See [`Octant3`].
    Octant3(Octant3<T>),
    /// See [`Octant4`].
    Octant4(Octant4<T>),
    /// See [`Octant5`].
    Octant5(Octant5<T>),
    /// See [`Octant6`].
    Octant6(Octant6<T>),
    /// See [`Octant7`].
    Octant7(Octant7<T>),
}

impl<T: Num> core::fmt::Debug for AnyOctant<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AnyOctant::")?;
        variant!(Self::{
            PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1,
            Diagonal0, Diagonal1, Diagonal2, Diagonal3,
            Octant0, Octant1, Octant2, Octant3, Octant4, Octant5, Octant6, Octant7,
        },
        self,
        me => me.fmt(f))
    }
}

macro_rules! octant {
    ($Octant:ident, $T:ty, $p1:expr, $p2:expr, $delta:expr) => {
        Self::$Octant($Octant::<$T>::new_inner($p1, $p2, $delta))
    };
    ($Octant:ident, $T:ty, $p1:expr, $p2:expr, $delta:expr, $clip:expr) => {
        map!($Octant::<$T>::clip_inner($p1, $p2, $delta, $clip), Self::$Octant)
    };
}

macro_rules! impl_any_octant {
    ($T:ty, cfg_size = $cfg_size:meta) => {
        impl AnyOctant<$T> {
            /// Constructs an [`AnyOctant`] over a half-open line segment.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Self {
                if y1 == y2 {
                    return Axis0::<$T>::new(y1, x1, x2).into_any_octant();
                }
                if x1 == x2 {
                    return Axis1::<$T>::new(x1, y1, y2).into_any_octant();
                }
                if x1 < x2 {
                    let dx = Math::<$T>::sub_tt(x2, x1);
                    if y1 < y2 {
                        let dy = Math::<$T>::sub_tt(y2, y1);
                        if dy < dx {
                             return octant!(Octant0, $T, (x1, y1), (x2, y2), (dx, dy));
                        }
                        if dx < dy {
                             return octant!(Octant1, $T, (x1, y1), (x2, y2), (dx, dy));
                        }
                        return quadrant!(Diagonal0, $T, (x1, y1), x2);
                    }
                    let dy = Math::<$T>::sub_tt(y1, y2);
                    if dy < dx {
                        return octant!(Octant2, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    if dx < dy {
                        return octant!(Octant3, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    return quadrant!(Diagonal1, $T, (x1, y1), x2);
                }
                let dx = Math::<$T>::sub_tt(x1, x2);
                if y1 < y2 {
                    let dy = Math::<$T>::sub_tt(y2, y1);
                    if dy < dx {
                        return octant!(Octant4, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    if dx < dy {
                        return octant!(Octant5, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    return quadrant!(Diagonal2, $T, (x1, y1), x2);
                }
                let dy = Math::<$T>::sub_tt(y1, y2);
                if dy < dx {
                    return octant!(Octant6, $T, (x1, y1), (x2, y2), (dx, dy));
                }
                if dx < dy {
                    return octant!(Octant7, $T, (x1, y1), (x2, y2), (dx, dy));
                }
                return quadrant!(Diagonal3, $T, (x1, y1), x2);
            }

            /// Clips a half-open line segment to a rectangular region
            /// and constructs an [`AnyOctant`] over the portion inside the clip.
            ///
            /// Returns [`None`] if the line segment lies outside the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                if y1 == y2 {
                    return map!(Axis0::<$T>::clip(y1, x1, x2, clip), Self::from_axis);
                }
                if x1 == x2 {
                    return map!(Axis1::<$T>::clip(x1, y1, y2, clip), Self::from_axis);
                }
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                if x1 < x2 {
                    return_if!(x2 < wx1 || wx2 < x1);
                    let dx = Math::<$T>::sub_tt(x2, x1);
                    if y1 < y2 {
                        return_if!(y2 < wy1 || wy2 < y1);
                        let dy = Math::<$T>::sub_tt(y2, y1);
                        if dy < dx {
                            // TODO: strict comparison for closed line segments
                            return_if!(x2 == wx1);
                            return octant!(Octant0, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                        }
                        if dx < dy {
                            return_if!(y2 == wy1);
                            return octant!(Octant1, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                        }
                        return quadrant!(Diagonal0, $T, (x1, y1), (x2, y2), clip);
                    }
                    return_if!(y1 < wy1 || wy2 < y2);
                    let dy = Math::<$T>::sub_tt(y1, y2);
                    if dy < dx {
                        return_if!(x2 == wx1);
                        return octant!(Octant2, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    if dx < dy {
                        return_if!(y2 == wy2);
                        return octant!(Octant3, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    return quadrant!(Diagonal1, $T, (x1, y1), (x2, y2), clip);
                }
                return_if!(x1 < wx1 || wx2 < x2);
                let dx = Math::<$T>::sub_tt(x1, x2);
                if y1 < y2 {
                    return_if!(y2 < wy1 || wy2 < y1);
                    let dy = Math::<$T>::sub_tt(y2, y1);
                    if dy < dx {
                        return_if!(x2 == wx2);
                        return octant!(Octant4, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    if dx < dy {
                        return_if!(y2 == wy1);
                        return octant!(Octant5, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    return quadrant!(Diagonal2, $T, (x1, y1), (x2, y2), clip);
                }
                return_if!(y1 < wy1 || wy2 < y2);
                let dy = Math::<$T>::sub_tt(y1, y2);
                if dy < dx {
                    return_if!(x2 == wx2);
                    return octant!(Octant6, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                }
                if dx < dy {
                    return_if!(y2 == wy2);
                    return octant!(Octant7, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                }
                return quadrant!(Diagonal3, $T, (x1, y1), (x2, y2), clip);
            }

            fwd!(
                $T,
                Self::{
                    PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1,
                    Diagonal0, Diagonal1, Diagonal2, Diagonal3,
                    Octant0, Octant1, Octant2, Octant3, Octant4, Octant5, Octant6, Octant7,
                }
            );
        }

        iter_fwd!(
            AnyOctant<$T>::{
                PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1,
                Diagonal0, Diagonal1, Diagonal2, Diagonal3,
                Octant0, Octant1, Octant2, Octant3, Octant4, Octant5, Octant6, Octant7,
            },
        );

        #[$cfg_size]
        iter_esi!(
            AnyOctant<$T>::{
                PositiveAxis0, NegativeAxis0, PositiveAxis1, NegativeAxis1,
                Diagonal0, Diagonal1, Diagonal2, Diagonal3,
                Octant0, Octant1, Octant2, Octant3, Octant4, Octant5, Octant6, Octant7,
            },
        );
    };
}

nums!(impl_any_octant, cfg_size, cfg_octant_64);

#[cfg(test)]
mod static_tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    const fn iterator_8() {
        assert_impl_all!(Octant0<i8>: ExactSizeIterator);
        assert_impl_all!(Octant0<u8>: ExactSizeIterator);
        assert_impl_all!(AnyOctant<i8>: ExactSizeIterator);
        assert_impl_all!(AnyOctant<u8>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_16() {
        assert_impl_all!(Octant0<i16>: ExactSizeIterator);
        assert_impl_all!(Octant0<u16>: ExactSizeIterator);
        assert_impl_all!(AnyOctant<i16>: ExactSizeIterator);
        assert_impl_all!(AnyOctant<u16>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_32() {
        #[cfg(target_pointer_width = "16")]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(Octant0<i32>: Iterator);
            assert_impl_all!(Octant0<u32>: Iterator);
            assert_impl_all!(AnyOctant<i32>: Iterator);
            assert_impl_all!(AnyOctant<u32>: Iterator);
            assert_not_impl_any!(Octant0<i32>: ExactSizeIterator);
            assert_not_impl_any!(Octant0<u32>: ExactSizeIterator);
            assert_not_impl_any!(AnyOctant<i32>: ExactSizeIterator);
            assert_not_impl_any!(AnyOctant<u32>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            assert_impl_all!(Octant0<i32>: ExactSizeIterator);
            assert_impl_all!(Octant0<u32>: ExactSizeIterator);
            assert_impl_all!(AnyOctant<i32>: ExactSizeIterator);
            assert_impl_all!(AnyOctant<u32>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_64() {
        #[cfg(feature = "octant_64")]
        {
            #[cfg(target_pointer_width = "64")]
            {
                assert_impl_all!(Octant0<i64>: ExactSizeIterator);
                assert_impl_all!(Octant0<u64>: ExactSizeIterator);
                assert_impl_all!(AnyOctant<i64>: ExactSizeIterator);
                assert_impl_all!(AnyOctant<u64>: ExactSizeIterator);
            }
            #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
            {
                use static_assertions::assert_not_impl_any;

                assert_impl_all!(Octant0<i64>: Iterator);
                assert_impl_all!(Octant0<u64>: Iterator);
                assert_impl_all!(AnyOctant<i64>: Iterator);
                assert_impl_all!(AnyOctant<u64>: Iterator);
                assert_not_impl_any!(Octant0<i64>: ExactSizeIterator);
                assert_not_impl_any!(Octant0<u64>: ExactSizeIterator);
                assert_not_impl_any!(AnyOctant<i64>: ExactSizeIterator);
                assert_not_impl_any!(AnyOctant<u64>: ExactSizeIterator);
            }
        }
        #[cfg(not(feature = "octant_64"))]
        {
            use static_assertions::assert_not_impl_any;

            assert_not_impl_any!(Octant0<i64>: Iterator);
            assert_not_impl_any!(Octant0<u64>: Iterator);
            assert_not_impl_any!(AnyOctant<i64>: Iterator);
            assert_not_impl_any!(AnyOctant<u64>: Iterator);
        }
    }

    #[test]
    const fn iterator_pointer_size() {
        #[cfg(target_pointer_width = "64")]
        {
            #[cfg(feature = "octant_64")]
            {
                assert_impl_all!(Octant0<isize>: ExactSizeIterator);
                assert_impl_all!(Octant0<usize>: ExactSizeIterator);
                assert_impl_all!(AnyOctant<isize>: ExactSizeIterator);
                assert_impl_all!(AnyOctant<usize>: ExactSizeIterator);
            }
            #[cfg(not(feature = "octant_64"))]
            {
                use static_assertions::assert_not_impl_any;

                assert_not_impl_any!(Octant0<isize>: Iterator);
                assert_not_impl_any!(Octant0<usize>: Iterator);
                assert_not_impl_any!(AnyOctant<isize>: Iterator);
                assert_not_impl_any!(AnyOctant<usize>: Iterator);
            }
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        {
            assert_impl_all!(Octant0<isize>: ExactSizeIterator);
            assert_impl_all!(Octant0<usize>: ExactSizeIterator);
            assert_impl_all!(AnyOctant<isize>: ExactSizeIterator);
            assert_impl_all!(AnyOctant<usize>: ExactSizeIterator);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axis::{NegativeAxis0, NegativeAxis1, PositiveAxis0, PositiveAxis1};
    use crate::diagonal::{Diagonal0, Diagonal1, Diagonal2, Diagonal3};

    #[test]
    fn axis_aligned_lines_are_special_cased() {
        assert_eq!(
            AnyOctant::PositiveAxis0(PositiveAxis0::<u8>::new(0, 0, 255).unwrap()),
            AnyOctant::<u8>::new((0, 0), (255, 0)),
        );
        assert_eq!(
            AnyOctant::PositiveAxis1(PositiveAxis1::<u8>::new(0, 0, 255).unwrap()),
            AnyOctant::<u8>::new((0, 0), (0, 255)),
        );
        assert_eq!(
            AnyOctant::NegativeAxis0(NegativeAxis0::<u8>::new(0, 255, 0).unwrap()),
            AnyOctant::<u8>::new((255, 0), (0, 0)),
        );
        assert_eq!(
            AnyOctant::NegativeAxis1(NegativeAxis1::<u8>::new(0, 255, 0).unwrap()),
            AnyOctant::<u8>::new((0, 255), (0, 0)),
        );
    }

    #[test]
    fn diagonal_lines_are_special_cased() {
        assert_eq!(
            AnyOctant::<u8>::new((0, 0), (255, 255)),
            AnyOctant::Diagonal0(Diagonal0::<u8>::new((0, 0), (255, 255)).unwrap()),
        );
        assert_eq!(
            AnyOctant::<u8>::new((0, 255), (255, 0)),
            AnyOctant::Diagonal1(Diagonal1::<u8>::new((0, 255), (255, 0)).unwrap()),
        );
        assert_eq!(
            AnyOctant::<u8>::new((255, 0), (0, 255)),
            AnyOctant::Diagonal2(Diagonal2::<u8>::new((255, 0), (0, 255)).unwrap()),
        );
        assert_eq!(
            AnyOctant::<u8>::new((255, 255), (0, 0)),
            AnyOctant::Diagonal3(Diagonal3::<u8>::new((255, 255), (0, 0)).unwrap()),
        );
    }

    #[test]
    fn exclusive_covers_whole_domain() {
        const MAX: u8 = u8::MAX;
        for i in 0..=MAX {
            assert_eq!(AnyOctant::<u8>::new((0, i), (MAX, MAX)).count(), MAX as usize);
            assert_eq!(AnyOctant::<u8>::new((MAX, MAX), (0, i)).count(), MAX as usize);
            assert_eq!(AnyOctant::<u8>::new((i, 0), (MAX, MAX)).count(), MAX as usize);
            assert_eq!(AnyOctant::<u8>::new((MAX, MAX), (i, 0)).count(), MAX as usize);
            assert_eq!(AnyOctant::<u8>::new((0, MAX), (MAX, i)).count(), MAX as usize);
            assert_eq!(AnyOctant::<u8>::new((MAX, i), (0, MAX)).count(), MAX as usize);
            assert_eq!(AnyOctant::<u8>::new((MAX, 0), (i, MAX)).count(), MAX as usize);
            assert_eq!(AnyOctant::<u8>::new((i, MAX), (MAX, 0)).count(), MAX as usize);
        }
    }
}
