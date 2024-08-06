//! ## Bresenham iterators
//!
//! This module provides a family of iterators for arbitrary directed line segments
//! backed by [Bresenham's algorithm][1].
//!
//! For an arbitrary directed line segment, use the [general Bresenham](Bresenham) iterator.
//! If you know more about the orientation and direction of the line segment, use one of the
//! specialized [diagonal](crate::Diagonal) or [orthogonal](crate::Orthogonal) iterators instead.
//!
//! [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

use crate::clip::Clip;
use crate::math::{Delta, Math, Num, Point};
use crate::symmetry::{fx, fy, xy};
use crate::utils::{map, reject_if};
use crate::{diagonal, orthogonal};

mod clip;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Bresenham octant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment in the given *octant* of [Bresenham's algorithm][1].
///
/// An octant is defined by its transformations relative to [`Octant0`]:
/// - `FY`: flip the `y` axis if `true`.
/// - `FX`: flip the `x` axis if `true`.
/// - `SWAP`: swap the `x` and `y` axes if `true`.
///
/// [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Octant<const FX: bool, const FY: bool, const SWAP: bool, T: Num> {
    x: T,
    y: T,
    error: T::I2,
    dx: T::U,
    dy: T::U,
    end: T,
}

/// Iterator over a directed line segment in the first [octant](Octant)
/// covering the `(0°, 45°]` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` increase,
/// with `x` changing faster than `y` (gentle slope).
pub type Octant0<T> = Octant<false, false, false, T>;

/// Iterator over a directed line segment in the second [octant](Octant)
/// covering the `(45°, 90°)` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` increase,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by swapping the `x` and `y` coordinates.
pub type Octant1<T> = Octant<false, false, true, T>;

/// Iterator over a directed line segment in the third [octant](Octant).
/// covering the `[315°, 360°)` sector of the Cartesian plane.
///
/// In this octant, `x` increases and `y` decreases,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate.
pub type Octant2<T> = Octant<false, true, false, T>;

/// Iterator over a directed line segment in the fourth [octant](Octant).
/// covering the `(270°, 315°)` sector of the Cartesian plane.
///
/// In this octant, `x` increases and `y` decreases,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant3<T> = Octant<false, true, true, T>;

/// Iterator over a directed line segment in the fifth [octant](Octant)
/// covering the `[135°, 180°)` sector of the Cartesian plane.
///
/// In this octant, `x` decreases and `y` increases,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate.
pub type Octant4<T> = Octant<true, false, false, T>;

/// Iterator over a directed line segment in the sixth [octant](Octant)
/// covering the `(90°, 135°)` sector of the Cartesian plane.
///
/// In this octant, `x` decreases and `y` increases,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant5<T> = Octant<true, false, true, T>;

/// Iterator over a directed line segment in the seventh [octant](Octant)
/// covering the `(180°, 225°]` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` decrease,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` and `y` coordinates.
pub type Octant6<T> = Octant<true, true, false, T>;

/// Iterator over a directed line segment in the eighth [octant](Octant)
/// covering the `(225°, 270°)` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` decrease,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping and swapping the `x` and `y` coordinates.
pub type Octant7<T> = Octant<true, true, true, T>;

