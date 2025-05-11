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
    /// `su == sign(u1 - u0)`.
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new_unchecked(v: C, u0: C, u1: C, su: S) -> Self {
        debug_assert!((u0 <= u1) == matches!(su, S::Pos));
        debug_assert!((u1 < u0) == matches!(su, S::Neg));
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
        let su = if u0 <= u1 { S::Pos } else { S::Neg };
        // SAFETY: su == sign(u1 - u0).
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
            // SAFETY: u0 <= u1.
            S::Pos => unsafe { ops::unchecked_abs_diff(self.u1, self.u0) },
            // SAFETY: u1 <= u0.
            S::Neg => unsafe { ops::unchecked_abs_diff(self.u0, self.u1) },
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
        // SAFETY:
        // * su > 0 => u0 < u1 => u0 + 1 cannot overflow.
        // * su < 0 => u1 < u0 => u0 - 1 cannot underflow.
        self.u0 = unsafe { ops::unchecked_add_sign(self.u0, self.su) };
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
        // SAFETY:
        // * su > 0 => u0 < u1 => u1 - 1 cannot underflow.
        // * su < 0 => u1 < u0 => u1 + 1 cannot overflow.
        let ut = unsafe { ops::unchecked_sub_sign(self.u1, self.su) };
        let (xt, yt) = if V { (self.v, ut) } else { (ut, self.v) };
        Some((xt, yt))
    }

    /// Consumes and returns the point immediately before the end of the iterator.
    /// This advances the iterator backwards.
    ///
    /// Returns [`None`] if the iterator has terminated.
    #[inline]
    #[must_use]
    pub const fn pop_tail(&mut self) -> Option<CxC> {
        let Some((xt, yt)) = self.tail() else { return None };
        self.u1 = if V { yt } else { xt };
        Some((xt, yt))
    }
}

impl<const V: bool> DoubleEndedIterator for Axis<V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}
