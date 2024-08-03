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
use crate::symmetry::{fx, fy, sorted, xy};
use crate::utils::map_opt;
use crate::{diagonal, orthogonal};

mod clip;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Bresenham octant iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a directed line segment in the given octant of [Bresenham's algorithm][1].
///
/// An octant is defined by its transformations relative to [`Octant0`]:
/// - `FY`: flip the `y` axis if `true`.
/// - `FX`: flip the `x` axis if `true`.
/// - `SWAP`: swap the `x` and `y` axes if `true`.
///
/// [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Octant<T: Num, const FX: bool, const FY: bool, const SWAP: bool> {
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
pub type Octant0<T> = Octant<T, false, false, false>;

/// Iterator over a directed line segment in the second [octant](Octant)
/// covering the `(45°, 90°)` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` increase,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by swapping the `x` and `y` coordinates.
pub type Octant1<T> = Octant<T, false, false, true>;

/// Iterator over a directed line segment in the third [octant](Octant).
/// covering the `[315°, 360°)` sector of the Cartesian plane.
///
/// In this octant, `x` increases and `y` decreases,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate.
pub type Octant2<T> = Octant<T, false, true, false>;

/// Iterator over a directed line segment in the fourth [octant](Octant).
/// covering the `(270°, 315°)` sector of the Cartesian plane.
///
/// In this octant, `x` increases and `y` decreases,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant3<T> = Octant<T, false, true, true>;

/// Iterator over a directed line segment in the fifth [octant](Octant)
/// covering the `[135°, 180°)` sector of the Cartesian plane.
///
/// In this octant, `x` decreases and `y` increases,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate.
pub type Octant4<T> = Octant<T, true, false, false>;

/// Iterator over a directed line segment in the sixth [octant](Octant)
/// covering the `(90°, 135°)` sector of the Cartesian plane.
///
/// In this octant, `x` decreases and `y` increases,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant5<T> = Octant<T, true, false, true>;

/// Iterator over a directed line segment in the seventh [octant](Octant)
/// covering the `(180°, 225°]` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` decrease,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` and `y` coordinates.
pub type Octant6<T> = Octant<T, true, true, false>;

/// Iterator over a directed line segment in the eighth [octant](Octant)
/// covering the `(225°, 270°)` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` decrease,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping and swapping the `x` and `y` coordinates.
pub type Octant7<T> = Octant<T, true, true, true>;

impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<i8, FX, FY, SWAP> {
    #[inline(always)]
    #[must_use]
    const fn new_inner((x1, y1): Point<i8>, (x2, y2): Point<i8>, (dx, dy): Delta<i8>) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        let error = Math::<i8>::error(xy!(dy, dx), Math::<i8>::half(xy!(dx, dy)));
        let end = xy!(x2, y2);
        Self { x: x1, y: y1, error, dx, dy, end }
    }

    #[inline(always)]
    #[must_use]
    const fn covers((x1, y1): Point<i8>, (x2, y2): Point<i8>) -> Option<Delta<i8>> {
        let dx = {
            let (a, b) = sorted!(FX, x1, x2, None);
            Math::<i8>::delta(b, a)
        };
        let dy = {
            let (a, b) = sorted!(FY, y1, y2, None);
            Math::<i8>::delta(b, a)
        };
        if xy!(dx < dy, dy <= dx) {
            return None;
        }
        Some((dx, dy))
    }

    /// Returns an iterator over a directed line segment
    /// if it is covered by the given [octant](Octant).
    ///
    /// The line segment is defined by its starting point and
    /// the absolute offsets along the `x` and `y` coordinates.
    ///
    /// Returns [`None`] if the line segment is not covered by the octant.
    #[inline]
    #[must_use]
    pub const fn new(start: Point<i8>, end: Point<i8>) -> Option<Self> {
        let Some(delta) = Self::covers(start, end) else {
            return None;
        };
        Some(Self::new_inner(start, end, delta))
    }

    /// Returns an iterator over a directed line segment,
    /// if it is covered by the [octant](Octant),
    /// clipped to the [rectangular region](Clip).
    ///
    /// The line segment is defined by its starting point and
    /// the absolute offsets along the `x` and `y` coordinates.
    ///
    /// Returns [`None`] if the line segment is not covered by the octant,
    /// or if the line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(start: Point<i8>, end: Point<i8>, clip: Clip<i8>) -> Option<Self> {
        let Some(delta) = Self::covers(start, end) else {
            return None;
        };
        Self::clip_inner(start, end, delta, clip)
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
    ///
    /// Optimized over [`i8::abs_diff`].
    #[inline]
    #[must_use]
    pub const fn length(&self) -> <i8 as Num>::U {
        let (a, b) = xy!(
            fx!((self.end, self.x), (self.x, self.end)),
            fy!((self.end, self.y), (self.y, self.end))
        );
        #[allow(clippy::cast_sign_loss)]
        <i8 as Num>::U::wrapping_sub(a as _, b as _)
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> Iterator for Octant<i8, FX, FY, SWAP> {
    type Item = Point<i8>;

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
        let length = self.length() as usize;
        (length, Some(length))
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> ExactSizeIterator
    for Octant<i8, FX, FY, SWAP>
{
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> core::iter::FusedIterator
    for Octant<i8, FX, FY, SWAP>
{
}

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

macro_rules! octants {
    (
        $num:ty,
        ($x1:ident, $y1:ident),
        ($x2:ident, $y2:ident),
        ($dx:ident, $dy:ident),
        $horizontal:expr,
        $vertical:expr,
        $diagonal_0:expr,
        $diagonal_1:expr,
        $diagonal_2:expr,
        $diagonal_3:expr,
        $octant_0:expr,
        $octant_1:expr,
        $octant_2:expr,
        $octant_3:expr,
        $octant_4:expr,
        $octant_5:expr,
        $octant_6:expr,
        $octant_7:expr$(,)?
    ) => {
        if $y1 == $y2 {
            use orthogonal::Horizontal;
            return $horizontal;
        }
        if $x1 == $x2 {
            use orthogonal::Vertical;
            return $vertical;
        }
        #[allow(clippy::cast_sign_loss)]
        {
            use diagonal::Quadrant;
            if $x1 < $x2 {
                let $dx = Math::<$num>::delta($x2, $x1);
                if $y1 < $y2 {
                    let $dy = Math::<$num>::delta($y2, $y1);
                    if $dy < $dx {
                        return $octant_0;
                    }
                    if $dx < $dy {
                        return $octant_1;
                    }
                    return $diagonal_0;
                }
                let $dy = Math::<$num>::delta($y1, $y2);
                if $dy < $dx {
                    return $octant_2;
                }
                if $dx < $dy {
                    return $octant_3;
                }
                return $diagonal_1;
            }
            let $dx = Math::<$num>::delta($x1, $x2);
            if $y1 < $y2 {
                let $dy = Math::<$num>::delta($y2, $y1);
                if $dy < $dx {
                    return $octant_4;
                }
                if $dx < $dy {
                    return $octant_5;
                }
                return $diagonal_2;
            }
            let $dy = Math::<$num>::delta($y1, $y2);
            if $dy < $dx {
                return $octant_6;
            }
            if $dx < $dy {
                return $octant_7;
            }
            return $diagonal_3;
        }
    };
}

impl Bresenham<i8> {
    /// Returns a [Bresenham] iterator over an arbitrary directed line segment.
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<i8>, (x2, y2): Point<i8>) -> Self {
        octants!(
            i8,
            (x1, y1),
            (x2, y2),
            (dx, dy),
            match Horizontal::new(y1, x1, x2) {
                Horizontal::Positive(me) => Self::SignedAxis0(me),
                Horizontal::Negative(me) => Self::SignedAxis1(me),
            },
            match Vertical::new(x1, y1, y2) {
                Vertical::Positive(me) => Self::SignedAxis2(me),
                Vertical::Negative(me) => Self::SignedAxis3(me),
            },
            Self::Quadrant0(Quadrant::new_inner((x1, y1), x2)),
            Self::Quadrant1(Quadrant::new_inner((x1, y1), x2)),
            Self::Quadrant2(Quadrant::new_inner((x1, y1), x2)),
            Self::Quadrant3(Quadrant::new_inner((x1, y1), x2)),
            Self::Octant0(Octant::new_inner((x1, y1), (x2, y2), (dx, dy))),
            Self::Octant1(Octant::new_inner((x1, y1), (x2, y2), (dx, dy))),
            Self::Octant2(Octant::new_inner((x1, y1), (x2, y2), (dx, dy))),
            Self::Octant3(Octant::new_inner((x1, y1), (x2, y2), (dx, dy))),
            Self::Octant4(Octant::new_inner((x1, y1), (x2, y2), (dx, dy))),
            Self::Octant5(Octant::new_inner((x1, y1), (x2, y2), (dx, dy))),
            Self::Octant6(Octant::new_inner((x1, y1), (x2, y2), (dx, dy))),
            Self::Octant7(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)))
        );
    }

    /// Returns a [Bresenham] iterator over an arbitrary directed line segment
    /// clipped to a [rectangular region](Clip).
    ///
    /// Returns [`None`] if the line segment does not intersect the [clipping region](Clip).
    #[inline]
    #[must_use]
    pub const fn clip((x1, y1): Point<i8>, (x2, y2): Point<i8>, clip: Clip<i8>) -> Option<Self> {
        octants!(
            i8,
            (x1, y1),
            (x2, y2),
            (dx, dy),
            map_opt!(Horizontal::clip(y1, x1, x2, clip), me => match me {
                Horizontal::Positive(me) => Self::SignedAxis0(me),
                Horizontal::Negative(me) => Self::SignedAxis1(me),
            }),
            map_opt!(Vertical::clip(x1, y1, y2, clip), me => match me {
                Vertical::Positive(me) => Self::SignedAxis2(me),
                Vertical::Negative(me) => Self::SignedAxis3(me),
            }),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant0),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant1),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant2),
            map_opt!(Quadrant::clip_inner((x1, y1), (x2, y2), clip), Self::Quadrant3),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant0),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant1),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant2),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant3),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant4),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant5),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant6),
            map_opt!(Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), clip), Self::Octant7),
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
    pub const fn length(&self) -> <i8 as Num>::U {
        delegate!(self, me => me.length())
    }
}

impl Iterator for Bresenham<i8> {
    type Item = Point<i8>;

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

impl ExactSizeIterator for Bresenham<i8> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Bresenham<i8> {}
