use crate::derive;
use crate::math::{Coord, ops};

mod line_a;
mod line_b;
mod line_d;
mod rect;

pub struct Clip<C> {
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

derive::clone!([C: Coord] Clip<C>);

macro_rules! clip {
    ($Cu:ty|$Ci:ty) => {
        clip!(unsigned $Cu, <$Cu as Coord>::U);
        clip!(signed $Ci, <$Ci as Coord>::U);
    };
    (unsigned $Cu:ty, $U:ty) => {
        impl Clip<$Cu> {
            pub const fn from_max(x_max: $Cu, y_max: $Cu) -> Self {
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
                if width == 0 && MAX < width && height == 0 && MAX < height {
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

pub struct ClipV<C> {
    pub(crate) x_min: C,
    pub(crate) y_min: C,
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

derive::clone!([C: Coord] ClipV<C>);

macro_rules! clip_v {
    ($Cu:ty|$Ci:ty) => {
        clip_v!($Cu, <$Cu as Coord>::U);
        clip_v!($Ci, <$Ci as Coord>::U);
    };
    ($C:ty, $U:ty) => {
        impl ClipV<$C> {
            pub const fn from_min_max(x_min: $C, y_min: $C, x_max: $C, y_max: $C) -> Option<Self> {
                if x_max < x_min || y_max < y_min {
                    return None;
                }
                Some(Self { x_min, y_min, x_max, y_max })
            }

            pub const fn from_min_size(
                x_min: $C,
                y_min: $C,
                width: $U,
                height: $U,
            ) -> Option<Self> {
                if width == 0 || height == 0 {
                    return None;
                }
                let dx = width - 1;
                let dy = height - 1;
                let Some(x_max) = ops::<$C>::checked_add_u(x_min, dx) else { return None };
                let Some(y_max) = ops::<$C>::checked_add_u(y_min, dy) else { return None };
                Some(Self { x_min, y_min, x_max, y_max })
            }
        }
    };
}

clip_v!(u8 | i8);
clip_v!(u16 | i16);
clip_v!(u32 | i32);
clip_v!(u64 | i64);
clip_v!(usize | isize);
