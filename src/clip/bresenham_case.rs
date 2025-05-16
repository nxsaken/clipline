use crate::bresenham_case::BresenhamCase;
use crate::clip::Clip;
use crate::math::CxC;

impl Clip {
    /// Clips a half-open line segment to this region.
    ///
    /// Returns a [`BresenhamCase<YX>`] over the portion of the segment inside this clipping
    /// region, or [`None`] if the segment is fully outside, or its slope does not match `YX`.
    #[inline]
    #[must_use]
    pub const fn bresenham_case<const YX: bool>(
        &self,
        (_x0, _y0): CxC,
        (_x1, _y1): CxC,
    ) -> Option<BresenhamCase<YX>> {
        todo!()
    }
}
