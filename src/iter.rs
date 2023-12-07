use crate::util::{
    bresenham_step, clip_rect_entry, clip_rect_exit, destandardize, standardize, Point,
};
use core::cmp::{max, min};
use core::iter::FusedIterator;
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

/// Enum representing the different variants of clipped line segment iterators.
///
/// This enum allows you to iterate over different types of line segments,
/// such as vertical, horizontal, gently-sloped, and steeply-sloped line segments.
///
/// # Note
///
/// It is recommended to match on this enum and iterate over the variants for less overhead.
/// Iterating over the enum directly will match on every call to [`Iterator::next()`].
///
/// If you specifically want to iterate over vertical or horizontal line segments,
/// see [`Vlipline`] and [`Hlipline`].
///
/// # Example
///
/// ```rust
/// # use clipline::{Clipline, Clipline::*};
/// #
/// # let draw_pixel = |x, y| {};
///
/// let line = ((0, 0), (10, 10)); // inclusive!
/// let clip_rect = ((2, 2), (8, 8)); // inclusive!
///
/// // Iterate over Clipline with indirection (with match overhead on each iteration)
/// for (x, y) in Clipline::new(line, clip_rect).expect("line intersects clip_rect") {
///     draw_pixel(x, y);
/// }
///
/// // Iterate over each case directly (faster, recommended)
/// match Clipline::new(line, clip_rect).unwrap() {
///     Vlipline(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
///     Hlipline(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
///     Gentleham(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
///     Steepnham(pixels) => {
///         for (x, y) in pixels {
///             draw_pixel(x, y);
///         }
///     }
/// }
/// ```
#[derive(Clone, Debug)]
pub enum Clipline<T> {
    Vlipline(Vlipline<T>),
    Hlipline(Hlipline<T>),
    Gentleham(Gentleham<T>),
    Steepnham(Steepnham<T>),
}

/// Iterator for vertical clipped lines.
///
/// This iterator allows you to iterate over a vertical line specified by its x-coordinate,
/// and the starting and ending y-coordinates, within a given clipping rectangle.
///
/// # Example
///
/// ```rust
/// # use clipline::Vlipline;
/// #
/// # let draw_pixel = |x, y| {};
///
/// let (x, y1, y2) = (3, 4, 8); // inclusive!
/// let clip_rect = ((2, 2), (5, 8)); // inclusive!
///
/// // Create a Vlipline and iterate over it.
/// for (x, y) in Vlipline::new(x, y1, y2, clip_rect).expect("line intersects clip_rect") {
///     draw_pixel(x, y);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Vlipline<T> {
    x: T,
    y1: T,
    y2: T,
    sy: T,
}

/// Iterator for horizontal clipped lines.
///
/// This iterator allows you to iterate over a horizontal line specified by its starting
/// and ending x-coordinates, and the y-coordinate, within a given clipping rectangle.
///
/// # Example
///
/// ```rust
/// # use clipline::Hlipline;
/// #
/// # let draw_pixel = |x, y| {};
///
/// let (x1, x2, y) = (2, 6, 3); // inclusive!
/// let clip_rect = ((2, 2), (8, 8)); // inclusive!
///
/// for (x, y) in Hlipline::new(x1, x2, y, clip_rect).expect("line intersects clip_rect") {
///     draw_pixel(x, y)
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Hlipline<T> {
    x1: T,
    x2: T,
    y: T,
    sx: T,
}

/// Iterator for gently-sloped clipped lines.
/// It is created via [`Clipline::new`].
///
/// # Example
///
/// ```rust
/// # use clipline::{Clipline, Clipline::*};
/// #
/// # let draw_pixel = |x, y| {};
///
/// let line = ((2, 2), (6, 4)); // inclusive!
/// let clip_rect = ((2, 2), (8, 8)); // inclusive!
///
/// // Create a Clipline and iterate only if the line is gently-sloped.
/// let clipline = Clipline::new(line, clip_rect).expect("line intersects clip_rect");
/// match clipline {
///     Gentleham(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
///     _ => {}
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Gentleham<T>(Bresenham<T>);

