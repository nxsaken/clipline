//! ## Math types
//!
//! Contains the math types and helper macros.

/// Numeric type representing a coordinate.
pub trait Num {
    /// Wide signed type for differences of [`Self::U`] values.
    type I2: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display;
    /// Unsigned type for absolute offsets.
    type U: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display;
    /// Wide unsigned type for multiplying offsets.
    type U2: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display;
}

/// A generic point on a Cartesian plane.
pub type Point<T> = (T, T);

/// Absolute offset between two [points](Point).
pub type Delta<T> = (<T as Num>::U, <T as Num>::U);

/// Quadratic offset between two [points](Point).
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

num_impl!(
    [i8, i16, u8, u16],
    [i16, i32, u16, u32],
    [i32, i64, u32, u64],
    // FIXME: possible footgun?
    [i64, i128, u64, u128],
);
#[cfg(target_pointer_width = "16")]
num_impl!([isize, i32, usize, u32]);
#[cfg(target_pointer_width = "32")]
num_impl!([isize, i64, usize, u64]);
#[cfg(target_pointer_width = "64")]
num_impl!([isize, i128, usize, u128]);

/// Generic math functions.
pub struct Math<T>(T);

macro_rules! math_impl {
    ($num:ty) => {
        impl Math<$num> {
            /// Subtracts two signed integers, returning the unsigned difference.
            ///
            /// *`min` must be less or equal to `max`.*
            pub const fn delta(max: $num, min: $num) -> <$num as Num>::U {
                debug_assert!(min <= max);
                #[allow(clippy::cast_sign_loss)]
                <$num as Num>::U::wrapping_sub(max as _, min as _)
            }

            /// Subtracts two unsigned integers, returning the wide signed difference.
            pub const fn error(a: <$num as Num>::U, b: <$num as Num>::U) -> <$num as Num>::I2 {
                <$num as Num>::I2::wrapping_sub(a as _, b as _)
            }

            /// Multiplies two narrow unsigned integers, widening the result.
            pub const fn wide_mul(a: <$num as Num>::U, b: <$num as Num>::U) -> <$num as Num>::U2 {
                <$num as Num>::U2::wrapping_mul(a as _, b as _)
            }

            /// Doubles a narrow unsigned integer, widening the result.
            pub const fn double(a: <$num as Num>::U) -> <$num as Num>::U2 {
                <$num as Num>::U2::wrapping_shl(a as _, 1)
            }

            /// Divides an unsigned integer by 2 with rounding.
            pub const fn half(a: <$num as Num>::U) -> <$num as Num>::U {
                let half = <$num as Num>::U2::wrapping_add(a as _, 1).wrapping_shr(1);
                debug_assert!(half <= <$num as Num>::U::MAX as _);
                #[allow(clippy::cast_possible_truncation)]
                return half as _;
            }

            /// Divides a wide unsigned integer by a non-zero narrow unsigned integer,
            /// returning the narrow quotient and remainder.
            ///
            /// ### Safety
            /// The divisor must be non-zero for this to be sound,
            /// and the quotient must fit into the narrow type.
            pub const unsafe fn div_rem(
                a: <$num as Num>::U2,
                b: <$num as Num>::U,
            ) -> (<$num as Num>::U, <$num as Num>::U) {
                debug_assert!(b != 0);
                let (Some(q), Some(r)) = (
                    <$num as Num>::U2::checked_div(a, b as _),
                    <$num as Num>::U2::checked_rem(a, b as _),
                ) else {
                    core::hint::unreachable_unchecked()
                };
                debug_assert!(q <= u8::MAX as _);
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
math_impl!(isize);
math_impl!(usize);
