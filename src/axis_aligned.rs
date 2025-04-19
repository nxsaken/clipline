//! ## Axis-aligned iterators

use crate::clip::Clip;
use crate::math::{Math, Num, Point};
use crate::symmetry::{f, vh};
use crate::utils::map;

mod clip;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signed-axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a line segment aligned to the given **signed axis**.
///
/// A signed axis is defined by the direction and axis-alignment of the line segments aligned to it:
/// - [negative](NegativeAxis) if `F`, [positive](PositiveAxis) otherwise.
/// - [vertical](SignedAxis1) if `V`, [horizontal](SignedAxis0) otherwise.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SignedAxis<const F: bool, const V: bool, T> {
    u: T,
    v1: T,
    v2: T,
}

/// Iterator over a line segment aligned to the given
/// **positive** [signed axis](SignedAxis).
pub type PositiveAxis<const V: bool, T> = SignedAxis<false, V, T>;

/// Iterator over a line segment aligned to the given
/// **negative** [signed axis](SignedAxis).
pub type NegativeAxis<const V: bool, T> = SignedAxis<true, V, T>;

/// Iterator over a line segment aligned to the given
/// **horizontal** [signed axis](SignedAxis).
pub type SignedAxis0<const F: bool, T> = SignedAxis<F, false, T>;

/// Iterator over a line segment aligned to the given
/// **vertical** [signed axis](SignedAxis).
pub type SignedAxis1<const F: bool, T> = SignedAxis<F, true, T>;

/// Iterator over a line segment aligned to the
/// **positive horizontal** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `0°`.
pub type PositiveAxis0<T> = SignedAxis0<false, T>;

/// Iterator over a line segment aligned to the
/// **negative horizontal** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `180°`.
pub type NegativeAxis0<T> = SignedAxis0<true, T>;

/// Iterator over a line segment aligned to the
/// **positive vertical** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `90°`.
pub type PositiveAxis1<T> = SignedAxis1<false, T>;

/// Iterator over a line segment aligned to the
/// **negative vertical** [signed axis](SignedAxis).
///
/// Covers line segments oriented at `270°`.
pub type NegativeAxis1<T> = SignedAxis1<true, T>;

/// Implements inherent axis methods or delegates to the variant implementation.
macro_rules! impl_axis_methods {
    (
        $self:ident,
        $T:ty,
        is_done = $is_done:expr,
        length = $length:expr,
        head = $head:expr,
        tail = $tail:expr,
        pop_head = $pop_head:expr,
        pop_tail = $pop_tail:expr
    ) => {
        /// Returns `true` if the iterator has terminated.
        #[inline]
        #[must_use]
        pub const fn is_done(&$self) -> bool { $is_done }

        /// Returns the remaining length of this iterator.
        #[inline]
        #[must_use]
        pub const fn length(&$self) -> <$T as Num>::U { $length }

        /// Returns the point at the start of the iterator.
        /// Does not advance the iterator.
        ///
        /// Returns [`None`] if the iterator has terminated.
        #[inline]
        #[must_use]
        pub const fn head(&$self) -> Option<Point<$T>> { $head }

        /// Returns the point immediately before the end of the iterator.
        /// Does not advance the iterator.
        ///
        /// Returns [`None`] if the iterator has terminated.
        ///
        /// ## Warning
        /// Calling `pop_tail` after `tail` will recompute the point.
        #[inline]
        #[must_use]
        pub const fn tail(&$self) -> Option<Point<$T>> { $tail }

        /// Consumes and returns the point at the start of the iterator.
        ///
        /// Returns [`None`] if the iterator has terminated.
        #[inline]
        #[must_use]
        pub const fn pop_head(&mut $self) -> Option<Point<$T>> { $pop_head }

        /// Consumes the point at the end of the iterator, and returns the point immediately before.
        ///
        /// Returns [`None`] if the iterator has terminated.
        #[inline]
        #[must_use]
        pub const fn pop_tail(&mut $self) -> Option<Point<$T>> { $pop_tail }
    };
    ($T:ty, $variant:ident) => {
        impl_axis_methods!(
            self,
            $T,
            is_done = $variant!(self, me => me.is_done()),
            length = $variant!(self, me => me.length()),
            head = $variant!(self, me => me.head()),
            tail = $variant!(self, me => me.tail()),
            pop_head = $variant!(self, me => me.pop_head()),
            pop_tail = $variant!(self, me => me.pop_tail())
        );
    }
}