/// Iterator for steeply-sloped clipped lines.
/// It is created via [`Clipline::new`].
///
/// # Example
///
/// ```rust
/// # use clipline::{Clipline, Clipline::*};
/// #
/// # let draw_pixel = |x, y| {};
///
/// let line = ((2, 2), (4, 6));
/// let clip_rect = ((2, 2), (8, 8));
///
/// // Create a Clipline and iterate only if the line is steeply-sloped.
/// match Clipline::new(line, clip_rect).expect("line intersects clip_rect") {
///     Steepnham(pixels) => pixels.for_each(|(x, y)| draw_pixel(x, y)),
///     _ => {}
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Steepnham<T>(Bresenham<T>);

#[derive(Clone, Debug)]
struct Bresenham<T> {
    tx: T,
    ty: T,
    dx2: T,
    dy2: T,
    xd: T,
    yd: T,
    err: T,
    term: T,
}

impl<T> Clipline<T>
where
    T: Copy
        + Ord
        + Neg<Output = T>
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + MulAssign
        + TryFrom<u8>,
{
    /// Creates an appropriate iterator based on the provided line segment and clipping rectangle.
    ///
    /// # Arguments
    ///
    /// * `line`: A tuple representing the endpoints of the line segment.
    /// The line segment will be iterated from start to end, inclusive.
    /// * `clip_rect`: A tuple representing the corners of the clipping rectangle, inclusive.
    /// The line segment will be clipped to this rectangle, and only the visible portion will be iterated.
    ///
    /// # Returns
    ///
    /// If any part of the line segment is visible within the clipping rectangle,
    /// the function returns an [`Option`] containing the appropriate [`Clipline`] variant.
    /// If the line segment is entirely outside the clipping region, the function returns [`None`].
    pub fn new(line: (Point<T>, Point<T>), clip_rect: (Point<T>, Point<T>)) -> Option<Self> {
        let ((x1, y1), (x2, y2)) = line;

        if x1 == x2 {
            return Vlipline::new(x1, y1, y2, clip_rect).map(Self::Vlipline);
        }

        if y1 == y2 {
            return Hlipline::new(x1, x2, y1, clip_rect).map(Self::Hlipline);
        }

        let ((wx1, wy1), (wx2, wy2)) = clip_rect;

        let (tx, x1, x2, wx1, wx2) = standardize(x1, x2, wx1, wx2)?;
        let (ty, y1, y2, wy1, wy2) = standardize(y1, y2, wy1, wy2)?;

        let dx = x2 - x1;
        let dy = y2 - y1;

        // TryFrom instead of From to support i8: https://stackoverflow.com/a/73783390/8707157
        let two = T::try_from(2).unwrap_or_else(|_| unreachable!());

        let (dx2, dy2) = (two * dx, two * dy);

        let bresenham = if dx >= dy {
            let (yd, xd, err) = clip_rect_entry(y1, x1, wy1, wy2, wx1, wx2, dx, dy2, dx2)?;
            let term = clip_rect_exit(y1, y2, x1, x2, wy2, dx, dy2, dx2);
            let (xd, yd, term) = destandardize(term, xd, yd, wx2, tx, ty);
            let dx2 = dx2 - dy2;
            let bresenham = Bresenham::new(tx, ty, dx2, dy2, xd, yd, err, term);
            Self::Gentleham(Gentleham(bresenham))
        } else {
            let (xd, yd, err) = clip_rect_entry(x1, y1, wx1, wx2, wy1, wy2, dy, dx2, dy2)?;
            let term = clip_rect_exit(x1, x2, y1, y2, wx2, dy, dx2, dy2);
            let (yd, xd, term) = destandardize(term, yd, xd, wy2, ty, tx);
            let dy2 = dy2 - dx2;
            let bresenham = Bresenham::new(tx, ty, dx2, dy2, xd, yd, err, term);
            Self::Steepnham(Steepnham(bresenham))
        };
        Some(bresenham)
    }
}

