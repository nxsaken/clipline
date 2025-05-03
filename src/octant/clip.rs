//! ### Octant clipping

use super::Octant;
use crate::clip::Clip;
use crate::macros::control_flow::return_if;
use crate::macros::derive::nums;
use crate::macros::symmetry::{fx, fy, xyf, yx, yxf};
use crate::math::{ops, Delta, Delta2, Num, Point};

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

macro_rules! impl_clip_octant {
    ($T:ty) => {
        #[expect(non_snake_case)]
        impl<const FX: bool, const FY: bool, const YX: bool> Octant<FX, FY, YX, $T> {
            #[inline(always)]
            #[must_use]
            const fn enters_u(u1: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> bool {
                yx!(fx!(u1 < wx1, wx2 < u1), fy!(u1 < wy1, wy2 < u1))
            }

            #[inline(always)]
            #[must_use]
            const fn enters_v(v1: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> bool {
                yx!(fy!(v1 < wy1, wy2 < v1), fx!(v1 < wx1, wx2 < v1))
            }

            #[inline(always)]
            #[must_use]
            const fn exits_u(u2: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> bool {
                yx!(fx!(wx2 < u2, u2 < wx1), fy!(wy2 < u2, u2 < wy1))
            }

            #[inline(always)]
            #[must_use]
            const fn exits_v(v2: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> bool {
                yx!(fy!(wy2 < v2, v2 < wy1), fx!(wx2 < v2, v2 < wx1))
            }

            #[inline(always)]
            #[must_use]
            const fn tu1(
                u1: $T,
                dv: <$T as Num>::U,
                &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>,
            ) -> <$T as Num>::U2 {
                let (a, b) = yx!(fx!((wx1, u1), (u1, wx2)), fy!((wy1, u1), (u1, wy2)));
                let Du1 = ops::<$T>::t_sub_t(a, b);
                ops::<$T>::u_mul_u(Du1, dv)
            }

            #[inline(always)]
            #[must_use]
            const fn tu2(
                u1: $T,
                dv: <$T as Num>::U,
                &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>,
            ) -> <$T as Num>::U2 {
                let (a, b) = yx!(fx!((wx2, u1), (u1, wx1)), fy!((wy2, u1), (u1, wy1)));
                let Du2 = ops::<$T>::t_sub_t(a, b);
                ops::<$T>::u_mul_u(Du2, dv)
            }

            #[inline(always)]
            #[must_use]
            const fn tv1(
                v1: $T,
                du: <$T as Num>::U,
                half_du: <$T as Num>::U,
                &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>,
            ) -> <$T as Num>::U2 {
                let (a, b) = yx!(fy!((wy1, v1), (v1, wy2)), fx!((wx1, v1), (v1, wx2)),);
                let Dv1 = ops::<$T>::t_sub_t(a, b);
                let Dv1_du = ops::<$T>::u_mul_u(Dv1, du);
                ops::<$T>::u2_sub_u(Dv1_du, half_du)
            }

            #[inline(always)]
            #[must_use]
            const fn tv2(
                v1: $T,
                du: <$T as Num>::U,
                half_du: <$T as Num>::U,
                &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>,
            ) -> <$T as Num>::U2 {
                let (a, b) = yx!(fy!((wy2, v1), (v1, wy1)), fx!((wx2, v1), (v1, wx1)),);
                let Dv2 = ops::<$T>::t_sub_t(a, b);
                let Dv2_du = ops::<$T>::u_mul_u(Dv2, du);
                ops::<$T>::u2_add_u(Dv2_du, half_du)
            }

            #[inline(always)]
            #[must_use]
            const fn cu1_v(
                u1: $T,
                tv1: <$T as Num>::U2,
                (half_du, dv): Delta<$T>,
                mut error: <$T as Num>::I2,
            ) -> ($T, <$T as Num>::I2) {
                // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
                let (mut q, r) = unsafe { ops::<$T>::u2_div_u(tv1, dv) };
                error = ops::<$T>::i2_sub_u(error, half_du);
                error = ops::<$T>::i2_sub_u(error, r);
                #[allow(unused_comparisons)]
                if 0 < r {
                    q = ops::<$T>::u_add_1(q);
                    error = ops::<$T>::i2_add_u(error, dv);
                };
                let cu1 = yxf!(ops::<$T>::t_add_u, ops::<$T>::t_sub_u; (u1, q));
                (cu1, error)
            }

            #[inline(always)]
            #[must_use]
            const fn cv1_u(
                v1: $T,
                tu1: <$T as Num>::U2,
                du: <$T as Num>::U,
                mut error: <$T as Num>::I2,
            ) -> ($T, <$T as Num>::I2) {
                // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
                let (mut q, r) = unsafe { ops::<$T>::u2_div_u(tu1, du) };
                error = ops::<$T>::i2_add_u(error, r);
                if {
                    let du = du as <$T as Num>::U2;
                    let r2 = ops::<$T>::u_mul_2(r);
                    du <= r2
                } {
                    q = ops::<$T>::u_add_1(q);
                    error = ops::<$T>::i2_sub_u(error, du);
                };
                let cv1 =  xyf!(ops::<$T>::t_add_u, ops::<$T>::t_sub_u; (v1, q));
                (cv1, error)
            }

            /// Clips at vertical entry.
            #[inline(always)]
            #[must_use]
            const fn c1_u(
                v1: $T,
                tu1: <$T as Num>::U2,
                du: <$T as Num>::U,
                error: <$T as Num>::I2,
                &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>,
            ) -> (Point<$T>, <$T as Num>::I2) {
                let cu1 = yx!(fx!(wx1, wx2), fy!(wy1, wy2));
                let (cv1, error) = Self::cv1_u(v1, tu1, du, error);
                ((cu1, cv1), error)
            }

            /// Clips at horizontal entry.
            #[inline(always)]
            #[must_use]
            const fn c1_v(
                u1: $T,
                tv1: <$T as Num>::U2,
                (half_du, dv): Delta<$T>,
                error: <$T as Num>::I2,
                &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>,
            ) -> (Point<$T>, <$T as Num>::I2) {
                let (cu1, error) = Self::cu1_v(u1, tv1, (half_du, dv), error);
                let cv1 = yx!(fy!(wy1, wy2), fx!(wx1, wx2));
                ((cu1, cv1), error)
            }

            #[inline(always)]
            #[must_use]
            const fn c1_uv(
                (u1, v1): Point<$T>,
                (tu1, tv1): Delta2<$T>,
                (du, dv): Delta<$T>,
                half_du: <$T as Num>::U,
                error: <$T as Num>::I2,
                clip: &Clip<$T>,
            ) -> (Point<$T>, <$T as Num>::I2) {
                if tv1 < tu1 {
                    Self::c1_u(v1, tu1, du, error, clip)
                } else {
                    Self::c1_v(u1, tv1, (half_du, dv), error, clip)
                }
            }

            #[inline(always)]
            #[must_use]
            const fn cu2_u(&Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> $T {
                // it is overflow-safe to add/sub 1 because of the exit condition
                let w = yx!(fx!(wx2, wx1), fy!(wy2, wy1));
                yxf!(ops::<$T>::t_add_1, ops::<$T>::t_sub_1; (w))
            }

            #[inline(always)]
            #[must_use]
            const fn cu2_v(
                u1: $T,
                tv2: <$T as Num>::U2,
                dv: <$T as Num>::U,
                r0: <$T as Num>::U,
            ) -> $T {
                // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
                let (mut q, r) = unsafe { ops::<$T>::u2_div_u(tv2, dv) };
                if r != 0 || r0 != 0 {
                    q = ops::<$T>::u_add_1(q);
                }
                yxf!(ops::<$T>::t_add_u, ops::<$T>::t_sub_u; (u1, q))
            }

            #[inline(always)]
            #[must_use]
            const fn cu2_uv(
                u1: $T,
                (tu2, tv2): Delta2<$T>,
                dv: <$T as Num>::U,
                r0: <$T as Num>::U,
                clip: &Clip<$T>,
            ) -> $T {
                if tu2 < tv2 {
                    Self::cu2_u(clip)
                } else {
                    Self::cu2_v(u1, tv2, dv, r0)
                }
            }

            #[expect(clippy::too_many_lines)]
            #[inline(always)]
            #[must_use]
            pub(super) const fn clip_inner(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                (dx, dy): Delta<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                let (u1, v1) = yx!((x1, y1), (y1, x1));
                let (u2, v2) = yx!((x2, y2), (y2, x2));
                let (du, dv) = yx!((dx, dy), (dy, dx));
                let (half_du, r0) = ops::<$T>::u_div_2(du);
                let half_du_r0 = ops::<$T>::q_add_r(half_du, r0); // FIXME: check this
                let error = ops::<$T>::u_sub_u(dv, half_du_r0);
                let (((cu1, cv1), error), end) = match (
                    Self::enters_u(u1, clip),
                    Self::enters_v(v1, clip),
                    Self::exits_u(u2, clip),
                    Self::exits_v(v2, clip),
                ) {
                    INSIDE_INSIDE => (((u1, v1), error), u2),
                    INSIDE_V_EXIT => {
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        (((u1, v1), error), Self::cu2_v(u1, tv2, dv, r0))
                    }
                    INSIDE_U_EXIT => (((u1, v1), error), Self::cu2_u(clip)),
                    INSIDE_UV_EXIT => {
                        let tu2 = Self::tu2(u1, dv, clip);
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        (((u1, v1), error), Self::cu2_uv(u1, (tu2, tv2), dv, r0, clip))
                    }
                    V_ENTRY_INSIDE => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        (Self::c1_v(u1, tv1, (half_du, dv), error, clip), u2)
                    }
                    V_ENTRY_V_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        (
                            Self::c1_v(u1, tv1, (half_du, dv), error, clip),
                            Self::cu2_v(u1, tv2, dv, r0),
                        )
                    }
                    V_ENTRY_U_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tu2 = Self::tu2(u1, dv, clip);
                        return_if!(tu2 < tv1);
                        (Self::c1_v(u1, tv1, (half_du, dv), error, clip), Self::cu2_u(clip))
                    }
                    V_ENTRY_UV_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tu2 = Self::tu2(u1, dv, clip);
                        return_if!(tu2 < tv1);
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        (
                            Self::c1_v(u1, tv1, (half_du, dv), error, clip),
                            Self::cu2_uv(u1, (tu2, tv2), dv, r0, clip),
                        )
                    }
                    U_ENTRY_INSIDE => {
                        (Self::c1_u(v1, Self::tu1(u1, dv, clip), du, error, clip), u2)
                    }
                    U_ENTRY_V_EXIT => {
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        (
                            Self::c1_u(v1, Self::tu1(u1, dv, clip), du, error, clip),
                            Self::cu2_v(u1, tv2, dv, r0),
                        )
                    }
                    U_ENTRY_U_EXIT => (
                        Self::c1_u(v1, Self::tu1(u1, dv, clip), du, error, clip),
                        Self::cu2_u(clip),
                    ),
                    U_ENTRY_UV_EXIT => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        return_if!(tv2 < tu1);
                        let tu2 = Self::tu2(u1, dv, clip);
                        (
                            Self::c1_u(v1, tu1, du, error, clip),
                            Self::cu2_uv(u1, (tu2, tv2), dv, r0, clip),
                        )
                    }
                    UV_ENTRY_INSIDE => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        (Self::c1_uv((u1, v1), (tu1, tv1), (du, dv), half_du, error, clip), u2)
                    }
                    UV_ENTRY_V_EXIT => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        (
                            Self::c1_uv((u1, v1), (tu1, tv1), (du, dv), half_du, error, clip),
                            Self::cu2_v(u1, tv2, dv, r0),
                        )
                    }
                    UV_ENTRY_U_EXIT => {
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        (
                            Self::c1_uv((u1, v1), (tu1, tv1), (du, dv), half_du, error, clip),
                            Self::cu2_u(clip),
                        )
                    }
                    UV_ENTRY_UV_EXIT => {
                        let tv1 = Self::tv1(v1, du, half_du, clip);
                        let tu2 = Self::tu2(u1, dv, clip);
                        return_if!(tu2 < tv1);
                        let tu1 = Self::tu1(u1, dv, clip);
                        let tv2 = Self::tv2(v1, du, half_du, clip);
                        return_if!(tv2 < tu1);
                        (
                            Self::c1_uv((u1, v1), (tu1, tv1), (du, dv), half_du, error, clip),
                            Self::cu2_uv(u1, (tu2, tv2), dv, r0, clip),
                        )
                    }
                };
                let (x, y) = yx!((cu1, cv1), (cv1, cu1));
                Some(Self { x, y, error, dx, dy, end })
            }
        }
    };
}

nums!(impl_clip_octant, cfg_octant_64);
