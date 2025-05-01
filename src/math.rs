//! ## Math types

use crate::macros::derive::nums;

/// Primitive numeric type.
pub trait Prim
where
    Self: Copy + Eq + Ord + Default,
    Self: core::hash::Hash + core::fmt::Debug,
{
}

macro_rules! impl_prim {
    ($($T:ty),+) => {
        $(impl Prim for $T {})+
    };
}

nums!(impl_prim);

/// Numeric type representing a coordinate.
pub trait Num: Prim {
    /// Wide signed type for differences of [`Self::U`] values.
    type I2: Prim;
    /// Unsigned type for absolute offsets.
    type U: Prim;
    /// Wide unsigned type for multiplying offsets.
    type U2: Prim;
}

/// A generic 2D point.
pub type Point<T> = (T, T);

/// Absolute offset between two [points](Point).
pub type Delta<T> = (<T as Num>::U, <T as Num>::U);

/// Product between two [`Delta`] offsets.
pub type Delta2<T> = (<T as Num>::U2, <T as Num>::U2);

macro_rules! impl_num {
    ($($(#[$cfg_size:meta])? [$i:ty, $i2:ty, $u:ty, $u2:ty]),+ $(,)?) => {
        $(
            $(#[$cfg_size])?
            impl Num for $i {
                type I2 = $i2;
                type U = $u;
                type U2 = $u2;
            }
            $(#[$cfg_size])?
            impl Num for $u {
                type I2 = $i2;
                type U = Self;
                type U2 = $u2;
            }
        )+
    };
}

impl_num!(
    [i8, i16, u8, u16],
    [i16, i32, u16, u32],
    [i32, i64, u32, u64],
    [i64, i128, u64, u128],
    #[cfg(target_pointer_width = "16")]
    [isize, i32, usize, u32],
    #[cfg(target_pointer_width = "32")]
    [isize, i64, usize, u64],
    #[cfg(target_pointer_width = "64")]
    [isize, i128, usize, u128],
);

/// Generic math functions.
pub struct Math<T>(T);

macro_rules! impl_math_base {
    ($T:ty) => {
        impl Math<$T> {
            /// Subtracts two integers, returning the unsigned difference.
            ///
            /// Requirement: `min <= max`.
            #[inline(always)]
            #[allow(clippy::cast_sign_loss)]
            pub const fn sub_tt(max: $T, min: $T) -> <$T as Num>::U {
                debug_assert!(min <= max);
                <$T as Num>::U::wrapping_sub(max as _, min as _)
            }

            /// Adds an unsigned offset to an integer.
            #[inline(always)]
            #[allow(clippy::cast_possible_wrap)]
            pub const fn add_tu(value: $T, delta: <$T as Num>::U) -> $T {
                value.wrapping_add(delta as _)
            }

            /// Subtracts an unsigned offset from an integer.
            ///
            /// Requirement: `delta <= value`.
            #[inline(always)]
            #[allow(clippy::cast_possible_wrap)]
            pub const fn sub_tu(value: $T, delta: <$T as Num>::U) -> $T {
                value.wrapping_sub(delta as _)
            }
        }
    };
}

nums!(impl_math_base);

macro_rules! impl_math_extended {
    ($T:ty) => {
        impl Math<$T> {
            /// Subtracts two unsigned integers, returning the wide signed difference.
            #[inline(always)]
            pub const fn sub_uu(a: <$T as Num>::U, b: <$T as Num>::U) -> <$T as Num>::I2 {
                <$T as Num>::I2::wrapping_sub(a as _, b as _)
            }

            /// Multiplies two narrow unsigned integers, widening the result.
            #[inline(always)]
            pub const fn mul_uu(a: <$T as Num>::U, b: <$T as Num>::U) -> <$T as Num>::U2 {
                <$T as Num>::U2::wrapping_mul(a as _, b as _)
            }

            /// Multiplies an unsigned integer by 2, widening the result.
            #[inline(always)]
            pub const fn mul_2u(a: <$T as Num>::U) -> <$T as Num>::U2 {
                <$T as Num>::U2::wrapping_shl(a as _, 1)
            }

            /// Divides an unsigned integer by 2 and returns the quotient with the remainder.
            #[inline(always)]
            pub const fn div_2u(a: <$T as Num>::U) -> (<$T as Num>::U, <$T as Num>::U) {
                (a.wrapping_shr(1), a & 1)
            }

            /// Divides a wide unsigned integer by a non-zero narrow unsigned integer,
            /// returning the narrow quotient and remainder.
            ///
            /// ### Safety
            /// The divisor must be non-zero, and the quotient must fit into the narrow type.
            #[inline(always)]
            pub const unsafe fn div_u2u(
                a: <$T as Num>::U2,
                b: <$T as Num>::U,
            ) -> (<$T as Num>::U, <$T as Num>::U) {
                debug_assert!(b != 0);
                let (Some(q), Some(r)) = (
                    <$T as Num>::U2::checked_div(a, b as _),
                    <$T as Num>::U2::checked_rem(a, b as _),
                ) else {
                    core::hint::unreachable_unchecked()
                };
                debug_assert!(q <= <$T as Num>::U::MAX as _);
                #[expect(clippy::cast_possible_truncation)]
                (q as _, r as _)
            }
        }
    };
}

nums!(impl_math_extended, cfg_octant_64);

#[cfg(test)]
mod static_tests {
    use super::*;

    #[cfg(target_pointer_width = "16")]
    #[test]
    const fn isize_16_bit_is_num() {
        static_assertions::assert_impl_all!(isize: Num);
        static_assertions::assert_type_eq_all!(<isize as Num>::I2, i32);
        static_assertions::assert_type_eq_all!(<isize as Num>::U2, u32);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    const fn isize_32_bit_is_num() {
        static_assertions::assert_impl_all!(isize: Num);
        static_assertions::assert_type_eq_all!(<isize as Num>::I2, i64);
        static_assertions::assert_type_eq_all!(<isize as Num>::U2, u64);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    const fn isize_64_bit_is_num() {
        static_assertions::assert_impl_all!(isize: Num);
        static_assertions::assert_type_eq_all!(<isize as Num>::I2, i128);
        static_assertions::assert_type_eq_all!(<isize as Num>::U2, u128);
    }
}