macro_rules! octant_impl {
    ($T:ty) => {
        impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<FX, FY, SWAP, $T> {
            #[inline(always)]
            #[must_use]
            const fn new_inner(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                (dx, dy): Delta<$T>,
            ) -> Self {
                #[allow(clippy::cast_possible_wrap)]
                let error = Math::<$T>::error(xy!(dy, dx), Math::<$T>::half(xy!(dx, dy)));
                let end = xy!(x2, y2);
                Self { x: x1, y: y1, error, dx, dy, end }
            }

            #[inline(always)]
            #[must_use]
            const fn covers((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Delta<$T>> {
                let (u1, u2) = fx!((x1, x2), (x2, x1));
                let dx = if u1 < u2 {
                    Math::<$T>::delta(u2, u1)
                } else {
                    return None;
                };
                let (v1, v2) = fy!((y1, y2), (y2, y1));
                let dy = if v1 < v2 {
                    Math::<$T>::delta(v2, v1)
                } else {
                    return None;
                };
                reject_if!(xy!(dx < dy, dy <= dx));
                Some((dx, dy))
            }

            /// Returns an iterator over a directed line segment
            /// if it is covered by the given [octant](Octant),
            /// otherwise returns [`None`].
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                let Some(delta) = Self::covers((x1, y1), (x2, y2)) else {
                    return None;
                };
                Some(Self::new_inner((x1, y1), (x2, y2), delta))
            }

            /// Returns an iterator over a directed line segment,
            /// if it is covered by the [octant](Octant),
            /// clipped to the [rectangular region](Clip).
            ///
            /// Returns [`None`] if the line segment is not covered by the octant,
            /// or if the line segment does not intersect the clipping region.
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                let (u1, u2) = fx!((x1, x2), (x2, x1));
                reject_if!(u2 < wx1 || wx2 <= u1);
                let (v1, v2) = fy!((y1, y2), (y2, y1));
                reject_if!(v2 < wy1 || wy2 <= v1);
                let Some(delta) = Self::covers((x1, y1), (x2, y2)) else {
                    return None;
                };
                Self::clip_inner((x1, y1), (x2, y2), delta, clip)
            }

            /// Returns `true` if the iterator has terminated.
            #[inline]
            #[must_use]
            pub const fn is_done(&self) -> bool {
                let (a, b) = xy!(
                    fx!((self.end, self.x), (self.x, self.end)),
                    fy!((self.end, self.y), (self.y, self.end))
                );
                a <= b
            }

            /// Returns the remaining length of this iterator.
            #[inline]
            #[must_use]
            pub const fn length(&self) -> <$T as Num>::U {
                let (a, b) = xy!(
                    fx!((self.end, self.x), (self.x, self.end)),
                    fy!((self.end, self.y), (self.y, self.end))
                );
                #[allow(clippy::cast_sign_loss)]
                <$T as Num>::U::wrapping_sub(a as _, b as _)
            }
        }

        impl<const FX: bool, const FY: bool, const SWAP: bool> Iterator
            for Octant<FX, FY, SWAP, $T>
        {
            type Item = Point<$T>;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.is_done() {
                    return None;
                }
                let (x, y) = (self.x, self.y);
                if 0 <= self.error {
                    xy!(
                        self.y = fy!(self.y.wrapping_add(1), self.y.wrapping_sub(1)),
                        self.x = fx!(self.x.wrapping_add(1), self.x.wrapping_sub(1)),
                    );
                    self.error = self.error.wrapping_sub_unsigned(xy!(self.dx, self.dy) as _);
                }
                xy!(
                    self.x = fx!(self.x.wrapping_add(1), self.x.wrapping_sub(1)),
                    self.y = fy!(self.y.wrapping_add(1), self.y.wrapping_sub(1)),
                );
                self.error = self.error.wrapping_add_unsigned(xy!(self.dy, self.dx) as _);
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

        impl<const FX: bool, const FY: bool, const SWAP: bool> core::iter::FusedIterator
            for Octant<FX, FY, SWAP, $T>
        {
        }
    };
}

octant_impl!(i8);
octant_impl!(u8);
octant_impl!(i16);
octant_impl!(u16);
octant_impl!(i32);
octant_impl!(u32);
#[cfg(feature = "bresenham_64")]
octant_impl!(i64);
#[cfg(feature = "bresenham_64")]
octant_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
octant_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
octant_impl!(usize);

macro_rules! octant_exact_size_iter_impl {
    ($T:ty) => {
        impl<const FX: bool, const FY: bool, const SWAP: bool> ExactSizeIterator
            for Octant<FX, FY, SWAP, $T>
        {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                self.is_done()
            }
        }
    };
}

