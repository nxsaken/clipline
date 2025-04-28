//! Conversions from and into axis-aligned iterators.

use super::{AnyAxis, Axis, SignedAxis};
use crate::macros::{f, v};
use crate::math::Num;
use crate::AnyOctant;

impl<const F: bool, const V: bool, T: Num> SignedAxis<F, V, T> {
    /// Transmutes the direction and orientation of this [`SignedAxis`].
    ///
    /// ## Safety
    ///
    /// The caller must ensure that `F0` and `V0` are equal to `F` and `V`.
    #[inline]
    const unsafe fn transmute<const F0: bool, const V0: bool>(self) -> SignedAxis<F0, V0, T> {
        debug_assert!(F == F0);
        debug_assert!(V == V0);
        SignedAxis { u: self.u, v1: self.v1, v2: self.v2 }
    }

    /// Constructs a [`SignedAxis`] by fixing the direction of an [`Axis`].
    ///
    /// Returns [`None`] if [`Axis`] is not aligned with `F`.
    #[inline]
    pub const fn try_from_axis(axis: Axis<V, T>) -> Option<Self> {
        // SAFETY: match sets the correct F for transmute(), or returns None; V is reused.
        unsafe {
            match axis {
                Axis::Positive(me) if !F => Some(me.transmute()),
                Axis::Negative(me) if F => Some(me.transmute()),
                _ => None,
            }
        }
    }

    /// Constructs a [`SignedAxis`] by fixing the direction and orientation of an [`AnyAxis`].
    ///
    /// Returns [`None`] if [`AnyAxis`] is not aligned with `F` and `V`.
    #[inline]
    pub const fn try_from_any_axis(axis: AnyAxis<T>) -> Option<Self> {
        // SAFETY: match sets the correct F and V for transmute(), or returns None.
        unsafe {
            match axis {
                AnyAxis::PositiveAxis0(me) if !F && !V => Some(me.transmute()),
                AnyAxis::PositiveAxis1(me) if !F && V => Some(me.transmute()),
                AnyAxis::NegativeAxis0(me) if F && !V => Some(me.transmute()),
                AnyAxis::NegativeAxis1(me) if F && V => Some(me.transmute()),
                _ => None,
            }
        }
    }

    /// Constructs a [`SignedAxis`] by fixing the direction and orientation of an [`AnyOctant`].
    ///
    /// Returns [`None`] if [`AnyOctant`] is not axis-aligned, or not aligned with `F` and `V`.
    #[inline]
    pub const fn try_from_any_octant(octant: AnyOctant<T>) -> Option<Self> {
        // SAFETY: match sets the correct F and V for transmute(), or returns None.
        unsafe {
            match octant {
                AnyOctant::PositiveAxis0(me) if !F && !V => Some(me.transmute()),
                AnyOctant::PositiveAxis1(me) if !F && V => Some(me.transmute()),
                AnyOctant::NegativeAxis0(me) if F && !V => Some(me.transmute()),
                AnyOctant::NegativeAxis1(me) if F && V => Some(me.transmute()),
                _ => None,
            }
        }
    }

    /// Erases the direction of this [`SignedAxis`], returning an [`Axis`].
    #[inline]
    pub const fn into_axis(self) -> Axis<V, T> {
        // SAFETY: f! sets up the correct F for transmute(); V is reused.
        unsafe {
            f! {
                Axis::Positive(self.transmute()),
                Axis::Negative(self.transmute()),
            }
        }
    }

    /// Erases the direction and orientation of this [`SignedAxis`], returning an [`AnyAxis`].
    #[inline]
    pub const fn into_any_axis(self) -> AnyAxis<T> {
        // SAFETY: f! and v! set the correct F and V for transmute().
        unsafe {
            f! {
                v! {
                    AnyAxis::PositiveAxis0(self.transmute()),
                    AnyAxis::PositiveAxis1(self.transmute()),
                },
                v! {
                    AnyAxis::NegativeAxis0(self.transmute()),
                    AnyAxis::NegativeAxis1(self.transmute()),
                }
            }
        }
    }

