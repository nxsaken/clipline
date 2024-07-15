//! ## Math utilities

/// Primitive numeric type.
#[allow(clippy::type_repetition_in_bounds)]
pub trait Num
where
    Self: Copy + Eq + Ord + Default,
    Self: core::hash::Hash,
    Self: core::fmt::Debug,
{
}

macro_rules! num_impl {
    ($signed:ty, $unsigned:ty) => {
        impl Num for $signed {}
        impl Num for $unsigned {}
    };
}

num_impl!(i8, u8);
num_impl!(i16, u16);
num_impl!(i32, u32);
num_impl!(i64, u64);
num_impl!(i128, u128);
num_impl!(isize, usize);

/// Numeric type for representing coordinates.
pub trait Coord: Num {
    /// Offset representation.
    type Delta: Num;
    /// Error representation.
    type Error: Num;
    /// Double offset representation.
    type Delta2: Num;
}

macro_rules! coord_impl {
    ($signed:ty, $unsigned:ty, $double_signed:ty, $double_unsigned:ty) => {
        impl Coord for $signed {
            type Delta = $unsigned;
            type Error = $double_signed;
            type Delta2 = $double_unsigned;
        }
        impl Coord for $unsigned {
            type Delta = $unsigned;
            type Error = $double_signed;
            type Delta2 = $double_unsigned;
        }
    };
}

coord_impl!(i8, u8, i16, u16);
coord_impl!(i16, u16, i32, u32);
coord_impl!(i32, u32, i64, u64);
coord_impl!(i64, u64, i128, u128);
#[cfg(target_pointer_width = "16")]
coord_impl!(isize, usize, i32, u32);
#[cfg(target_pointer_width = "32")]
coord_impl!(isize, usize, i64, u64);
#[cfg(target_pointer_width = "64")]
coord_impl!(isize, usize, i128, u128);

// TODO(#12): support all integer types, including unsigned.
/// A generic point on a Cartesian plane.
pub type Point<T> = (T, T);

/// A generic offset between two [points](Point) on a Cartesian plane.
pub type Delta<T> = (<T as Coord>::Delta, <T as Coord>::Delta);

/// A generic double offset between two [points](Point) on a Cartesian plane.
pub type Delta2<T> = (<T as Coord>::Delta2, <T as Coord>::Delta2);
