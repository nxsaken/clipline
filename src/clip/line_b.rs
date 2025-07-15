use crate::clip::ClipV;
use crate::line_b::{LineB, LineBu, LineBx, LineBy};
use crate::math::Coord;

impl<C: Coord> ClipV<C> {
    pub const fn line_bu<const YX: bool>(
        &self,
        x0: C,
        y0: C,
        x1: C,
        y1: C,
    ) -> Option<LineBu<YX, C>> {
        None
    }

    pub const fn line_bx(&self, x0: C, y0: C, x1: C, y1: C) -> Option<LineBx<C>> {
        None
    }

    pub const fn line_by(&self, x0: C, y0: C, x1: C, y1: C) -> Option<LineBy<C>> {
        None
    }

    pub const fn line_b(&self, x0: C, y0: C, x1: C, y1: C) -> Option<LineB<C>> {
        None
    }
}
