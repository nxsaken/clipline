use crate::derive;
use crate::math::{Coord, ops};
use crate::util::try_opt;

pub struct LineD<C: Coord> {
    pub(crate) x0: C,
    pub(crate) y0: C,
    pub(crate) x1: C,
    pub(crate) sx: i8,
    pub(crate) sy: i8,
}

derive::clone!([C: Coord] LineD<C>);

macro_rules! line_d {
    (
        $Cu:ty|$Ci:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        line_d!(
            $Cu,
            <$Cu as Coord>::U,
            <$Cu as Coord>::I$(,
            exact = [$($ptr_size),+])?
        );
        line_d!(
            $Ci,
            <$Ci as Coord>::U,
            <$Ci as Coord>::I$(,
            exact = [$($ptr_size),+])?
        );
    };
    (
        $C:ty,
        $U:ty,
        $I:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        impl LineD<$C> {
            pub const fn new(x0: $C, y0: $C, x1: $C, y1: $C) -> Option<Self> {
                let (dx, sx) = ops::<$C>::abs_diff_sign(x1, x0);
                let (dy, sy) = ops::<$C>::abs_diff_sign(y1, y0);
                if dx != dy {
                    return None;
                }
                Some(Self { x0, y0, x1, sx, sy })
            }

            pub const fn to_line_d2(self) -> LineD2<$C> {
                let Self { x0, y0, x1, sx, sy } = self;
                let dx = x0.abs_diff(x1);
                let y1 = ops::<$C>::add_u_signed(self.y0, dx, sy);
                LineD2 { x0, y0, x1, y1, sx, sy }
            }

            derive::iter_methods!(
                C = $C,
                U = $U,
                self = self,
                fn is_empty = self.x0 == self.x1,
                fn len = self.x0.abs_diff(self.x1),
                fn head = {
                    if self.is_empty() {
                        return None;
                    }
                    Some((self.x0, self.y0))
                },
                fn pop_head = {
                    let (x0, y0) = try_opt!(self.head());
                    self.x0 = ops::<$C>::add_i(self.x0, self.sx as $I);
                    self.y0 = ops::<$C>::add_i(self.y0, self.sy as $I);
                    Some((x0, y0))
                },
                fn tail = {
                    if self.is_empty() {
                        return None;
                    }
                    let xt = ops::<$C>::sub_i(self.x1, self.sx as $I);
                    let dxt = ops::<$C>::abs_diff_signed(self.x0, xt, self.sx);
                    let yt = ops::<$C>::add_u_signed(self.y0, dxt, self.sy);
                    Some((xt, yt))
                },
                fn pop_tail = {
                    let (xt, yt) = try_opt!(self.tail());
                    self.x1 = xt;
                    Some((xt, yt))
                }
            );
        }

        derive::iter_fwd!(LineD<$C>$(, exact = [$($ptr_size),+])?);
        derive::iter_rev!(LineD<$C>);
    };
}

line_d!(u8 | i8);
line_d!(u16 | i16);
line_d!(u32 | i32, exact = ["32", "64"]);
line_d!(u64 | i64, exact = ["64"]);
line_d!(usize | isize);

pub struct LineD2<C: Coord> {
    pub(crate) x0: C,
    pub(crate) y0: C,
    pub(crate) x1: C,
    pub(crate) y1: C,
    pub(crate) sx: i8,
    pub(crate) sy: i8,
}

derive::clone!([C: Coord] LineD2<C>);

impl<C: Coord> LineD2<C> {
    pub const fn to_line_d(self) -> LineD<C> {
        let Self { x0, y0, x1, sx, sy, .. } = self;
        LineD { x0, y0, x1, sx, sy }
    }
}

macro_rules! line_d2 {
    (
        $Cu:ty|$Ci:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        line_d2!(
            $Cu,
            <$Cu as Coord>::U,
            <$Cu as Coord>::I$(,
            exact = [$($ptr_size),+])?
        );
        line_d2!(
            $Ci,
            <$Ci as Coord>::U,
            <$Ci as Coord>::I$(,
            exact = [$($ptr_size),+])?
        );
    };
    (
        $C:ty,
        $U:ty,
        $I:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        impl LineD2<$C> {
            pub const fn new(x0: $C, y0: $C, x1: $C, y1: $C) -> Option<Self> {
                let (dx, sx) = ops::<$C>::abs_diff_sign(x1, x0);
                let (dy, sy) = ops::<$C>::abs_diff_sign(y1, y0);
                if dx != dy {
                    return None;
                }
                Some(Self { x0, y0, x1, y1, sx, sy })
            }

            derive::iter_methods!(
                C = $C,
                U = $U,
                self = self,
                fn is_empty = self.x0 == self.x1,
                fn len = self.x0.abs_diff(self.x1),
                fn head = {
                    if self.is_empty() {
                        return None;
                    }
                    Some((self.x0, self.y0))
                },
                fn pop_head = {
                    let (x0, y0) = try_opt!(self.head());
                    self.x0 = ops::<$C>::add_i(self.x0, self.sx as $I);
                    self.y0 = ops::<$C>::add_i(self.y0, self.sy as $I);
                    Some((x0, y0))
                },
                fn tail = {
                    if self.is_empty() {
                        return None;
                    }
                    let xt = ops::<$C>::sub_i(self.x1, self.sx as $I);
                    let yt = ops::<$C>::sub_i(self.y1, self.sy as $I);
                    Some((xt, yt))
                },
                fn pop_tail = {
                    let (xt, yt) = try_opt!(self.tail());
                    self.x1 = xt;
                    self.y1 = yt;
                    Some((xt, yt))
                }
            );
        }

        derive::iter_fwd!(LineD2<$C>$(, exact = [$($ptr_size),+])?);
        derive::iter_rev!(LineD2<$C>);
    };
}

line_d2!(u8 | i8);
line_d2!(u16 | i16, exact = ["16", "32", "64"]);
line_d2!(u32 | i32, exact = ["32", "64"]);
line_d2!(u64 | i64, exact = ["64"]);
line_d2!(usize | isize);
