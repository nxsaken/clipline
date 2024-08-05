//! ### Bresenham clipping
//!
//! This module provides [clipping](Clip) utilities for
//! [slanted](Octant) directed line segments.

use super::Octant;
use crate::clip::Clip;
use crate::math::{Delta, Delta2, Math, Num, Point};
use crate::symmetry::{fx, fy, xy};

const O: bool = false;
const I: bool = true;

type LineCode = (bool, bool, bool, bool);

const INSIDE_INSIDE: LineCode = (O, O, O, O);
const INSIDE_V_EXIT: LineCode = (O, O, O, I);
const INSIDE_U_EXIT: LineCode = (O, O, I, O);
const INSIDE_UV_EXIT: LineCode = (O, O, I, I);
const V_ENTRY_INSIDE: LineCode = (O, I, O, O);
const V_ENTRY_V_EXIT: LineCode = (O, I, O, I);
const V_ENTRY_U_EXIT: LineCode = (O, I, I, O);
const V_ENTRY_UV_EXIT: LineCode = (O, I, I, I);
const U_ENTRY_INSIDE: LineCode = (I, O, O, O);
const U_ENTRY_V_EXIT: LineCode = (I, O, O, I);
const U_ENTRY_U_EXIT: LineCode = (I, O, I, O);
const U_ENTRY_UV_EXIT: LineCode = (I, O, I, I);
const UV_ENTRY_INSIDE: LineCode = (I, I, O, O);
const UV_ENTRY_V_EXIT: LineCode = (I, I, O, I);
const UV_ENTRY_U_EXIT: LineCode = (I, I, I, O);
const UV_ENTRY_UV_EXIT: LineCode = (I, I, I, I);

