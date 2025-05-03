/// Implements inherent forward iterator methods or delegates to variants.
macro_rules! fwd {
    (
        $self:ident,
        $T:ty,
        is_done = $is_done:expr,
        length = $length:expr,
        head = $head:expr,
        pop_head = $pop_head:expr,
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

        /// Consumes and returns the point at the start of the iterator.
        ///
        /// Returns [`None`] if the iterator has terminated.
        #[inline]
        #[must_use]
        pub const fn pop_head(&mut $self) -> Option<Point<$T>> { $pop_head }
    };
    ($T:ty, $Enum:ident::{$($Variant:ident),* $(,)?}) => {
        fwd!(
            self,
            $T,
            is_done = variant!($Enum::{$($Variant),*}, self, me => me.is_done()),
            length = variant!($Enum::{$($Variant),*}, self, me => me.length()),
            head = variant!($Enum::{$($Variant),*}, self, me => me.head()),
            pop_head = variant!($Enum::{$($Variant),*}, self, me => me.pop_head()),
        );
    };
}

/// Implements inherent reversed iterator methods or delegates to variants.
macro_rules! rev {
    (
        $self:ident,
        $T:ty,
        tail = $tail:expr,
        pop_tail = $pop_tail:expr$(,)?
    ) => {
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

        /// Consumes the point at the end of the iterator, and returns the point immediately before.
        ///
        /// Returns [`None`] if the iterator has terminated.
        #[inline]
        #[must_use]
        pub const fn pop_tail(&mut $self) -> Option<Point<$T>> { $pop_tail }
    };
    ($T:ty, $Enum:ident::{$($Variant:ident),* $(,)?}) => {
        rev!(
            self,
            $T,
            tail = variant!($Enum::{$($Variant),*}, self, me => me.tail()),
            pop_tail = variant!($Enum::{$($Variant),*}, self, me => me.pop_tail()),
        );
    };
}

/// Implements [`Iterator`] or delegates to variants.
///
/// Also implements [`core::iter::FusedIterator`].
macro_rules! iter_fwd {
    (
        $Iter:ident<$(const $BOOL:ident,)* $T:ty>,
        $self:ident,
        next = $next:expr,
        size_hint = $size_hint:expr
        $(, |$init:ident, $f:ident| {
            fold = $fold:expr,
            try_fold = $try_fold:expr $(,)?
        })? $(,)?
    ) => {
        impl<$(const $BOOL: bool),*> Iterator for $Iter<$($BOOL,)? $T> {
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

        impl<$(const $BOOL: bool),*> core::iter::FusedIterator for $Iter<$($BOOL,)? $T> {}
    };
    ($Enum:ident<$(const $BOOL:ident,)* $T:ty>::{$($Variant:ident),* $(,)?} $(,)?) => {
        iter_fwd!(
            $Enum<$(const $BOOL,)* $T>,
            self,
            next = variant!($Enum::{$($Variant),*}, self, me => me.next()),
            size_hint = variant!($Enum::{$($Variant),*}, self, me => me.size_hint()),
            |init, f| {
                fold = variant!($Enum::{$($Variant),*}, self, me => me.fold(init, f)),
                try_fold = variant!($Enum::{$($Variant),*}, self, me => me.try_fold(init, f)),
            },
        );
    };
}

/// Implements [`ExactSizeIterator`] or delegates to variants.
macro_rules! iter_esi {
    (
        $Iter:ident<$(const $BOOL:ident,)* $T:ty>,
        $self:ident,
        is_empty = $is_empty:expr $(,)?
    ) => {
        impl<$(const $BOOL: bool),*> ExactSizeIterator for $Iter<$($BOOL,)? $T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&$self) -> bool { $is_empty }
        }
    };
    ($Enum:ident<$(const $BOOL:ident,)* $T:ty>::{$($Variant:ident),* $(,)?} $(,)?) => {
        iter_esi!(
            $Enum<$(const $BOOL,)* $T>,
            self,
            is_empty = variant!($Enum::{$($Variant),*}, self, me => me.is_empty()),
        );
    };
}