impl<T> Vlipline<T>
where
    T: Ord + Neg<Output = T> + TryFrom<u8>,
{
    /// Creates a vertical clipped line segment iterator.
    ///
    /// This function will return an iterator for a vertical line segment specified by its x-coordinate,
    /// and the starting and ending y-coordinates, clipped to a given rectangle.
    ///
    /// # Arguments
    ///
    /// * `x1`: The x-coordinate of the line segment.
    /// * `y1`: The starting y-coordinate of the line segment.
    /// * `y2`: The ending y-coordinate of the line segment, inclusive.
    ///
    /// * `clip_rect`: A tuple representing the corners of the clipping rectangle, inclusive.
    /// The line segment will be clipped to this rectangle, and only the visible portion will be iterated.
    ///
    /// # Returns
    ///
    /// If any part of the line segment is visible within the clipping rectangle,
    /// the function returns an [`Option`] containing the [`Vlipline`] iterator.
    /// If the line segment is entirely outside the clipping region, the function returns [`None`].
    pub fn new(x: T, y1: T, y2: T, clip_rect: (Point<T>, Point<T>)) -> Option<Self> {
        let ((wx1, wy1), (wx2, wy2)) = clip_rect;
        if x < wx1 || x > wx2 {
            return None;
        }
        let one = T::try_from(1).unwrap_or_else(|_| unreachable!());
        if y1 > y2 {
            if y2 > wy2 || y1 < wy1 {
                return None;
            }
            Some(Self {
                x,
                y1: min(y1, wy2),
                y2: max(y2, wy1),
                sy: -one,
            })
        } else {
            if y1 > wy2 || y2 < wy1 {
                return None;
            }
            Some(Self {
                x,
                y1: max(y1, wy1),
                y2: min(y2, wy2),
                sy: one,
            })
        }
    }
}

impl<T> Hlipline<T>
where
    T: Ord + Neg<Output = T> + TryFrom<u8>,
{
    /// Creates a horizontal clipped line segment iterator.
    ///
    /// This function will return an iterator for a horizontal line segment specified by its starting
    /// and ending x-coordinates, and the y-coordinate, clipped to a given rectangle.
    ///
    /// # Arguments
    ///
    /// * `x1`: The starting x-coordinate of the line segment.
    /// * `x2`: The ending x-coordinate of the line segment, inclusive.
    /// * `y`: The y-coordinate of the line segment.
    ///
    /// * `clip_rect`: A tuple representing the corners of the clipping rectangle, inclusive.
    /// The line segment will be clipped to this rectangle, and only the visible portion will be iterated.
    ///
    /// # Returns
    ///
    /// If any part of the line segment is visible within the clipping rectangle,
    /// the function returns an [`Option`] containing the [`Hlipline`] iterator.
    /// If the line segment is entirely outside the clipping region, the function returns [`None`].
    pub fn new(x1: T, x2: T, y: T, clip_rect: (Point<T>, Point<T>)) -> Option<Self> {
        let ((wx1, wy1), (wx2, wy2)) = clip_rect;
        if y < wy1 || y > wy2 {
            return None;
        }
        let one = T::try_from(1).unwrap_or_else(|_| unreachable!());
        if x1 > x2 {
            if x2 > wx2 || x1 < wx1 {
                return None;
            }
            Some(Self {
                x1: min(x1, wx2),
                x2: max(x2, wx1),
                y,
                sx: -one,
            })
        } else {
            if x1 > wx2 || x2 < wx1 {
                return None;
            }
            Some(Self {
                x1: max(x1, wx1),
                x2: min(x2, wx2),
                y,
                sx: one,
            })
        }
    }
}

impl<T> Bresenham<T> {
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    const fn new(tx: T, ty: T, dx2: T, dy2: T, xd: T, yd: T, err: T, term: T) -> Self {
        Self {
            tx,
            ty,
            dx2,
            dy2,
            xd,
            yd,
            err,
            term,
        }
    }
}

// -----------------------------------------------

impl<T> Iterator for Clipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + TryFrom<u8>
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + TryInto<usize> + Add<Output = <T as AbsDiff>::Output>,
{
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Vlipline(iter) => iter.next(),
            Self::Hlipline(iter) => iter.next(),
            Self::Gentleham(iter) => iter.next(),
            Self::Steepnham(iter) => iter.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Vlipline(iter) => iter.size_hint(),
            Self::Hlipline(iter) => iter.size_hint(),
            Self::Gentleham(iter) => iter.size_hint(),
            Self::Steepnham(iter) => iter.size_hint(),
        }
    }
}

