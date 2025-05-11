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
        let (sx, dx) = ops::sd(x0, x1);
        let (sy, dy) = ops::sd(y0, y1);
        if (YX && dy <= dx || !YX && dx < dy)
            // SAFETY: sx and sy match the directions from x0 to x1 and from y0 to y1.
            || unsafe { self.rejects_bbox((x0, y0), (x1, y1), (sx, sy)) }
        {
            return None;
        }
        todo!()
    }
}
