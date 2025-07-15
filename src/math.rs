pub trait Coord: Copy {
    type U: Copy;
    type I: Copy;
    type U2: Copy;
    type I2: Copy;
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
    (signed $unsigned:block else $signed:block) => { $signed };
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
            pub const fn add_u_signed(lhs: $UI, rhs: $U, sign: i8) -> $UI {
                if sign > 0 { Self::add_u(lhs, rhs) } else { Self::sub_u(lhs, rhs) }
            }
            pub const fn sub_u_signed(lhs: $UI, rhs: $U, sign: i8) -> $UI {
                if sign > 0 { Self::sub_u(lhs, rhs) } else { Self::add_u(lhs, rhs) }
            }
            pub const fn abs_diff_const_signed<const F: bool>(lhs: $UI, rhs: $UI) -> $U {
                if F {
                    Self::abs_diff(rhs, lhs)
                } else {
                    Self::abs_diff(lhs, rhs)
                }
            }
            pub const fn abs_diff_signed(lhs: $UI, rhs: $UI, sign: i8) -> $U {
                if sign > 0 {
                    Self::abs_diff(lhs, rhs)
                } else {
                    Self::abs_diff(rhs, lhs)
                }
            }
            pub const fn abs_diff_sign(lhs: $UI, rhs: $UI) -> ($U, i8) {
                if rhs <= lhs {(
                    Self::abs_diff(lhs, rhs), 1)
                } else {
                    (Self::abs_diff(rhs, lhs), -1)
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
            pub const fn checked_add_u(lhs: $UI, rhs: $U) -> Option<$UI> {
                if_unsigned!($signedness {
                    lhs.checked_add(rhs)
                } else {
                    lhs.checked_add_unsigned(rhs)
                })
            }
            pub const fn checked_sub_u(lhs: $UI, rhs: $U) -> Option<$UI> {
                if_unsigned!($signedness {
                    lhs.checked_sub(rhs)
                } else {
                    lhs.checked_sub_unsigned(rhs)
                })
            }
            pub const fn add_i(lhs: $UI, rhs: $I) -> $UI {
                if_unsigned!($signedness {
                    let (res, o) = lhs.overflowing_add_signed(rhs);
                    if o && cfg!(debug_assertions) {
                        panic!("overflow in add_i");
                    }
                    res
                } else {
                    lhs + rhs
                })
            }
            pub const fn sub_i(lhs: $UI, rhs: $I) -> $UI {
                if_unsigned!($signedness {
                    let (res, o) = lhs.overflowing_sub(rhs as $U);
                    if (o ^ (rhs < 0)) && cfg!(debug_assertions) {
                        panic!("overflow in sub_i");
                    }
                    res
                } else {
                    lhs - rhs
                })
            }
            pub const fn abs_diff(lhs: $UI, rhs: $UI) -> $U {
                if_unsigned!($signedness {
                    lhs - rhs
                } else {
                    debug_assert!(rhs <= lhs);
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
