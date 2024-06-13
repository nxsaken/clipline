//! ## Bresenham line segment iterators
//!
//! This module provides a family of iterators for arbitrary directed line segments
//! powered by [Bresenham's algorithm][1].
//!
//! For an arbitrary directed line segment, use the [general Bresenham](Bresenham) iterator.
//! If you know more about the orientation and direction of the line segment,
//! use one of the [Bresenham octant](Octant), [diagonal](diagonal::Diagonal),
//! [orthogonal](axis_aligned::Orthogonal) or [axis-aligned](axis_aligned::AxisAligned)
//! iterators instead.
//!
//! [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

#[cfg(feature = "bresenham_diagonals")]
use crate::diagonal;
use crate::{axis_aligned, Point};
#[cfg(feature = "clip")]
use crate::{map_option, Region};

#[cfg(feature = "clip")]
mod kuzmin;

/// A generic offset between two [points](Point) on a Cartesian plane.
#[cfg(feature = "bresenham")]
type Offset<T> = (T, T);

/// Iterator over a directed line segment in the given octant of [Bresenham's algorithm][1].
///
/// An octant is defined by its transformations relative to [`Octant0`]:
/// - `FY`: flip the `y` axis if `true`.
/// - `FX`: flip the `x` axis if `true`.
/// - `SWAP`: swap the `x` and `y` axes if `true`.
///
/// [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Octant<const FX: bool, const FY: bool, const SWAP: bool> {
    x1: isize,
    y1: isize,
    error: isize,
    // TODO(#13): implement `DoubleEndedIterator`.
    end: isize,
    dx2: isize,
    dy2: isize,
}

/// Iterator over a directed line segment in the first [`Octant`].
///
/// This octant covers the sector where both `x` and `y` are increasing,
/// with `x` changing faster than `y` (gentle slope, covers `(0°, 45°)`).
pub type Octant0 = Octant<false, false, false>;

/// Iterator over a directed line segment in the second [`Octant`].
///
/// This octant covers the sector where both `x` and `y` are
/// increasing, with `y` changing faster than `x` (steep slope, covers `[45°, 90°)`).
///
/// Can be obtained from [`Octant0`] by swapping the `x` and `y` coordinates.
pub type Octant1 = Octant<false, false, true>;

/// Iterator over a directed line segment in the third [`Octant`].
///
/// This octant covers the sector where `x` is increasing and `y` is
/// decreasing, with `x` changing faster than `y` (gentle slope, covers `(315°, 360°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate.
pub type Octant2 = Octant<false, true, false>;

/// Iterator over a directed line segment in the fourth [`Octant`].
///
/// This octant covers the sector where `x` is increasing and `y` is
/// decreasing, with `y` changing faster than `x` (steep slope, covers `(270°, 315°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant3 = Octant<false, true, true>;

/// Iterator over a directed line segment in the fifth [`Octant`].
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// increasing, with `x` changing faster than `y` (gentle slope, covers `(135°, 180°)`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate.
pub type Octant4 = Octant<true, false, false>;

/// Iterator over a directed line segment in the sixth [`Octant`].
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// increasing, with `y` changing faster than `x` (steep slope, covers `[90°, 135°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant5 = Octant<true, false, true>;

/// Iterator over a directed line segment in the seventh [`Octant`].
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// decreasing, with `x` changing faster than `y` (gentle slope, covers `[180°, 225°)`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` and `y` coordinates.
pub type Octant6 = Octant<true, true, false>;

/// Iterator over a directed line segment in the eighth [`Octant`].
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// decreasing, with `y` changing faster than `x` (steep slope, covers `[225°, 270°]`).
///
/// Can be obtained from [`Octant0`] by flipping and swapping the `x` and `y` coordinates.
pub type Octant7 = Octant<true, true, true>;

impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<FX, FY, SWAP> {
    /// Returns an iterator over a directed line segment
    /// if it is covered by the [`Octant`].
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if !FX && dx <= 0 || FX && 0 < dx || !FY && dy <= 0 || FY && 0 < dy {
            return None;
        }
        let dx = if !FX { dx } else { -dx };
        let dy = if !FY { dy } else { -dy };
        if !SWAP && dx <= dy || SWAP && dy < dx {
            return None;
        }
        Some(Self::new_inner((x1, y1), (x2, y2), (dx, dy)))
    }

    #[inline(always)]
    #[must_use]
    const fn new_inner(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (dx, dy): Offset<isize>,
    ) -> Self {
        let (dx2, dy2) = (dx << 1, dy << 1);
        let error = if !SWAP { dy2 - dx } else { dx2 - dy };
        let end = if !SWAP { x2 } else { y2 };
        Self { x1, y1, error, end, dx2, dy2 }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        !SWAP && (!FX && self.end <= self.x1 || FX && self.x1 <= self.end)
            || SWAP && (!FY && self.end <= self.y1 || FY && self.y1 <= self.end)
    }
}

