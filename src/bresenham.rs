//! ## Bresenham line segment iterators
//!
//! This module provides a family of iterators
//! for directed line segments powered by [Bresenham's algorithm][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

#[cfg(feature = "bresenham_diagonals")]
use crate::diagonal;
#[cfg(feature = "kuzmin")]
use crate::kuzmin;
use crate::{axis_aligned, Point};

/// Iterator over a directed line segment in the given octant of [Bresenham's algorithm][1].
///
/// An octant is defined by its transformations relative to [`Octant0`]:
/// - `FY`: flip the `y` axis if `true`.
/// - `FX`: flip the `x` axis if `true`.
/// - `SWAP`: swap the `x` and `y` axes if `true`.
///
/// [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Octant<const FY: bool, const FX: bool, const SWAP: bool> {
    x1: isize,
    y1: isize,
    dx2: isize,
    dy2: isize,
    error: isize,
    // TODO(#13): implement `DoubleEndedIterator`.
    end: isize,
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
/// This octant covers the sector where `x` is decreasing and `y` is
/// increasing, with `x` changing faster than `y` (gentle slope, covers `(135°, 180°)`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate.
pub type Octant2 = Octant<false, true, false>;

/// Iterator over a directed line segment in the fourth [`Octant`].
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// increasing, with `y` changing faster than `x` (steep slope, covers `[90°, 135°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant3 = Octant<false, true, true>;

/// Iterator over a directed line segment in the fifth [`Octant`].
///
/// This octant covers the sector where `x` is increasing and `y` is
/// decreasing, with `x` changing faster than `y` (gentle slope, covers `(315°, 360°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate.
pub type Octant4 = Octant<true, false, false>;

/// Iterator over a directed line segment in the sixth [`Octant`].
///
/// This octant covers the sector where `x` is increasing and `y` is
/// decreasing, with `y` changing faster than `x` (steep slope, covers `(270°, 315°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate,
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

impl<const FY: bool, const FX: bool, const SWAP: bool> Octant<FY, FX, SWAP> {
    /// Creates an iterator over a directed line segment
    /// if covered by the [`Octant`].
    #[inline]
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if FY && 0 < dy || !FY && dy <= 0 || FX && 0 < dx || !FX && dx <= 0 {
            return None;
        }
        let dy = if FY { -dy } else { dy };
        let dx = if FX { -dx } else { dx };
        if SWAP && dy < dx || !SWAP && dx <= dy {
            return None;
        }
        Some(Self::raw_unchecked((x1, y1), (x2, y2), (dx, dy)))
    }

    /// Creates an iterator over a directed line segment
    /// if it is covered by the [`Octant`]
    /// and overlaps the [clipping][1] window.
    ///
    /// The line segment will be clipped to the window
    /// using [Kuzmin's algorithm][2].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Line_clipping
    /// [2]: kuzmin
    #[cfg(feature = "kuzmin")]
    #[inline]
    #[must_use]
    pub const fn clip(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (wx1, wy1): Point<isize>,
        (wx2, wy2): Point<isize>,
    ) -> Option<Self> {
        let (dx, dy) = (x2 - x1, y2 - y1);
        if FY && 0 < dy || !FY && dy <= 0 || FX && 0 < dx || !FX && dx <= 0 {
            return None;
        }
        let dy = if FY { -dy } else { dy };
        let dx = if FX { -dx } else { dx };
        if SWAP && dy < dx || !SWAP && dx <= dy {
            return None;
        }
        Self::clip_unchecked((x1, y1), (x2, y2), (dx, dy), (wx1, wy1), (wx2, wy2))
    }

    #[cfg(feature = "kuzmin")]
    #[inline(always)]
    #[must_use]
    const fn clip_unchecked(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (dx, dy): (isize, isize),
        window_min: Point<isize>,
        window_max: Point<isize>,
    ) -> Option<Self> {
        let (dx2, dy2) = (dx << 1, dy << 1);
        let Some(((_cx1, _cy1), error)) =
            kuzmin::enter::<FY, FX, SWAP>((x1, y1), (dx, dy), (dx2, dy2), window_min, window_max)
        else {
            return None;
        };
        let end =
            kuzmin::exit::<FY, FX, SWAP>((x1, y1), (x2, y2), (dx, dy), (dx2, dy2), window_max);
        Some(Self {
            x1,
            y1,
            dx2,
            dy2,
            error,
            end,
        })
    }

    #[inline(always)]
    #[must_use]
    const fn raw_unchecked(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (dx, dy): (isize, isize),
    ) -> Self {
        let (dx2, dy2) = (dx << 1, dy << 1);
        let error = if SWAP { dx2 - dy } else { dy2 - dx };
        let end = if SWAP { y2 } else { x2 };
        Self {
            x1,
            y1,
            dx2,
            dy2,
            error,
            end,
        }
    }

    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        match SWAP {
            true => match FY {
                true => self.y1 <= self.end,
                false => self.end <= self.y1,
            },
            false => match FX {
                true => self.x1 <= self.end,
                false => self.end <= self.x1,
            },
        }
    }
}

