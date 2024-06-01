//! Bresenham iterators.

use crate::{
    AxisAligned, Horizontal, NegativeHorizontal, NegativeVertical, Point, PositiveHorizontal,
    PositiveVertical, Vertical,
};

/// Line segment iterator backed by one of the eight octants of
/// [Bresenham's algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
///
/// An octant is one of the eight sectors of a Cartesian plane
/// spanning an angle of 45 degrees.
///
/// The generic parameters represent symmetries relative to the first octant:
/// - `FY`: flip the `y` coordinate if `true`.
/// - `FX`: flip the `x` coordinate if `true`.
/// - `SWAP`: swap the `x` and `y` coordinates if `true`.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Octant<const FY: bool, const FX: bool, const SWAP: bool> {
    x: isize,
    y: isize,
    dx2: isize,
    dy2: isize,
    error: isize,
    // TODO: store both end coordinates and impl DoubleEndedIterator.
    end: isize,
}

impl<const FY: bool, const FX: bool, const SWAP: bool> Iterator for Octant<FY, FX, SWAP> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.terminated() {
            return None;
        }
        let (x, y) = (self.x, self.y);
        if self.error >= 0 {
            match SWAP {
                true => self.x += if FX { -1 } else { 1 },
                false => self.y += if FY { -1 } else { 1 },
            }
            self.error -= self.dx2;
        }
        match SWAP {
            true => self.y += if FY { -1 } else { 1 },
            false => self.x += if FX { -1 } else { 1 },
        }
        self.error += self.dy2;
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = match SWAP {
            true => isize::abs_diff(self.y, self.end),
            false => isize::abs_diff(self.x, self.end),
        };
        (length, Some(length))
    }
}

impl<const FY: bool, const FX: bool, const SWAP: bool> Octant<FY, FX, SWAP> {
    /// Creates an [`Octant`] instance if the line segment
    /// defined by the given endpoints belongs to the octant.
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
        Some(Self::new_unchecked((x1, y1), (x2, y2), (dx, dy)))
    }

    #[inline(always)]
    #[must_use]
    const fn new_unchecked(
        (x1, y1): Point<isize>,
        (x2, y2): Point<isize>,
        (dx, dy): (isize, isize),
    ) -> Self {
        let (dx, dy) = if SWAP { (dy, dx) } else { (dx, dy) };
        // TODO: check overflow.
        let (dx2, dy2) = (dx << 1, dy << 1);
        let error = dy2 - dx;
        let end = if SWAP { y2 } else { x2 };
        Self {
            x: x1,
            y: y1,
            dx2,
            dy2,
            error,
            end,
        }
    }

    /// Checks if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn terminated(&self) -> bool {
        match SWAP {
            true => match FY {
                true => self.y <= self.end,
                false => self.end <= self.y,
            },
            false => match FX {
                true => self.x <= self.end,
                false => self.end <= self.x,
            },
        }
    }
}

/// Special case of the [`Octant`] iterator representing the first octant.
///
/// This octant covers the sector where both `x` and `y` are increasing,
/// with `x` changing faster than `y` (gentle slope, covers `(0°, 45°)`).
pub type Octant0 = Octant<false, false, false>;

/// Special case of the [`Octant`] iterator representing the second octant.
///
/// This octant covers the sector where both `x` and `y` are
/// increasing, with `y` changing faster than `x` (steep slope, covers `[45°, 90°)`).
///
/// Can be obtained from [`Octant0`] by swapping the `x` and `y` coordinates.
pub type Octant1 = Octant<false, false, true>;

/// Special case of the [`Octant`] iterator representing the third octant.
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// increasing, with `x` changing faster than `y` (gentle slope, covers `(135°, 180°)`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate.
pub type Octant2 = Octant<false, true, false>;

/// Special case of the [`Octant`] iterator representing the fourth octant.
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// increasing, with `y` changing faster than `x` (steep slope, covers `[90°, 135°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant3 = Octant<false, true, true>;

/// Special case of the [`Octant`] iterator representing the fifth octant.
///
/// This octant covers the sector where `x` is increasing and `y` is
/// decreasing, with `x` changing faster than `y` (gentle slope, covers `(315°, 360°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate.
pub type Octant4 = Octant<true, false, false>;

