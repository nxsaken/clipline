use crate::bresenham_case::BresenhamCase;
use crate::clip::Clip;
use crate::math::{ops, CxC};

impl Clip {
    /// Clips a half-open line segment to this region.
    ///
    /// Returns a [`BresenhamCase<YX>`] over the portion of the segment inside this clipping
    /// region, or [`None`] if the segment is fully outside, or its slope does not match `YX`.
    #[inline]
    #[must_use]
    pub const fn bresenham_case<const YX: bool>(
        &self,
        (x0, y0): CxC,
        (x1, y1): CxC,
    ) -> Option<BresenhamCase<YX>> {
        let (sx, dx) = ops::abs_diff(x1, x0);
        let (sy, dy) = ops::abs_diff(y1, y0);
        if (YX && dy <= dx || !YX && dx < dy)
            // SAFETY: sx = sign(x1 - x0) and sy = sign(y1 - y0).
            || unsafe { self.rejects_bbox((x0, y0), (x1, y1), (sx, sy)) }
        {
            return None;
        }
        todo!()
    }
}
