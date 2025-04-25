//! Conversions between axis-aligned iterators.

use super::{AnyAxis, Axis, SignedAxis};
use crate::macros::{f, hv};

impl<const F: bool, const V: bool, T> SignedAxis<F, V, T> {
    /// Transmutes the direction and orientation of this [`SignedAxis`].
    ///
    /// ## Safety
    ///
    /// The caller must ensure that `F0` and `V0` are equal to `F` and `V`.
    #[inline]
    const unsafe fn transmute<const F0: bool, const V0: bool>(self) -> SignedAxis<F0, V0, T>
    where
        T: Copy,
    {
        SignedAxis { u: self.u, v1: self.v1, v2: self.v2 }
    }

    /// Constructs a [`SignedAxis`] by fixing the direction of an [`Axis`].
    ///
    /// Returns [`None`] if [`Axis`] is not aligned with `F`.
    #[inline]
    pub const fn from_axis(axis: Axis<V, T>) -> Option<Self>
    where
        T: Copy,
    {
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
    pub const fn from_any_axis(axis: AnyAxis<T>) -> Option<Self>
    where
        T: Copy,
    {
        // SAFETY: match sets the correct F and V for transmute(), or returns None.
        unsafe {
            match axis {
                AnyAxis::PositiveAxis0(me) if !F && !V => Some(me.transmute()),
                AnyAxis::NegativeAxis0(me) if F && !V => Some(me.transmute()),
                AnyAxis::PositiveAxis1(me) if !F && V => Some(me.transmute()),
                AnyAxis::NegativeAxis1(me) if F && V => Some(me.transmute()),
                _ => None,
            }
        }
    }

    /// Erases the direction of this [`SignedAxis`], returning an [`Axis`].
    #[inline]
    pub const fn into_axis(self) -> Axis<V, T>
    where
        T: Copy,
    {
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
    pub const fn into_any_axis(self) -> AnyAxis<T>
    where
        T: Copy,
    {
        // SAFETY: f! and hv! set the correct F and V for transmute().
        unsafe {
            f! {
                hv! {
                    AnyAxis::PositiveAxis0(self.transmute()),
                    AnyAxis::PositiveAxis1(self.transmute()),
                },
                hv! {
                    AnyAxis::NegativeAxis0(self.transmute()),
                    AnyAxis::NegativeAxis1(self.transmute()),
                }
            }
        }
    }
}

impl<const V: bool, T> Axis<V, T> {
    /// Constructs an [`Axis`] by erasing the direction of a [`SignedAxis`].
    #[inline]
    pub const fn from_signed_axis<const F: bool>(axis: SignedAxis<F, V, T>) -> Self
    where
        T: Copy,
    {
        axis.into_axis()
    }

    /// Constructs an [`Axis`] by fixing the orientation of an [`AnyAxis`].
    ///
    /// Returns [`None`] if the [`AnyAxis`] is not aligned with `V`.
    #[inline]
    pub const fn from_any_axis(axis: AnyAxis<T>) -> Option<Self>
    where
        T: Copy,
    {
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

    /// Fixes the direction of this [`Axis`], returning a [`SignedAxis`].
    ///
    /// Returns [`None`] if this [`Axis`] is not aligned with `F`.
    #[inline]
    pub const fn into_signed_axis<const F: bool>(self) -> Option<SignedAxis<F, V, T>>
    where
        T: Copy,
    {
        SignedAxis::from_axis(self)
    }

    /// Erases the direction of this [`Axis`], returning an [`Axis`].
    #[inline]
    pub const fn into_any_axis(self) -> AnyAxis<T>
    where
        T: Copy,
    {
        // SAFETY: hv! sets the correct V for transmute(); match sets the correct F at runtime.
        unsafe {
            match self {
                Self::Positive(axis) => hv!(
                    AnyAxis::PositiveAxis0(axis.transmute()),
                    AnyAxis::PositiveAxis1(axis.transmute()),
                ),
                Self::Negative(axis) => hv!(
                    AnyAxis::NegativeAxis0(axis.transmute()),
                    AnyAxis::NegativeAxis1(axis.transmute()),
                ),
            }
        }
    }
}

impl<T> AnyAxis<T> {
    /// Constructs an [`AnyAxis`] by erasing the direction and orientation of a [`SignedAxis`].
    #[inline]
    pub const fn from_signed_axis<const F: bool, const V: bool>(axis: SignedAxis<F, V, T>) -> Self
    where
        T: Copy,
    {
        axis.into_any_axis()
    }

    /// Constructs an [`AnyAxis`] by erasing the orientation of an [`Axis`].
    #[inline]
    pub const fn from_axis<const V: bool>(axis: Axis<V, T>) -> Self
    where
        T: Copy,
    {
        axis.into_any_axis()
    }

    /// Fixes the direction and orientation of this [`AnyAxis`], returning a [`SignedAxis`].
    ///
    /// Returns [`None`] if this [`AnyAxis`] is not aligned with `F` and `V`.
    #[inline]
    pub const fn into_signed_axis<const F: bool, const V: bool>(self) -> Option<SignedAxis<F, V, T>>
    where
        T: Copy,
    {
        SignedAxis::from_any_axis(self)
    }

    /// Fixes the orientation of this [`AnyAxis`], returning an [`Axis`].
    ///
    /// Returns [`None`] if this [`AnyAxis`] is not aligned with `V`.
    #[inline]
    pub const fn into_axis<const V: bool>(self) -> Option<Axis<V, T>>
    where
        T: Copy,
    {
        Axis::from_any_axis(self)
    }
}

impl<const F: bool, const V: bool, T> From<SignedAxis<F, V, T>> for Axis<V, T>
where
    T: Copy,
{
    #[inline]
    fn from(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_axis()
    }
}

impl<const F: bool, const V: bool, T> From<SignedAxis<F, V, T>> for AnyAxis<T>
where
    T: Copy,
{
    #[inline]
    fn from(axis: SignedAxis<F, V, T>) -> Self {
        axis.into_any_axis()
    }
}

impl<const V: bool, T> From<Axis<V, T>> for AnyAxis<T>
where
    T: Copy,
{
    #[inline]
    fn from(axis: Axis<V, T>) -> Self {
        axis.into_any_axis()
    }
}

impl<const F: bool, const V: bool, T> TryFrom<Axis<V, T>> for SignedAxis<F, V, T>
where
    T: Copy,
{
    type Error = ();

    #[inline]
    fn try_from(axis: Axis<V, T>) -> Result<Self, Self::Error> {
        axis.into_signed_axis().ok_or(())
    }
}

impl<const V: bool, T> TryFrom<AnyAxis<T>> for Axis<V, T>
where
    T: Copy,
{
    type Error = ();

    #[inline]
    fn try_from(axis: AnyAxis<T>) -> Result<Self, Self::Error> {
        axis.into_axis().ok_or(())
    }
}

impl<const F: bool, const V: bool, T> TryFrom<AnyAxis<T>> for SignedAxis<F, V, T>
where
    T: Copy,
{
    type Error = ();

    #[inline]
    fn try_from(axis: AnyAxis<T>) -> Result<Self, Self::Error> {
        axis.into_signed_axis().ok_or(())
    }
}
