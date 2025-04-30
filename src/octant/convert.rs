//! Conversions from and into octant iterators.

use super::{AnyOctant, Octant};
use crate::axis::{AnyAxis, Axis, SignedAxis};
use crate::diagonal::{AnyDiagonal, Diagonal};
use crate::macros::symmetry::{fx, fy, yx};
use crate::math::Num;

impl<const FX: bool, const FY: bool, const YX: bool, T: Num> Octant<FX, FY, YX, T> {
    /// Transmutes the directions and orientation of this [`Octant`].
    ///
    /// ## Safety
    ///
    /// The caller must ensure that `FX0`, `FY0` and `YX0` are equal to `FX`, `FY` and `YX`.
    #[inline]
    const unsafe fn transmute<const FX0: bool, const FY0: bool, const YX0: bool>(
        self,
    ) -> Octant<FX0, FY0, YX0, T> {
        debug_assert!(FX == FX0);
        debug_assert!(FY == FY0);
        debug_assert!(YX == YX0);
        Octant { x: self.x, y: self.y, error: self.error, dx: self.dx, dy: self.dy, end: self.end }
    }

    /// Constructs an [`Octant`] by fixing the directions and orientation of an [`AnyOctant`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is axis-aligned, diagonal,
    /// or not aligned with `FX`, `FY` and `YX`.
    #[inline]
    pub const fn try_from_any_octant(octant: AnyOctant<T>) -> Option<Self> {
        // SAFETY: match sets the correct FX and FY for transmute(), or returns None.
        unsafe {
            match octant {
                AnyOctant::Octant0(me) if !FX && !FY && !YX => Some(me.transmute()),
                AnyOctant::Octant1(me) if !FX && !FY && YX => Some(me.transmute()),
                AnyOctant::Octant2(me) if !FX && FY && !YX => Some(me.transmute()),
                AnyOctant::Octant3(me) if !FX && FY && YX => Some(me.transmute()),
                AnyOctant::Octant4(me) if FX && !FY && !YX => Some(me.transmute()),
                AnyOctant::Octant5(me) if FX && !FY && YX => Some(me.transmute()),
                AnyOctant::Octant6(me) if FX && FY && !YX => Some(me.transmute()),
                AnyOctant::Octant7(me) if FX && FY && YX => Some(me.transmute()),
                _ => None,
            }
        }
    }

    /// Erases the directions and orientation of this [`Octant`], returning an [`AnyOctant`].
    #[inline]
    pub const fn into_any_octant(self) -> AnyOctant<T> {
        // SAFETY: fx!, fy! and yx! set up the correct FX, FY and YX for transmute().
        unsafe {
            fx! {
                fy! {
                    yx!(AnyOctant::Octant0(self.transmute()), AnyOctant::Octant1(self.transmute())),
                    yx!(AnyOctant::Octant2(self.transmute()), AnyOctant::Octant3(self.transmute())),
                },
                fy! {
                    yx!(AnyOctant::Octant4(self.transmute()), AnyOctant::Octant5(self.transmute())),
                    yx!(AnyOctant::Octant6(self.transmute()), AnyOctant::Octant7(self.transmute())),
                },
            }
        }
    }
}

impl<T: Num> AnyOctant<T> {
    /// Constructs an [`AnyOctant`] by erasing the direction and orientation of a [`SignedAxis`].
    #[inline]
    pub const fn from_signed_axis<const F: bool, const V: bool>(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_any_octant()
    }

    /// Constructs an [`AnyOctant`] by erasing the orientation of an [`Axis`].
    #[inline]
    pub const fn from_axis<const V: bool>(axis: Axis<V, T>) -> Self {
        axis.into_any_octant()
    }

    /// Constructs an [`AnyOctant`] by erasing the orthogonality of an [`AnyAxis`].
    #[inline]
    pub const fn from_any_axis(axis: AnyAxis<T>) -> Self {
        axis.into_any_octant()
    }

    /// Constructs an [`AnyOctant`] by erasing the directions of a [`Diagonal`].
    #[inline]
    pub const fn from_diagonal<const FX: bool, const FY: bool>(
        diagonal: Diagonal<FX, FY, T>,
    ) -> Self {
        diagonal.into_any_octant()
    }

    /// Constructs an [`AnyOctant`] by erasing the diagonality of an [`AnyDiagonal`].
    #[inline]
    pub const fn from_any_diagonal(diagonal: AnyDiagonal<T>) -> Self {
        diagonal.into_any_octant()
    }

    /// Constructs an [`AnyOctant`] by erasing the directions and orientation of an [`Octant`].
    #[inline]
    pub const fn from_octant<const FX: bool, const FY: bool, const YX: bool>(
        octant: Octant<FX, FY, YX, T>,
    ) -> Self {
        octant.into_any_octant()
    }

