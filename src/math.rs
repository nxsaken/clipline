/// A signed or unsigned coordinate.
pub type C = i8;

/// An unsigned offset or extent.
pub type U = u8;

/// A wide product of unsigned values.
pub type U2 = u16;

/// A wide signed difference of unsigned values.
pub type I2 = i16;

/// A sign representing a step by one unit in the positive or negative direction.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Default)]
#[repr(i8)]
pub enum S {
    /// A step in the positive direction.
    #[default]
    Pos = 1,
    /// A step in the negative direction.
    Neg = -1,
}

/// A pair of coordinates.
pub type CxC = (C, C);

/// A pair of unsigned offsets or extents.
pub type UxU = (U, U);

/// A pair of signs.
pub type SxS = (S, S);

/// Math operations.
#[allow(non_camel_case_types)]
pub struct ops;

impl ops {
    /// Returns the difference between two coordinates
    /// as an unsigned integer without an overflow check.
    ///
    /// # Safety
    ///
    /// `rhs <= lhs`.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_abs_diff(lhs: C, rhs: C) -> U {
        debug_assert!(rhs <= lhs);
        #[expect(clippy::cast_sign_loss)]
        U::wrapping_sub(lhs as U, rhs as U)
    }

    /// Returns the sign of `lhs - rhs`, and `|lhs - rhs|` as an unsigned integer.
    #[inline]
    #[must_use]
    pub const fn abs_diff(lhs: C, rhs: C) -> (S, U) {
        let (sign, max, min) = if rhs <= lhs { (S::Pos, lhs, rhs) } else { (S::Neg, rhs, lhs) };
        // SAFETY: min <= max.
        let diff = unsafe { Self::unchecked_abs_diff(max, min) };
        (sign, diff)
    }

    /// Adds an unsigned integer to a coordinate without an overflow check.
    ///
    /// # Safety
    ///
    /// `lhs + rhs` must not overflow.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_add_unsigned(lhs: C, rhs: U) -> C {
        let maybe = lhs.checked_add_unsigned(rhs);
        if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: lhs + rhs cannot overflow.
            unsafe { maybe.unwrap_unchecked() }
        }
    }

    /// Subtracts an unsigned integer from a coordinate without an underflow check.
    ///
    /// # Safety
    ///
    /// `lhs - rhs` must not underflow.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_sub_unsigned(lhs: C, rhs: U) -> C {
        let maybe = lhs.checked_sub_unsigned(rhs);
        if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: lhs - rhs cannot underflow.
            unsafe { maybe.unwrap_unchecked() }
        }
    }

    /// Adds a signed unit to a coordinate without an overflow check.
    ///
    /// # Safety
    ///
    /// `lhs + rhs` must not overflow or underflow.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_add_sign(lhs: C, rhs: S) -> C {
        let maybe = lhs.checked_add(rhs as C);
        if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: lhs + rhs cannot overflow.
            unsafe { maybe.unwrap_unchecked() }
        }
    }

    /// Subtracts a signed unit from a coordinate without an overflow check.
    ///
    /// # Safety
    ///
    /// `lhs - rhs` must not overflow or underflow.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_sub_sign(lhs: C, rhs: S) -> C {
        let maybe = lhs.checked_sub(rhs as C);
        if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: lhs - rhs cannot overflow.
            unsafe { maybe.unwrap_unchecked() }
        }
    }

    /// Adds an unsigned integer to a wide signed integer without an overflow check.
    ///
    /// # Safety
    ///
    /// `lhs + rhs` must not overflow.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_wide_add_unsigned(lhs: I2, rhs: U) -> I2 {
        let maybe = lhs.checked_add_unsigned(rhs as U2);
        if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: lhs + rhs cannot overflow.
            unsafe { maybe.unwrap_unchecked() }
        }
    }

    /// Subtracts an unsigned integer from a wide signed integer without an underflow check.
    ///
    /// # Safety
    ///
    /// `lhs - rhs` must not underflow.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_wide_sub_unsigned(lhs: I2, rhs: U) -> I2 {
        let maybe = lhs.checked_sub_unsigned(rhs as U2);
        if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: lhs - rhs cannot underflow.
            unsafe { maybe.unwrap_unchecked() }
        }
    }

    /// Divides an unsigned integer in half and calculates the remainder.
    #[inline]
    #[must_use]
    pub const fn half_rem(lhs: U) -> (U, U) {
        let maybe = lhs.checked_shr(1);
        let half = if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: 1 is smaller than the number of bits in a U.
            unsafe { maybe.unwrap_unchecked() }
        };
        let rem = lhs & 1;
        (half, rem)
    }

    /// Adds an unsigned integer to an unsigned integer without an overflow check.
    ///
    /// # Safety
    ///
    /// `lhs + rhs` must not overflow.
    #[inline]
    #[must_use]
    pub const unsafe fn unchecked_unsigned_add(lhs: U, rhs: U) -> U {
        let maybe = lhs.checked_add(rhs);
        if cfg!(debug_assertions) {
            maybe.unwrap()
        } else {
            // SAFETY: lhs + rhs cannot overflow.
            unsafe { maybe.unwrap_unchecked() }
        }
    }
}
