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

use crate::{clip, orthogonal, Clip, Offset, Point};

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
pub struct Octant<T, const FX: bool, const FY: bool, const SWAP: bool> {
    x1: T,
    y1: T,
    error: T,
    end: T,
    dx2: T,
    dy2: T,
}

/// Iterator over a directed line segment in the first [octant](Octant)
/// covering the `(0°, 45°)` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` increase,
/// with `x` changing faster than `y` (gentle slope).
pub type Octant0<T> = Octant<T, false, false, false>;

/// Iterator over a directed line segment in the second [octant](Octant)
/// covering the `[45°, 90°)` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` increase,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by swapping the `x` and `y` coordinates.
pub type Octant1<T> = Octant<T, false, false, true>;

/// Iterator over a directed line segment in the third [octant](Octant).
/// covering the `(315°, 360°]` sector of the Cartesian plane.
///
/// In this octant, `x` increases and `y` decreases,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate.
pub type Octant2<T> = Octant<T, false, true, false>;

/// Iterator over a directed line segment in the fourth [octant](Octant).
/// covering the `(270°, 315°]` sector of the Cartesian plane.
///
/// In this octant, `x` increases and `y` decreases,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant3<T> = Octant<T, false, true, true>;

/// Iterator over a directed line segment in the fifth [octant](Octant)
/// covering the `(135°, 180°)` sector of the Cartesian plane.
///
/// In this octant, `x` decreases and `y` increases,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate.
pub type Octant4<T> = Octant<T, true, false, false>;

/// Iterator over a directed line segment in the sixth [octant](Octant)
/// covering the `[90°, 135°]` sector of the Cartesian plane.
///
/// In this octant, `x` decreases and `y` increases,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant5<T> = Octant<T, true, false, true>;

/// Iterator over a directed line segment in the seventh [octant](Octant)
/// covering the `[180°, 225°)` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` decrease,
/// with `x` changing faster than `y` (gentle slope).
///
/// Can be obtained from [`Octant0`] by flipping the `x` and `y` coordinates.
pub type Octant6<T> = Octant<T, true, true, false>;

/// Iterator over a directed line segment in the eighth [octant](Octant)
/// covering the `[225°, 270°]` sector of the Cartesian plane.
///
/// In this octant, both `x` and `y` decrease,
/// with `y` changing faster than `x` (steep slope).
///
/// Can be obtained from [`Octant0`] by flipping and swapping the `x` and `y` coordinates.
pub type Octant7<T> = Octant<T, true, true, true>;

impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<isize, FX, FY, SWAP> {
    /// Returns an iterator over a directed line segment covered by the given [octant](Octant).
    ///
    /// *Assumes that the line segment is covered by the given octant.*
    #[inline(always)]
    #[must_use]
    const fn new_unchecked(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (dx, dy): Offset<isize>,
    ) -> Self {
        let (dx2, dy2) = (dx << 1, dy << 1);
        let error = if !SWAP { dy2 - dx } else { dx2 - dy };
        let end = if !SWAP { x2 } else { y2 };
        Self { x1, y1, error, end, dx2, dy2 }
    }

    /// Returns an iterator over a directed line segment
    /// if it is covered by the given [octant](Octant).
    ///
    /// The line segment is defined by its starting point and
    /// the absolute offsets along the `x` and `y` coordinates.
    ///
    /// Returns [`None`] if the offsets don't match the steepness of the octant.
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (dx, dy): Offset<isize>) -> Option<Self> {
        if !FX && dx == 0 || !FY && dy == 0 || !SWAP && dx <= dy || SWAP && dy < dx {
            return None;
        }
        let x2 = if !FX { x1 + dx } else { x1 - dx };
        let y2 = if !FY { y1 + dy } else { y1 - dy };
        Some(Self::new_unchecked((x1, y1), (x2, y2), (dx, dy)))
    }

    /// Returns an iterator over a directed line segment covered by the [octant](Octant),
    /// clipped to the [rectangular region](Clip).
    ///
    /// Returns [`None`] if the line segment does not intersect the [clipping region](Clip).
    ///
    /// *Assumes that the line segment is covered by the given octant.*
    #[must_use]
    #[inline(always)]
    const fn clip_unchecked(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (dx, dy): Offset<isize>, // absolute value
        clip: &Clip<isize>,
    ) -> Option<Self> {
        if clip::diagonal::out_of_bounds::<FX, FY>((x1, y1), (x2, y2), clip) {
            return None;
        }
        let (dx2, dy2) = (dx << 1, dy << 1);
        let Some(((cx1, cy1), error)) =
            clip::kuzmin::enter::<FX, FY, SWAP>((x1, y1), (dx, dy), (dx2, dy2), clip)
        else {
            return None;
        };
        Some(Self {
            x1: cx1,
            y1: cy1,
            error,
            end: clip::kuzmin::exit::<FX, FY, SWAP>((x1, y1), (x2, y2), (dx, dy), (dx2, dy2), clip),
            dx2,
            dy2,
        })
    }

    /// Returns an iterator over a directed line segment,
    /// if it is covered by the [octant](Octant),
    /// clipped to the [rectangular region](Clip).
    ///
    /// The line segment is defined by its starting point and
    /// the absolute offsets along the `x` and `y` coordinates.
    ///
    /// Returns [`None`] if the offsets don't match the steepness of the octant,
    /// or if the line segment does not intersect the clipping region.
    #[inline]
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (dx, dy): Offset<isize>,
        clip: &Clip<isize>,
    ) -> Option<Self> {
        if !FX && dx == 0 || !FY && dy == 0 || !SWAP && dx <= dy || SWAP && dy < dx {
            return None;
        }
        let x2 = if !FX { x1 + dx } else { x1 - dx };
        let y2 = if !FY { y1 + dy } else { y1 - dy };
        Self::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip)
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        !SWAP && (!FX && self.end <= self.x1 || FX && self.x1 <= self.end)
            || SWAP && (!FY && self.end <= self.y1 || FY && self.y1 <= self.end)
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> Iterator for Octant<isize, FX, FY, SWAP> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = (self.x1, self.y1);
        if 0 <= self.error {
            match SWAP {
                false => self.y1 += if !FY { 1 } else { -1 },
                true => self.x1 += if !FX { 1 } else { -1 },
            }
            self.error -= if !SWAP { self.dx2 } else { self.dy2 };
        }
        match SWAP {
            false => self.x1 += if !FX { 1 } else { -1 },
            true => self.y1 += if !FY { 1 } else { -1 },
        }
        self.error += if !SWAP { self.dy2 } else { self.dx2 };
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // slightly optimized over `isize::abs_diff`,
        // see its implementation for the proof that the `isize` -> `usize` cast is legal
        #[allow(clippy::cast_sign_loss)]
        let length = match (SWAP, FX, FY) {
            (false, false, _) => usize::wrapping_sub(self.end as usize, self.x1 as usize),
            (false, true, _) => usize::wrapping_sub(self.x1 as usize, self.end as usize),
            (true, _, false) => usize::wrapping_sub(self.end as usize, self.y1 as usize),
            (true, _, true) => usize::wrapping_sub(self.y1 as usize, self.end as usize),
        };
        (length, Some(length))
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> ExactSizeIterator
    for Octant<isize, FX, FY, SWAP>
{
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> core::iter::FusedIterator
    for Octant<isize, FX, FY, SWAP>
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
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Bresenham<T> {
    /// Horizontal line segment at `0°`, see [`PositiveHorizontal`](crate::PositiveHorizontal).
    SignedAxis0(orthogonal::PositiveHorizontal<T>),
    /// Horizontal line segment at `180°`, see [`NegativeHorizontal`](crate::NegativeHorizontal).
    SignedAxis1(orthogonal::NegativeHorizontal<T>),
    /// Vertical line segment at `90°`, see [`PositiveVertical`](crate::PositiveVertical).
    SignedAxis2(orthogonal::PositiveVertical<T>),
    /// Vertical line segment at `270°`, see [`NegativeVertical`](crate::NegativeVertical).
    SignedAxis3(orthogonal::NegativeVertical<T>),
    /// Gently-sloped line segment in `(0°, 45°)`, see [`Octant0`].
    Octant0(Octant0<T>),
    /// Steeply-sloped line segment in `[45°, 90°)`, see [`Octant1`].
    Octant1(Octant1<T>),
    /// Gently-sloped line segment in `(315°, 360°)`, see [`Octant2`].
    Octant2(Octant2<T>),
    /// Steeply-sloped line segment in `(270°, 315°]`, see [`Octant3`].
    Octant3(Octant3<T>),
    /// Gently-sloped line segment in `(135°, 180°)`, see [`Octant4`].
    Octant4(Octant4<T>),
    /// Steeply-sloped line segment in `(90°, 135°]`, see [`Octant5`].
    Octant5(Octant5<T>),
    /// Gently-sloped line segment in `(180°, 225°)`, see [`Octant6`].
    Octant6(Octant6<T>),
    /// Steeply-sloped line segment in `[225°, 270°)`, see [`Octant7`].
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

impl Bresenham<isize> {
    /// Returns a [Bresenham] iterator over an arbitrary directed line segment.
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Self {
        if y1 == y2 {
            use orthogonal::Horizontal;
            return match Horizontal::new(y1, x1, x2) {
                Horizontal::Positive(me) => Self::SignedAxis0(me),
                Horizontal::Negative(me) => Self::SignedAxis1(me),
            };
        }
        if x1 == x2 {
            use orthogonal::Vertical;
            return match Vertical::new(x1, y1, y2) {
                Vertical::Positive(me) => Self::SignedAxis2(me),
                Vertical::Negative(me) => Self::SignedAxis3(me),
            };
        }
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dx {
            if 0 < dy {
                if dy < dx {
                    return Self::Octant0(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)));
                }
                return Self::Octant1(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)));
            }
            let dy = -dy;
            if dy < dx {
                return Self::Octant2(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)));
            }
            return Self::Octant3(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)));
        }
        let dx = -dx;
        if 0 < dy {
            if dy < dx {
                return Self::Octant4(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)));
            }
            return Self::Octant5(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)));
        }
        let dy = -dy;
        if dy < dx {
            return Self::Octant6(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)));
        }
        Self::Octant7(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy)))
    }

    /// Returns a [Bresenham] iterator over an arbitrary directed line segment
    /// clipped to a [rectangular region](Clip).
    ///
    /// Returns [`None`] if the line segment does not intersect the [clipping region](Clip).
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        clip: &Clip<isize>,
    ) -> Option<Self> {
        if y1 == y2 {
            use orthogonal::Horizontal;
            return clip::map_option!(
                Horizontal::clip(y1, x1, x2, clip),
                me => match me {
                    Horizontal::Positive(me) => Self::SignedAxis0(me),
                    Horizontal::Negative(me) => Self::SignedAxis1(me),
                }
            );
        }
        if x1 == x2 {
            use orthogonal::Vertical;
            return clip::map_option!(
                Vertical::clip(x1, y1, y2, clip),
                me => match me {
                    Vertical::Positive(me) => Self::SignedAxis2(me),
                    Vertical::Negative(me) => Self::SignedAxis3(me),
                }
            );
        }
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dx {
            if 0 < dy {
                if dy < dx {
                    return clip::map_option!(
                        Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
                        me => Self::Octant0(me)
                    );
                }
                return clip::map_option!(
                    Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
                    me => Self::Octant1(me)
                );
            }
            let dy = -dy;
            if dy < dx {
                return clip::map_option!(
                    Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
                    me => Self::Octant2(me)
                );
            }
            return clip::map_option!(
                Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
                me => Self::Octant3(me)
            );
        }
        let dx = -dx;
        if 0 < dy {
            if dy < dx {
                return clip::map_option!(
                    Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
                    me => Self::Octant4(me)
                );
            }
            return clip::map_option!(
                Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
                me => Self::Octant5(me)
            );
        }
        let dy = -dy;
        if dy < dx {
            return clip::map_option!(
                Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
                me => Self::Octant6(me)
            );
        }
        clip::map_option!(
            Octant::clip_unchecked((x1, y1), (x2, y2), (dx, dy), clip),
            me => Self::Octant7(me)
        )
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        delegate!(self, me => me.is_done())
    }
}

impl Iterator for Bresenham<isize> {
    type Item = Point<isize>;

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

impl ExactSizeIterator for Bresenham<isize> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Bresenham<isize> {}