octant_exact_size_iter_impl!(i8);
octant_exact_size_iter_impl!(u8);
octant_exact_size_iter_impl!(i16);
octant_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
octant_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
octant_exact_size_iter_impl!(u32);
#[cfg(feature = "bresenham_64")]
octant_exact_size_iter_impl!(i64);
#[cfg(feature = "bresenham_64")]
octant_exact_size_iter_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_exact_size_iter_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_exact_size_iter_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
octant_exact_size_iter_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
octant_exact_size_iter_impl!(usize);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary Bresenham iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over an arbitrary directed line segment backed by [Bresenham's algorithm][1].
///
/// Chooses a sub-iterator variant based on the orientation and direction of the line segment.
///
/// If you know the alignment of the line segment beforehand, consider the more specific
/// [octant](Octant), [diagonal](crate::Diagonal), [orthogonal](crate::Orthogonal)
/// and [axis-aligned](crate::AxisAligned) iterators instead.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it chooses
/// the underlying iterator only once instead of on every call to [`Iterator::next`].
///
/// [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Bresenham<T: Num> {
    /// Horizontal line segment at `0°`, see [`PositiveHorizontal`](crate::PositiveHorizontal).
    SignedAxis0(orthogonal::PositiveHorizontal<T>),
    /// Horizontal line segment at `180°`, see [`NegativeHorizontal`](crate::NegativeHorizontal).
    SignedAxis1(orthogonal::NegativeHorizontal<T>),
    /// Vertical line segment at `90°`, see [`PositiveVertical`](crate::PositiveVertical).
    SignedAxis2(orthogonal::PositiveVertical<T>),
    /// Vertical line segment at `270°`, see [`NegativeVertical`](crate::NegativeVertical).
    SignedAxis3(orthogonal::NegativeVertical<T>),
    /// Diagonal line segment at `45°`, see [`Quadrant0`](diagonal::Quadrant0).
    Quadrant0(diagonal::Quadrant0<T>),
    /// Diagonal line segment at `315°`, see [`Quadrant1`](diagonal::Quadrant1).
    Quadrant1(diagonal::Quadrant1<T>),
    /// Diagonal line segment at `135°`, see [`Quadrant2`](diagonal::Quadrant2).
    Quadrant2(diagonal::Quadrant2<T>),
    /// Diagonal line segment at `225°`, see [`Quadrant3`](diagonal::Quadrant3).
    Quadrant3(diagonal::Quadrant3<T>),
    /// Gently-sloped line segment in `(0°, 45°)`, see [`Octant0`].
    Octant0(Octant0<T>),
    /// Steeply-sloped line segment in `(45°, 90°)`, see [`Octant1`].
    Octant1(Octant1<T>),
    /// Gently-sloped line segment in `(315°, 360°)`, see [`Octant2`].
    Octant2(Octant2<T>),
    /// Steeply-sloped line segment in `(270°, 315°)`, see [`Octant3`].
    Octant3(Octant3<T>),
    /// Gently-sloped line segment in `(135°, 180°)`, see [`Octant4`].
    Octant4(Octant4<T>),
    /// Steeply-sloped line segment in `(90°, 135°)`, see [`Octant5`].
    Octant5(Octant5<T>),
    /// Gently-sloped line segment in `(180°, 225°)`, see [`Octant6`].
    Octant6(Octant6<T>),
    /// Steeply-sloped line segment in `(225°, 270°)`, see [`Octant7`].
    Octant7(Octant7<T>),
}

/// Delegates calls to octant variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::SignedAxis0($me) => $call,
            Self::SignedAxis1($me) => $call,
            Self::SignedAxis2($me) => $call,
            Self::SignedAxis3($me) => $call,
            Self::Quadrant0($me) => $call,
            Self::Quadrant1($me) => $call,
            Self::Quadrant2($me) => $call,
            Self::Quadrant3($me) => $call,
            Self::Octant0($me) => $call,
            Self::Octant1($me) => $call,
            Self::Octant2($me) => $call,
            Self::Octant3($me) => $call,
            Self::Octant4($me) => $call,
            Self::Octant5($me) => $call,
            Self::Octant6($me) => $call,
            Self::Octant7($me) => $call,
        }
    };
}

macro_rules! octant {
    ($Octant:ident, $T:ty, $p1:expr, $p2:expr, $delta:expr) => {
        Self::$Octant($Octant::<$T>::new_inner($p1, $p2, $delta))
    };
    ($Octant:ident, $T:ty, $p1:expr, $p2:expr, $delta:expr, $clip:expr) => {
        map!($Octant::<$T>::clip_inner($p1, $p2, $delta, $clip), Self::$Octant)
    };
}

