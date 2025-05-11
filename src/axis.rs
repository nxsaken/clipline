use crate::clip::Clip;
use crate::math::{ops, CxC, C, S, U};

/// An iterator over the rasterized points of a half-open axis-aligned line segment.
///
/// `V` determines the orientation of the line segment:
/// - `false`: horizontal (with endpoints `(u0, v)` and `(u1, v)`).
/// - `true`: vertical (with endpoints `(v, u0)` and `(v, u1)`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Axis<const V: bool> {
    /// The fixed coordinate along the other axis.
    v: C,
    /// The current start coordinate along this axis.
    u0: C,
    /// The current end coordinate along this axis.
    u1: C,
    /// The step sign along this axis.
    su: S,
}

/// An [`Axis<false>`] iterator over a half-open horizontal line segment.
pub type AxisH = Axis<false>;

/// An [`Axis<true>`] iterator over a half-open vertical line segment.
pub type AxisV = Axis<true>;

impl<const V: bool> Axis<V> {
    /// Constructs an [`Axis<V>`] iterator from its internal parts.
    ///
    /// # Safety
    ///
    /// `su` must match the direction from `u0` to `u1`.
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new_unchecked(v: C, u0: C, u1: C, su: S) -> Self {
        debug_assert!((u0 <= u1) == matches!(su, S::P));
        debug_assert!((u1 < u0) == matches!(su, S::N));
        Self { v, u0, u1, su }
    }

    /// Returns an [`Axis<V>`] iterator for a half-open axis-aligned line segment
    /// at a fixed `v` coordinate, spanning from `u0` to `u1`.
    ///
    /// `V` determines the orientation of the line segment:
    /// - `false`: horizontal (with endpoints `(u0, v)` and `(u1, v)`).
    /// - `true`: vertical (with endpoints `(v, u0)` and `(v, u1)`).
    #[inline]
    #[must_use]
    pub const fn new(v: C, u0: C, u1: C) -> Self {
        let su = if u0 <= u1 { S::P } else { S::N };
        // SAFETY: su matches the direction from u0 to u1.
        unsafe { Self::new_unchecked(v, u0, u1, su) }
    }

    /// A convenience alias for [`Clip::axis<V>`].
    #[inline]
    #[must_use]
    pub const fn clip_new(v: C, u0: C, u1: C, clip: &Clip) -> Option<Self> {
        clip.axis(v, u0, u1)
    }

    /// Returns a copy of this [`Axis<V>`] iterator.
    #[inline]
    #[must_use]
    pub const fn copy(&self) -> Self {
        Self { ..*self }
    }
}

impl<const V: bool> Axis<V> {
    /// Returns `true` if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.u0 == self.u1
    }

    /// Returns the remaining length of this iterator.
    #[inline]
    #[must_use]
    pub const fn length(&self) -> U {
        match self.su {
            // SAFETY: self.u0 <= self.u1.
            S::P => unsafe { ops::d_unchecked(self.u1, self.u0) },
            // SAFETY: self.u1 <= self.u0.
            S::N => unsafe { ops::d_unchecked(self.u0, self.u1) },
        }
    }

    /// Returns the point at the start of the iterator.
    /// This does not advance the iterator.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn head(&self) -> Option<CxC> {
        if self.is_done() {
            return None;
        }
        let (x0, y0) = if V { (self.v, self.u0) } else { (self.u0, self.v) };
        Some((x0, y0))
    }

    /// Consumes and returns the point at the start of the iterator.
    /// This advances the iterator forwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_head(&mut self) -> Option<CxC> {
        let Some((x0, y0)) = self.head() else { return None };
        self.u0 = self.u0.wrapping_add(self.su as C);
        Some((x0, y0))
    }
}

impl<const V: bool> Iterator for Axis<V> {
    type Item = CxC;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_head()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = usize::from(self.length());
        (len, Some(len))
    }
}

impl<const V: bool> core::iter::FusedIterator for Axis<V> {}

impl<const V: bool> ExactSizeIterator for Axis<V> {
    #[inline]
    fn len(&self) -> usize {
        usize::from(self.length())
    }
}

impl<const V: bool> Axis<V> {
    /// Returns the point immediately before the end of the iterator.
    /// This does not advance the iterator.
    ///
    /// Returns [`None`] if the iterator has terminated.
    ///
    /// # Performance
    ///
    /// This method performs trivial arithmetic to compute the last point.
    /// Avoid pairing this with [`Self::pop_tail`], as it will redo that work.
    #[inline]
    #[must_use]
    pub const fn tail(&self) -> Option<CxC> {
        if self.is_done() {
            return None;
        }
        let u1 = self.u1.wrapping_sub(self.su as C);
        let (x1, y1) = if V { (self.v, u1) } else { (u1, self.v) };
        Some((x1, y1))
    }

    /// Consumes and returns the point immediately before the end of the iterator.
    /// This advances the iterator backwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_tail(&mut self) -> Option<CxC> {
        let Some((x1, y1)) = self.tail() else { return None };
        self.u1 = if V { y1 } else { x1 };
        Some((x1, y1))
    }
}

impl<const V: bool> DoubleEndedIterator for Axis<V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}
