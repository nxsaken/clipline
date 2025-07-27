use crate::clip::{Clip, Viewport};
use crate::rect::Rect;

impl Clip<i8> {
    #[inline]
    const fn rect(&self, _x0: i8, _y0: i8, _x1: i8, _y1: i8) -> Option<Rect<i8>> {
        None
    }

    #[inline]
    const fn rect_proj(&self, _x0: i8, _y0: i8, _x1: i8, _y1: i8) -> Option<Rect<u8>> {
        None
    }
}

impl Viewport<i8> {
    #[inline]
    const fn rect(&self, _x0: i8, _y0: i8, _x1: i8, _y1: i8) -> Option<Rect<i8>> {
        None
    }

    #[inline]
    const fn rect_proj(&self, _x0: i8, _y0: i8, _x1: i8, _y1: i8) -> Option<Rect<u8>> {
        None
    }
}