impl<T> Iterator for Vlipline<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + AddAssign + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + TryInto<usize> + Add<Output = <T as AbsDiff>::Output>,
{
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.y1 * self.sy > self.y2 * self.sy {
            return None;
        }
        let (x, y) = (self.x, self.y1);
        self.y1 += self.sy;
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let one = <<T as AbsDiff>::Output>::try_from(1).unwrap_or_else(|_| unreachable!());
        if let Ok(len) = (T::abs_diff(self.y1, self.y2) + one).try_into() {
            (len, Some(len))
        } else {
            (0, None)
        }
    }
}

impl<T> Iterator for Hlipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + TryInto<usize> + Add<Output = <T as AbsDiff>::Output>,
{
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x1 * self.sx > self.x2 * self.sx {
            return None;
        }
        let (x, y) = (self.x1, self.y);
        self.x1 += self.sx;
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let one = <T as AbsDiff>::Output::try_from(1).unwrap_or_else(|_| unreachable!());
        if let Ok(len) = (T::abs_diff(self.x1, self.x2) + one).try_into() {
            (len, Some(len))
        } else {
            (0, None)
        }
    }
}

impl<T> Iterator for Gentleham<T>
where
    T: Copy + Ord + Sub<Output = T> + AddAssign + SubAssign + TryFrom<u8> + AbsDiff,
    <T as AbsDiff>::Output: TryInto<usize>,
{
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self(b) = self;
        if b.xd == b.term {
            return None;
        }
        let (x, y) = (b.xd, b.yd);
        (b.err, b.xd, b.yd) = bresenham_step(b.err, b.xd, b.yd, b.tx, b.ty, b.dx2, b.dy2);
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let xd = self.0.xd;
        let term = self.0.term;
        if let Ok(len) = T::abs_diff(xd, term).try_into() {
            (len, Some(len))
        } else {
            (0, None)
        }
    }
}

impl<T> Iterator for Steepnham<T>
where
    T: Copy + Ord + Sub<Output = T> + AddAssign + SubAssign + TryFrom<u8> + AbsDiff,
    <T as AbsDiff>::Output: TryInto<usize>,
{
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self(b) = self;
        if b.yd == b.term {
            return None;
        }
        let (x, y) = (b.xd, b.yd);
        (b.err, b.yd, b.xd) = bresenham_step(b.err, b.yd, b.xd, b.ty, b.tx, b.dy2, b.dx2);
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) where {
        let yd = self.0.yd;
        let term = self.0.term;
        if let Ok(len) = T::abs_diff(yd, term).try_into() {
            (len, Some(len))
        } else {
            (0, None)
        }
    }
}

// -----------------------------------------------

impl<T> DoubleEndedIterator for Vlipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + Into<usize> + Add<Output = <T as AbsDiff>::Output>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y1 * self.sy > self.y2 * self.sy {
            return None;
        }
        let (x, y) = (self.x, self.y2);
        self.y2 -= self.sy;
        Some((x, y))
    }
}

impl<T> DoubleEndedIterator for Hlipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + Into<usize> + Add<Output = <T as AbsDiff>::Output>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x1 * self.sx > self.x2 * self.sx {
            return None;
        }
        let (x, y) = (self.x2, self.y);
        self.x2 -= self.sx;
        Some((x, y))
    }
}

// -----------------------------------------------

impl<T> ExactSizeIterator for Clipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + TryFrom<u8>
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + Into<usize> + Add<Output = <T as AbsDiff>::Output>,
{
}
impl<T> ExactSizeIterator for Vlipline<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + AddAssign + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + Into<usize> + Add<Output = <T as AbsDiff>::Output>,
{
}
impl<T> ExactSizeIterator for Hlipline<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + AddAssign + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + Into<usize> + Add<Output = <T as AbsDiff>::Output>,
{
}
impl<T> ExactSizeIterator for Gentleham<T>
where
    T: Copy
        + Ord
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + TryFrom<u8>
        + AbsDiff,
    <T as AbsDiff>::Output: Into<usize>,
{
}
impl<T> ExactSizeIterator for Steepnham<T>
where
    T: Copy
        + Ord
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + TryFrom<u8>
        + AbsDiff,
    <T as AbsDiff>::Output: Into<usize>,
{
}