/// Special case of the [`Octant`] iterator representing the sixth octant.
///
/// This octant covers the sector where `x` is increasing and `y` is
/// decreasing, with `y` changing faster than `x` (steep slope, covers `(270°, 315°]`).
///
/// Can be obtained from [`Octant0`] by flipping the `y` coordinate,
/// and swapping the `x` and `y` coordinates.
pub type Octant5 = Octant<true, false, true>;

/// Special case of the [`Octant`] iterator representing the seventh octant.
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// decreasing, with `x` changing faster than `y` (gentle slope, covers `[180°, 225°)`).
///
/// Can be obtained from [`Octant0`] by flipping the `x` and `y` coordinates.
pub type Octant6 = Octant<true, true, false>;

/// Special case of the [`Octant`] iterator representing the eighth octant.
///
/// This octant covers the sector where `x` is decreasing and `y` is
/// decreasing, with `y` changing faster than `x` (steep slope, covers `[225°, 270°]`).
///
/// Can be obtained from [`Octant0`] by flipping and swapping the `x` and `y` coordinates.
pub type Octant7 = Octant<true, true, true>;

impl<const FY: bool, const FX: bool, const SWAP: bool> ExactSizeIterator for Octant<FY, FX, SWAP> {}

impl<const FY: bool, const FX: bool, const SWAP: bool> core::iter::FusedIterator
    for Octant<FY, FX, SWAP>
{
}

/// Line segment iterator backed by
/// [Bresenham's algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
///
/// This enum encapsulates different variants of line segment iterators
/// corresponding to the eight [`Octant`]s defined in Bresenham's algorithm,
/// as well as the [`AxisAligned`] cases. The appropriate variant
/// is chosen to efficiently generate points along the line segment.
///
/// **Note:** if you know the slope of the line segment beforehand,
/// it's recommended to directly construct the relevant variant.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Bresenham {
    /// See [`PositiveHorizontal`].
    ///
    /// Such lines are covered by [`Octant4`], but this implementation is faster.
    PositiveHorizontal(PositiveHorizontal),
    /// See [`NegativeHorizontal`].
    ///
    /// Such lines are covered by [`Octant6`], but this implementation is faster.
    NegativeHorizontal(NegativeHorizontal),
    /// See [`PositiveVertical`].
    ///
    /// Such lines are covered by [`Octant3`], but this implementation is faster.
    PositiveVertical(PositiveVertical),
    /// See [`NegativeVertical`].
    ///
    /// Such lines are covered by [`Octant7`], but this implementation is faster.
    NegativeVertical(NegativeVertical),
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