#[cfg(feature = "clip")]
impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<FX, FY, SWAP> {
    /// Returns an iterator over a directed line segment
    /// if it is covered by the [`Octant`] and overlaps the clipping [`Region`].
    ///
    /// The line segment will be clipped to the window using [Kuzmin's algorithm](kuzmin).
    #[inline]
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        region: Region<isize>,
    ) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if !FX && dx <= 0 || FX && 0 < dx || !FY && dy <= 0 || FY && 0 < dy {
            return None;
        }
        let dx = if !FX { dx } else { -dx };
        let dy = if !FY { dy } else { -dy };
        if !SWAP && dx <= dy || SWAP && dy < dx {
            return None;
        }
        Self::clip_inner((x1, y1), (x2, y2), (dx, dy), region)
    }

    #[inline(always)]
    #[must_use]
    const fn clip_inner(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (dx, dy): Offset<isize>,
        region: Region<isize>,
    ) -> Option<Self> {
        let (dx2, dy2) = (dx << 1, dy << 1);
        let Some(((cx1, cy1), error, end)) =
            kuzmin::clip::<FX, FY, SWAP>((x1, y1), (x2, y2), (dx, dy), (dx2, dy2), region)
        else {
            return None;
        };
        Some(Self {
            x1: cx1,
            y1: cy1,
            error,
            end,
            dx2: if !SWAP { dx2 - dy2 } else { dx2 },
            dy2: if !SWAP { dy2 } else { dy2 - dx2 },
        })
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> Iterator for Octant<FX, FY, SWAP> {
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

impl<const FX: bool, const FY: bool, const SWAP: bool> ExactSizeIterator for Octant<FX, FY, SWAP> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FX: bool, const FY: bool, const SWAP: bool> core::iter::FusedIterator
    for Octant<FX, FY, SWAP>
{
}

/// Iterator over an arbitrary directed line segment backed by [Bresenham's algorithm][1].
///
/// Chooses a sub-iterator variant based on the orientation and direction of the line segment.
///
/// If you know the alignment of the line segment beforehand, consider the more specific
/// [octant](Octant), [diagonal](diagonal::Diagonal), [orthogonal](axis_aligned::Orthogonal)
/// and [axis-aligned](axis_aligned::AxisAligned) iterators instead.
///
/// **Note**: an optimized implementation of [`Bresenham::fold`] is provided.
/// This makes [`Bresenham::for_each`] faster than a `for` loop, since it chooses
/// the underlying iterator only once instead of on every call to [`Bresenham::next`].
///
/// [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Bresenham {
    /// Optimized [positive horizontal](axis_aligned::PositiveHorizontal) signed-axis-aligned case.
    Orthogonal0(axis_aligned::PositiveHorizontal),
    /// Optimized [negative horizontal](axis_aligned::NegativeHorizontal) signed-axis-aligned case.
    Orthogonal1(axis_aligned::NegativeHorizontal),
    /// Optimized [positive vertical](axis_aligned::PositiveVertical) signed-axis-aligned case.
    Orthogonal2(axis_aligned::PositiveVertical),
    /// Optimized [negative vertical](axis_aligned::NegativeVertical) signed-axis-aligned case.
    Orthogonal3(axis_aligned::NegativeVertical),
    /// Optimized [positive-positive](diagonal::Quadrant0) diagonal case.
    #[cfg(feature = "bresenham_diagonals")]
    Diagonal0(diagonal::Quadrant0),
    /// Optimized [positive-negative](diagonal::Quadrant1) diagonal case.
    #[cfg(feature = "bresenham_diagonals")]
    Diagonal1(diagonal::Quadrant1),
    /// Optimized [negative-positive](diagonal::Quadrant2) diagonal case.
    #[cfg(feature = "bresenham_diagonals")]
    Diagonal2(diagonal::Quadrant2),
    /// Optimized [negative-negative](diagonal::Quadrant3) diagonal case.
    #[cfg(feature = "bresenham_diagonals")]
    Diagonal3(diagonal::Quadrant3),
    /// See [`Octant0`].
    Octant0(Octant0),
    /// See [`Octant1`].
    Octant1(Octant1),
    /// See [`Octant2`].
    Octant2(Octant2),
    /// See [`Octant3`].
    Octant3(Octant3),
    /// See [`Octant4`].
    Octant4(Octant4),
    /// See [`Octant5`].
    Octant5(Octant5),
    /// See [`Octant6`].
    Octant6(Octant6),
    /// See [`Octant7`].
    Octant7(Octant7),
}

/// Delegates calls to octant variants.
macro_rules! delegate {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::Orthogonal0($me) => $call,
            Self::Orthogonal1($me) => $call,
            Self::Orthogonal2($me) => $call,
            Self::Orthogonal3($me) => $call,
            #[cfg(feature = "bresenham_diagonals")]
            Self::Diagonal0($me) => $call,
            #[cfg(feature = "bresenham_diagonals")]
            Self::Diagonal1($me) => $call,
            #[cfg(feature = "bresenham_diagonals")]
            Self::Diagonal2($me) => $call,
            #[cfg(feature = "bresenham_diagonals")]
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

impl Bresenham {
    /// Returns a [Bresenham] iterator over an arbitrary directed line segment.
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Self {
        if y1 == y2 {
            use axis_aligned::Horizontal;
            return match Horizontal::new(y1, x1, x2) {
                Horizontal::Positive(me) => Self::Orthogonal0(me),
                Horizontal::Negative(me) => Self::Orthogonal1(me),
            };
        }
        if x1 == x2 {
            use axis_aligned::Vertical;
            return match Vertical::new(x1, y1, y2) {
                Vertical::Positive(me) => Self::Orthogonal2(me),
                Vertical::Negative(me) => Self::Orthogonal3(me),
            };
        }
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dx {
            if 0 < dy {
                if dy < dx {
                    return Self::Octant0(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)));
                }
                #[cfg(feature = "bresenham_diagonals")]
                if dy == dx {
                    return Self::Diagonal0(diagonal::Quadrant::new_inner((x1, y1), (x2, y2)));
                }
                return Self::Octant1(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)));
            }
            let dy = -dy;
            if dy < dx {
                return Self::Octant2(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)));
            }
            #[cfg(feature = "bresenham_diagonals")]
            if dy == dx {
                return Self::Diagonal1(diagonal::Quadrant::new_inner((x1, y1), (x2, y2)));
            }
            return Self::Octant3(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)));
        }
        let dx = -dx;
        if 0 < dy {
            if dy < dx {
                return Self::Octant4(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)));
            }
            #[cfg(feature = "bresenham_diagonals")]
            if dy == dx {
                return Self::Diagonal2(diagonal::Quadrant::new_inner((x1, y1), (x2, y2)));
            }
            return Self::Octant5(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)));
        }
        let dy = -dy;
        if dy < dx {
            return Self::Octant6(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)));
        }
        #[cfg(feature = "bresenham_diagonals")]
        if dy == dx {
            return Self::Diagonal3(diagonal::Quadrant::new_inner((x1, y1), (x2, y2)));
        }
        Self::Octant7(Octant::new_inner((x1, y1), (x2, y2), (dx, dy)))
    }

    /// Returns a [Bresenham] iterator over an arbitrary directed line segment
    /// clipped to a [rectangular region](Region).
    ///
    /// Returns [`None`] if the line segment does not intersect the region.
    #[cfg(feature = "clip")]
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        region: Region<isize>,
    ) -> Option<Self> {
        if y1 == y2 {
            use axis_aligned::Horizontal;
            return map_option!(
                Horizontal::clip(y1, x1, x2, region),
                me => match me {
                    Horizontal::Positive(me) => Self::Orthogonal0(me),
                    Horizontal::Negative(me) => Self::Orthogonal1(me),
                }
            );
        }
        if x1 == x2 {
            use axis_aligned::Vertical;
            return map_option!(
                Vertical::clip(x1, y1, y2, region),
                me => match me {
                    Vertical::Positive(me) => Self::Orthogonal2(me),
                    Vertical::Negative(me) => Self::Orthogonal3(me),
                }
            );
        }
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dx {
            if 0 < dy {
                if dy < dx {
                    return map_option!(
                        Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
                        me => Self::Octant0(me)
                    );
                }
                #[cfg(feature = "bresenham_diagonals")]
                if dy == dx {
                    return map_option!(
                        diagonal::Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
                        me => Self::Diagonal0(me)
                    );
                }
                return map_option!(
                    Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
                    me => Self::Octant1(me)
                );
            }
            let dy = -dy;
            if dy < dx {
                return map_option!(
                    Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
                    me => Self::Octant2(me)
                );
            }
            #[cfg(feature = "bresenham_diagonals")]
            if dy == dx {
                return map_option!(
                    diagonal::Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
                    me => Self::Diagonal1(me)
                );
            }
            return map_option!(
                Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
                me => Self::Octant3(me)
            );
        }
        let dx = -dx;
        if 0 < dy {
            if dy < dx {
                return map_option!(
                    Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
                    me => Self::Octant4(me)
                );
            }
            #[cfg(feature = "bresenham_diagonals")]
            if dy == dx {
                return map_option!(
                    diagonal::Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
                    me => Self::Diagonal2(me)
                );
            }
            return map_option!(
                Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
                me => Self::Octant5(me)
            );
        }
        let dy = -dy;
        if dy < dx {
            return map_option!(
                Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
                me => Self::Octant6(me)
            );
        }
        #[cfg(feature = "bresenham_diagonals")]
        if dy == dx {
            return map_option!(
                diagonal::Quadrant::clip_inner((x1, y1), (x2, y2), dx, region),
                me => Self::Diagonal3(me)
            );
        }
        map_option!(
            Octant::clip_inner((x1, y1), (x2, y2), (dx, dy), region),
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

impl Iterator for Bresenham {
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

impl ExactSizeIterator for Bresenham {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl core::iter::FusedIterator for Bresenham {}