// -----------------------------------------------

impl<T> FusedIterator for Clipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + TryFrom<u8>
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + TryInto<usize> + Add<Output = <T as AbsDiff>::Output>,
{
}
impl<T> FusedIterator for Vlipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + TryInto<usize> + Add<Output = <T as AbsDiff>::Output>,
{
}
impl<T> FusedIterator for Hlipline<T>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + AbsDiff,
    <T as AbsDiff>::Output: TryFrom<u8> + TryInto<usize> + Add<Output = <T as AbsDiff>::Output>,
{
}
impl<T> FusedIterator for Gentleham<T>
where
    T: Copy
        + Ord
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + TryFrom<u8>
        + AbsDiff,
    <T as AbsDiff>::Output: TryInto<usize>,
{
}
impl<T> FusedIterator for Steepnham<T>
where
    T: Copy
        + Ord
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + TryFrom<u8>
        + AbsDiff,
    <T as AbsDiff>::Output: TryInto<usize>,
{
}

// -----------------------------------------------

/// The absolute difference operation.
trait AbsDiff<Rhs = Self> {
    /// The resulting type after applying the `+` operator.
    type Output;

    /// Computes the absolute difference between `self` and `other`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs_diff(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_abs_diff {
    ($signed:ty, $unsigned:ty) => {
        impl AbsDiff for $signed {
            type Output = $unsigned;

            fn abs_diff(self, rhs: Self) -> Self::Output {
                <$signed>::abs_diff(self, rhs)
            }
        }
    };
}

impl_abs_diff!(i8, u8);
impl_abs_diff!(i16, u16);
impl_abs_diff!(i32, u32);
impl_abs_diff!(i64, u64);
impl_abs_diff!(i128, u128);
impl_abs_diff!(isize, usize);

impl_abs_diff!(u8, u8);
impl_abs_diff!(u16, u16);
impl_abs_diff!(u32, u32);
impl_abs_diff!(u64, u64);
impl_abs_diff!(u128, u128);
impl_abs_diff!(usize, usize);