impl Bresenham {
    /// Constructs a new [`Bresenham`] iterator from the given endpoints.
    ///
    /// Determines the [`Octant`] of the line segment
    /// and initializes the appropriate variant.
    #[must_use]
    pub const fn new((x1, y1): Point<isize>, (x2, y2): Point<isize>) -> Self {
        if y1 == y2 {
            return match Horizontal::new(x1, y1, y2) {
                AxisAligned::Positive(me) => Self::PositiveHorizontal(me),
                AxisAligned::Negative(me) => Self::NegativeHorizontal(me),
            };
        }
        if x1 == x2 {
            return match Vertical::new(x1, y1, y2) {
                AxisAligned::Positive(me) => Self::PositiveVertical(me),
                AxisAligned::Negative(me) => Self::NegativeVertical(me),
            };
        }
        // TODO: check overflow.
        let (dx, dy) = (x2 - x1, y2 - y1);
        if 0 < dy {
            if 0 < dx {
                match dy < dx {
                    true => Self::Octant0(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                    false => Self::Octant1(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                }
            } else {
                let dx = -dx;
                match dy < dx {
                    true => Self::Octant2(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                    false => Self::Octant3(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                }
            }
        } else {
            let dy = -dy;
            if 0 < dx {
                match dy < dx {
                    true => Self::Octant4(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                    false => Self::Octant5(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                }
            } else {
                let dx = -dx;
                match dy < dx {
                    true => Self::Octant6(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                    false => Self::Octant7(Octant::new_unchecked((x1, y1), (x2, y2), (dx, dy))),
                }
            }
        }
    }

    /// Checks if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn terminated(&self) -> bool {
        match self {
            Self::PositiveHorizontal(me) => me.terminated(),
            Self::NegativeHorizontal(me) => me.terminated(),
            Self::PositiveVertical(me) => me.terminated(),
            Self::NegativeVertical(me) => me.terminated(),
            Self::Octant0(me) => me.terminated(),
            Self::Octant1(me) => me.terminated(),
            Self::Octant2(me) => me.terminated(),
            Self::Octant3(me) => me.terminated(),
            Self::Octant4(me) => me.terminated(),
            Self::Octant5(me) => me.terminated(),
            Self::Octant6(me) => me.terminated(),
            Self::Octant7(me) => me.terminated(),
        }
    }
}

impl Iterator for Bresenham {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::PositiveHorizontal(me) => me.next(),
            Self::NegativeHorizontal(me) => me.next(),
            Self::PositiveVertical(me) => me.next(),
            Self::NegativeVertical(me) => me.next(),
            Self::Octant0(me) => me.next(),
            Self::Octant1(me) => me.next(),
            Self::Octant2(me) => me.next(),
            Self::Octant3(me) => me.next(),
            Self::Octant4(me) => me.next(),
            Self::Octant5(me) => me.next(),
            Self::Octant6(me) => me.next(),
            Self::Octant7(me) => me.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::PositiveHorizontal(me) => me.size_hint(),
            Self::NegativeHorizontal(me) => me.size_hint(),
            Self::PositiveVertical(me) => me.size_hint(),
            Self::NegativeVertical(me) => me.size_hint(),
            Self::Octant0(me) => me.size_hint(),
            Self::Octant1(me) => me.size_hint(),
            Self::Octant2(me) => me.size_hint(),
            Self::Octant3(me) => me.size_hint(),
            Self::Octant4(me) => me.size_hint(),
            Self::Octant5(me) => me.size_hint(),
            Self::Octant6(me) => me.size_hint(),
            Self::Octant7(me) => me.size_hint(),
        }
    }

    #[cfg(feature = "try_fold")]
    #[inline]
    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: core::ops::Try<Output = B>,
    {
        match self {
            Self::PositiveHorizontal(me) => me.try_fold(init, f),
            Self::NegativeHorizontal(me) => me.try_fold(init, f),
            Self::PositiveVertical(me) => me.try_fold(init, f),
            Self::NegativeVertical(me) => me.try_fold(init, f),
            Self::Octant0(me) => me.try_fold(init, f),
            Self::Octant1(me) => me.try_fold(init, f),
            Self::Octant2(me) => me.try_fold(init, f),
            Self::Octant3(me) => me.try_fold(init, f),
            Self::Octant4(me) => me.try_fold(init, f),
            Self::Octant5(me) => me.try_fold(init, f),
            Self::Octant6(me) => me.try_fold(init, f),
            Self::Octant7(me) => me.try_fold(init, f),
        }
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self {
            Self::PositiveHorizontal(me) => me.fold(init, f),
            Self::NegativeHorizontal(me) => me.fold(init, f),
            Self::PositiveVertical(me) => me.fold(init, f),
            Self::NegativeVertical(me) => me.fold(init, f),
            Self::Octant0(me) => me.fold(init, f),
            Self::Octant1(me) => me.fold(init, f),
            Self::Octant2(me) => me.fold(init, f),
            Self::Octant3(me) => me.fold(init, f),
            Self::Octant4(me) => me.fold(init, f),
            Self::Octant5(me) => me.fold(init, f),
            Self::Octant6(me) => me.fold(init, f),
            Self::Octant7(me) => me.fold(init, f),
        }
    }
}

impl<const FY: bool, const FX: bool, const SWAP: bool> TryFrom<Bresenham> for Octant<FY, FX, SWAP> {
    type Error = Bresenham;

    #[inline]
    fn try_from(value: Bresenham) -> Result<Self, Self::Error> {
        #[rustfmt::skip]
        macro_rules! try_from_bresenham_case {
            ($value:ident, $octant:ident) => {
                if let Bresenham::$octant(Octant { x, y, dx2, dy2, error, end: term, }) = $value {
                    return Ok(Self { x, y, dx2, dy2, error, end:term, });
                }
            };
        }
        match (FY, FX, SWAP) {
            (false, false, false) => try_from_bresenham_case!(value, Octant0),
            (false, false, true) => try_from_bresenham_case!(value, Octant1),
            (false, true, false) => try_from_bresenham_case!(value, Octant2),
            (false, true, true) => try_from_bresenham_case!(value, Octant3),
            (true, false, false) => try_from_bresenham_case!(value, Octant4),
            (true, false, true) => try_from_bresenham_case!(value, Octant5),
            (true, true, false) => try_from_bresenham_case!(value, Octant6),
            (true, true, true) => try_from_bresenham_case!(value, Octant7),
        }
        Err(value)
    }
}
