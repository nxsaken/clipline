use crate::derive;
use crate::math::{Coord, if_unsigned, ops};

mod line_a;
mod line_b;
mod line_d;
mod rect;

pub struct Clip<C: Coord> {
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

pub struct Viewport<C: Coord> {
    pub(crate) x_min: C,
    pub(crate) y_min: C,
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

derive::clone!([C: Coord] Clip<C>);
derive::clone!([C: Coord] Viewport<C>);

#[rustfmt::skip]
macro_rules! if_clip {
    (Clip     $Clip:block else $Viewport:block) => { $Clip };
    (Viewport $Clip:block else $Viewport:block) => { $Viewport };
}

use if_clip;

macro_rules! clip {
    ($U:ty|$I:ty) => {
        clip!(@impl Clip<unsigned $U>, $U);
        clip!(@impl Clip<signed $I>, $U);
        clip!(@impl Viewport<$U>, $U);
        clip!(@impl Viewport<$I>, $U);

        clip!(@impl Clip<$U>);
        clip!(@impl Clip<$I>);
        clip!(@impl Viewport<$U>);
        clip!(@impl Viewport<$I>);
    };
    (@impl Clip<$signess:ident $UI:ty>, $U:ty) => {
        impl Clip<$UI> {
            pub const fn from_max(
                x_max: $UI,
                y_max: $UI,
            ) -> if_unsigned!($signess [Self] else [Option<Self>]) {
                if_unsigned!($signess {
                    Self { x_max, y_max }
                } else {
                    if x_max < 0 || y_max < 0 {
                        return None;
                    }
                    Some(Self { x_max, y_max })
                })
            }

            pub const fn from_size(width: $U, height: $U) -> Option<Self> {
                let (x_max, y_max) = if_unsigned!($signess {
                    if width == 0 || height == 0 {
                        return None;
                    }
                    let x_max = width - 1;
                    let y_max = height - 1;
                    (x_max, y_max)
                } else {
                    const MAX: $U = <$UI>::MAX as $U + 1;
                    if width == 0 || height == 0 || MAX < width || MAX < height {
                        return None;
                    }
                    let x_max = (width - 1) as $UI;
                    let y_max = (height - 1) as $UI;
                    (x_max, y_max)
                });
                Some(Self { x_max, y_max })
            }
        }
    };
    (@impl Viewport<$UI:ty>, $U:ty) => {
        impl Viewport<$UI> {
            pub const fn from_min_max(
                x_min: $UI,
                y_min: $UI,
                x_max: $UI,
                y_max: $UI,
            ) -> Option<Self> {
                if x_max < x_min || y_max < y_min {
                    return None;
                }
                Some(Self { x_min, y_min, x_max, y_max })
            }

            pub const fn from_min_size(
                x_min: $UI,
                y_min: $UI,
                width: $U,
                height: $U,
            ) -> Option<Self> {
                if width == 0 || height == 0 {
                    return None;
                }
                let dx = width - 1;
                let dy = height - 1;
                let Some(x_max) = ops::<$UI>::checked_add_u(x_min, dx) else { return None };
                let Some(y_max) = ops::<$UI>::checked_add_u(y_min, dy) else { return None };
                Some(Self { x_min, y_min, x_max, y_max })
            }
        }
    };
    (@impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            const fn x_min(&self) -> $UI {
                if_clip!($Self { 0 } else { self.x_min })
            }

            const fn y_min(&self) -> $UI {
                if_clip!($Self { 0 } else { self.y_min })
            }

            const fn yx(&self) -> Self {
                if_clip!($Self {
                    Self {
                        x_max: self.y_max,
                        y_max: self.x_max,
                    }
                } else {
                    Self {
                        x_min: self.y_min,
                        y_min: self.x_min,
                        x_max: self.y_max,
                        y_max: self.x_max,
                    }
                })
            }
        }
    };
}

clip!(u8 | i8);
clip!(u16 | i16);
clip!(u32 | i32);
clip!(u64 | i64);
clip!(usize | isize);
