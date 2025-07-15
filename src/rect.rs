use crate::derive;
use crate::math::Coord;

pub struct Rect<C: Coord> {
    pub(crate) x0: C,
    pub(crate) y0: C,
    pub(crate) x1: C,
    pub(crate) y1: C,
}

derive::clone!([C: Coord] Rect<C>);

impl<C: Coord> Rect<C> {
    pub const fn new(x0: C, y0: C, x1: C, y1: C) -> Self {
        Self { x0, y0, x1, y1 }
    }
}
