use crate::Clip;
use crate::clip::ClipV;
use crate::line_d::{LineD, LineD2};

impl Clip<i16> {
    const unsafe fn reject_bbox_half_open<const FX: bool, const FY: bool>(
        &self,
        x0: i16,
        y0: i16,
        x1: i16,
        y1: i16,
    ) -> bool {
        !FX && (x1 <= 0 || self.x_max < x0)
            || FX && (x0 < 0 || self.x_max <= x1)
            || !FY && (y1 <= 0 || self.y_max < y0)
            || FY && (y0 < 0 || self.y_max <= y1)
    }
}

impl Clip<u16> {
    pub const fn line_d(&self, x0: u16, y0: u16, x1: u16, y1: u16) -> Option<LineD<u16>> {
        None
    }

    pub const fn line_d2(&self, x0: u16, y0: u16, x1: u16, y1: u16) -> Option<LineD2<u16>> {
        None
    }
}

impl Clip<i16> {
    pub const fn line_d(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD<i16>> {
        None
    }

    pub const fn line_d2(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD2<i16>> {
        None
    }

    pub const fn line_d_o(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD<u16>> {
        None
    }

    pub const fn line_d2_o(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD2<u16>> {
        None
    }
}

impl ClipV<i16> {
    pub const fn line_d(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD<i16>> {
        None
    }

    pub const fn line_d2(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD2<i16>> {
        None
    }

    pub const fn line_d_o(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD<u16>> {
        None
    }

    pub const fn line_d2_o(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> Option<LineD2<u16>> {
        None
    }
}