/// Implements iterator traits or delegates to the variant implementations:
/// - [`Iterator`]
/// - [`DoubleEndedIterator`]
/// - [`core::iter::FusedIterator`]
/// - [`ExactSizeIterator`] (conditionally on `cfg_esi`)
macro_rules! impl_axis_iters {
    (
        $Axis:ident<$(const $FV:ident,)* $T:ty>,
        $self:ident,
        next = $next:expr,
        next_back = $next_back:expr,
        size_hint = $size_hint:expr,
        is_empty = $is_empty:expr
        $(, |$init:ident, $f:ident| {
            fold = $fold:expr,
            try_fold = $try_fold:expr,
            rfold = $rfold:expr,
            try_rfold = $try_rfold:expr
        })?
        $(, cfg_esi = $cfg_esi:meta)?
    ) => {
        impl<$(const $FV: bool),*> Iterator for $Axis<$($FV,)? $T> {
            type Item = Point<$T>;

            #[inline]
            fn next(&mut $self) -> Option<Self::Item> { $next }

            #[inline]
            fn size_hint(&$self) -> (usize, Option<usize>) { $size_hint }

            $(
            #[inline]
            fn fold<B, F>($self, $init: B, $f: F) -> B
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> B,
            { $fold }

            #[cfg(feature = "try_fold")]
            #[inline]
            fn try_fold<B, F, R>(&mut $self, $init: B, $f: F) -> R
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> R,
                R: core::ops::Try<Output = B>,
            { $try_fold }
            )?
        }

        impl<$(const $FV: bool),*> DoubleEndedIterator for $Axis<$($FV,)? $T> {
            #[inline]
            fn next_back(&mut $self) -> Option<Self::Item> { $next_back }

            $(
            #[inline]
            fn rfold<B, F>($self, $init: B, $f: F) -> B
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> B,
            { $rfold }

            #[cfg(feature = "try_fold")]
            #[inline]
            fn try_rfold<B, F, R>(&mut $self, $init: B, $f: F) -> R
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> R,
                R: core::ops::Try<Output = B>,
            { $try_rfold }
            )?
        }

        impl<$(const $FV: bool),*> core::iter::FusedIterator for $Axis<$($FV,)? $T> {}

        $(#[$cfg_esi])?
        impl<$(const $FV: bool),*> ExactSizeIterator for $Axis<$($FV,)? $T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&$self) -> bool { $is_empty }
        }
    };
    (
        $Axis:ident<$(const $FV:ident,)* $T:ty>,
        $variant:ident
        $(, cfg_esi = $cfg_esi:meta)?
    ) => {
        impl_axis_iters!(
            $Axis<$(const $FV,)* $T>,
            self,
            next = $variant!(self, me => me.next()),
            next_back = $variant!(self, me => me.next_back()),
            size_hint = $variant!(self, me => me.size_hint()),
            is_empty = $variant!(self, me => me.is_empty()),
            |init, f| {
                fold = $variant!(self, me => me.fold(init, f)),
                try_fold = $variant!(self, me => me.try_fold(init, f)),
                rfold = $variant!(self, me => me.rfold(init, f)),
                try_rfold = $variant!(self, me => me.try_rfold(init, f))
            }
            $(, cfg_esi = $cfg_esi)?
        );
    }
}

