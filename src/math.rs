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

#[allow(non_camel_case_types)]
pub(crate) struct ops<C: Coord>(C);

macro_rules! coord_ops {
    (common $C:ty, $U:ty) => {
        impl ops<$C> {
            pub const fn min(a: $C, b: $C) -> $C {
                if a <= b { a } else { b }
            }
            pub const fn max(a: $C, b: $C) -> $C {
                if b <= a { a } else { b }
            }
            pub const fn min_adj(incl: $C, excl: $C) -> $C {
                if incl < excl { incl + 1 } else { excl }
            }
            pub const fn max_adj(incl: $C, excl: $C) -> $C {
                if excl < incl { incl - 1 } else { excl }
            }
            pub const fn abs_diff_signed(lhs: $C, rhs: $C, sign: i8) -> $U {
                if sign > 0 {
                    Self::abs_diff(lhs, rhs)
                } else {
                    Self::abs_diff(rhs, lhs)
                }
            }
            pub const fn abs_diff_sign(lhs: $C, rhs: $C) -> ($U, i8) {
                if rhs <= lhs {(
                    Self::abs_diff(lhs, rhs), 1)
                } else {
                    (Self::abs_diff(rhs, lhs), -1)
                }
            }
        }
    };
    (unsigned $U:ty, $I:ty) => {
        impl ops<$U> {
            pub const fn add_u(lhs: $U, rhs: $U) -> $U {
                lhs + rhs
            }
            pub const fn sub_u(lhs: $U, rhs: $U) -> $U {
                lhs - rhs
            }
            pub const fn add_u_signed(lhs: $U, rhs: $U, sign: i8) -> $U {
                if sign > 0 { Self::add_u(lhs, rhs) } else { Self::sub_u(lhs, rhs) }
            }
            pub const fn sub_u_signed(lhs: $U, rhs: $U, sign: i8) -> $U {
                if sign > 0 { Self::sub_u(lhs, rhs) } else { Self::add_u(lhs, rhs) }
            }
            pub const fn checked_add_u(lhs: $U, rhs: $U) -> Option<$U> {
                lhs.checked_add(rhs)
            }
            pub const fn checked_sub_u(lhs: $U, rhs: $U) -> Option<$U> {
                lhs.checked_sub(rhs)
            }
            pub const fn add_i(lhs: $U, rhs: $I) -> $U {
                let (res, o) = lhs.overflowing_add_signed(rhs);
                if o && cfg!(debug_assertions) {
                    panic!("overflow in add_signed");
                }
                res
            }
            pub const fn sub_i(lhs: $U, rhs: $I) -> $U {
                let (res, o) = lhs.overflowing_sub(rhs as $U);
                if (o ^ (rhs < 0)) && cfg!(debug_assertions) {
                    panic!("overflow in sub_signed");
                }
                res
            }
            pub const fn abs_diff(lhs: $U, rhs: $U) -> $U {
                lhs - rhs
            }
        }
    };
    (signed $I:ty, $U:ty) => {
        impl ops<$I> {
            pub const fn add_u(lhs: $I, rhs: $U) -> $I {
                let (res, o) = lhs.overflowing_add_unsigned(rhs);
                if o && cfg!(debug_assertions) {
                    panic!("overflow in add_unsigned");
                }
                res
            }
            pub const fn sub_u(lhs: $I, rhs: $U) -> $I {
                let (res, o) = lhs.overflowing_sub_unsigned(rhs);
                if o && cfg!(debug_assertions) {
                    panic!("overflow in add_unsigned");
                }
                res
            }
            pub const fn add_u_signed(lhs: $I, rhs: $U, sign: i8) -> $I {
                if sign > 0 { Self::add_u(lhs, rhs) } else { Self::sub_u(lhs, rhs) }
            }
            pub const fn sub_u_signed(lhs: $I, rhs: $U, sign: i8) -> $I {
                if sign > 0 { Self::sub_u(lhs, rhs) } else { Self::add_u(lhs, rhs) }
            }
            pub const fn checked_add_u(lhs: $I, rhs: $U) -> Option<$I> {
                lhs.checked_add_unsigned(rhs)
            }
            pub const fn checked_sub_u(lhs: $I, rhs: $U) -> Option<$I> {
                lhs.checked_sub_unsigned(rhs)
            }
            pub const fn add_i(lhs: $I, rhs: $I) -> $I {
                lhs + rhs
            }
            pub const fn sub_i(lhs: $I, rhs: $I) -> $I {
                lhs - rhs
            }
            pub const fn abs_diff(lhs: $I, rhs: $I) -> $U {
                debug_assert!(rhs <= lhs);
                <$U>::wrapping_sub(lhs as $U, rhs as $U)
            }
        }
    };
    ($U:ty, $I:ty) => (
        coord_ops!(unsigned $U, $I);
        coord_ops!(signed $I, $U);
        coord_ops!(common $U, $U);
        coord_ops!(common $I, $U);
    )
}

coord_ops!(u8, i8);
coord_ops!(u16, i16);
coord_ops!(u32, i32);
coord_ops!(u64, i64);
coord_ops!(usize, isize);
