use crate::derive;
use crate::math::{Coord, ops};
use crate::util::try_opt;

pub struct LineAu<const YX: bool, C: Coord> {
    pub(crate) u0: C,
    pub(crate) u1: C,
    pub(crate) v: C,
    pub(crate) su: i8,
}

pub type LineAx<C> = LineAu<false, C>;

pub type LineAy<C> = LineAu<true, C>;

derive::clone!([const YX: bool, C: Coord] LineAu<YX, C>);

impl<const YX: bool, C: Coord> core::fmt::Debug for LineAu<YX, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = if YX { "LineAy" } else { "LineAx" };
        let v = if YX { "x" } else { "y" };
        let u0 = if YX { "y0" } else { "x0" };
        let u1 = if YX { "y1" } else { "x1" };
        f.debug_struct(name).field(v, &self.v).field(u0, &self.u0).field(u1, &self.u1).finish()
    }
}

macro_rules! line_au {
    (
        $Cu:ty|$Ci:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        line_au!(
            $Cu,
            <$Cu as Coord>::U$(,
            exact = [$($ptr_size),+])?
        );
        line_au!(
            $Ci,
            <$Ci as Coord>::U$(,
            exact = [$($ptr_size),+])?
        );
    };
    (
        $C:ty,
        $U:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        impl<const YX: bool> LineAu<YX, $C> {
            pub const fn new(v: $C, u0: $C, u1: $C) -> Self {
                let su = if u0 <= u1 { 1 } else { -1 };
                Self { u0, u1, v, su }
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
                    let (x0, y0) = if YX { (self.v, self.u0) } else { (self.u0, self.v) };
                    Some((x0, y0))
                },
                fn pop_head = {
                    let (x0, y0) = try_opt!(self.head());
                    self.u0 = ops::<$C>::add_i(self.u0, self.su as _);
                    Some((x0, y0))
                },
                fn tail = {
                    if self.is_empty() {
                        return None;
                    }
                    let ut = ops::<$C>::sub_i(self.u1, self.su as _);
                    let (xt, yt) = if YX { (self.v, ut) } else { (ut, self.v) };
                    Some((xt, yt))
                },
                fn pop_tail = {
                    let (xt, yt) = try_opt!(self.tail());
                    self.u1 = if YX { yt } else { xt };
                    Some((xt, yt))
                }
            );
        }

        derive::iter_fwd!(LineAu<const YX, $C>$(, exact = [$($ptr_size),+])?);
        derive::iter_rev!(LineAu<const YX, $C>);
    };
}

line_au!(u8 | i8);
line_au!(u16 | i16, exact = ["16", "32", "64"]);
line_au!(u32 | i32, exact = ["32", "64"]);
line_au!(u64 | i64, exact = ["64"]);
line_au!(usize | isize);

#[derive(Debug)]
pub enum LineA<C: Coord> {
    Ax(LineAx<C>),
    Ay(LineAy<C>),
}

derive::clone!([C: Coord] LineA<C> {Ax, Ay});

macro_rules! line_a {
    (
        $Cu:ty|$Ci:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        line_a!(
            $Cu,
            <$Cu as Coord>::U$(,
            exact = [$($ptr_size),+])?
        );
        line_a!(
            $Ci,
            <$Ci as Coord>::U$(,
            exact = [$($ptr_size),+])?
        );
    };
    (
        $C:ty,
        $U:ty$(,
        exact = [$($ptr_size:literal),+])?
    ) => {
        impl LineA<$C> {
            pub const fn new(x0: $C, y0: $C, x1: $C, y1: $C) -> Option<Self> {
                if y0 == y1 {
                    Some(Self::Ax(LineAx::<$C>::new(y0, x0, x1)))
                } else if x0 == x1 {
                    Some(Self::Ay(LineAy::<$C>::new(x0, y0, y1)))
                } else {
                    None
                }
            }

            derive::iter_methods!(
                C = $C,
                U = $U,
                self = self,
                fn is_empty = match self {
                    Self::Ax(line) => line.is_empty(),
                    Self::Ay(line) => line.is_empty(),
                },
                fn len = match self {
                    Self::Ax(line) => line.len(),
                    Self::Ay(line) => line.len(),
                },
                fn head = match self {
                    Self::Ax(line) => line.head(),
                    Self::Ay(line) => line.head(),
                },
                fn pop_head = match self {
                    Self::Ax(line) => line.pop_head(),
                    Self::Ay(line) => line.pop_head(),
                }
            );
        }

        derive::iter_fwd!(
            LineA<$C>,
            fn fold(self, accum, f) = match self {
                Self::Ax(line) => line.fold(accum, f),
                Self::Ay(line) => line.fold(accum, f),
            }$(,
            exact = [$($ptr_size),+])?
        );
    };
}

line_a!(u8 | i8);
line_a!(u16 | i16, exact = ["16", "32", "64"]);
line_a!(u32 | i32, exact = ["32", "64"]);
line_a!(u64 | i64, exact = ["64"]);
line_a!(usize | isize);