// -----------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    type Pnt = (i8, i8);

    fn test_line_scenario(
        line: (Pnt, Pnt),
        clip_rect: (Pnt, Pnt),
        actual_points: &mut [Pnt],
    ) -> usize {
        let mut num_points = 0;
        if let Some(cline) = Clipline::new(line, clip_rect) {
            for (i, p) in cline.enumerate() {
                actual_points[i] = p;
                num_points += 1;
            }
        }
        num_points
    }

    #[test]
    fn test_vertical_line_outside_clip_rect() {
        let line = ((0, 0), (0, 5));
        let clip_rect = ((2, 2), (2, 4));

        let mut actual_points = [];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[]);
    }

    #[test]
    fn test_vertical_line_inside_clip_rect() {
        let line = ((2, 2), (2, 4));
        let clip_rect = ((0, 0), (5, 5));

        let mut actual_points = [(0, 0); 3];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[(2, 2), (2, 3), (2, 4)]);
    }

    #[test]
    fn test_vertical_line_intersecting_clip_rect() {
        let line = ((1, 1), (1, 6));
        let clip_rect = ((0, 0), (2, 4));

        let mut actual_points = [(0, 0); 4];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            &[(1, 1), (1, 2), (1, 3), (1, 4)]
        );
    }

    #[test]
    fn test_horizontal_line_outside_clip_rect() {
        let line = ((0, 0), (5, 0));
        let clip_rect = ((2, 2), (4, 2));

        let mut actual_points = [];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[]);
    }

    #[test]
    fn test_horizontal_line_inside_clip_rect() {
        let line = ((2, 2), (4, 2));
        let clip_rect = ((0, 0), (5, 5));

        let mut actual_points = [(0, 0); 3];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[(2, 2), (3, 2), (4, 2)]);
    }

    #[test]
    fn test_horizontal_line_intersecting_clip_rect() {
        let line = ((1, 1), (6, 1));
        let clip_rect = ((0, 0), (4, 2));

        let mut actual_points = [(0, 0); 4];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            &[(1, 1), (2, 1), (3, 1), (4, 1)]
        );
    }

    #[test]
    fn test_gentle_slope_positive_line_outside_rect() {
        let line = ((0, 0), (5, 2));
        let clip_rect = ((0, 3), (2, 5));

        let mut actual_points = [];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[]);
    }

    #[test]
    fn test_gentle_slope_negative_line_outside_rect() {
        let line = ((5, 2), (0, 0));
        let clip_rect = ((2, 5), (4, 6));

        let mut actual_points = [];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[]);
    }

    #[test]
    fn test_steep_slope_positive_line_outside_rect() {
        let line = ((0, 0), (2, 5));
        let clip_rect = ((3, 0), (4, 5));

        let mut actual_points = [];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[]);
    }

    #[test]
    fn test_steep_slope_negative_line_outside_rect() {
        let line = ((2, 5), (0, 0));
        let clip_rect = ((3, 0), (4, 1));

        let mut actual_points = [];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &[]);
    }

    #[test]
    fn test_vertical_bottom_up_line_order() {
        let line = ((0, 0), (0, 5));
        let clip_rect = ((0, 0), (0, 5));

        let mut actual_points = [(0, 0); 6];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            &[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]
        );
    }

    #[test]
    fn test_vertical_top_down_line_order() {
        let line = ((0, 5), (0, 0));
        let clip_rect = ((0, 1), (0, 4));

        let mut actual_points = [(0, 0); 4];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            &[(0, 4), (0, 3), (0, 2), (0, 1)]
        );
    }

    #[test]
    fn test_horizontal_left_right_line_order() {
        let line = ((-1, 0), (5, 0));
        let clip_rect = ((-2, 0), (4, 0));

        let mut actual_points = [(0, 0); 6];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            [(-1, 0), (0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
    }

    #[test]
    fn test_horizontal_right_left_line_order() {
        let line = ((6, 0), (-2, 0));
        let clip_rect = ((-1, 0), (5, 0));

        let mut actual_points = [(0, 0); 7];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            [(5, 0), (4, 0), (3, 0), (2, 0), (1, 0), (0, 0), (-1, 0)]
        );
    }

    #[test]
    fn test_gentle_slope_positive_line_order() {
        let line = ((0, 0), (5, 2));
        let clip_rect = ((1, 0), (5, 2));

        let mut actual_points = [(0, 0); 5];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            &[(1, 0), (2, 1), (3, 1), (4, 2), (5, 2)]
        );
    }

    #[test]
    fn test_gentle_slope_negative_line_order() {
        let line = ((5, 2), (0, 0));
        let clip_rect = ((0, 0), (5, 2));

        let mut actual_points = [(0, 0); 6];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            &[(5, 2), (4, 2), (3, 1), (2, 1), (1, 0), (0, 0)]
        );
    }

    #[test]
    fn test_steep_slope_positive_line_order() {
        let line = ((0, 0), (2, 5));
        let clip_rect = ((0, 0), (2, 5));

        let expected_points = [(0, 0), (0, 1), (1, 2), (1, 3), (2, 4), (2, 5)];
        let mut actual_points = [(0, 0); 6];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(&actual_points[..num_points], &expected_points);
    }

    #[test]
    fn test_steep_slope_negative_line_order() {
        let line = ((2, 5), (0, 0));
        let clip_rect = ((0, 0), (2, 5));

        let mut actual_points = [(0, 0); 6];
        let num_points = test_line_scenario(line, clip_rect, &mut actual_points);

        assert_eq!(
            &actual_points[..num_points],
            &[(2, 5), (2, 4), (1, 3), (1, 2), (0, 1), (0, 0)]
        );
    }

    #[test]
    fn test_size_hint_horizontal() {
        let clip = Hlipline::<i8>::new(0, 9, 0, ((0, 0), (10, 10))).unwrap();
        assert_eq!(clip.size_hint(), (10, Some(10)));
        assert_eq!(clip.len(), 10);
        let mut count = 0;
        clip.for_each(|_| count += 1);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_size_hint_vertical() {
        let clip = Vlipline::<i8>::new(0, 0, 9, ((0, 0), (10, 10))).unwrap();
        assert_eq!(clip.size_hint(), (10, Some(10)));
        assert_eq!(clip.len(), 10);
        let mut count = 0;
        clip.for_each(|_| count += 1);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_size_hint_steep() {
        let clip = Clipline::<i8>::new(((0, 0), (9, 3)), ((0, 0), (10, 10))).unwrap();
        assert_eq!(clip.size_hint(), (10, Some(10)));
        assert_eq!(clip.len(), 10);
        let mut count = 0;
        clip.for_each(|_| count += 1);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_size_hint_gentle() {
        let clip = Clipline::<i8>::new(((0, 0), (8, 9)), ((0, 0), (10, 10))).unwrap();
        assert_eq!(clip.size_hint(), (10, Some(10)));
        assert_eq!(clip.len(), 10);
        let mut count = 0;
        clip.for_each(|_| count += 1);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_double_ended_vertical() {
        let mut clip = Vlipline::<i8>::new(0, 0, 3, ((0, 0), (3, 3))).unwrap();
        assert_eq!(clip.next_back(), Some((0, 3)));
        assert_eq!(clip.next_back(), Some((0, 2)));
        assert_eq!(clip.next(), Some((0, 0)));
        assert_eq!(clip.next(), Some((0, 1)));
        assert_eq!(clip.next_back(), None);
        assert_eq!(clip.next(), None);
    }

    #[test]
    fn test_double_ended_horizontal() {
        let mut clip = Hlipline::<i8>::new(0, 3, 0, ((0, 0), (3, 3))).unwrap();
        assert_eq!(clip.next_back(), Some((3, 0)));
        assert_eq!(clip.next_back(), Some((2, 0)));
        assert_eq!(clip.next(), Some((0, 0)));
        assert_eq!(clip.next(), Some((1, 0)));
        assert_eq!(clip.next_back(), None);
        assert_eq!(clip.next(), None);
    }

    #[test]
    fn test_all_signed_integers() {
        let points: [(isize, isize); 2] = [(0, 0), (1, 1)];
        fn assert(
            points: [(isize, isize); 2],
            x: impl TryInto<isize> + Sized,
            y: impl TryInto<isize> + Sized,
        ) {
            assert!(points.contains(&(
                x.try_into().unwrap_or_else(|_| unreachable!()),
                y.try_into().unwrap_or_else(|_| unreachable!())
            )))
        }
        Clipline::<i8>::new(((0, 0), (1, 1)), ((0, 0), (1, 1)))
            .unwrap_or_else(|| unreachable!())
            .for_each(|(x, y)| assert(points, x, y));
        Clipline::<i16>::new(((0, 0), (1, 1)), ((0, 0), (1, 1)))
            .unwrap_or_else(|| unreachable!())
            .for_each(|(x, y)| assert(points, x, y));
        Clipline::<i32>::new(((0, 0), (1, 1)), ((0, 0), (1, 1)))
            .unwrap_or_else(|| unreachable!())
            .for_each(|(x, y)| assert(points, x, y));
        Clipline::<i64>::new(((0, 0), (1, 1)), ((0, 0), (1, 1)))
            .unwrap_or_else(|| unreachable!())
            .for_each(|(x, y)| assert(points, x, y));
        Clipline::<i128>::new(((0, 0), (1, 1)), ((0, 0), (1, 1)))
            .unwrap_or_else(|| unreachable!())
            .for_each(|(x, y)| assert(points, x, y));
        Clipline::<isize>::new(((0, 0), (1, 1)), ((0, 0), (1, 1)))
            .unwrap_or_else(|| unreachable!())
            .for_each(|(x, y)| assert(points, x, y));
    }
}