/// Implements [`DoubleEndedIterator`] or delegates to variants.
macro_rules! iter_rev {
    (
        $Iter:ident<$(const $BOOL:ident,)* $T:ty>,
        $self:ident,
        next_back = $next_back:expr
        $(, |$init:ident, $f:ident| {
            rfold = $rfold:expr,
            try_rfold = $try_rfold:expr $(,)?
        })? $(,)?
    ) => {
        impl<$(const $BOOL: bool),*> DoubleEndedIterator for $Iter<$($BOOL,)? $T> {
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
    };
    ($Enum:ident<$(const $BOOL:ident,)* $T:ty>::{$($Variant:ident),* $(,)?}) => {
        iter_rev!(
            $Enum<$(const $BOOL,)* $T>,
            self,
            next_back = variant!($Enum::{$($Variant),*}, self, me => me.next_back()),
            |init, f| {
                rfold = variant!($Enum::{$($Variant),*}, self, me => me.rfold(init, f)),
                try_rfold = variant!($Enum::{$($Variant),*}, self, me => me.try_rfold(init, f))
            }
        );
    };
}

/// Applies the macro m to multiple integer types.
macro_rules! nums {
    ($m:ident) => {
        $m!(i8);
        $m!(u8);
        $m!(i16);
        $m!(u16);
        $m!(i32);
        $m!(u32);
        $m!(i64);
        $m!(u64);
        $m!(isize);
        $m!(usize);
    };
    ($m:ident, signed_unsigned) => {
        $m!(i8, signed);
        $m!(u8, unsigned);
        $m!(i16, signed);
        $m!(u16, unsigned);
        $m!(i32, signed);
        $m!(u32, unsigned);
        $m!(i64, signed);
        $m!(u64, unsigned);
        $m!(isize, signed);
        $m!(usize, unsigned);
    };
    ($m:ident, cfg_size) => {
        $m!(i8, cfg_size = cfg(all()));
        $m!(u8, cfg_size = cfg(all()));
        $m!(i16, cfg_size = cfg(all()));
        $m!(u16, cfg_size = cfg(all()));
        $m!(i32, cfg_size = cfg(any(target_pointer_width = "64", target_pointer_width = "32")));
        $m!(u32, cfg_size = cfg(any(target_pointer_width = "64", target_pointer_width = "32")));
        $m!(i64, cfg_size = cfg(any(target_pointer_width = "64")));
        $m!(u64, cfg_size = cfg(any(target_pointer_width = "64")));
        $m!(isize, cfg_size = cfg(all()));
        $m!(usize, cfg_size = cfg(all()));
    };
    ($m:ident, cfg_octant_64) => {
        $m!(i8);
        $m!(u8);
        $m!(i16);
        $m!(u16);
        $m!(i32);
        $m!(u32);
        #[cfg(feature = "octant_64")]
        $m!(i64);
        #[cfg(feature = "octant_64")]
        $m!(u64);
        #[cfg(any(
            target_pointer_width = "16",
            target_pointer_width = "32",
            all(target_pointer_width = "64", feature = "octant_64")
        ))]
        $m!(isize);
        #[cfg(any(
            target_pointer_width = "16",
            target_pointer_width = "32",
            all(target_pointer_width = "64", feature = "octant_64")
        ))]
        $m!(usize);
    };
    ($m:ident, cfg_size, cfg_octant_64) => {
        $m!(i8, cfg_size = cfg(all()));
        $m!(u8, cfg_size = cfg(all()));
        $m!(i16, cfg_size = cfg(all()));
        $m!(u16, cfg_size = cfg(all()));
        $m!(i32, cfg_size = cfg(any(target_pointer_width = "64", target_pointer_width = "32")));
        $m!(u32, cfg_size = cfg(any(target_pointer_width = "64", target_pointer_width = "32")));
        #[cfg(feature = "octant_64")]
        $m!(i64, cfg_size = cfg(any(target_pointer_width = "64")));
        #[cfg(feature = "octant_64")]
        $m!(u64, cfg_size = cfg(any(target_pointer_width = "64")));
        #[cfg(any(
            target_pointer_width = "16",
            target_pointer_width = "32",
            all(target_pointer_width = "64", feature = "octant_64")
        ))]
        $m!(isize, cfg_size = cfg(all()));
        #[cfg(any(
            target_pointer_width = "16",
            target_pointer_width = "32",
            all(target_pointer_width = "64", feature = "octant_64")
        ))]
        $m!(usize, cfg_size = cfg(all()));
    };
}

pub(crate) use {fwd, iter_esi, iter_fwd, iter_rev, nums, rev};