macro_rules! impl_signed_axis {
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
        impl<const F: bool, const V: bool> SignedAxis<F, V, $T> {
            #[inline(always)]
            #[must_use]
            const fn new_inner(u: $T, v1: $T, v2: $T) -> Self {
                Self { u, v1, v2 }
            }

            /// Returns an iterator over a *half-open* line segment if it is aligned to
            /// the given [signed axis](SignedAxis), otherwise returns [`None`].
            ///
            /// - A [horizontal](SignedAxis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](SignedAxis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Option<Self> {
                if f!(v2 <= v1, v1 <= v2) {
                    return None;
                }
                Some(Self::new_inner(u, v1, v2))
            }

            /// Clips a *half-open* line segment to a [rectangular region](Clip)
            /// if it aligned to the given [signed axis](SignedAxis),
            /// and returns an iterator over it.
            ///
            /// - A [horizontal](SignedAxis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](SignedAxis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            ///
            /// Returns [`None`] if the line segment is not aligned to the signed axis,
            /// or if it does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                if f!(v2 <= v1, v1 <= v2) {
                    return None;
                }
                Self::clip_inner(u, v1, v2, clip)
            }

            impl_axis_methods!(
                self,
                $T,
                is_done = f!(self.v2 <= self.v1, self.v1 <= self.v2),
                length = Math::<$T>::delta(f!(self.v2, self.v1), f!(self.v1, self.v2)),
                head = {
                    if self.is_done() {
                        return None;
                    }
                    let (x, y) = vh!((self.v1, self.u), (self.u, self.v1));
                    Some((x, y))
                },
                tail = {
                    if self.is_done() {
                        return None;
                    }
                    let v2 = f!(self.v2.wrapping_sub(1), self.v2.wrapping_add(1));
                    let (x, y) = vh!((v2, self.u), (self.u, v2));
                    Some((x, y))
                },
                pop_head = {
                    let Some((x, y)) = self.head() else {
                        return None;
                    };
                    self.v1 = f!(self.v1.wrapping_add(1), self.v1.wrapping_sub(1));
                    Some((x, y))
                },
                pop_tail = {
                    let Some((x, y)) = self.tail() else {
                        return None;
                    };
                    self.v2 = vh!(x, y);
                    Some((x, y))
                }
            );
        }

        impl_axis_iters!(
            SignedAxis<const F, const V, $T>,
            self,
            next = self.pop_head(),
            next_back = self.pop_tail(),
            size_hint = {
                #[allow(unreachable_patterns)]
                match usize::try_from(self.length()) {
                    Ok(length) => (length, Some(length)),
                    Err(_) => (usize::MAX, None),
                }
            },
            is_empty = self.is_done()
            $(, cfg_esi = $cfg_esi)?
        );
    };
}

/// Applies the macro `m` to multiple integer types.
macro_rules! all_nums {
    ($m:ident) => {
        $m!(i8);
        $m!(u8);
        $m!(i16);
        $m!(u16);
        $m!(i32, cfg_esi = cfg(any(target_pointer_width = "32", target_pointer_width = "64")));
        $m!(u32, cfg_esi = cfg(any(target_pointer_width = "32", target_pointer_width = "64")));
        $m!(i64, cfg_esi = cfg(target_pointer_width = "64"));
        $m!(u64, cfg_esi = cfg(target_pointer_width = "64"));
        $m!(isize);
        $m!(usize);
    };
}

all_nums!(impl_signed_axis);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Axis-aligned iterators
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a line segment aligned to the given **axis**,
/// with the direction determined at runtime.
///
/// An axis is defined by the orientation of the line segments it covers:
/// [vertical](Axis1) if `V`, [horizontal](Axis0) otherwise.
///
/// If you know the [direction](SignedAxis) of the line segment,
/// consider [`PositiveAxis`] and [`NegativeAxis`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Axis<const V: bool, T> {
    /// See [`PositiveAxis`].
    Positive(PositiveAxis<V, T>),
    /// See [`NegativeAxis`].
    Negative(NegativeAxis<V, T>),
}

/// Iterator over a line segment aligned to the **horizontal** [axis](Axis),
/// with the direction determined at runtime.
///
/// If you know the [direction](SignedAxis) of the line segment,
/// consider [`PositiveAxis0`] and [`NegativeAxis0`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction only once instead of on every call to [`Iterator::next`].
pub type Axis0<T> = Axis<false, T>;

/// Iterator over a line segment aligned to the **vertical** [axis](Axis),
/// with the direction determined at runtime.
///
/// If you know the [direction](SignedAxis) of the line segment,
/// consider [`PositiveAxis1`] and [`NegativeAxis1`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the direction only once instead of on every call to [`Iterator::next`].
pub type Axis1<T> = Axis<true, T>;

macro_rules! match_axis {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::Positive($me) => $call,
            Self::Negative($me) => $call,
        }
    };
}

