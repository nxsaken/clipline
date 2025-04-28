//! Macros.

/// Delegates `$call` to all `$Enum::$Variant($me)`.
macro_rules! variant {
    ($Enum:ident::{$($Variant:ident),* $(,)?}, $self:ident, $me:ident => $call:expr) => {
        match $self {
            $($Enum::$Variant($me) => $call,)*
        }
    };
}

/// Implements inherent iterator methods or delegates to variants.
macro_rules! impl_fwd {
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
        impl_fwd!(
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
macro_rules! impl_rev {
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
        impl_rev!(
            self,
            $T,
            tail = variant!($Enum::{$($Variant),*}, self, me => me.tail()),
            pop_tail = variant!($Enum::{$($Variant),*}, self, me => me.pop_tail()),
        );
    };
}

/// Implements [`Iterator`] or delegates to variants.
///
/// Also implements:
/// - [`core::iter::FusedIterator`]
/// - [`ExactSizeIterator`] (conditionally on `cfg_esi`)
macro_rules! impl_iter_fwd {
    (
        $Iter:ident<$(const $BOOL:ident,)* $T:ty>,
        $self:ident,
        next = $next:expr,
        size_hint = $size_hint:expr,
        is_empty = $is_empty:expr
        $(, |$init:ident, $f:ident| {
            fold = $fold:expr,
            try_fold = $try_fold:expr $(,)?
        })?
        $(, cfg_esi = $cfg_esi:meta)? $(,)?
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

        $(#[$cfg_esi])?
        impl<$(const $BOOL: bool),*> ExactSizeIterator for $Iter<$($BOOL,)? $T> {
            #[cfg(feature = "is_empty")]
            #[inline]
            fn is_empty(&$self) -> bool { $is_empty }
        }
    };
    (
        $Enum:ident<$(const $BOOL:ident,)* $T:ty>::{$($Variant:ident),* $(,)?}
        $(, cfg_esi = $cfg_esi:meta)? $(,)?
    ) => {
        impl_iter_fwd!(
            $Enum<$(const $BOOL,)* $T>,
            self,
            next = variant!($Enum::{$($Variant),*}, self, me => me.next()),
            size_hint = variant!($Enum::{$($Variant),*}, self, me => me.size_hint()),
            is_empty = variant!($Enum::{$($Variant),*}, self, me => me.is_empty()),
            |init, f| {
                fold = variant!($Enum::{$($Variant),*}, self, me => me.fold(init, f)),
                try_fold = variant!($Enum::{$($Variant),*}, self, me => me.try_fold(init, f)),
            }
            $(, cfg_esi = $cfg_esi)?
        );
    };
}

/// Implements [`DoubleEndedIterator`] or delegates to variants.
macro_rules! impl_iter_rev {
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
        impl_iter_rev!(
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

/// Applies the macro `m` to multiple integer types.
macro_rules! all_nums {
    ($m:ident $(,)?) => {
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

pub(crate) use {all_nums, impl_fwd, impl_iter_fwd, impl_iter_rev, impl_rev, variant};

/// Selects an expression based on `V`.
macro_rules! v {
    ($h:expr, $v:expr $(,)?) => {
        if !V {
            $h
        } else {
            $v
        }
    };
}

/// Selects an expression based on `YX`.
macro_rules! yx {
    ($x:expr, $y:expr $(,)?) => {
        if !YX {
            $x
        } else {
            $y
        }
    };
    ($x_y:expr) => {{
        let (x, y) = $x_y;
        if !YX {
            x
        } else {
            y
        }
    }};
}

/// Selects an expression based on `F`.
macro_rules! f {
    ($pos:expr, $neg:expr $(,)?) => {
        if !F {
            $pos
        } else {
            $neg
        }
    };
}

/// Selects an expression based on `FX`.
macro_rules! fx {
    ($pos:expr, $neg:expr $(,)?) => {
        if !FX {
            $pos
        } else {
            $neg
        }
    };
}

/// Selects an expression based on `FY`.
macro_rules! fy {
    ($pos:expr, $neg:expr $(,)?) => {
        if !FY {
            $pos
        } else {
            $neg
        }
    };
}

pub(crate) use {f, fx, fy, v, yx};

/// An [`Option::map`] for `const` contexts.
macro_rules! map {
    ($opt:expr, |$some:ident| $body:expr $(,)?) => {
        match $opt {
            None => None,
            Some($some) => Some($body),
        }
    };
    ($opt:expr, $func:expr $(,)?) => {
        match $opt {
            None => None,
            Some(me) => Some($func(me)),
        }
    };
}

/// Short-circuits with [`None`] or `$ret` if `$cond` is `true`.
macro_rules! return_if {
    ($cond:expr) => {
        if $cond {
            return None;
        }
    };
    ($cond:expr, $ret:expr) => {
        if $cond {
            return $ret;
        }
    };
}

pub(crate) use {map, return_if};
