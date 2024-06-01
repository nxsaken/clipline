//! Axis-aligned line segment iterators.

use crate::Point;

/// Iterator over an axis-aligned line segment in a given direction.
///
/// The generic parameters represent the line segment's orientation and direction:
/// - `VERT`: vertical if `true`, horizontal otherwise.
/// - `FLIP`: negative if `true`, positive otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SignedAxisAligned<const VERT: bool, const FLIP: bool> {
    u: isize,
    v1: isize,
    v2: isize,
}

/// Iterator over a horizontal line segment in the negative direction.
pub type NegativeHorizontal = SignedAxisAligned<false, true>;
/// Iterator over a horizontal line segment in the positive direction.
pub type PositiveHorizontal = SignedAxisAligned<false, false>;
/// Iterator over a vertical line segment in the negative direction.
pub type NegativeVertical = SignedAxisAligned<true, true>;
/// Iterator over a vertical line segment in the positive direction.
pub type PositiveVertical = SignedAxisAligned<true, false>;

impl<const VERT: bool, const FLIP: bool> SignedAxisAligned<VERT, FLIP> {
    /// Creates a new iterator over a [`SignedAxisAligned`] line segment.
    ///
    /// ## Arguments
    /// * `u` - the fixed coordinate (`x` for vertical lines, `y` for horizontal).
    /// * `v1`, `v2` - the starting and ending *(exclusive)* coordinates along the variable axis.
    #[inline]
    #[must_use]
    pub const fn new(u: isize, v1: isize, v2: isize) -> Self {
        Self { u, v1, v2 }
    }

    /// Checks if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn terminated(&self) -> bool {
        match FLIP {
            true => self.v1 <= self.v2,
            false => self.v2 <= self.v1,
        }
    }
}

impl<const VERT: bool, const FLIP: bool> Iterator for SignedAxisAligned<VERT, FLIP> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.terminated() {
            return None;
        }
        let (x, y) = match VERT {
            true => (self.u, self.v1),
            false => (self.v1, self.u),
        };
        self.v1 += if FLIP { -1 } else { 1 };
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = isize::abs_diff(self.v1, self.v2);
        (length, Some(length))
    }
}

impl<const VERT: bool, const FLIP: bool> DoubleEndedIterator for SignedAxisAligned<VERT, FLIP> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.terminated() {
            return None;
        }
        self.v2 -= if FLIP { -1 } else { 1 };
        let (x, y) = match VERT {
            true => (self.u, self.v2),
            false => (self.v2, self.u),
        };
        Some((x, y))
    }
}

impl<const VERT: bool, const FLIP: bool> ExactSizeIterator for SignedAxisAligned<VERT, FLIP> {
    #[cfg(feature = "is_empty")]
    #[inline]
    fn is_empty(&self) -> bool {
        self.terminated()
    }
}

impl<const VERT: bool, const FLIP: bool> core::iter::FusedIterator
    for SignedAxisAligned<VERT, FLIP>
{
}

/// Iterator over an axis-aligned line segment.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AxisAligned<const VERT: bool> {
    /// Iterator with a positive step.
    ///
    /// See [`SignedAxisAligned`].
    Positive(SignedAxisAligned<VERT, false>),
    /// Iterator with a negative step.
    ///
    /// See [`SignedAxisAligned`].
    Negative(SignedAxisAligned<VERT, true>),
}

/// Iterator over a horizontal [`AxisAligned`] line segment.
pub type Horizontal = AxisAligned<false>;
/// Iterator over a vertical [`AxisAligned`] line segment.
pub type Vertical = AxisAligned<true>;

impl<const VERT: bool> AxisAligned<VERT> {
    /// Creates a new iterator over an [`AxisAligned`] line segment.
    ///
    /// ## Arguments
    /// * `u` - the fixed coordinate (`x` for vertical lines, `y` for horizontal).
    /// * `v1`, `v2` - the starting and ending *(exclusive)* coordinates along the variable axis.
    #[inline]
    #[must_use]
    pub const fn new(u: isize, v1: isize, v2: isize) -> Self {
        if v1 <= v2 {
            Self::Positive(SignedAxisAligned::new(u, v1, v2))
        } else {
            Self::Negative(SignedAxisAligned::new(u, v1, v2))
        }
    }

    /// Checks if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn terminated(&self) -> bool {
        match self {
            Self::Positive(me) => me.terminated(),
            Self::Negative(me) => me.terminated(),
        }
    }
}

impl<const VERT: bool> Iterator for AxisAligned<VERT> {
    type Item = Point<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Positive(me) => me.next(),
            Self::Negative(me) => me.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Positive(me) => me.size_hint(),
            Self::Negative(me) => me.size_hint(),
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
            Self::Positive(me) => me.try_fold(init, f),
            Self::Negative(me) => me.try_fold(init, f),
        }
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self {
            Self::Positive(me) => me.fold(init, f),
            Self::Negative(me) => me.fold(init, f),
        }
    }
}

impl<const VERT: bool> DoubleEndedIterator for AxisAligned<VERT> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Positive(me) => me.next_back(),
            Self::Negative(me) => me.next_back(),
        }
    }

    #[cfg(feature = "try_fold")]
    #[inline]
    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: core::ops::Try<Output = B>,
    {
        match self {
            Self::Positive(me) => me.try_rfold(init, f),
            Self::Negative(me) => me.try_rfold(init, f),
        }
    }

    #[inline]
    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self {
            Self::Positive(me) => me.rfold(init, f),
            Self::Negative(me) => me.rfold(init, f),
        }
    }
}

impl<const VERT: bool> ExactSizeIterator for AxisAligned<VERT> {}

impl<const VERT: bool> core::iter::FusedIterator for AxisAligned<VERT> {}
