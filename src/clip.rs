use crate::macros::*;
use crate::math::{Coord, ops};

mod line_a;
mod line_b;
mod line_d;
mod point;

/// A closed[^1] rectangular clipping region with a zero origin and a maximum corner.
///
/// Use [`Viewport`] if the region needs an arbitrary position.
///
/// [^1]: `x_max` and `y_max` are inside the region.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Clip<C: Coord> {
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

/// A closed[^1] rectangular clipping region with a minimum and maximum corner.
///
/// Use [`Clip`] if the region is always positioned at zero.
///
/// [^1]: `x_min`, `y_min`, `x_max` and `y_max` are inside the region.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Viewport<C: Coord> {
    pub(crate) x_min: C,
    pub(crate) y_min: C,
    pub(crate) x_max: C,
    pub(crate) y_max: C,
}

macro_rules! clip {
    ($U:ty|$I:ty) => {
        clip!(@impl Clip<unsigned $U>, $U);
        clip!(@impl Clip<signed $I>, $U);
        clip!(@impl Viewport<$U>, $U);
        clip!(@impl Viewport<$I>, $U);
    };
    (@impl Clip<$signedness:ident $UI:ty>, $U:ty) => {
        impl Clip<$UI> {
            /// Returns a [`Clip`] with the given maximum
            #[doc = if_unsigned!($signedness <
                "corner."
            > else <
                "corner, or [`None`] if `x_max` or `y_max` is negative."
            >)]
            #[inline]
            pub const fn from_max(
                x_max: $UI,
                y_max: $UI,
            ) -> if_unsigned!($signedness [Self] else [Option<Self>]) {
                if_unsigned!($signedness {
                    Self { x_max, y_max }
                } else {
                    if x_max < 0 || y_max < 0 {
                        return None;
                    }
                    Some(Self { x_max, y_max })
                })
            }

            /// Returns a [`Clip`] with the given size, or [`None`] if
            #[doc = if_unsigned!($signedness <
                "`width` or `height` is zero."
            > else <
                "either:\n\
                - `width` or `height` is zero,\n\
                - `width - 1` or `height - 1` is greater than the largest possible coordinate."
            >)]
            #[inline]
            pub const fn from_size(width: $U, height: $U) -> Option<Self> {
                let (x_max, y_max) = if_unsigned!($signedness {
                    if width == 0 || height == 0 {
                        return None;
                    }
                    let x_max = width - 1;
                    let y_max = height - 1;
                    (x_max, y_max)
                } else {
                    const MAX: $U = <$UI>::MAX as $U + 1;
                    if width == 0 || height == 0 || MAX < width || MAX < height {
                        return None;
                    }
                    let x_max = (width - 1) as $UI;
                    let y_max = (height - 1) as $UI;
                    (x_max, y_max)
                });
                Some(Self { x_max, y_max })
            }
        }
    };
    (@impl Viewport<$UI:ty>, $U:ty) => {
        impl Viewport<$UI> {
            /// Returns a [`Viewport`] with the given minimum and maximum corners,
            /// or [`None`] if `x_max < x_min` or `y_max < y_min`.
            #[inline]
            pub const fn from_min_max(
                x_min: $UI,
                y_min: $UI,
                x_max: $UI,
                y_max: $UI,
            ) -> Option<Self> {
                if x_max < x_min || y_max < y_min {
                    return None;
                }
                Some(Self { x_min, y_min, x_max, y_max })
            }

            /// Returns a [`Viewport`] with the given minimum corner and size,
            /// or [`None`] if either:
            /// - `width` or `height` is zero,
            /// - `x_min + width` or `y_min + height` overflows.
            #[inline]
            pub const fn from_min_size(
                x_min: $UI,
                y_min: $UI,
                width: $U,
                height: $U,
            ) -> Option<Self> {
                if width == 0 || height == 0 {
                    return None;
                }
                let dx = width - 1;
                let dy = height - 1;
                let x_max = try_opt!(ops::<$UI>::chadd_u(x_min, dx));
                let y_max = try_opt!(ops::<$UI>::chadd_u(y_min, dy));
                Some(Self { x_min, y_min, x_max, y_max })
            }
        }
    };
    (@impl[$($generics:tt)+] MinMax for $Self:ident<$UI:ty> { $self:ident, $x_min:expr, $y_min:expr }) => {
        impl<$($generics)*> $Self<$UI> {
            /// Returns the minimum `x` coordinate of this clipping region.
            #[inline]
            pub const fn x_min(&$self) -> $UI {
                $x_min
            }

            /// Returns the minimum `y` coordinate of this clipping region.
            #[inline]
            pub const fn y_min(&$self) -> $UI {
                $y_min
            }

            /// Returns the maximum `x` coordinate of this clipping region.
            #[inline]
            pub const fn x_max(&self) -> $UI {
                self.x_max
            }

            /// Returns the maximum `y` coordinate of this clipping region.
            #[inline]
            pub const fn y_max(&self) -> $UI {
                self.y_max
            }

            #[inline]
            const fn u_min<const YX: bool>(&self) -> $UI {
                if YX { self.y_min() } else { self.x_min() }
            }

            #[inline]
            const fn v_min<const YX: bool>(&self) -> $UI {
                if YX { self.x_min() } else { self.y_min() }
            }

            #[inline]
            const fn u_max<const YX: bool>(&self) -> $UI {
                if YX { self.y_max } else { self.x_max }
            }

            #[inline]
            const fn v_max<const YX: bool>(&self) -> $UI {
                if YX { self.x_max } else { self.y_max }
            }

            #[inline]
            const fn uv_min_max<const YX: bool>(&self) -> ($UI, $UI, $UI, $UI) {
                (
                    self.u_min::<YX>(),
                    self.v_min::<YX>(),
                    self.u_max::<YX>(),
                    self.v_max::<YX>(),
                )
            }
        }
    };
}

clone!([C: Coord] Clip<C>);
clone!([C: Coord] Viewport<C>);

clip!(@impl[C: Coord] MinMax for Clip<C> { self, C::ZERO, C::ZERO });
clip!(@impl[C: Coord] MinMax for Viewport<C> { self, self.x_min, self.y_min });

clip!(u8 | i8);
clip!(u16 | i16);
clip!(u32 | i32);
clip!(u64 | i64);
clip!(usize | isize);
