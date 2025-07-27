use crate::derive;
use crate::math::Coord;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Rect<C: Coord> {
    pub(crate) _x0: C,
    pub(crate) _y0: C,
    pub(crate) _x1: C,
    pub(crate) _y1: C,
}

derive::clone!([C: Coord] Rect<C>);

impl<C: Coord> Rect<C> {
    #[inline]
    const fn new(_x0: C, _y0: C, _x1: C, _y1: C) -> Self {
        Self { _x0, _y0, _x1, _y1 }
    }
}
