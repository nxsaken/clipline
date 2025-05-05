use super::Clip;
use crate::axis::{Axis, Axis0, Axis1};
use crate::math::{C, S};

impl Clip {
    /// Clips a half-open axis-aligned line segment to this region.
    ///
    /// Returns an [`Axis<V>`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    ///
    /// `V` determines the orientation of the line segment:
    /// - `false`: horizontal, from `(u0, v)` to `(u1, v)`.
    /// - `true`: vertical, from `(v, u0)` to `(v, u1)`.
    #[expect(clippy::similar_names)]
    #[inline]
    #[must_use]
    pub const fn axis<const V: bool>(&self, v: C, u0: C, u1: C) -> Option<Axis<V>> {
        let (wv0, wv1) = if V { (self.x0, self.x1) } else { (self.y0, self.y1) };
        let (wu0, wu1) = if V { (self.y0, self.y1) } else { (self.x0, self.x1) };
        if v < wv0 || wv1 < v {
            return None;
        }
        let (su, cu0, cu1) = if u0 <= u1 {
            let (nu0, nu1) = (u0, u1);
            if nu1 <= wu0 || wu1 < nu0 {
                return None;
            }
            let cu0 = if nu0 < wu0 { wu0 } else { nu0 };
            let cu1 = if wu1 < nu1 { wu1.wrapping_add(1) } else { nu1 };
            (S::P, cu0, cu1)
        } else {
            let (nu0, nu1) = (u1, u0);
            if nu1 <= wu0 || wu1 < nu0 {
                return None;
            }
            let cu0 = if wu1 < nu0 { wu1 } else { nu0 };
            let cu1 = if nu1 < wu0 { wu0.wrapping_sub(1) } else { nu1 };
            (S::N, cu0, cu1)
        };
        // SAFETY: su matches the direction from cu0 to cu1.
        let axis = unsafe { Axis::new_unchecked(v, cu0, cu1, su) };
        Some(axis)
    }

    /// Clips a half-open horizontal line segment to this region.
    ///
    /// Returns an [`Axis0`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    #[inline]
    #[must_use]
    pub const fn axis_0(&self, y: C, x0: C, x1: C) -> Option<Axis0> {
        self.axis::<false>(y, x0, x1)
    }

    /// Clips a half-open vertical line segment to this region.
    ///
    /// Returns an [`Axis1`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    #[inline]
    #[must_use]
    pub const fn axis_1(&self, x: C, y0: C, y1: C) -> Option<Axis1> {
        self.axis::<true>(x, y0, y1)
    }
}
