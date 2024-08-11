//! ## Math types

/// Numeric type representing a coordinate.
pub trait Num {
    /// Wide signed type for differences of [`Self::U`] values.
    type I2: Copy + Eq + Ord + core::fmt::Debug;
    /// Unsigned type for absolute offsets.
    type U: Copy + Eq + Ord + core::fmt::Debug;
    /// Wide unsigned type for multiplying offsets.
    type U2: Copy + Eq + Ord + core::fmt::Debug;
}

/// A generic 2D point.
pub type Point<T> = (T, T);

/// Absolute offset between two [points](Point).
pub type Delta<T> = (<T as Num>::U, <T as Num>::U);

/// Product between two [`Delta`] offsets.
pub type Delta2<T> = (<T as Num>::U2, <T as Num>::U2);

macro_rules! num_impl {
    ($([$i:ty, $i2:ty, $u:ty, $u2:ty]$(,)?)+) => {
        $(
        impl Num for $i {
            type I2 = $i2;
            type U = $u;
            type U2 = $u2;
        }
        impl Num for $u {
            type I2 = $i2;
            type U = Self;
            type U2 = $u2;
        }
        )+
    };
}

num_impl!([i8, i16, u8, u16], [i16, i32, u16, u32], [i32, i64, u32, u64], [i64, i128, u64, u128],);
#[cfg(target_pointer_width = "16")]
num_impl!([isize, i32, usize, u32]);
#[cfg(target_pointer_width = "32")]
num_impl!([isize, i64, usize, u64]);
#[cfg(target_pointer_width = "64")]
num_impl!([isize, i128, usize, u128]);

/// Generic math functions.
pub struct Math<T>(T);

macro_rules! min_math_impl {
    ($T:ty) => {
        impl Math<$T> {
            /// Subtracts two signed integers, returning the unsigned difference.
            ///
            /// *`min` must be less or equal to `max`.*
            #[inline(always)]
            pub const fn delta(max: $T, min: $T) -> <$T as Num>::U {
                debug_assert!(min <= max);
                #[allow(clippy::cast_sign_loss)]
                <$T as Num>::U::wrapping_sub(max as _, min as _)
            }
        }
    };
}

min_math_impl!(i8);
min_math_impl!(u8);
min_math_impl!(i16);
min_math_impl!(u16);
min_math_impl!(i32);
min_math_impl!(u32);
min_math_impl!(i64);
min_math_impl!(u64);
min_math_impl!(isize);
min_math_impl!(usize);

macro_rules! math_impl {
    ($T:ty) => {
        impl Math<$T> {
            /// Subtracts two unsigned integers, returning the wide signed difference.
            #[inline(always)]
            pub const fn error(a: <$T as Num>::U, b: <$T as Num>::U) -> <$T as Num>::I2 {
                <$T as Num>::I2::wrapping_sub(a as _, b as _)
            }

            /// Multiplies two narrow unsigned integers, widening the result.
            #[inline(always)]
            pub const fn wide_mul(a: <$T as Num>::U, b: <$T as Num>::U) -> <$T as Num>::U2 {
                <$T as Num>::U2::wrapping_mul(a as _, b as _)
            }

            /// Doubles a narrow unsigned integer, widening the result.
            #[inline(always)]
            pub const fn double(a: <$T as Num>::U) -> <$T as Num>::U2 {
                <$T as Num>::U2::wrapping_shl(a as _, 1)
            }

            /// Divides an unsigned integer by 2 and returns the quotient with the remainder.
            #[inline(always)]
            pub const fn half(a: <$T as Num>::U) -> (<$T as Num>::U, <$T as Num>::U) {
                (a.wrapping_shr(1), a & 1)
            }

            /// Divides a wide unsigned integer by a non-zero narrow unsigned integer,
            /// returning the narrow quotient and remainder.
            ///
            /// ### Safety
            /// The divisor must be non-zero, and the quotient must fit into the narrow type.
            #[inline(always)]
            pub const unsafe fn div_rem(
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
                #[allow(clippy::cast_possible_truncation)]
                (q as _, r as _)
            }
        }
    };
}

math_impl!(i8);
math_impl!(u8);
math_impl!(i16);
math_impl!(u16);
math_impl!(i32);
math_impl!(u32);
math_impl!(i64);
math_impl!(u64);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
math_impl!(isize);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
math_impl!(usize);
#[cfg(target_pointer_width = "64")]
math_impl!(isize);
#[cfg(target_pointer_width = "64")]
math_impl!(usize);

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