impl<const FY: bool, const FX: bool, const SWAP: bool> Iterator for Octant<FY, FX, SWAP> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let (x, y) = (self.x1, self.y1);
        if self.error >= 0 {
            match SWAP {
                true => self.x1 += if FX { -1 } else { 1 },
                false => self.y1 += if FY { -1 } else { 1 },
            }
            self.error -= if SWAP { self.dy2 } else { self.dx2 };
        }
        match SWAP {
            true => self.y1 += if FY { -1 } else { 1 },
            false => self.x1 += if FX { -1 } else { 1 },
        }
        self.error += if SWAP { self.dx2 } else { self.dy2 };
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // slightly optimized over `isize::abs_diff`,
        // see its implementation for the proof that this cast is legal
        #[allow(clippy::cast_sign_loss)]
        let length = match SWAP {
            true => match FY {
                true => usize::wrapping_sub(self.y1 as usize, self.end as usize),
                false => usize::wrapping_sub(self.end as usize, self.y1 as usize),
            },
            false => match FX {
                true => usize::wrapping_sub(self.x1 as usize, self.end as usize),
                false => usize::wrapping_sub(self.end as usize, self.x1 as usize),
            },
        };
        (length, Some(length))
    }
}

impl<const FY: bool, const FX: bool, const SWAP: bool> ExactSizeIterator for Octant<FY, FX, SWAP> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_done()
    }
}

impl<const FY: bool, const FX: bool, const SWAP: bool> core::iter::FusedIterator
    for Octant<FY, FX, SWAP>
{
}

/// Iterator over a directed line segment backed by [Bresenham's algorithm][1].
///
/// Contains specialized sub-iterators, and picks the appropriate variant
/// based on the orientation and direction of the line segment.
///
/// **Note**: an optimized implementation of [`Bresenham::fold`] is provided.
/// This makes [`Bresenham::for_each`] faster than a `for` loop, since it checks
/// the iteration octant only once instead of on every call to [`Bresenham::next`].
///
/// [1]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Bresenham {
    /// Optimized [`axis_aligned::Horizontal`] case.
    Horizontal(axis_aligned::Horizontal),
    /// Optimized [`axis_aligned::Vertical`] case.
    Vertical(axis_aligned::Vertical),
    /// Optimized diagonal [`diagonal::Quadrant0`] case.
    #[cfg(feature = "bresenham_diagonals")]
    Diagonal0(diagonal::Quadrant0),
    /// Optimized diagonal [`diagonal::Quadrant1`] case.
    #[cfg(feature = "bresenham_diagonals")]
    Diagonal1(diagonal::Quadrant1),
    /// Optimized diagonal [`diagonal::Quadrant2`] case.
    #[cfg(feature = "bresenham_diagonals")]
    Diagonal2(diagonal::Quadrant2),
    /// Optimized diagonal [`diagonal::Quadrant3`] case.
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
            Self::Horizontal($me) => $call,
            Self::Vertical($me) => $call,
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
    /// Constructs a [`Bresenham`] iterator from `(x1, y1)` to `(x2, y2)`, *exclusive*.
    ///
    /// **Note:** if you know the kind of the line segment beforehand,
    /// it's recommended to construct the relevant variant directly,
    /// since it saves an extra check.
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Self {
        if y1 == y2 {
            return Self::Horizontal(axis_aligned::Horizontal::new(y1, x1, x2));
        }
        if x1 == x2 {
            return Self::Vertical(axis_aligned::Vertical::new(x1, y1, y2));
        }
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dy {
            if 0 < dx {
                if dy < dx {
                    return Self::Octant0(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)));
                }
                #[cfg(feature = "bresenham_diagonals")]
                if dy == dx {
                    return Self::Diagonal0(diagonal::Quadrant::new_unchecked((x1, y1), (x2, y2)));
                }
                Self::Octant1(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)))
            } else {
                let dx = -dx;
                if dy < dx {
                    return Self::Octant2(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)));
                }
                #[cfg(feature = "bresenham_diagonals")]
                if dy == dx {
                    return Self::Diagonal1(diagonal::Quadrant::new_unchecked((x1, y1), (x2, y2)));
                }
                Self::Octant3(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)))
            }
        } else {
            let dy = -dy;
            if 0 < dx {
                if dy < dx {
                    return Self::Octant4(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)));
                }
                #[cfg(feature = "bresenham_diagonals")]
                if dy == dx {
                    return Self::Diagonal2(diagonal::Quadrant::new_unchecked((x1, y1), (x2, y2)));
                }
                Self::Octant5(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)))
            } else {
                let dx = -dx;
                if dy < dx {
                    return Self::Octant6(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)));
                }
                #[cfg(feature = "bresenham_diagonals")]
                if dy == dx {
                    return Self::Diagonal3(diagonal::Quadrant::new_unchecked((x1, y1), (x2, y2)));
                }
                Self::Octant7(Octant::raw_unchecked((x1, y1), (x2, y2), (dx, dy)))
            }
        }
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