    /// Erases the direction and orientation of this [`SignedAxis`], returning an [`AnyOctant`].
    #[inline]
    pub const fn into_any_octant(self) -> AnyOctant<T> {
        // SAFETY: f! and v! set up the correct FX and FY for transmute().
        unsafe {
            f! {
                v! {
                    AnyOctant::PositiveAxis0(self.transmute()),
                    AnyOctant::PositiveAxis1(self.transmute()),
                },
                v! {
                    AnyOctant::NegativeAxis0(self.transmute()),
                    AnyOctant::NegativeAxis1(self.transmute()),
                }
            }
        }
    }
}

impl<const V: bool, T: Num> Axis<V, T> {
    /// Constructs an [`Axis`] by erasing the direction of a [`SignedAxis`].
    #[inline]
    pub const fn from_signed_axis<const F: bool>(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_axis()
    }

    /// Constructs an [`Axis`] by fixing the orientation of an [`AnyAxis`].
    ///
    /// Returns [`None`] if the [`AnyAxis`] is not aligned with `V`.
    #[inline]
    pub const fn try_from_any_axis(axis: AnyAxis<T>) -> Option<Self> {
        // SAFETY: match sets the correct F and V for transmute(), or returns an error.
        unsafe {
            match axis {
                AnyAxis::PositiveAxis0(axis) if !V => Some(Self::Positive(axis.transmute())),
                AnyAxis::NegativeAxis0(axis) if !V => Some(Self::Negative(axis.transmute())),
                AnyAxis::PositiveAxis1(axis) if V => Some(Self::Positive(axis.transmute())),
                AnyAxis::NegativeAxis1(axis) if V => Some(Self::Negative(axis.transmute())),
                _ => None,
            }
        }
    }

    /// Constructs an [`Axis`] from an [`AnyOctant`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not axis-aligned, or not aligned with `V`.
    #[inline]
    pub const fn try_from_any_octant(octant: AnyOctant<T>) -> Option<Self> {
        // SAFETY: match sets the correct F and V for transmute(), or returns None.
        unsafe {
            match octant {
                AnyOctant::PositiveAxis0(me) if !V => Some(Self::Positive(me.transmute())),
                AnyOctant::NegativeAxis0(me) if !V => Some(Self::Negative(me.transmute())),
                AnyOctant::PositiveAxis1(me) if V => Some(Self::Positive(me.transmute())),
                AnyOctant::NegativeAxis1(me) if V => Some(Self::Negative(me.transmute())),
                _ => None,
            }
        }
    }

    /// Fixes the direction of this [`Axis`], returning a [`SignedAxis`].
    ///
    /// Returns [`None`] if this [`Axis`] is not aligned with `F`.
    #[inline]
    pub const fn try_into_signed_axis<const F: bool>(self) -> Option<SignedAxis<F, V, T>> {
        SignedAxis::try_from_axis(self)
    }

    /// Erases the direction of this [`Axis`], returning an [`Axis`].
    #[inline]
    pub const fn into_any_axis(self) -> AnyAxis<T> {
        // SAFETY: v! sets the correct V for transmute(); match sets the correct F at runtime.
        unsafe {
            match self {
                Self::Positive(axis) => v!(
                    AnyAxis::PositiveAxis0(axis.transmute()),
                    AnyAxis::PositiveAxis1(axis.transmute()),
                ),
                Self::Negative(axis) => v!(
                    AnyAxis::NegativeAxis0(axis.transmute()),
                    AnyAxis::NegativeAxis1(axis.transmute()),
                ),
            }
        }
    }

    /// Erases the orientation of this [`Axis`], returning an [`AnyOctant`].
    #[inline]
    pub const fn into_any_octant(self) -> AnyOctant<T> {
        // SAFETY: v! sets the correct V for transmute(); match sets the correct F at runtime.
        unsafe {
            v! {
                match self {
                    Self::Positive(me) => AnyOctant::PositiveAxis0(me.transmute()),
                    Self::Negative(me) => AnyOctant::NegativeAxis0(me.transmute()),
                },
                match self {
                    Self::Positive(me) => AnyOctant::PositiveAxis1(me.transmute()),
                    Self::Negative(me) => AnyOctant::NegativeAxis1(me.transmute()),
                }
            }
        }
    }
}