macro_rules! bresenham_impl {
    ($T:ty) => {
        impl Bresenham<$T> {
            /// Returns a [Bresenham] iterator over an arbitrary directed line segment.
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Self {
                use diagonal::{Quadrant0, Quadrant1, Quadrant2, Quadrant3};
                if y1 == y2 {
                    use orthogonal::Horizontal;
                    return match Horizontal::<$T>::new(y1, x1, x2) {
                        Horizontal::Positive(me) => Self::SignedAxis0(me),
                        Horizontal::Negative(me) => Self::SignedAxis1(me),
                    };
                }
                if x1 == x2 {
                    use orthogonal::Vertical;
                    return match Vertical::<$T>::new(x1, y1, y2) {
                        Vertical::Positive(me) => Self::SignedAxis2(me),
                        Vertical::Negative(me) => Self::SignedAxis3(me),
                    };
                }
                if x1 < x2 {
                    let dx = Math::<$T>::delta(x2, x1);
                    if y1 < y2 {
                        let dy = Math::<$T>::delta(y2, y1);
                        if dy < dx {
                             return octant!(Octant0, $T, (x1, y1), (x2, y2), (dx, dy));
                        }
                        if dx < dy {
                             return octant!(Octant1, $T, (x1, y1), (x2, y2), (dx, dy));
                        }
                        return diagonal::quadrant!(Quadrant0, $T, (x1, y1), x2);
                    }
                    let dy = Math::<$T>::delta(y1, y2);
                    if dy < dx {
                        return octant!(Octant2, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    if dx < dy {
                        return octant!(Octant3, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    return diagonal::quadrant!(Quadrant1, $T, (x1, y1), x2);
                }
                let dx = Math::<$T>::delta(x1, x2);
                if y1 < y2 {
                    let dy = Math::<$T>::delta(y2, y1);
                    if dy < dx {
                        return octant!(Octant4, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    if dx < dy {
                        return octant!(Octant5, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    return diagonal::quadrant!(Quadrant2, $T, (x1, y1), x2);
                }
                let dy = Math::<$T>::delta(y1, y2);
                if dy < dx {
                    return octant!(Octant6, $T, (x1, y1), (x2, y2), (dx, dy));
                }
                if dx < dy {
                    return octant!(Octant7, $T, (x1, y1), (x2, y2), (dx, dy));
                }
                return diagonal::quadrant!(Quadrant3, $T, (x1, y1), x2);
            }

            /// Returns a [Bresenham] iterator over an arbitrary directed line segment
            /// clipped to a [rectangular region](Clip), or [`None`] if it does not
            /// intersect the region.
            ///
            /// **Note**: `(x2, y2)` is not included.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                use diagonal::{Quadrant0, Quadrant1, Quadrant2, Quadrant3};
                if y1 == y2 {
                    use orthogonal::Horizontal;
                    return map!(
                        Horizontal::<$T>::clip(y1, x1, x2, clip),
                        me => match me {
                            Horizontal::Positive(me) => Self::SignedAxis0(me),
                            Horizontal::Negative(me) => Self::SignedAxis1(me),
                        }
                    );
                }
                if x1 == x2 {
                    use orthogonal::Vertical;
                    return map!(
                        Vertical::<$T>::clip(x1, y1, y2, clip),
                        me => match me {
                            Vertical::Positive(me) => Self::SignedAxis2(me),
                            Vertical::Negative(me) => Self::SignedAxis3(me),
                        }
                    );
                }
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                if x1 < x2 {
                    reject_if!(x2 < wx1 || wx2 <= x1);
                    let dx = Math::<$T>::delta(x2, x1);
                    if y1 < y2 {
                        reject_if!(y2 < wy1 || wy2 <= y1);
                        let dy = Math::<$T>::delta(y2, y1);
                        if dy < dx {
                            return octant!(Octant0, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                        }
                        if dx < dy {
                            return octant!(Octant1, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                        }
                        return diagonal::quadrant!(Quadrant0, $T, (x1, y1), (x2, y2), clip);
                    }
                    reject_if!(y1 < wy1 || wy2 <= y2);
                    let dy = Math::<$T>::delta(y1, y2);
                    if dy < dx {
                        return octant!(Octant2, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    if dx < dy {
                        return octant!(Octant3, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    return diagonal::quadrant!(Quadrant1, $T, (x1, y1), (x2, y2), clip);
                }
                reject_if!(x1 < wx1 || wx2 <= x2);
                let dx = Math::<$T>::delta(x1, x2);
                if y1 < y2 {
                    reject_if!(y2 < wy1 || wy2 <= y1);
                    let dy = Math::<$T>::delta(y2, y1);
                    if dy < dx {
                        return octant!(Octant4, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    if dx < dy {
                        return octant!(Octant5, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    return diagonal::quadrant!(Quadrant2, $T, (x1, y1), (x2, y2), clip);
                }
                reject_if!(y1 < wy1 || wy2 <= y2);
                let dy = Math::<$T>::delta(y1, y2);
                if dy < dx {
                    return octant!(Octant6, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                }
                if dx < dy {
                    return octant!(Octant7, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                }
                return diagonal::quadrant!(Quadrant3, $T, (x1, y1), (x2, y2), clip);
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

        impl Iterator for Bresenham<$T> {
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

        impl core::iter::FusedIterator for Bresenham<$T> {}
    };
}

bresenham_impl!(i8);
bresenham_impl!(u8);
bresenham_impl!(i16);
bresenham_impl!(u16);
bresenham_impl!(i32);
bresenham_impl!(u32);
#[cfg(feature = "bresenham_64")]
bresenham_impl!(i64);
#[cfg(feature = "bresenham_64")]
bresenham_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
bresenham_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
bresenham_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
bresenham_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
bresenham_impl!(usize);

macro_rules! bresenham_exact_size_iter_impl {
    ($T:ty) => {
        impl ExactSizeIterator for Bresenham<$T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                delegate!(self, me => me.is_empty())
            }
        }
    };
}

bresenham_exact_size_iter_impl!(i8);
bresenham_exact_size_iter_impl!(u8);
bresenham_exact_size_iter_impl!(i16);
bresenham_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
bresenham_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
bresenham_exact_size_iter_impl!(u32);
#[cfg(feature = "bresenham_64")]
bresenham_exact_size_iter_impl!(i64);
#[cfg(feature = "bresenham_64")]
bresenham_exact_size_iter_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
bresenham_exact_size_iter_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
bresenham_exact_size_iter_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
bresenham_exact_size_iter_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "bresenham_64"))]
bresenham_exact_size_iter_impl!(usize);

#[cfg(test)]
mod static_tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    const fn iterator_8() {
        assert_impl_all!(Octant0<i8>: ExactSizeIterator);
        assert_impl_all!(Octant0<u8>: ExactSizeIterator);
        assert_impl_all!(Bresenham<i8>: ExactSizeIterator);
        assert_impl_all!(Bresenham<u8>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_16() {
        assert_impl_all!(Octant0<i16>: ExactSizeIterator);
        assert_impl_all!(Octant0<u16>: ExactSizeIterator);
        assert_impl_all!(Bresenham<i16>: ExactSizeIterator);
        assert_impl_all!(Bresenham<u16>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_32() {
        #[cfg(target_pointer_width = "16")]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(Octant0<i32>: Iterator);
            assert_impl_all!(Octant0<u32>: Iterator);
            assert_impl_all!(Bresenham<i32>: Iterator);
            assert_impl_all!(Bresenham<u32>: Iterator);
            assert_not_impl_any!(Octant0<i32>: ExactSizeIterator);
            assert_not_impl_any!(Octant0<u32>: ExactSizeIterator);
            assert_not_impl_any!(Bresenham<i32>: ExactSizeIterator);
            assert_not_impl_any!(Bresenham<u32>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            assert_impl_all!(Octant0<i32>: ExactSizeIterator);
            assert_impl_all!(Octant0<u32>: ExactSizeIterator);
            assert_impl_all!(Bresenham<i32>: ExactSizeIterator);
            assert_impl_all!(Bresenham<u32>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_64() {
        #[cfg(feature = "bresenham_64")]
        {
            #[cfg(target_pointer_width = "64")]
            {
                assert_impl_all!(Octant0<i64>: ExactSizeIterator);
                assert_impl_all!(Octant0<u64>: ExactSizeIterator);
                assert_impl_all!(Bresenham<i64>: ExactSizeIterator);
                assert_impl_all!(Bresenham<u64>: ExactSizeIterator);
            }
            #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
            {
                use static_assertions::assert_not_impl_any;

                assert_impl_all!(Octant0<i64>: Iterator);
                assert_impl_all!(Octant0<u64>: Iterator);
                assert_impl_all!(Bresenham<i64>: Iterator);
                assert_impl_all!(Bresenham<u64>: Iterator);
                assert_not_impl_any!(Octant0<i64>: ExactSizeIterator);
                assert_not_impl_any!(Octant0<u64>: ExactSizeIterator);
                assert_not_impl_any!(Bresenham<i64>: ExactSizeIterator);
                assert_not_impl_any!(Bresenham<u64>: ExactSizeIterator);
            }
        }
        #[cfg(not(feature = "bresenham_64"))]
        {
            use static_assertions::assert_not_impl_any;

            assert_not_impl_any!(Octant0<i64>: Iterator);
            assert_not_impl_any!(Octant0<u64>: Iterator);
            assert_not_impl_any!(Bresenham<i64>: Iterator);
            assert_not_impl_any!(Bresenham<u64>: Iterator);
        }
    }

    #[test]
    const fn iterator_pointer_size() {
        #[cfg(target_pointer_width = "64")]
        {
            #[cfg(feature = "bresenham_64")]
            {
                assert_impl_all!(Octant0<isize>: ExactSizeIterator);
                assert_impl_all!(Octant0<usize>: ExactSizeIterator);
                assert_impl_all!(Bresenham<isize>: ExactSizeIterator);
                assert_impl_all!(Bresenham<usize>: ExactSizeIterator);
            }
            #[cfg(not(feature = "bresenham_64"))]
            {
                use static_assertions::assert_not_impl_any;

                assert_not_impl_any!(Octant0<isize>: Iterator);
                assert_not_impl_any!(Octant0<usize>: Iterator);
                assert_not_impl_any!(Bresenham<isize>: Iterator);
                assert_not_impl_any!(Bresenham<usize>: Iterator);
            }
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        {
            assert_impl_all!(Octant0<isize>: ExactSizeIterator);
            assert_impl_all!(Octant0<usize>: ExactSizeIterator);
            assert_impl_all!(Bresenham<isize>: ExactSizeIterator);
            assert_impl_all!(Bresenham<usize>: ExactSizeIterator);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagonal::{Quadrant0, Quadrant1, Quadrant2, Quadrant3};
    use crate::orthogonal::{
        NegativeHorizontal, NegativeVertical, PositiveHorizontal, PositiveVertical,
    };

    #[test]
    fn orthogonal_lines_are_special_cased() {
        assert_eq!(
            Bresenham::SignedAxis0(PositiveHorizontal::<u8>::new(0, 0, 255).unwrap()),
            Bresenham::<u8>::new((0, 0), (255, 0)),
        );
        assert_eq!(
            Bresenham::SignedAxis1(NegativeHorizontal::<u8>::new(0, 255, 0).unwrap()),
            Bresenham::<u8>::new((255, 0), (0, 0)),
        );
        assert_eq!(
            Bresenham::SignedAxis2(PositiveVertical::<u8>::new(0, 0, 255).unwrap()),
            Bresenham::<u8>::new((0, 0), (0, 255)),
        );
        assert_eq!(
            Bresenham::SignedAxis3(NegativeVertical::<u8>::new(0, 255, 0).unwrap()),
            Bresenham::<u8>::new((0, 255), (0, 0)),
        );
    }

    #[test]
    fn diagonal_lines_are_special_cased() {
        assert_eq!(
            Bresenham::<u8>::new((0, 0), (255, 255)),
            Bresenham::Quadrant0(Quadrant0::<u8>::new((0, 0), (255, 255)).unwrap()),
        );
        assert_eq!(
            Bresenham::<u8>::new((0, 255), (255, 0)),
            Bresenham::Quadrant1(Quadrant1::<u8>::new((0, 255), (255, 0)).unwrap()),
        );
        assert_eq!(
            Bresenham::<u8>::new((255, 0), (0, 255)),
            Bresenham::Quadrant2(Quadrant2::<u8>::new((255, 0), (0, 255)).unwrap()),
        );
        assert_eq!(
            Bresenham::<u8>::new((255, 255), (0, 0)),
            Bresenham::Quadrant3(Quadrant3::<u8>::new((255, 255), (0, 0)).unwrap()),
        );
    }

    #[test]
    fn exclusive_covers_whole_domain() {
        const MAX: u8 = u8::MAX;
        for i in 0..=MAX {
            assert_eq!(Bresenham::<u8>::new((0, i), (MAX, MAX)).count(), MAX as usize);
            assert_eq!(Bresenham::<u8>::new((MAX, MAX), (0, i)).count(), MAX as usize);
            assert_eq!(Bresenham::<u8>::new((i, 0), (MAX, MAX)).count(), MAX as usize);
            assert_eq!(Bresenham::<u8>::new((MAX, MAX), (i, 0)).count(), MAX as usize);
            assert_eq!(Bresenham::<u8>::new((0, MAX), (MAX, i)).count(), MAX as usize);
            assert_eq!(Bresenham::<u8>::new((MAX, i), (0, MAX)).count(), MAX as usize);
            assert_eq!(Bresenham::<u8>::new((MAX, 0), (i, MAX)).count(), MAX as usize);
            assert_eq!(Bresenham::<u8>::new((i, MAX), (MAX, 0)).count(), MAX as usize);
        }
    }
}
