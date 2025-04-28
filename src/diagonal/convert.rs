//! Conversions from and into diagonal iterators.

use crate::macros::{fx, fy};
use crate::math::Num;
use crate::{AnyDiagonal, AnyOctant, Diagonal};

impl<const FX: bool, const FY: bool, T: Num> Diagonal<FX, FY, T> {
    /// Transmutes the directions of this [`Diagonal`].
    ///
    /// ## Safety
    ///
    /// The caller must ensure that `FX0` and `FY0` are equal to `FX` and `FY`.
    #[inline]
    const unsafe fn transmute<const FX0: bool, const FY0: bool>(self) -> Diagonal<FX0, FY0, T> {
        debug_assert!(FX == FX0);
        debug_assert!(FY == FY0);
        Diagonal { x1: self.x1, y1: self.y1, x2: self.x2 }
    }

    /// Constructs a [`Diagonal`] by fixing the directions of an [`AnyDiagonal`].
    ///
    /// Returns [`None`] if the [`AnyDiagonal`] is not aligned with `FX` and `FY`.
    #[inline]
    pub const fn try_from_any_diagonal(diagonal: AnyDiagonal<T>) -> Option<Self> {
        // SAFETY: match sets the correct FX and FY for transmute(), or returns None.
        unsafe {
            match diagonal {
                AnyDiagonal::Diagonal0(me) if !FX && !FY => Some(me.transmute()),
                AnyDiagonal::Diagonal1(me) if !FX && FY => Some(me.transmute()),
                AnyDiagonal::Diagonal2(me) if FX && !FY => Some(me.transmute()),
                AnyDiagonal::Diagonal3(me) if FX && FY => Some(me.transmute()),
                _ => None,
            }
        }
    }

    /// Constructs a [`Diagonal`] by fixing the directions of an [`AnyOctant`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not diagonal or not aligned with `FX` and `FY`.
    #[inline]
    pub const fn try_from_any_octant(octant: AnyOctant<T>) -> Option<Self> {
        // SAFETY: match sets the correct FX and FY for transmute(), or returns None.
        unsafe {
            match octant {
                AnyOctant::Diagonal0(me) if !FX && !FY => Some(me.transmute()),
                AnyOctant::Diagonal1(me) if !FX && FY => Some(me.transmute()),
                AnyOctant::Diagonal2(me) if FX && !FY => Some(me.transmute()),
                AnyOctant::Diagonal3(me) if FX && FY => Some(me.transmute()),
                _ => None,
            }
        }
    }

    /// Erases the directions of this [`Diagonal`], returning an [`AnyDiagonal`].
    #[inline]
    pub const fn into_any_diagonal(self) -> AnyDiagonal<T> {
        // SAFETY: fx! and fy! set up the correct FX and FY for transmute().
        unsafe {
            fx! {
                fy! {
                    AnyDiagonal::Diagonal0(self.transmute()),
                    AnyDiagonal::Diagonal1(self.transmute()),
                },
                fy! {
                    AnyDiagonal::Diagonal2(self.transmute()),
                    AnyDiagonal::Diagonal3(self.transmute()),
                }
            }
        }
    }

    /// Erases the directions of this [`Diagonal`], returning an [`AnyOctant`].
    #[inline]
    pub const fn into_any_octant(self) -> AnyOctant<T> {
        // SAFETY: fx! and fy! set up the correct FX and FY for transmute().
        unsafe {
            fx! {
                fy! {
                    AnyOctant::Diagonal0(self.transmute()),
                    AnyOctant::Diagonal1(self.transmute()),
                },
                fy! {
                    AnyOctant::Diagonal2(self.transmute()),
                    AnyOctant::Diagonal3(self.transmute()),
                }
            }
        }
    }
}

impl<T: Num> AnyDiagonal<T> {
    /// Constructs an [`AnyDiagonal`] by erasing the directions of a [`Diagonal`].
    #[inline]
    pub const fn from_diagonal<const FX: bool, const FY: bool>(
        diagonal: Diagonal<FX, FY, T>,
    ) -> Self {
        diagonal.into_any_diagonal()
    }

    /// Constructs an [`AnyDiagonal`] from an [`AnyOctant`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not diagonal.
    #[inline]
    pub const fn try_from_any_octant(octant: AnyOctant<T>) -> Option<Self> {
        match octant {
            AnyOctant::Diagonal0(me) => Some(Self::Diagonal0(me)),
            AnyOctant::Diagonal1(me) => Some(Self::Diagonal1(me)),
            AnyOctant::Diagonal2(me) => Some(Self::Diagonal2(me)),
            AnyOctant::Diagonal3(me) => Some(Self::Diagonal3(me)),
            _ => None,
        }
    }

    /// Fixes the directions of this [`AnyDiagonal`], returning a [`Diagonal`].
    ///
    /// Returns [`None`] if this [`AnyDiagonal`] is not aligned with `FX` and `FY`.
    #[inline]
    pub const fn try_into_diagonal<const FX: bool, const FY: bool>(
        self,
    ) -> Option<Diagonal<FX, FY, T>> {
        Diagonal::try_from_any_diagonal(self)
    }

    /// Erases the diagonality of this [`AnyDiagonal`], returning an [`AnyOctant`].
    #[inline]
    pub const fn into_any_octant(self) -> AnyOctant<T> {
        match self {
            Self::Diagonal0(me) => AnyOctant::Diagonal0(me),
            Self::Diagonal1(me) => AnyOctant::Diagonal1(me),
            Self::Diagonal2(me) => AnyOctant::Diagonal2(me),
            Self::Diagonal3(me) => AnyOctant::Diagonal3(me),
        }
    }
}

impl<const FX: bool, const FY: bool, T: Num> From<Diagonal<FX, FY, T>> for AnyDiagonal<T> {
    #[inline]
    fn from(diagonal: Diagonal<FX, FY, T>) -> Self {
        diagonal.into_any_diagonal()
    }
}

impl<const FX: bool, const FY: bool, T: Num> TryFrom<AnyDiagonal<T>> for Diagonal<FX, FY, T> {
    type Error = ();

    #[inline]
    fn try_from(diagonal: AnyDiagonal<T>) -> Result<Self, Self::Error> {
        diagonal.try_into_diagonal().ok_or(())
    }
}
