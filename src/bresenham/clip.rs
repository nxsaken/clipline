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

impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<i8, FX, FY, SWAP> {
    #[inline(always)]
    #[must_use]
    const fn trivial_reject(
        (x1, y1): Point<i8>,
        (x2, y2): Point<i8>,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> bool {
        fx!(x2 < wx1 || wx2 <= x1, x1 < wx1 || wx2 <= x2)
            || fy!(y2 < wy1 || wy2 <= y1, y1 < wy1 || wy2 <= y2)
    }

    /// Checks if the line segment enters the clipping region through a vertical side.
    #[inline(always)]
    #[must_use]
    const fn enters_u(u1: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fx!(u1 < wx1, wx2 < u1), fy!(u1 < wy1, wy2 < u1))
    }

    /// Checks if the line segment enters the clipping region through a horizontal side.
    #[inline(always)]
    #[must_use]
    const fn enters_v(v1: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fy!(v1 < wy1, wy2 < v1), fx!(v1 < wx1, wx2 < v1))
    }

    /// Checks if the line segment exits the clipping region through a vertical side.
    #[inline(always)]
    #[must_use]
    const fn exits_u(u2: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fx!(wx2 < u2, u2 < wx1), fy!(wy2 < u2, u2 < wy1))
    }

    /// Checks if the line segment exits the clipping region through a horizontal side.
    #[inline(always)]
    #[must_use]
    const fn exits_v(v2: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fy!(wy2 < v2, v2 < wy1), fx!(wx2 < v2, v2 < wx1))
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    #[must_use]
    const fn tu1(
        u1: i8,
        dv: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Du1 = xy!(
            fx!(Math::<i8>::delta(wx1, u1), Math::<i8>::delta(u1, wx2)),
            fy!(Math::<i8>::delta(wy1, u1), Math::<i8>::delta(u1, wy2)),
        );
        Math::<i8>::wide_mul(Du1, dv)
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    #[must_use]
    const fn tu2(
        u1: i8,
        dv: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Du2 = xy!(
            fx!(Math::<i8>::delta(wx2, u1), Math::<i8>::delta(u1, wx1)),
            fy!(Math::<i8>::delta(wy2, u1), Math::<i8>::delta(u1, wy1)),
        );
        Math::<i8>::wide_mul(Du2, dv)
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    #[must_use]
    const fn tv1(
        v1: i8,
        du: <i8 as Num>::U,
        half_du: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Dv1 = xy!(
            fy!(Math::<i8>::delta(wy1, v1), Math::<i8>::delta(v1, wy2)),
            fx!(Math::<i8>::delta(wx1, v1), Math::<i8>::delta(v1, wx2)),
        );
        Math::<i8>::wide_mul(Dv1, du).wrapping_sub(half_du as _)
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    #[must_use]
    const fn tv2_naive(
        v1: i8,
        du: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Dv2 = xy!(
            fy!(Math::<i8>::delta(wy2, v1), Math::<i8>::delta(v1, wy1)),
            fx!(Math::<i8>::delta(wx2, v1), Math::<i8>::delta(v1, wx1)),
        );
        Math::<i8>::wide_mul(Dv2, du)
    }

    #[inline(always)]
    #[must_use]
    const fn tv2(naive: <i8 as Num>::U2, half_du: <i8 as Num>::U) -> <i8 as Num>::U2 {
        naive.wrapping_add(half_du as _)
    }

    #[inline(always)]
    #[must_use]
    const fn cu1(
        u1: i8,
        (half_du, dv): Delta<i8>,
        tv1: <i8 as Num>::U2,
        mut error: <i8 as Num>::I2,
    ) -> (i8, <i8 as Num>::I2) {
        // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
        let (mut q, r) = unsafe { Math::<i8>::div_rem(tv1, dv) };
        error = error.wrapping_sub(half_du as _).wrapping_sub_unsigned(r as _);
        if 0 < r {
            q = xy!(
                fx!(q.wrapping_add(1), q.wrapping_add(1)),
                fy!(q.wrapping_add(1), q.wrapping_add(1))
            );
            error = error.wrapping_add_unsigned(dv as _);
        };
        let cu1 = xy!(
            fx!(u1.wrapping_add_unsigned(q), u1.wrapping_sub_unsigned(q)),
            fy!(u1.wrapping_add_unsigned(q), u1.wrapping_sub_unsigned(q)),
        );
        (cu1, error)
    }

    #[inline(always)]
    #[must_use]
    const fn cv1(
        v1: i8,
        du: <i8 as Num>::U,
        tu1: <i8 as Num>::U2,
        mut error: <i8 as Num>::I2,
    ) -> (i8, <i8 as Num>::I2) {
        // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
        let (mut q, r) = unsafe { Math::<i8>::div_rem(tu1, du) };
        error = error.wrapping_add_unsigned(r as _);
        let du = du as <i8 as Num>::U2;
        let r2 = Math::<i8>::double(r);
        if du <= r2 {
            q = q.wrapping_add(1);
            error = error.wrapping_sub_unsigned(du as _);
        };
        let cv1 = xy!(
            fy!(v1.wrapping_add_unsigned(q), v1.wrapping_sub_unsigned(q)),
            fx!(v1.wrapping_add_unsigned(q), v1.wrapping_sub_unsigned(q)),
        );
        (cv1, error)
    }

    #[inline(always)]
    #[must_use]
    const fn cu2(Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> i8 {
        // it is overflow-safe to add/sub 1 because of the exit condition
        xy!(
            fx!(wx2.wrapping_add(1), wx1.wrapping_sub(1)),
            fy!(wy2.wrapping_add(1), wy1.wrapping_sub(1))
        )
    }

    #[inline(always)]
    #[must_use]
    const fn cv2(u1: i8, dv: <i8 as Num>::U, tv2: <i8 as Num>::U2) -> i8 {
        // SAFETY: the line segment is slanted and non-empty, thus dv != 0.
        let (mut q, r) = unsafe { Math::<i8>::div_rem(tv2, dv) };
        if 0 == r {
            q = q.wrapping_sub(1);
        }
        // it is overflow-safe to add/sub 1 because of the exit condition
        xy!(
            fx!(
                u1.wrapping_add_unsigned(q).wrapping_add(1),
                u1.wrapping_sub_unsigned(q).wrapping_sub(1)
            ),
            fy!(
                u1.wrapping_add_unsigned(q).wrapping_add(1),
                u1.wrapping_sub_unsigned(q).wrapping_sub(1)
            ),
        )
    }

    /// Clips at vertical entry.
    #[inline(always)]
    #[must_use]
    const fn c1_u(
        (u1, v1): Point<i8>,
        (du, dv): Delta<i8>,
        error: <i8 as Num>::I2,
        clip: Clip<i8>,
    ) -> (Point<i8>, <i8 as Num>::I2) {
        let tu1 = Self::tu1(u1, dv, clip);
        let Clip { wx1, wy1, wx2, wy2 } = clip;
        let cu1 = xy!(fx!(wx1, wx2), fy!(wy1, wy2));
        let (cv1, error) = Self::cv1(v1, du, tu1, error);
        ((cu1, cv1), error)
    }

    /// Clips at horizontal entry.
    #[inline(always)]
    #[must_use]
    const fn c1_v(
        u1: i8,
        (half_du, dv): Delta<i8>,
        tv1: <i8 as Num>::U2,
        error: <i8 as Num>::I2,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> (Point<i8>, <i8 as Num>::I2) {
        let (cu1, error) = Self::cu1(u1, (half_du, dv), tv1, error);
        let cv1 = xy!(fy!(wy1, wy2), fx!(wx1, wx2));
        ((cu1, cv1), error)
    }

    #[inline(always)]
    #[must_use]
    const fn c1(
        (u1, v1): Point<i8>,
        (du, dv): Delta<i8>,
        half_du: <i8 as Num>::U,
        (tu1, tv1): Delta2<i8>,
        error: <i8 as Num>::I2,
        clip: Clip<i8>,
    ) -> (Point<i8>, <i8 as Num>::I2) {
        if tv1 < tu1 {
            // vertical entry
            Self::c1_u((u1, v1), (du, dv), error, clip)
        } else {
            // horizontal entry
            Self::c1_v(u1, (half_du, dv), tv1, error, clip)
        }
    }

    /// Clips at vertical exit.
    #[inline(always)]
    #[must_use]
    const fn c2_u(clip: Clip<i8>) -> i8 {
        Self::cu2(clip)
    }

    /// Clips at horizontal exit.
    #[inline(always)]
    #[must_use]
    const fn c2_v(
        (u1, v1): Point<i8>,
        (du, dv): Delta<i8>,
        half_du: <i8 as Num>::U,
        clip: Clip<i8>,
    ) -> i8 {
        let tv2_naive = Self::tv2_naive(v1, du, clip);
        let tv2 = Self::tv2(tv2_naive, half_du);
        Self::cv2(u1, dv, tv2)
    }

    #[inline(always)]
    #[must_use]
    const fn c2(
        u1: i8,
        (half_du, dv): Delta<i8>,
        (tu2, tv2_naive): Delta2<i8>,
        clip: Clip<i8>,
    ) -> i8 {
        let tv2 = Self::tv2(tv2_naive, half_du);
        if tu2 < tv2 {
            Self::cu2(clip)
        } else {
            Self::cv2(u1, dv, tv2)
        }
    }

    #[allow(clippy::too_many_lines)]
    #[inline(always)]
    #[must_use]
    pub(super) const fn clip_inner(
        (x1, y1): Point<i8>,
        (x2, y2): Point<i8>,
        (dx, dy): Delta<i8>,
        clip: Clip<i8>,
    ) -> Option<Self> {
        if Self::trivial_reject((x1, y1), (x2, y2), clip) {
            return None;
        }
        let (u1, v1) = xy!((x1, y1), (y1, x1));
        let (u2, v2) = xy!((x2, y2), (y2, x2));
        let (du, dv) = xy!((dx, dy), (dy, dx));
        let half_du = Math::<i8>::half(du);
        let error = Math::<i8>::error(dv, Math::<i8>::half(du));
        let (((cu1, cv1), error), end) = match (
            Self::enters_u(u1, clip),
            Self::enters_v(v1, clip),
            Self::exits_u(u2, clip),
            Self::exits_v(v2, clip),
        ) {
            INSIDE_INSIDE => (((u1, v1), error), u2),
            INSIDE_V_EXIT => (((u1, v1), error), Self::c2_v((u1, v1), (du, dv), half_du, clip)),
            INSIDE_U_EXIT => (((u1, v1), error), Self::c2_u(clip)),
            INSIDE_UV_EXIT => {
                let tu2 = Self::tu2(u1, dv, clip);
                let tv2_naive = Self::tv2_naive(v1, du, clip);
                (((u1, v1), error), Self::c2(u1, (half_du, dv), (tu2, tv2_naive), clip))
            }
            V_ENTRY_INSIDE => {
                let tv1 = Self::tv1(v1, du, half_du, clip);
                (Self::c1_v(u1, (half_du, dv), tv1, error, clip), u2)
            }
            V_ENTRY_V_EXIT => {
                let tv1 = Self::tv1(v1, du, half_du, clip);
                (
                    Self::c1_v(u1, (half_du, dv), tv1, error, clip),
                    Self::c2_v((u1, v1), (du, dv), half_du, clip),
                )
            }
            V_ENTRY_U_EXIT => {
                let tv1 = Self::tv1(v1, du, half_du, clip);
                let tu2 = Self::tu2(u1, dv, clip);
                if tu2 < tv1 {
                    return None;
                }
                (Self::c1_v(u1, (half_du, dv), tv1, error, clip), Self::c2_u(clip))
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
                    Self::c2(u1, (half_du, dv), (tu2, tv2_naive), clip),
                )
            }
            U_ENTRY_INSIDE => (Self::c1_u((u1, v1), (du, dv), error, clip), u2),
            U_ENTRY_V_EXIT => (
                Self::c1_u((u1, v1), (du, dv), error, clip),
                Self::c2_v((u1, v1), (du, dv), half_du, clip),
            ),
            U_ENTRY_U_EXIT => (Self::c1_u((u1, v1), (du, dv), error, clip), Self::c2_u(clip)),
            U_ENTRY_UV_EXIT => {
                let tu1 = Self::tu1(u1, dv, clip);
                let tv2_naive = Self::tv2_naive(v1, du, clip);
                if tv2_naive < tu1 {
                    return None;
                }
                let tu2 = Self::tu2(u1, dv, clip);
                (
                    Self::c1_u((u1, v1), (du, dv), error, clip),
                    Self::c2(u1, (half_du, dv), (tu2, tv2_naive), clip),
                )
            }
            UV_ENTRY_INSIDE => {
                let tu1 = Self::tu1(u1, dv, clip);
                let tv1 = Self::tv1(v1, du, half_du, clip);
                (Self::c1((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip), u2)
            }
            UV_ENTRY_V_EXIT => {
                let tu1 = Self::tu1(u1, dv, clip);
                let tv1 = Self::tv1(v1, du, half_du, clip);
                (
                    Self::c1((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip),
                    Self::c2_v((u1, v1), (du, dv), half_du, clip),
                )
            }
            UV_ENTRY_U_EXIT => {
                let tu1 = Self::tu1(u1, dv, clip);
                let tv1 = Self::tv1(v1, du, half_du, clip);
                (Self::c1((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip), Self::c2_u(clip))
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
                    Self::c1((u1, v1), (du, dv), half_du, (tu1, tv1), error, clip),
                    Self::c2(u1, (half_du, dv), (tu2, tv2_naive), clip),
                )
            }
        };
        let (x, y) = xy!((cu1, cv1), (cv1, cu1));
        Some(Self { x, y, error, dx, dy, end })
    }
}
