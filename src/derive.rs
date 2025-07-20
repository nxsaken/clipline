macro_rules! clone {
    ([$($generics:tt)+] $ty:ty) => {
        impl<$($generics)*> $ty {
            #[inline]
            pub const fn clone(&self) -> Self {
                Self { ..*self }
            }
        }
    };
    ([$($generics:tt)+] $ty:ty {$($var:ident),+}) => {
        impl<$($generics)*> $ty {
            #[inline]
            pub const fn clone(&self) -> Self {
                match self {
                    $(Self::$var(v) => Self::$var(v.clone()),)+
                }
            }
        }
    };
}

macro_rules! iter_methods {
    (
        C = $C:ty,
        U = $U:ty,
        self = $self:ident,
        fn is_empty = $is_empty:expr,
        fn len = $len:expr,
        fn head = $head:expr,
        fn pop_head = $pop_head:expr$(,
        fn tail = $tail:expr,
        fn pop_tail = $pop_tail:expr)?
    ) => {
        #[inline]
        pub const fn is_empty(&$self) -> bool { $is_empty }
        #[inline]
        pub const fn len(&$self) -> $U { $len }
        #[inline]
        pub const fn head(&$self) -> Option<($C, $C)> { $head }
        #[inline]
        pub const fn pop_head(&mut $self) -> Option<($C, $C)> { $pop_head }$(
        #[inline]
        pub const fn tail(&$self) -> Option<($C, $C)> { $tail }
        #[inline]
        pub const fn pop_tail(&mut $self) -> Option<($C, $C)> { $pop_tail })?
    };
}

macro_rules! iter_fwd {
    (
        $Line:ident<$(const $YX:ident,)? $C:ty>$(,
        fn fold($self:ident, $accum:ident, $f:ident) = $fold:expr)?
    ) => {
        impl<$(const $YX: bool)?> Iterator for $Line<$($YX,)? $C> {
            type Item = ($C, $C);
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.pop_head()
            }
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = usize::from(self.len());
                (len, Some(len))
            }$(
            #[inline]
            fn fold<B, F>($self, $accum: B, $f: F) -> B
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> B
            {
                $fold
            })?
        }
        impl<$(const $YX: bool)?> ExactSizeIterator for $Line<$($YX,)? $C> {}
        impl<$(const $YX: bool)?> core::iter::FusedIterator for $Line<$($YX,)? $C> {}
    };
    (
        $Line:ident<$(const $YX:ident,)? $C:ty>$(,
        fn fold($self:ident, $accum:ident, $f:ident) = $fold:expr)?,
        exact = [$($ptr_size:literal),+]
    ) => {
        impl<$(const $YX: bool)?> Iterator for $Line<$($YX,)? $C> {
            type Item = ($C, $C);
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.pop_head()
            }
            #[cfg(any($(target_pointer_width = $ptr_size),+))]
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = usize::try_from(self.len()).expect("cannot overflow");
                (len, Some(len))
            }
            #[cfg(not(any($(target_pointer_width = $ptr_size),+)))]
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                if let Ok(len) = usize::try_from(self.len()) {
                    (len, Some(len))
                } else {
                    (usize::MAX, None)
                }
            }$(
            #[inline]
            fn fold<B, F>($self, $accum: B, $f: F) -> B
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> B
            {
                $fold
            })?
        }
        #[cfg(any($(target_pointer_width = $ptr_size),+))]
        impl<$(const $YX: bool)?> ExactSizeIterator for $Line<$($YX,)? $C> {}
        impl<$(const $YX: bool)?> core::iter::FusedIterator for $Line<$($YX,)? $C> {}
    };
}

macro_rules! iter_rev {
    (
        $Line:ident<$(const $YX:ident,)? $C:ty>$(,
        fn rfold($self:ident, $accum:ident, $f:ident) = $rfold:expr)?
    ) => {
        impl<$(const $YX: bool)?> DoubleEndedIterator for $Line<$($YX,)? $C> {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.pop_tail()
            }$(
            #[inline]
            fn rfold<B, F>(mut $self, mut $accum: B, mut $f: F) -> B
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> B,
            {
                $rfold
            })?
        }
    };
}

pub(crate) use {clone, iter_fwd, iter_methods, iter_rev};
