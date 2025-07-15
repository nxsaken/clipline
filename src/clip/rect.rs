use crate::clip::{Clip, ClipV};
use crate::rect::Rect;

impl Clip<i8> {
    pub const fn rect(&self, x0: i8, y0: i8, x1: i8, y1: i8) -> Option<Rect<i8>> {
        None
    }

    pub const fn rect_o(&self, x0: i8, y0: i8, x1: i8, y1: i8) -> Option<Rect<u8>> {
        None
    }
}

impl ClipV<i8> {
    pub const fn rect(&self, x0: i8, y0: i8, x1: i8, y1: i8) -> Option<Rect<i8>> {
        None
    }

    pub const fn rect_o(&self, x0: i8, y0: i8, x1: i8, y1: i8) -> Option<Rect<u8>> {
        None
    }
}
