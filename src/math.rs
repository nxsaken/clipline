pub trait Num
where
    Self: Copy + Eq + Ord + Default,
    Self: core::hash::Hash,
    Self: core::fmt::Debug + core::fmt::Display,
{
}

macro_rules! primitive {
    ($($T:ty),+) => {
        $(impl Num for $T {})+
    };
}

primitive!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

pub trait Coord: Num {
    type U: Num;
    type I: Num;
    type U2: Num;
    type I2: Num;
}

macro_rules! coord {
    ($U:ty, $I:ty, $U2:ty, $I2:ty) => {
        impl Coord for $U {
            type U = $U;
            type I = $I;
            type U2 = $U2;
            type I2 = $I2;
        }
        impl Coord for $I {
            type U = $U;
            type I = $I;
            type U2 = $U2;
            type I2 = $I2;
        }
    };
}

coord!(u8, i8, u16, i16);
coord!(u16, i16, u32, i32);
coord!(u32, i32, u64, i64);
coord!(u64, i64, u128, i128);
#[cfg(target_pointer_width = "64")]
coord!(usize, isize, u128, i128);
#[cfg(target_pointer_width = "32")]
coord!(usize, isize, u64, i64);
#[cfg(target_pointer_width = "16")]
coord!(usize, isize, u32, i32);

#[rustfmt::skip]
macro_rules! if_unsigned {
    (unsigned $unsigned:block else $signed:block) => { $unsigned };
    (signed   $unsigned:block else $signed:block) => { $signed };
    (unsigned [$unsigned:ty] else [$signed:ty]) => { $unsigned };
    (signed   [$unsigned:ty] else [$signed:ty]) => { $signed };
}

pub(crate) use if_unsigned;

#[allow(non_camel_case_types)]
pub(crate) struct ops<C: Coord>(C);

macro_rules! coord_ops {
    (common $UI:ty, $U:ty) => {
        impl ops<$UI> {
            pub const fn min(a: $UI, b: $UI) -> $UI {
                if a <= b { a } else { b }
            }
            pub const fn max(a: $UI, b: $UI) -> $UI {
                if b <= a { a } else { b }
            }
            pub const fn min_adj(incl: $UI, excl: $UI) -> $UI {
                if incl < excl { incl + 1 } else { excl }
            }
            pub const fn max_adj(incl: $UI, excl: $UI) -> $UI {
                if excl < incl { incl - 1 } else { excl }
            }
            pub const fn add_fu<const F: bool>(lhs: $UI, rhs: $U) -> $UI {
                if F { Self::sub_u(lhs, rhs) } else { Self::add_u(lhs, rhs) }
            }
            pub const fn sub_fu<const F: bool>(lhs: $UI, rhs: $U) -> $UI {
                if F { Self::add_u(lhs, rhs) } else { Self::sub_u(lhs, rhs) }
            }
            pub const fn wadd_su(lhs: $UI, rhs: $U, sign: i8) -> $UI {
                if sign > 0 {
                    Self::wadd_u(lhs, rhs)
                } else {
                    Self::wsub_u(lhs, rhs)
                }
            }
            pub const fn usub_f<const F: bool>(lhs: $UI, rhs: $UI) -> $U {
                if F {
                    Self::usub(rhs, lhs)
                } else {
                    Self::usub(lhs, rhs)
                }
            }
            pub const fn wusub_s(lhs: $UI, rhs: $UI, sign: i8) -> $U {
                if sign > 0 {
                    Self::wusub(lhs, rhs)
                } else {
                    Self::wusub(rhs, lhs)
                }
            }
            pub const fn susub(lhs: $UI, rhs: $UI) -> ($U, i8) {
                if rhs <= lhs {(
                    Self::usub(lhs, rhs), 1)
                } else {
                    (Self::usub(rhs, lhs), -1)
                }
            }
        }
    };
    ($signedness:ident $UI:ty, $U:ty, $I:ty) => {
        impl ops<$UI> {
            pub const fn add_u(lhs: $UI, rhs: $U) -> $UI {
                if_unsigned!($signedness {
                    lhs + rhs
                } else {
                    let (res, o) = lhs.overflowing_add_unsigned(rhs);
                    if o && cfg!(debug_assertions) {
                        panic!("overflow in add_u");
                    }
                    res
                })
            }
            pub const fn sub_u(lhs: $UI, rhs: $U) -> $UI {
                if_unsigned!($signedness {
                    lhs - rhs
                } else {
                    let (res, o) = lhs.overflowing_sub_unsigned(rhs);
                    if o && cfg!(debug_assertions) {
                        panic!("overflow in sub_u");
                    }
                    res
                })
            }
            pub const fn wadd_u(lhs: $UI, rhs: $U) -> $UI {
                // todo: document wrapping behavior when projecting
                if_unsigned!($signedness {
                    lhs.wrapping_add(rhs)
                } else {
                    lhs.wrapping_add_unsigned(rhs)
                })
            }
            pub const fn wsub_u(lhs: $UI, rhs: $U) -> $UI {
                // todo: document wrapping behavior when projecting
                if_unsigned!($signedness {
                    lhs.wrapping_sub(rhs)
                } else {
                    lhs.wrapping_sub_unsigned(rhs)
                })
            }
            pub const fn chadd_u(lhs: $UI, rhs: $U) -> Option<$UI> {
                if_unsigned!($signedness {
                    lhs.checked_add(rhs)
                } else {
                    lhs.checked_add_unsigned(rhs)
                })
            }
            pub const fn wadd_i(lhs: $UI, rhs: $I) -> $UI {
                // todo: document wrapping behavior when projecting
                if_unsigned!($signedness {
                    lhs.wrapping_add_signed(rhs)
                } else {
                    lhs.wrapping_add(rhs)
                })
            }
            pub const fn wsub_i(lhs: $UI, rhs: $I) -> $UI {
                // todo: document wrapping behavior when projecting
                if_unsigned!($signedness {
                    lhs.wrapping_sub(rhs as $UI)
                } else {
                    lhs.wrapping_sub(rhs)
                })
            }
            pub const fn usub(lhs: $UI, rhs: $UI) -> $U {
                if_unsigned!($signedness {
                    lhs - rhs
                } else {
                    debug_assert!(rhs <= lhs);
                    <$U>::wrapping_sub(lhs as $U, rhs as $U)
                })
            }
            pub const fn wusub(lhs: $UI, rhs: $UI) -> $U {
                // todo: document wrapping behavior when projecting
                if_unsigned!($signedness {
                    lhs.wrapping_sub(rhs)
                } else {
                    <$U>::wrapping_sub(lhs as $U, rhs as $U)
                })
            }
        }
    };
    ($U:ty | $I:ty) => (
        coord_ops!(unsigned $U, $U, $I);
        coord_ops!(signed $I, $U, $I);
        coord_ops!(common $U, $U);
        coord_ops!(common $I, $U);
    )
}

coord_ops!(u8 | i8);
coord_ops!(u16 | i16);
coord_ops!(u32 | i32);
coord_ops!(u64 | i64);
coord_ops!(usize | isize);