macro_rules! impl_axis {
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
        impl<const V: bool> Axis<V, $T> {
            /// Returns an iterator over a *half-open* line segment aligned to the given [axis](Axis).
            ///
            /// - A [horizontal](Axis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](Axis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            #[inline]
            #[must_use]
            pub const fn new(u: $T, v1: $T, v2: $T) -> Self {
                if v1 <= v2 {
                    Self::Positive(PositiveAxis::<V, $T>::new_inner(u, v1, v2))
                } else {
                    Self::Negative(NegativeAxis::<V, $T>::new_inner(u, v1, v2))
                }
            }

            /// Clips a *half-open* line segment aligned to the given [axis](Axis)
            /// to a [rectangular region](Clip), and returns an iterator over it.
            ///
            /// - A [horizontal](Axis0) line segment has endpoints `(v1, u)` and `(v2, u)`.
            /// - A [vertical](Axis1) line segment has endpoints `(u, v1)` and `(u, v2)`.
            ///
            /// Returns [`None`] if the line segment is empty or does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                if v1 < v2 {
                    map!(PositiveAxis::<V, $T>::clip_inner(u, v1, v2, clip), Self::Positive)
                } else if v2 < v1 {
                    map!(NegativeAxis::<V, $T>::clip_inner(u, v1, v2, clip), Self::Negative)
                } else {
                    None
                }
            }

            impl_axis_methods!($T, match_axis);
        }

        impl_axis_iters!(Axis<const V, $T>, match_axis $(, cfg_esi = $cfg_esi)?);
    };
}

all_nums!(impl_axis);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arbitrary axis-aligned iterator
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator over a [horizontal](Axis0) or [vertical](Axis1) line segment,
/// with the axis-alignment and direction determined at runtime.
///
/// If you know the [axis-alignment](Axis) of the line segment, use [`Axis0`] or [`Axis1`].
///
/// **Note**: an optimized implementation of [`Iterator::fold`] is provided.
/// This makes [`Iterator::for_each`] faster than a `for` loop, since it checks
/// the signed axis only once instead of on every call to [`Iterator::next`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AnyAxis<T> {
    /// Horizontal line segment at `0°`, see [`PositiveAxis0`].
    PositiveAxis0(PositiveAxis0<T>),
    /// Vertical line segment at `90°`, see [`PositiveAxis1`].
    PositiveAxis1(PositiveAxis1<T>),
    /// Horizontal line segment at `180°`, see [`NegativeAxis0`].
    NegativeAxis0(NegativeAxis0<T>),
    /// Vertical line segment at `270°`, see [`NegativeAxis1`].
    NegativeAxis1(NegativeAxis1<T>),
}

/// Delegates calls to signed-axis variants.
macro_rules! match_axis {
    ($self:ident, $me:ident => $call:expr) => {
        match $self {
            Self::PositiveAxis0($me) => $call,
            Self::NegativeAxis0($me) => $call,
            Self::PositiveAxis1($me) => $call,
            Self::NegativeAxis1($me) => $call,
        }
    };
}

macro_rules! impl_any_axis {
    ($T:ty $(, cfg_esi = $cfg_esi:meta)?) => {
        impl AnyAxis<$T> {
            /// Returns an iterator over a *half-open* line segment
            /// if it is aligned to any [axis](Axis), otherwise returns [`None`].
            #[inline]
            #[must_use]
            pub const fn new((x1, y1): Point<$T>, (x2, y2): Point<$T>) -> Option<Self> {
                if y1 == y2 {
                    return match Axis0::<$T>::new(y1, x1, x2) {
                        Axis::Positive(me) => Some(Self::PositiveAxis0(me)),
                        Axis::Negative(me) => Some(Self::NegativeAxis0(me)),
                    };
                }
                if x1 == x2 {
                    return match Axis1::<$T>::new(x1, y1, y2) {
                        Axis::Positive(me) => Some(Self::PositiveAxis1(me)),
                        Axis::Negative(me) => Some(Self::NegativeAxis1(me)),
                    };
                }
                None
            }

            /// Clips a *half-open* line segment to a [rectangular region](Clip)
            /// if it is aligned to any [axis](Axis), and returns an iterator over it.
            ///
            /// Returns [`None`] if the line segment is not axis-aligned,
            /// is empty, or does not intersect the clipping region.
            #[inline]
            #[must_use]
            pub const fn clip((x1, y1): Point<$T>, (x2, y2): Point<$T>, clip: &Clip<$T>) -> Option<Self> {
                if y1 == y2 {
                    return map!(
                        Axis0::<$T>::clip(y1, x1, x2, clip),
                        me => match me {
                            Axis::Positive(me) => Self::PositiveAxis0(me),
                            Axis::Negative(me) => Self::NegativeAxis0(me),
                        }
                    );
                }
                if x1 == x2 {
                    return map!(
                        Axis1::<$T>::clip(x1, y1, y2, clip),
                        me => match me {
                            Axis::Positive(me) => Self::PositiveAxis1(me),
                            Axis::Negative(me) => Self::NegativeAxis1(me),
                        }
                    );
                }
                None
            }

            impl_axis_methods!($T, match_axis);
        }

        impl_axis_iters!(AnyAxis<$T>, match_axis $(, cfg_esi = $cfg_esi)?);
    };
}