impl<T: Num> AnyAxis<T> {
    /// Constructs an [`AnyAxis`] by erasing the direction and orientation of a [`SignedAxis`].
    #[inline]
    pub const fn from_signed_axis<const F: bool, const V: bool>(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_any_axis()
    }

    /// Constructs an [`AnyAxis`] by erasing the orientation of an [`Axis`].
    #[inline]
    pub const fn from_axis<const V: bool>(axis: Axis<V, T>) -> Self {
        axis.into_any_axis()
    }

    /// Constructs an [`AnyAxis`] from an [`AnyOctant`].
    ///
    /// Returns [`None`] if the [`AnyOctant`] is not axis-aligned.
    #[inline]
    pub const fn try_from_any_octant(octant: AnyOctant<T>) -> Option<Self> {
        match octant {
            AnyOctant::PositiveAxis0(me) => Some(Self::PositiveAxis0(me)),
            AnyOctant::PositiveAxis1(me) => Some(Self::PositiveAxis1(me)),
            AnyOctant::NegativeAxis0(me) => Some(Self::NegativeAxis0(me)),
            AnyOctant::NegativeAxis1(me) => Some(Self::NegativeAxis1(me)),
            _ => None,
        }
    }

    /// Fixes the direction and orientation of this [`AnyAxis`], returning a [`SignedAxis`].
    ///
    /// Returns [`None`] if this [`AnyAxis`] is not aligned with `F` and `V`.
    #[inline]
    pub const fn try_into_signed_axis<const F: bool, const V: bool>(
        self,
    ) -> Option<SignedAxis<F, V, T>> {
        SignedAxis::try_from_any_axis(self)
    }

    /// Fixes the orientation of this [`AnyAxis`], returning an [`Axis`].
    ///
    /// Returns [`None`] if this [`AnyAxis`] is not aligned with `V`.
    #[inline]
    pub const fn try_into_axis<const V: bool>(self) -> Option<Axis<V, T>> {
        Axis::try_from_any_axis(self)
    }

    /// Erases the orthogonality of this [`AnyAxis`], returning an [`AnyOctant`].
    #[inline]
    pub const fn into_any_octant(self) -> AnyOctant<T> {
        match self {
            Self::PositiveAxis0(me) => AnyOctant::PositiveAxis0(me),
            Self::PositiveAxis1(me) => AnyOctant::PositiveAxis1(me),
            Self::NegativeAxis0(me) => AnyOctant::NegativeAxis0(me),
            Self::NegativeAxis1(me) => AnyOctant::NegativeAxis1(me),
        }
    }
}

impl<const F: bool, const V: bool, T: Num> From<SignedAxis<F, V, T>> for Axis<V, T> {
    #[inline]
    fn from(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_axis()
    }
}

impl<const F: bool, const V: bool, T: Num> From<SignedAxis<F, V, T>> for AnyAxis<T> {
    #[inline]
    fn from(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_any_axis()
    }
}

impl<const V: bool, T: Num> From<Axis<V, T>> for AnyAxis<T> {
    #[inline]
    fn from(axis: Axis<V, T>) -> Self {
        axis.into_any_axis()
    }
}

impl<const F: bool, const V: bool, T: Num> TryFrom<Axis<V, T>> for SignedAxis<F, V, T> {
    type Error = ();

    #[inline]
    fn try_from(axis: Axis<V, T>) -> Result<Self, Self::Error> {
        axis.try_into_signed_axis().ok_or(())
    }
}

impl<const F: bool, const V: bool, T: Num> TryFrom<AnyAxis<T>> for SignedAxis<F, V, T> {
    type Error = ();

    #[inline]
    fn try_from(axis: AnyAxis<T>) -> Result<Self, Self::Error> {
        axis.try_into_signed_axis().ok_or(())
    }
}

impl<const V: bool, T: Num> TryFrom<AnyAxis<T>> for Axis<V, T> {
    type Error = ();

    #[inline]
    fn try_from(axis: AnyAxis<T>) -> Result<Self, Self::Error> {
        axis.try_into_axis().ok_or(())
    }
}
