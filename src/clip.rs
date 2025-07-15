use crate::derive;
use crate::math::{Coord, ops};

mod line_a;
mod line_b;
mod line_d;
mod rect;

pub struct Clip<C: Coord> {
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

derive::clone!([C: Coord] Clip<C>);

macro_rules! clip {
    ($U:ty|$I:ty) => {
        clip!(unsigned $U);
        clip!(signed $I, $U);
    };
    (unsigned $U:ty) => {
        impl Clip<$U> {
            pub const fn from_max(x_max: $U, y_max: $U) -> Self {
                Self { x_max, y_max }
            }

            pub const fn from_size(width: $U, height: $U) -> Option<Self> {
                if width == 0 || height == 0 {
                    return None;
                }
                let x_max = width - 1;
                let y_max = height - 1;
                Some(Self { x_max, y_max })
            }
        }
    };
    (signed $Ci:ty, $U:ty) => {
        impl Clip<$Ci> {
            pub const fn from_max(x_max: $Ci, y_max: $Ci) -> Option<Self> {
                if x_max < 0 || y_max < 0 {
                    return None;
                }
                Some(Self { x_max, y_max })
            }

            pub const fn from_size(width: $U, height: $U) -> Option<Self> {
                const MAX: $U = <$Ci>::MAX as $U + 1;
                if width == 0 || height == 0 || MAX < width || MAX < height {
                    return None;
                }
                let x_max = (width - 1) as $Ci;
                let y_max = (height - 1) as $Ci;
                Some(Self { x_max, y_max })
            }
        }
    };
}

clip!(u8 | i8);
clip!(u16 | i16);
clip!(u32 | i32);
clip!(u64 | i64);
clip!(usize | isize);

pub struct Viewport<C: Coord> {
    pub(crate) x_min: C,
    pub(crate) y_min: C,
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

derive::clone!([C: Coord] Viewport<C>);

macro_rules! viewport {
    ($U:ty|$I:ty) => {
        viewport!($U, $U);
        viewport!($I, $U);
    };
    ($UI:ty, $U:ty) => {
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
}

viewport!(u8 | i8);
viewport!(u16 | i16);
viewport!(u32 | i32);
viewport!(u64 | i64);
viewport!(usize | isize);

#[rustfmt::skip]
macro_rules! if_clip {
    (Clip     $Clip:block else $Viewport:block) => { $Clip };
    (Viewport $Clip:block else $Viewport:block) => { $Viewport };
}

#[rustfmt::skip]
macro_rules! if_clip_u {
    (unsigned        Clip       $unsigned_clip:block else $other:block) => { $unsigned_clip };
    ($signess:ident $Clip:ident $unsigned_clip:block else $other:block) => { $other };
}

use {if_clip, if_clip_u};