all_nums!(impl_any_axis);

#[cfg(test)]
mod static_tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    const fn iterator_8() {
        assert_impl_all!(PositiveAxis0<i8>: ExactSizeIterator);
        assert_impl_all!(PositiveAxis0<u8>: ExactSizeIterator);
        assert_impl_all!(Axis0<i8>: ExactSizeIterator);
        assert_impl_all!(Axis0<u8>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<i8>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<u8>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_16() {
        assert_impl_all!(PositiveAxis0<i16>: ExactSizeIterator);
        assert_impl_all!(PositiveAxis0<u16>: ExactSizeIterator);
        assert_impl_all!(Axis0<i16>: ExactSizeIterator);
        assert_impl_all!(Axis0<u16>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<i16>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<u16>: ExactSizeIterator);
    }

    #[test]
    const fn iterator_32() {
        #[cfg(target_pointer_width = "16")]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(PositiveAxis0<i32>: Iterator);
            assert_impl_all!(PositiveAxis0<u32>: Iterator);
            assert_impl_all!(Axis0<i32>: Iterator);
            assert_impl_all!(Axis0<u32>: Iterator);
            assert_impl_all!(AnyAxis<i32>: Iterator);
            assert_impl_all!(AnyAxis<u32>: Iterator);
            assert_not_impl_any!(PositiveAxis0<i32>: ExactSizeIterator);
            assert_not_impl_any!(PositiveAxis0<u32>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<i32>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<u32>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<i32>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<u32>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            assert_impl_all!(PositiveAxis0<i32>: ExactSizeIterator);
            assert_impl_all!(PositiveAxis0<u32>: ExactSizeIterator);
            assert_impl_all!(Axis0<i32>: ExactSizeIterator);
            assert_impl_all!(Axis0<u32>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<i32>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<u32>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_64() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_impl_all!(PositiveAxis0<i64>: ExactSizeIterator);
            assert_impl_all!(PositiveAxis0<u64>: ExactSizeIterator);
            assert_impl_all!(Axis0<i64>: ExactSizeIterator);
            assert_impl_all!(Axis0<u64>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<i64>: ExactSizeIterator);
            assert_impl_all!(AnyAxis<u64>: ExactSizeIterator);
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        {
            use static_assertions::assert_not_impl_any;

            assert_impl_all!(PositiveAxis0<i64>: Iterator);
            assert_impl_all!(PositiveAxis0<u64>: Iterator);
            assert_impl_all!(Axis0<i64>: Iterator);
            assert_impl_all!(Axis0<u64>: Iterator);
            assert_impl_all!(AnyAxis<i64>: Iterator);
            assert_impl_all!(AnyAxis<u64>: Iterator);
            assert_not_impl_any!(PositiveAxis0<i64>: ExactSizeIterator);
            assert_not_impl_any!(PositiveAxis0<u64>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<i64>: ExactSizeIterator);
            assert_not_impl_any!(Axis0<u64>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<i64>: ExactSizeIterator);
            assert_not_impl_any!(AnyAxis<u64>: ExactSizeIterator);
        }
    }

    #[test]
    const fn iterator_pointer_size() {
        assert_impl_all!(PositiveAxis0<isize>: ExactSizeIterator);
        assert_impl_all!(PositiveAxis0<usize>: ExactSizeIterator);
        assert_impl_all!(Axis0<isize>: ExactSizeIterator);
        assert_impl_all!(Axis0<usize>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<isize>: ExactSizeIterator);
        assert_impl_all!(AnyAxis<usize>: ExactSizeIterator);
    }
}