    /// Fixes the direction and orientation of this [`AnyOctant`], returning a [`SignedAxis`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not axis-aligned, or not aligned with `F` and `V`.
    #[inline]
    pub const fn try_into_signed_axis<const F: bool, const V: bool>(
        self,
    ) -> Option<SignedAxis<F, V, T>> {
        SignedAxis::try_from_any_octant(self)
    }

    /// Fixes the orientation of this [`AnyOctant`], returning an [`Axis`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not axis-aligned, or not aligned with `V`.
    #[inline]
    pub const fn try_into_axis<const V: bool>(self) -> Option<Axis<V, T>> {
        Axis::try_from_any_octant(self)
    }

    /// Refines this [`AnyOctant`] into an [`AnyAxis`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not axis-aligned.
    #[inline]
    pub const fn try_into_any_axis(self) -> Option<AnyAxis<T>> {
        AnyAxis::try_from_any_octant(self)
    }

    /// Fixes the directions of this [`AnyOctant`], returning a [`Diagonal`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not diagonal, or not aligned with `FX` and `FY`.
    #[inline]
    pub const fn try_into_diagonal<const FX: bool, const FY: bool>(
        self,
    ) -> Option<Diagonal<FX, FY, T>> {
        Diagonal::try_from_any_octant(self)
    }

    /// Refines this [`AnyOctant`] into an [`AnyDiagonal`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not diagonal.
    #[inline]
    pub const fn try_into_any_diagonal(self) -> Option<AnyDiagonal<T>> {
        AnyDiagonal::try_from_any_octant(self)
    }

    /// Fixes the directions and orientation of this [`AnyOctant`], returning an [`Octant`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is axis-aligned, diagonal,
    /// or not aligned with `FX`, `FY` and `YX`.
    #[inline]
    pub const fn try_into_octant<const FX: bool, const FY: bool, const YX: bool>(
        self,
    ) -> Option<Octant<FX, FY, YX, T>> {
        Octant::try_from_any_octant(self)
    }
}

impl<const F: bool, const V: bool, T: Num> From<SignedAxis<F, V, T>> for AnyOctant<T> {
    #[inline]
    fn from(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_any_octant()
    }
}

impl<const V: bool, T: Num> From<Axis<V, T>> for AnyOctant<T> {
    #[inline]
    fn from(axis: Axis<V, T>) -> Self {
        axis.into_any_octant()
    }
}

impl<T: Num> From<AnyAxis<T>> for AnyOctant<T> {
    #[inline]
    fn from(axis: AnyAxis<T>) -> Self {
        axis.into_any_octant()
    }
}

impl<const FX: bool, const FY: bool, T: Num> From<Diagonal<FX, FY, T>> for AnyOctant<T> {
    #[inline]
    fn from(diagonal: Diagonal<FX, FY, T>) -> Self {
        diagonal.into_any_octant()
    }
}

impl<const FX: bool, const FY: bool, const YX: bool, T: Num> From<Octant<FX, FY, YX, T>>
    for AnyOctant<T>
{
    #[inline]
    fn from(octant: Octant<FX, FY, YX, T>) -> Self {
        octant.into_any_octant()
    }
}

impl<const F: bool, const V: bool, T: Num> TryFrom<AnyOctant<T>> for SignedAxis<F, V, T> {
    type Error = ();

    #[inline]
    fn try_from(octant: AnyOctant<T>) -> Result<Self, Self::Error> {
        octant.try_into_signed_axis().ok_or(())
    }
}

impl<const V: bool, T: Num> TryFrom<AnyOctant<T>> for Axis<V, T> {
    type Error = ();

    #[inline]
    fn try_from(octant: AnyOctant<T>) -> Result<Self, Self::Error> {
        octant.try_into_axis().ok_or(())
    }
}

impl<T: Num> TryFrom<AnyOctant<T>> for AnyAxis<T> {
    type Error = ();

    #[inline]
    fn try_from(octant: AnyOctant<T>) -> Result<Self, Self::Error> {
        octant.try_into_any_axis().ok_or(())
    }
}

impl<const FX: bool, const FY: bool, T: Num> TryFrom<AnyOctant<T>> for Diagonal<FX, FY, T> {
    type Error = ();

    #[inline]
    fn try_from(octant: AnyOctant<T>) -> Result<Self, Self::Error> {
        octant.try_into_diagonal().ok_or(())
    }
}

impl<const FX: bool, const FY: bool, const YX: bool, T: Num> TryFrom<AnyOctant<T>>
    for Octant<FX, FY, YX, T>
{
    type Error = ();

    #[inline]
    fn try_from(octant: AnyOctant<T>) -> Result<Self, Self::Error> {
        octant.try_into_octant().ok_or(())
    }
}
