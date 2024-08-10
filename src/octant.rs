//! ## Octant iterators

use crate::clip::Clip;
use crate::math::{Delta, Math, Num, Point};
use crate::symmetry::{fx, fy, xy};
use crate::utils::{map, reject_if};
use crate::{axis_aligned, diagonal};

mod clip;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Bresenham octant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a line segment in the given **octant**,
/// backed by [Bresenham's algorithm][1].
///
/// An octant is defined by its symmetries relative to [`Octant0`]:
/// - `FX`: flip the `x` axis if `true`.
/// - `FY`: flip the `y` axis if `true`.
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

/// Iterator over a line segment in the
/// [octant](Octant) where `x` and `y` **both increase**,
/// with `x` changing faster than `y` *(gentle slope)*.
///
/// Covers line segments spanning the `(0°, 45°]` sector.
pub type Octant0<T> = Octant<false, false, false, T>;

/// Iterator over a line segment in the
/// [octant](Octant) where `x` and `y` **both increase**,
/// with `y` changing faster than `x` *(steep slope)*.
///
/// Covers line segments spanning the `(45°, 90°)` sector.
pub type Octant1<T> = Octant<false, false, true, T>;

/// Iterator over a line segment in the
/// [octant](Octant) where `x` **increases** and `y` **decreases**,
/// with `x` changing faster than `y` *(gentle slope)*.
///
/// Covers line segments spanning the `[315°, 360°)` sector.
pub type Octant2<T> = Octant<false, true, false, T>;

/// Iterator over a line segment in the
/// [octant](Octant) where `x` **increases** and `y` **decreases**,
/// with `y` changing faster than `x` *(steep slope)*.
///
/// Covers line segments spanning the `(270°, 315°)` sector.
pub type Octant3<T> = Octant<false, true, true, T>;

/// Iterator over a line segment in the
/// [octant](Octant) where `x` **decreases** and `y` **increases**,
/// with `x` changing faster than `y` *(gentle slope)*.
///
/// Covers line segments spanning the `[135°, 180°)` sector.
pub type Octant4<T> = Octant<true, false, false, T>;

/// Iterator over a line segment in the
/// [octant](Octant) where `x` **decreases** and `y` **increases**,
/// with `y` changing faster than `x` *(steep slope)*.
///
/// Covers line segments spanning the `(90°, 135°)` sector.
pub type Octant5<T> = Octant<true, false, true, T>;

/// Iterator over a line segment in the
/// [octant](Octant) where `x` and `y` **both decrease**,
/// with `x` changing faster than `y` *(gentle slope)*.
///
/// Covers line segments spanning the `(180°, 225°]` sector.
pub type Octant6<T> = Octant<true, true, false, T>;

/// Iterator over a line segment in the
/// [octant](Octant) where `x` and `y` **both decrease**,
/// with `y` changing faster than `x` *(steep slope)*.
///
/// Covers line segments spanning the `(225°, 270°)` sector.
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
                let (half_du, r) = Math::<$T>::half(xy!(dx, dy));
                let error = Math::<$T>::error(xy!(dy, dx), half_du.wrapping_add(r));
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

            /// Returns an iterator over a *half-open* line segment
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

            /// Returns an iterator over a *half-open* line segment,
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
                // TODO: strict comparison for closed line segments
                reject_if!(xy!(u2 <= wx1, u2 < wx1) || wx2 < u1);
                let (v1, v2) = fy!((y1, y2), (y2, y1));
                reject_if!(xy!(v2 < wy1, v2 <= wy1) || wy2 < v1);
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
#[cfg(feature = "octant_64")]
octant_impl!(i64);
#[cfg(feature = "octant_64")]
octant_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
octant_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
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
#[cfg(feature = "octant_64")]
octant_exact_size_iter_impl!(i64);
#[cfg(feature = "octant_64")]
octant_exact_size_iter_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_exact_size_iter_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
octant_exact_size_iter_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
octant_exact_size_iter_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
octant_exact_size_iter_impl!(usize);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over an arbitrary line segment.
///
/// Chooses a specialized iterator variant **at runtime** based
/// on the orientation and direction of the line segment.
///
/// If you know the orientation of the line segment, use one of the [octant](Octant),
/// [diagonal](crate::Diagonal), or [axis-aligned](crate::Axis) iterators.
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it chooses
/// the underlying iterator only once instead of on every call to [`Iterator::next`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AnyOctant<T: Num> {
    /// Horizontal line segment at `0°`, see [`PositiveAxis0`](crate::PositiveAxis0).
    PositiveAxis0(axis_aligned::PositiveAxis0<T>),
    /// Vertical line segment at `90°`, see [`NegativeAxis0`](crate::NegativeAxis0).
    NegativeAxis0(axis_aligned::NegativeAxis0<T>),
    /// Horizontal line segment at `180°`, see [`PositiveAxis1`](crate::PositiveAxis1).
    PositiveAxis1(axis_aligned::PositiveAxis1<T>),
    /// Vertical line segment at `270°`, see [`NegativeAxis1`](crate::NegativeAxis1).
    NegativeAxis1(axis_aligned::NegativeAxis1<T>),
    /// Diagonal line segment at `45°`, see [`Diagonal0`](crate::Diagonal0).
    Diagonal0(diagonal::Diagonal0<T>),
    /// Diagonal line segment at `315°`, see [`Diagonal1`](crate::Diagonal1).
    Diagonal1(diagonal::Diagonal1<T>),
    /// Diagonal line segment at `135°`, see [`Diagonal2`](crate::Diagonal2).
    Diagonal2(diagonal::Diagonal2<T>),
    /// Diagonal line segment at `225°`, see [`Diagonal3`](crate::Diagonal3).
    Diagonal3(diagonal::Diagonal3<T>),
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

macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::PositiveAxis0($me) => $call,
            Self::NegativeAxis0($me) => $call,
            Self::PositiveAxis1($me) => $call,
            Self::NegativeAxis1($me) => $call,
            Self::Diagonal0($me) => $call,
            Self::Diagonal1($me) => $call,
            Self::Diagonal2($me) => $call,
            Self::Diagonal3($me) => $call,
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

macro_rules! any_octant_impl {
    ($T:ty) => {
        impl AnyOctant<$T> {
            /// Returns an iterator over an arbitrary *half-open* line segment.
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Self {
                use diagonal::{Diagonal0, Diagonal1, Diagonal2, Diagonal3};
                if y1 == y2 {
                    use axis_aligned::Axis0;
                    return match Axis0::<$T>::new(y1, x1, x2) {
                        Axis0::Positive(me) => Self::PositiveAxis0(me),
                        Axis0::Negative(me) => Self::NegativeAxis0(me),
                    };
                }
                if x1 == x2 {
                    use axis_aligned::Axis1;
                    return match Axis1::<$T>::new(x1, y1, y2) {
                        Axis1::Positive(me) => Self::PositiveAxis1(me),
                        Axis1::Negative(me) => Self::NegativeAxis1(me),
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
                        return diagonal::quadrant!(Diagonal0, $T, (x1, y1), x2);
                    }
                    let dy = Math::<$T>::delta(y1, y2);
                    if dy < dx {
                        return octant!(Octant2, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    if dx < dy {
                        return octant!(Octant3, $T, (x1, y1), (x2, y2), (dx, dy));
                    }
                    return diagonal::quadrant!(Diagonal1, $T, (x1, y1), x2);
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
                    return diagonal::quadrant!(Diagonal2, $T, (x1, y1), x2);
                }
                let dy = Math::<$T>::delta(y1, y2);
                if dy < dx {
                    return octant!(Octant6, $T, (x1, y1), (x2, y2), (dx, dy));
                }
                if dx < dy {
                    return octant!(Octant7, $T, (x1, y1), (x2, y2), (dx, dy));
                }
                return diagonal::quadrant!(Diagonal3, $T, (x1, y1), x2);
            }

            /// Clips an arbitrary *half-open* line segment to a [rectangular region](Clip),
            /// and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                use diagonal::{Diagonal0, Diagonal1, Diagonal2, Diagonal3};
                if y1 == y2 {
                    use axis_aligned::Axis0;
                    return map!(
                        Axis0::<$T>::clip(y1, x1, x2, clip),
                        me => match me {
                            Axis0::Positive(me) => Self::PositiveAxis0(me),
                            Axis0::Negative(me) => Self::NegativeAxis0(me),
                        }
                    );
                }
                if x1 == x2 {
                    use axis_aligned::Axis1;
                    return map!(
                        Axis1::<$T>::clip(x1, y1, y2, clip),
                        me => match me {
                            Axis1::Positive(me) => Self::PositiveAxis1(me),
                            Axis1::Negative(me) => Self::NegativeAxis1(me),
                        }
                    );
                }
                let &Clip { wx1, wy1, wx2, wy2 } = clip;
                if x1 < x2 {
                    reject_if!(x2 < wx1 || wx2 < x1);
                    let dx = Math::<$T>::delta(x2, x1);
                    if y1 < y2 {
                        reject_if!(y2 < wy1 || wy2 < y1);
                        let dy = Math::<$T>::delta(y2, y1);
                        if dy < dx {
                            // TODO: strict comparison for closed line segments
                            reject_if!(x2 == wx1);
                            return octant!(Octant0, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                        }
                        if dx < dy {
                            reject_if!(y2 == wy1);
                            return octant!(Octant1, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                        }
                        return diagonal::quadrant!(Diagonal0, $T, (x1, y1), (x2, y2), clip);
                    }
                    reject_if!(y1 < wy1 || wy2 < y2);
                    let dy = Math::<$T>::delta(y1, y2);
                    if dy < dx {
                        reject_if!(x2 == wx1);
                        return octant!(Octant2, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    if dx < dy {
                        reject_if!(y2 == wy2);
                        return octant!(Octant3, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    return diagonal::quadrant!(Diagonal1, $T, (x1, y1), (x2, y2), clip);
                }
                reject_if!(x1 < wx1 || wx2 < x2);
                let dx = Math::<$T>::delta(x1, x2);
                if y1 < y2 {
                    reject_if!(y2 < wy1 || wy2 < y1);
                    let dy = Math::<$T>::delta(y2, y1);
                    if dy < dx {
                        reject_if!(x2 == wx2);
                        return octant!(Octant4, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    if dx < dy {
                        reject_if!(y2 == wy1);
                        return octant!(Octant5, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                    }
                    return diagonal::quadrant!(Diagonal2, $T, (x1, y1), (x2, y2), clip);
                }
                reject_if!(y1 < wy1 || wy2 < y2);
                let dy = Math::<$T>::delta(y1, y2);
                if dy < dx {
                    reject_if!(x2 == wx2);
                    return octant!(Octant6, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                }
                if dx < dy {
                    reject_if!(y2 == wy2);
                    return octant!(Octant7, $T, (x1, y1), (x2, y2), (dx, dy), clip);
                }
                return diagonal::quadrant!(Diagonal3, $T, (x1, y1), (x2, y2), clip);
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

        impl Iterator for AnyOctant<$T> {
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

        impl core::iter::FusedIterator for AnyOctant<$T> {}
    };
}

any_octant_impl!(i8);
any_octant_impl!(u8);
any_octant_impl!(i16);
any_octant_impl!(u16);
any_octant_impl!(i32);
any_octant_impl!(u32);
#[cfg(feature = "octant_64")]
any_octant_impl!(i64);
#[cfg(feature = "octant_64")]
any_octant_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
any_octant_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
any_octant_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
any_octant_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
any_octant_impl!(usize);

macro_rules! any_octant_exact_size_iter_impl {
    ($T:ty) => {
        impl ExactSizeIterator for AnyOctant<$T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&self) -> bool {
                delegate!(self, me => me.is_empty())
            }
        }
    };
}

any_octant_exact_size_iter_impl!(i8);
any_octant_exact_size_iter_impl!(u8);
any_octant_exact_size_iter_impl!(i16);
any_octant_exact_size_iter_impl!(u16);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
any_octant_exact_size_iter_impl!(i32);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
any_octant_exact_size_iter_impl!(u32);
#[cfg(feature = "octant_64")]
any_octant_exact_size_iter_impl!(i64);
#[cfg(feature = "octant_64")]
any_octant_exact_size_iter_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
any_octant_exact_size_iter_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
any_octant_exact_size_iter_impl!(usize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
any_octant_exact_size_iter_impl!(isize);
#[cfg(all(target_pointer_width = "64", feature = "octant_64"))]
any_octant_exact_size_iter_impl!(usize);

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
    use crate::axis_aligned::{NegativeAxis0, NegativeAxis1, PositiveAxis0, PositiveAxis1};
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
