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
