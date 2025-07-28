macro_rules! clone {
    ([$($generics:tt)+] $ty:ty) => {
        impl<$($generics)*> $ty {
            /// Clones this iterator.
            #[inline]
            pub const fn clone(&self) -> Self {
                Self { ..*self }
            }
        }
    };
    ([$($generics:tt)+] $ty:ty {$($var:ident),+}) => {
        impl<$($generics)*> $ty {
            /// Clones this iterator.
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
        /// Returns `true` if the iterator is empty.
        #[inline]
        pub const fn is_empty(&$self) -> bool { $is_empty }

        /// Returns the remaining length of this iterator.
        #[inline]
        pub const fn len(&$self) -> $U { $len }

        /// Returns the point at the start of the iterator.
        /// This does not advance the iterator.
        ///
        /// Returns [`None`] if the iterator is empty.
        #[inline]
        pub const fn head(&$self) -> Option<($C, $C)> { $head }

        /// Consumes and returns the point at the start of the iterator.
        /// This advances the iterator forwards.
        ///
        /// Returns [`None`] if the iterator is empty.
        #[inline]
        pub const fn pop_head(&mut $self) -> Option<($C, $C)> { $pop_head }$(

        /// Returns the last point of the iterator.
        /// This does not advance the iterator.
        ///
        /// Returns [`None`] if the iterator is empty.
        ///
        /// # Performance
        ///
        /// This method may perform extra arithmetic to compute the last point.
        /// Avoid pairing this with [`Self::pop_tail`], or the work will be redone.
        #[inline]
        pub const fn tail(&$self) -> Option<($C, $C)> { $tail }

        /// Consumes and returns the last point of the iterator.
        /// This advances the iterator backwards.
        ///
        /// Returns [`None`] if the iterator is empty.
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
            fn rfold<B, F>($self, $accum: B, $f: F) -> B
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

#[rustfmt::skip]
macro_rules! if_unsigned {
    (unsigned $unsigned:block else $signed:block) => { $unsigned };
    (signed   $unsigned:block else $signed:block) => { $signed };
    (unsigned [$unsigned:ty] else [$signed:ty]) => { $unsigned };
    (signed   [$unsigned:ty] else [$signed:ty]) => { $signed };
    (unsigned <$unsigned:literal> else <$signed:literal>) => { $unsigned };
    (signed   <$unsigned:literal> else <$signed:literal>) => { $signed };
}

pub(crate) use if_unsigned;

macro_rules! try_opt {
    ($opt:expr) => {
        match $opt {
            Some(v) => v,
            None => return None,
        }
    };
}

pub(crate) use try_opt;
