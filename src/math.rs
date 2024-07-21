//! ## Math types
//!
//! Contains the math types and helper macros.

pub trait Num {
    type I2: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display;
    type U: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display;
    type U2: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display;
}

/// A generic point on a Cartesian plane.
pub type Point<T> = (T, T);

/// Absolute offset between two [points](Point).
pub type Delta<T> = (<T as Num>::U, <T as Num>::U);

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

impl Math<i8> {
    /// Subtracts two signed integers, returning the unsigned difference.
    ///
    /// *`min` must be less or equal to `max`.*
    pub const fn delta(max: i8, min: i8) -> <i8 as Num>::U {
        debug_assert!(min <= max);
        #[allow(clippy::cast_sign_loss)]
        <i8 as Num>::U::wrapping_sub(max as _, min as _)
    }

    /// Subtracts two unsigned integers, returning the wide signed difference.
    pub const fn error(a: <i8 as Num>::U, b: <i8 as Num>::U) -> <i8 as Num>::I2 {
        <i8 as Num>::I2::wrapping_sub(a as _, b as _)
    }

    /// Multiplies two narrow unsigned integers, widening the result.
    pub const fn wide_mul(a: <i8 as Num>::U, b: <i8 as Num>::U) -> <i8 as Num>::U2 {
        <i8 as Num>::U2::wrapping_mul(a as _, b as _)
    }

    /// Doubles a narrow unsigned integer, widening the result.
    pub const fn double(a: <i8 as Num>::U) -> <i8 as Num>::U2 {
        <i8 as Num>::U2::wrapping_shl(a as _, 1)
    }

    /// Divides an unsigned integer by 2 with rounding.
    pub const fn half(a: <i8 as Num>::U) -> <i8 as Num>::U {
        let half = <i8 as Num>::U2::wrapping_add(a as _, 1).wrapping_shr(1);
        debug_assert!(half <= <i8 as Num>::U::MAX as _);
        #[allow(clippy::cast_possible_truncation)]
        return half as _;
    }

    /// Divides a wide unsigned integer by a non-zero narrow unsigned integer,
    /// returning the narrow quotient and remainder.
    ///
    /// *The divisor must be non-zero for this to be sound,
    /// and the quotient must fit into the narrow type.*
    pub const fn div_rem(
        a: <i8 as Num>::U2,
        b: <i8 as Num>::U,
    ) -> (<i8 as Num>::U, <i8 as Num>::U) {
        debug_assert!(b != 0);
        let (Some(q), Some(r)) =
            (<i8 as Num>::U2::checked_div(a, b as _), <i8 as Num>::U2::checked_rem(a, b as _))
        else {
            // SAFETY: the dividend is non-zero.
            unsafe { core::hint::unreachable_unchecked() }
        };
        debug_assert!(q <= u8::MAX as _);
        #[allow(clippy::cast_possible_truncation)]
        (q as _, r as _)
    }
}