macro_rules! clip_impl {
    ($T:ty, $add:ident, $sub:ident) => {
        impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<FX, FY, SWAP, $T> {
            /// Checks if the line segment enters the clipping region through a vertical side.
            #[inline(always)]
            #[must_use]
            const fn enters_u(u1: $T, Clip { wx1, wy1, wx2, wy2 }: Clip<$T>) -> bool {
                xy!(fx!(u1 < wx1, wx2 < u1), fy!(u1 < wy1, wy2 < u1))
            }

            /// Checks if the line segment enters the clipping region through a horizontal side.
            #[inline(always)]
            #[must_use]
            const fn enters_v(v1: $T, Clip { wx1, wy1, wx2, wy2 }: Clip<$T>) -> bool {
                xy!(fy!(v1 < wy1, wy2 < v1), fx!(v1 < wx1, wx2 < v1))
            }

            /// Checks if the line segment exits the clipping region through a vertical side.
            #[inline(always)]
            #[must_use]
            const fn exits_u(u2: $T, Clip { wx1, wy1, wx2, wy2 }: Clip<$T>) -> bool {
                xy!(fx!(wx2 < u2, u2 < wx1), fy!(wy2 < u2, u2 < wy1))
            }

            /// Checks if the line segment exits the clipping region through a horizontal side.
            #[inline(always)]
            #[must_use]
            const fn exits_v(v2: $T, Clip { wx1, wy1, wx2, wy2 }: Clip<$T>) -> bool {
                xy!(fy!(wy2 < v2, v2 < wy1), fx!(wx2 < v2, v2 < wx1))
            }

            #[allow(non_snake_case)]
            #[inline(always)]
            #[must_use]
            const fn tu1(
                u1: $T,
                dv: <$T as Num>::U,
                Clip { wx1, wy1, wx2, wy2 }: Clip<$T>,
            ) -> <$T as Num>::U2 {
                let Du1 = xy!(
                    fx!(Math::<$T>::delta(wx1, u1), Math::<$T>::delta(u1, wx2)),
                    fy!(Math::<$T>::delta(wy1, u1), Math::<$T>::delta(u1, wy2)),
                );
                Math::<$T>::wide_mul(Du1, dv)
            }

            #[allow(non_snake_case)]
            #[inline(always)]
            #[must_use]
            const fn tu2(
                u1: $T,
                dv: <$T as Num>::U,
                Clip { wx1, wy1, wx2, wy2 }: Clip<$T>,
            ) -> <$T as Num>::U2 {
                let Du2 = xy!(
                    fx!(Math::<$T>::delta(wx2, u1), Math::<$T>::delta(u1, wx1)),
                    fy!(Math::<$T>::delta(wy2, u1), Math::<$T>::delta(u1, wy1)),
                );
                Math::<$T>::wide_mul(Du2, dv)
            }

            #[allow(non_snake_case)]
            #[inline(always)]
            #[must_use]
            const fn tv1(
                v1: $T,
                du: <$T as Num>::U,
                half_du: <$T as Num>::U,
                Clip { wx1, wy1, wx2, wy2 }: Clip<$T>,
            ) -> <$T as Num>::U2 {
                let Dv1 = xy!(
                    fy!(Math::<$T>::delta(wy1, v1), Math::<$T>::delta(v1, wy2)),
                    fx!(Math::<$T>::delta(wx1, v1), Math::<$T>::delta(v1, wx2)),
                );
                Math::<$T>::wide_mul(Dv1, du).wrapping_sub(half_du as _)
            }

            #[allow(non_snake_case)]
            #[inline(always)]
            #[must_use]
            const fn tv2_naive(
                v1: $T,
                du: <$T as Num>::U,
                Clip { wx1, wy1, wx2, wy2 }: Clip<$T>,
            ) -> <$T as Num>::U2 {
                let Dv2 = xy!(
                    fy!(Math::<$T>::delta(wy2, v1), Math::<$T>::delta(v1, wy1)),
                    fx!(Math::<$T>::delta(wx2, v1), Math::<$T>::delta(v1, wx1)),
                );
                Math::<$T>::wide_mul(Dv2, du)
            }

            #[inline(always)]
            #[must_use]
            const fn tv2(naive: <$T as Num>::U2, half_du: <$T as Num>::U) -> <$T as Num>::U2 {
                naive.wrapping_add(half_du as _)
            }

            #[inline(always)]
            #[must_use]
            const fn cu1_v(
                u1: $T,
                (half_du, dv): Delta<$T>,
                tv1: <$T as Num>::U2,
                mut error: <$T as Num>::I2,
            ) -> ($T, <$T as Num>::I2) {
                // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
                let (mut q, r) = unsafe { Math::<$T>::div_rem(tv1, dv) };
                error = error.wrapping_sub(half_du as _).$sub(r as _);
                if 0 < r {
                    q = xy!(
                        fx!(q.wrapping_add(1), q.wrapping_add(1)),
                        fy!(q.wrapping_add(1), q.wrapping_add(1))
                    );
                    error = error.$add(dv as _);
                };
                let cu1 = xy!(fx!(u1.$add(q), u1.$sub(q)), fy!(u1.$add(q), u1.$sub(q)),);
                (cu1, error)
            }

            #[inline(always)]
            #[must_use]
            const fn cv1_u(
                v1: $T,
                du: <$T as Num>::U,
                tu1: <$T as Num>::U2,
                mut error: <$T as Num>::I2,
            ) -> ($T, <$T as Num>::I2) {
                // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
                let (mut q, r) = unsafe { Math::<$T>::div_rem(tu1, du) };
                error = error.$add(r as _);
                if {
                    let du = du as <$T as Num>::U2;
                    let r2 = Math::<$T>::double(r);
                    du <= r2
                } {
                    q = q.wrapping_add(1);
                    error = error.$sub(du as _);
                };
                let cv1 = xy!(fy!(v1.$add(q), v1.$sub(q)), fx!(v1.$add(q), v1.$sub(q)),);
                (cv1, error)
            }

            /// Clips at vertical entry.
            #[inline(always)]
            #[must_use]
            const fn c1_u(
                v1: $T,
                du: <$T as Num>::U,
                tu1: <$T as Num>::U2,
                error: <$T as Num>::I2,
                Clip { wx1, wy1, wx2, wy2 }: Clip<$T>,
            ) -> (Point<$T>, <$T as Num>::I2) {
                let cu1 = xy!(fx!(wx1, wx2), fy!(wy1, wy2));
                let (cv1, error) = Self::cv1_u(v1, du, tu1, error);
                ((cu1, cv1), error)
            }

            /// Clips at horizontal entry.
            #[inline(always)]
            #[must_use]
            const fn c1_v(
                u1: $T,
                (half_du, dv): Delta<$T>,
                tv1: <$T as Num>::U2,
                error: <$T as Num>::I2,
                Clip { wx1, wy1, wx2, wy2 }: Clip<$T>,
            ) -> (Point<$T>, <$T as Num>::I2) {
                let (cu1, error) = Self::cu1_v(u1, (half_du, dv), tv1, error);
                let cv1 = xy!(fy!(wy1, wy2), fx!(wx1, wx2));
                ((cu1, cv1), error)
            }

            #[inline(always)]
            #[must_use]
            const fn c1_uv(
                (u1, v1): Point<$T>,
                (du, dv): Delta<$T>,
                half_du: <$T as Num>::U,
                (tu1, tv1): Delta2<$T>,
                error: <$T as Num>::I2,
                clip: Clip<$T>,
            ) -> (Point<$T>, <$T as Num>::I2) {
                if tv1 < tu1 {
                    Self::c1_u(v1, du, tu1, error, clip)
                } else {
                    Self::c1_v(u1, (half_du, dv), tv1, error, clip)
                }
            }

            #[inline(always)]
            #[must_use]
            const fn cu2_u(Clip { wx1, wy1, wx2, wy2 }: Clip<$T>) -> $T {
                // it is overflow-safe to add/sub 1 because of the exit condition
                xy!(
                    fx!(wx2.wrapping_add(1), wx1.wrapping_sub(1)),
                    fy!(wy2.wrapping_add(1), wy1.wrapping_sub(1))
                )
            }

            #[inline(always)]
            #[must_use]
            const fn cu2_v(u1: $T, dv: <$T as Num>::U, tv2: <$T as Num>::U2) -> $T {
                // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
                let (mut q, r) = unsafe { Math::<$T>::div_rem(tv2, dv) };
                if 0 == r {
                    q = q.wrapping_sub(1);
                }
                // it is overflow-safe to add/sub 1 because of the exit condition
                xy!(
                    fx!(u1.$add(q).wrapping_add(1), u1.$sub(q).wrapping_sub(1)),
                    fy!(u1.$add(q).wrapping_add(1), u1.$sub(q).wrapping_sub(1)),
                )
            }

            #[inline(always)]
            #[must_use]
            const fn cu2_uv(
                u1: $T,
                (half_du, dv): Delta<$T>,
                (tu2, tv2_naive): Delta2<$T>,
                clip: Clip<$T>,
            ) -> $T {
                let tv2 = Self::tv2(tv2_naive, half_du);
                if tu2 < tv2 {
                    Self::cu2_u(clip)
                } else {
                    Self::cu2_v(u1, dv, tv2)
                }
            }

            #[allow(clippy::too_many_lines)]
            #[inline(always)]
            #[must_use]
            pub(super) const fn clip_inner(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                (dx, dy): Delta<$T>,
                clip: Clip<$T>,
            ) -> Option<Self> {
                let (u1, v1) = xy!((x1, y1), (y1, x1));
                let (u2, v2) = xy!((x2, y2), (y2, x2));
                let (du, dv) = xy!((dx, dy), (dy, dx));
                let half_du = Math::<$T>::half(du);
                let error = Math::<$T>::error(dv, Math::<$T>::half(du));
                let (((cu1, cv1), error), end) = match (
                    Self::enters_u(u1, clip),
                    Self::enters_v(v1, clip),
                    Self::exits_u(u2, clip),
                    Self::exits_v(v2, clip),
                ) {
                    INSIDE_INSIDE => (((u1, v1), error), u2),
                    INSIDE_V_EXIT => {
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        let tv2 = Self::tv2(tv2_naive, half_du);
                        (((u1, v1), error), Self::cu2_v(u1, dv, tv2))
                    }
                    INSIDE_U_EXIT => (((u1, v1), error), Self::cu2_u(clip)),
                    INSIDE_UV_EXIT => {
                        let tu2 = Self::tu2(u1, dv, clip);
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        (((u1, v1), error), Self::cu2_uv(u1, (half_du, dv), (tu2, tv2_naive), clip))
                    }
                    V_ENTRY_INSIDE => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        (Self::c1_v(u1, (half_du, dv), tv1, error, clip), u2)
                    }
                    V_ENTRY_V_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        let tv2 = Self::tv2(tv2_naive, half_du);
                        (Self::c1_v(u1, (half_du, dv), tv1, error, clip), Self::cu2_v(u1, dv, tv2))
                    }
                    V_ENTRY_U_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tu2 = Self::tu2(u1, dv, clip);
                        if tu2 < tv1 {
                            return None;
                        }
                        (Self::c1_v(u1, (half_du, dv), tv1, error, clip), Self::cu2_u(clip))
                    }
                    V_ENTRY_UV_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tu2 = Self::tu2(u1, dv, clip);
                        if tu2 < tv1 {
                            return None;
                        }
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        (
                            Self::c1_v(u1, (half_du, dv), tv1, error, clip),
                            Self::cu2_uv(u1, (half_du, dv), (tu2, tv2_naive), clip),
                        )
                    }
                    U_ENTRY_INSIDE => {
                        (Self::c1_u(v1, du, Self::tu1(u1, dv, clip), error, clip), u2)
                    }
                    U_ENTRY_V_EXIT => {
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        let tv2 = Self::tv2(tv2_naive, half_du);
                        (
                            Self::c1_u(v1, du, Self::tu1(u1, dv, clip), error, clip),
                            Self::cu2_v(u1, dv, tv2),
                        )
                    }
                    U_ENTRY_U_EXIT => (
                        Self::c1_u(v1, du, Self::tu1(u1, dv, clip), error, clip),
                        Self::cu2_u(clip),
                    ),
                    U_ENTRY_UV_EXIT => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        if tv2_naive < tu1 {
                            return None;
                        }
                        let tu2 = Self::tu2(u1, dv, clip);
                        (
                            Self::c1_u(v1, du, tu1, error, clip),
                            Self::cu2_uv(u1, (half_du, dv), (tu2, tv2_naive), clip),
                        )
                    }
                    UV_ENTRY_INSIDE => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        (Self::c1_uv((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip), u2)
                    }
                    UV_ENTRY_V_EXIT => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        let tv2 = Self::tv2(tv2_naive, half_du);
                        (
                            Self::c1_uv((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip),
                            Self::cu2_v(u1, dv, tv2),
                        )
                    }
                    UV_ENTRY_U_EXIT => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        (
                            Self::c1_uv((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip),
                            Self::cu2_u(clip),
                        )
                    }
                    UV_ENTRY_UV_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tu2 = Self::tu2(u1, dv, clip);
                        if tu2 < tv1 {
                            return None;
                        }
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv2_naive = Self::tv2_naive(v1, du, clip);
                        if tv2_naive < tu1 {
                            return None;
                        }
                        (
                            Self::c1_uv((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip),
                            Self::cu2_uv(u1, (half_du, dv), (tu2, tv2_naive), clip),
                        )
                    }
                };
                let (x, y) = xy!((cu1, cv1), (cv1, cu1));
                Some(Self { x, y, error, dx, dy, end })
            }
        }
    };
}

clip_impl!(i8, wrapping_add_unsigned, wrapping_sub_unsigned);
clip_impl!(u8, wrapping_add, wrapping_sub);
clip_impl!(i16, wrapping_add_unsigned, wrapping_sub_unsigned);
clip_impl!(u16, wrapping_add, wrapping_sub);
clip_impl!(i32, wrapping_add_unsigned, wrapping_sub_unsigned);
clip_impl!(u32, wrapping_add, wrapping_sub);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
clip_impl!(isize, wrapping_add_unsigned, wrapping_sub_unsigned);
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
clip_impl!(usize, wrapping_add, wrapping_sub);
