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
    P = 1,
    /// A step in the negative direction.
    N = -1,
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
    /// as an absolute value without an overflow check.
    ///
    /// # Safety
    ///
    /// `c0` must be less or equal to `c1`.
    #[inline]
    #[must_use]
    pub const unsafe fn d_unchecked(c1: C, c0: C) -> U {
        debug_assert!(c0 <= c1);
        #[expect(clippy::cast_sign_loss)]
        U::wrapping_sub(c1 as U, c0 as U)
    }

    /// Returns the absolute difference between `c0` and `c1`,
    /// and the sign of `c1 - c0`.
    #[inline]
    #[must_use]
    pub const fn sd(c0: C, c1: C) -> (S, U) {
        let (sign, min, max) = if c0 <= c1 { (S::P, c0, c1) } else { (S::N, c1, c0) };
        #[expect(clippy::cast_sign_loss)]
        let delta = U::wrapping_sub(max as U, min as U);
        (sign, delta)
    }
}
