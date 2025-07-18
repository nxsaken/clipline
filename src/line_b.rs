use crate::derive;
use crate::math::{Coord, ops};
use crate::util::try_opt;

pub struct LineBu<const YX: bool, C: Coord> {
    pub(crate) u0: C,
    pub(crate) v0: C,
    pub(crate) du: C::U,
    pub(crate) dv: C::U,
    pub(crate) err: C::I2,
    pub(crate) u1: C,
    pub(crate) su: i8,
    pub(crate) sv: i8,
}

pub type LineBx<C> = LineBu<false, C>;

pub type LineBy<C> = LineBu<true, C>;

derive::clone!([const YX: bool, C: Coord] LineBu<YX, C>);

macro_rules! line_bu {
    (
        $Cu:ty|$Ci:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        line_bu!(
            $Cu,
            <$Cu as Coord>::U,
            <$Cu as Coord>::I,
            <$Cu as Coord>::U2,
            <$Cu as Coord>::I2$(,
            exact = [$($ptr_size),+])?
        );
        line_bu!(
            $Ci,
            <$Ci as Coord>::U,
            <$Ci as Coord>::I,
            <$Ci as Coord>::U2,
            <$Ci as Coord>::I2$(,
            exact = [$($ptr_size),+])?
        );
    };
    (
        $C:ty,
        $U:ty,
        $I:ty,
        $U2:ty,
        $I2:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        impl<const YX: bool> LineBu<YX, $C> {
            pub const fn new(x0: $C, y0: $C, x1: $C, y1: $C) -> Option<Self> {
                let (dx, sx) = ops::<$C>::abs_diff_sign(x1, x0);
                let (dy, sy) = ops::<$C>::abs_diff_sign(y1, y0);
                if YX && dy <= dx || !YX && dx < dy {
                    return None;
                }
                let (u0, v0, u1, du, dv, su, sv) = if YX {
                    (y0, x0, y1, dy, dx, sy, sx)
                } else {
                    (x0, y0, x1, dx, dy, sx, sy)
                };
                let err = dv as $I2 - du.div_ceil(2) as $I2;
                Some(Self { u0, v0, du, dv, err, u1, su, sv })
            }

            pub(crate) const fn new_au(v0: $C, u0: $C, u1: $C, su: i8) -> Self {
                Self { u0, v0, du: 0, dv: 0, err: -1, u1, su, sv: 0 }
            }

            derive::iter_methods!(
                C = $C,
                U = $U,
                self = self,
                fn is_empty = self.u0 == self.u1,
                fn len = self.u0.abs_diff(self.u1),
                fn head = {
                    if self.is_empty() {
                        return None;
                    }
                    let (x0, y0) = if YX { (self.v0, self.u0) } else { (self.u0, self.v0) };
                    Some((x0, y0))
                },
                fn pop_head = {
                    let (x0, y0) = try_opt!(self.head());
                    if 0 <= self.err {
                        self.v0 = ops::<$C>::add_i(self.v0, self.sv as $I);
                        self.err -= self.du as $I2;
                    }
                    self.u0 = ops::<$C>::add_i(self.u0, self.su as $I);
                    self.err += self.dv as $I2;
                    Some((x0, y0))
                }
            );
        }

        derive::iter_fwd!(
            LineBu<const YX, $C>$(,
            exact = [$($ptr_size),+])?
        );
    };
}

line_bu!(u8 | i8);
line_bu!(u16 | i16, exact = ["16", "32", "64"]);
line_bu!(u32 | i32, exact = ["32", "64"]);
line_bu!(u64 | i64, exact = ["64"]);
line_bu!(usize | isize);

pub enum LineB<C: Coord> {
    Bx(LineBx<C>),
    By(LineBy<C>),
}

derive::clone!([C: Coord] LineB<C> {Bx, By});

macro_rules! line_b {
    (
        $Cu:ty|$Ci:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        line_b!(
            $Cu,
            <$Cu as Coord>::U,
            <$Cu as Coord>::I,
            <$Cu as Coord>::U2,
            <$Cu as Coord>::I2$(,
            exact = [$($ptr_size),+])?
        );
        line_b!(
            $Ci,
            <$Ci as Coord>::U,
            <$Ci as Coord>::I,
            <$Ci as Coord>::U2,
            <$Ci as Coord>::I2$(,
            exact = [$($ptr_size),+])?
        );
    };
    (
        $C:ty,
        $U:ty,
        $I:ty,
        $U2:ty,
        $I2:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        impl LineB<$C> {
            pub const fn new(x0: $C, y0: $C, x1: $C, y1: $C) -> Self {
                let (dx, sx) = ops::<$C>::abs_diff_sign(x1, x0);
                let (dy, sy) = ops::<$C>::abs_diff_sign(y1, y0);
                if dx < dy {
                    let (u0, v0, u1, du, dv, su, sv) = (y0, x0, y1, dy, dx, sy, sx);
                    let err = dv as $I2 - du.div_ceil(2) as $I2;
                    Self::By(LineBy { u0, v0, du, dv, err, u1, su, sv })
                } else {
                    let (u0, v0, u1, du, dv, su, sv) = (x0, y0, x1, dx, dy, sx, sy);
                    let err = dv as $I2 - du.div_ceil(2) as $I2;
                    Self::Bx(LineBx { u0, v0, du, dv, err, u1, su, sv })
                }
            }

            derive::iter_methods!(
                C = $C,
                U = $U,
                self = self,
                fn is_empty = match self {
                    Self::Bx(line) => line.is_empty(),
                    Self::By(line) => line.is_empty(),
                },
                fn len = match self {
                    Self::Bx(line) => line.len(),
                    Self::By(line) => line.len(),
                },
                fn head = match self {
                    Self::Bx(line) => line.head(),
                    Self::By(line) => line.head(),
                },
                fn pop_head = match self {
                    Self::Bx(line) => line.pop_head(),
                    Self::By(line) => line.pop_head(),
                }
            );
        }

        derive::iter_fwd!(
            LineB<$C>,
            fn fold(self, accum, f) = match self {
                Self::Bx(line) => line.fold(accum, f),
                Self::By(line) => line.fold(accum, f),
            }$(,
            exact = [$($ptr_size),+])?
        );
    };
}

line_b!(u8 | i8);
line_b!(u16 | i16, exact = ["16", "32", "64"]);
line_b!(u32 | i32, exact = ["32", "64"]);
line_b!(u64 | i64, exact = ["64"]);
line_b!(usize | isize);
