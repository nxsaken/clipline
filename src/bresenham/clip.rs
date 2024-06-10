//! ### Bresenham clipping
//!
//! This module provides [clipping](Clip) utilities for
//! [slanted](Octant) directed line segments.

use super::Octant;
use crate::clip::Clip;
use crate::math::{Delta, Math, Num, Point};
use crate::symmetry::{fx, fy, xy};

pub type Clipped<T> = (Point<T>, <T as Num>::I2, T);

impl<const FX: bool, const FY: bool, const SWAP: bool> Octant<i8, FX, FY, SWAP> {
    const fn trivial_reject(
        (x1, y1): Point<i8>,
        (x2, y2): Point<i8>,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> bool {
        fx!(x2 < wx1 || wx2 <= x1, x1 < wx1 || wx2 <= x2)
            || fy!(y2 < wy1 || wy2 <= y1, y1 < wy1 || wy2 <= y2)
    }

    /// Checks if the line segment enters the clipping region through a vertical side.
    const fn enters_u(u1: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fx!(u1 < wx1, wx2 < u1), fy!(u1 < wy1, wy2 < u1))
    }

    /// Checks if the line segment enters the clipping region through a horizontal side.
    const fn enters_v(v1: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fy!(v1 < wy1, wy2 < v1), fx!(v1 < wx1, wx2 < v1))
    }

    /// Checks if the line segment exits the clipping region through a vertical side.
    const fn exits_u(u2: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fx!(wx2 < u2, u2 < wx1), fy!(wy2 < u2, u2 < wy1))
    }

    /// Checks if the line segment exits the clipping region through a horizontal side.
    const fn exits_v(v2: i8, Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> bool {
        xy!(fy!(wy2 < v2, v2 < wy1), fx!(wx2 < v2, v2 < wx1))
    }

    #[allow(non_snake_case)]
    const fn tu1(
        u1: i8,
        dv: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Du1 = xy!(
            fx!(Math::delta(wx1, u1), Math::delta(u1, wx2)),
            fy!(Math::delta(wy1, u1), Math::delta(u1, wy2)),
        );
        Math::wide_mul(Du1, dv)
    }

    #[allow(non_snake_case)]
    const fn tu2(
        u1: i8,
        dv: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Du2 = xy!(
            fx!(Math::delta(wx2, u1), Math::delta(u1, wx1)),
            fy!(Math::delta(wy2, u1), Math::delta(u1, wy1)),
        );
        Math::wide_mul(Du2, dv)
    }

    #[allow(non_snake_case)]
    const fn tv1(
        v1: i8,
        du: <i8 as Num>::U,
        half_du: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Dv1 = xy!(
            fy!(Math::delta(wy1, v1), Math::delta(v1, wy2)),
            fx!(Math::delta(wx1, v1), Math::delta(v1, wx2)),
        );
        Math::wide_mul(Dv1, du).wrapping_sub(half_du as _)
    }

    #[allow(non_snake_case)]
    const fn tv2_naive(
        v1: i8,
        du: <i8 as Num>::U,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> <i8 as Num>::U2 {
        let Dv2 = xy!(
            fy!(Math::delta(wy2, v1), Math::delta(v1, wy1)),
            fx!(Math::delta(wx2, v1), Math::delta(v1, wx1)),
        );
        Math::wide_mul(Dv2, du)
    }

    #[allow(non_snake_case)]
    const fn tv2(naive: <i8 as Num>::U2, half_du: <i8 as Num>::U) -> <i8 as Num>::U2 {
        naive.wrapping_add(half_du as _)
    }

    const fn cu1_naive(Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> i8 {
        xy!(fx!(wx1, wx2), fy!(wy1, wy2))
    }

    const fn cv1_naive(Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> i8 {
        xy!(fy!(wy1, wy2), fx!(wx1, wx2))
    }

    const fn cu1(
        u1: i8,
        half_du: <i8 as Num>::U,
        dv: <i8 as Num>::U,
        tv1: <i8 as Num>::U2,
        mut error: <i8 as Num>::I2,
    ) -> (i8, <i8 as Num>::I2) {
        let (mut q, r) = Math::div_rem(tv1, dv);
        error = error.wrapping_sub(half_du as _).wrapping_sub_unsigned(r as _);
        if 0 < r {
            q = xy!(
                fx!(q.wrapping_add(1), q.wrapping_sub(1)),
                fy!(q.wrapping_add(1), q.wrapping_sub(1))
            );
            error = error.wrapping_add_unsigned(dv as _);
        };
        let cu1 = xy!(
            fx!(u1.wrapping_add_unsigned(q), u1.wrapping_sub_unsigned(q)),
            fy!(u1.wrapping_add_unsigned(q), u1.wrapping_sub_unsigned(q)),
        );
        (cu1, error)
    }

    const fn cv1(
        v1: i8,
        du: <i8 as Num>::U,
        tu1: <i8 as Num>::U2,
        mut error: <i8 as Num>::I2,
    ) -> (i8, <i8 as Num>::I2) {
        let (mut q, r) = Math::div_rem(tu1, du);
        error = error.wrapping_add_unsigned(r as _);
        let du = du as <i8 as Num>::U2;
        let r2 = Math::double(r);
        if du <= r2 {
            q = xy!(
                fx!(q.wrapping_add(1), q.wrapping_sub(1)),
                fy!(q.wrapping_add(1), q.wrapping_sub(1))
            );
            error = error.wrapping_sub_unsigned(du as _);
        };
        let cv1 = xy!(
            fy!(v1.wrapping_add_unsigned(q), v1.wrapping_sub_unsigned(q)),
            fx!(v1.wrapping_add_unsigned(q), v1.wrapping_sub_unsigned(q)),
        );
        (cv1, error)
    }

    const fn end_naive(Clip { wx1, wy1, wx2, wy2 }: Clip<i8>) -> i8 {
        xy!(fx!(wx2, wx1), fy!(wy2, wy1))
    }

    const fn end(u1: i8, dv: <i8 as Num>::U, tv2: <i8 as Num>::U2) -> i8 {
        let (mut q, r) = Math::div_rem(tv2, dv);
        if 0 == r {
            q = xy!(
                fx!(q.wrapping_sub(1), q.wrapping_add(1)),
                fy!(q.wrapping_sub(1), q.wrapping_add(1))
            );
        }
        xy!(
            fx!(u1.wrapping_add_unsigned(q), u1.wrapping_sub_unsigned(q)),
            fy!(u1.wrapping_add_unsigned(q), u1.wrapping_sub_unsigned(q)),
        )
    }

    #[allow(clippy::cognitive_complexity)]
    pub(super) const fn clip_inner(
        (x1, y1): Point<i8>,
        (x2, y2): Point<i8>,
        (dx, dy): Delta<i8>,
        clip: Clip<i8>,
    ) -> Option<Clipped<i8>> {
        const O: bool = false;
        const I: bool = true;
        if Self::trivial_reject((x1, y1), (x2, y2), clip) {
            return None;
        }
        let (u1, v1) = xy!((x1, y1), (y1, x1));
        let (u2, v2) = xy!((x2, y2), (y2, x2));
        let (du, dv) = xy!((dx, dy), (dy, dx));
        let half_du = Math::half(du);
        let error = Math::error(dv, Math::half(du));
        let (c1, error, end) = match (
            Self::enters_u(u1, clip),
            Self::enters_v(v1, clip),
            Self::exits_u(u2, clip),
            Self::exits_v(v2, clip),
        ) {
            (O, O, O, O) => ((x1, y1), error, xy!(x2, y2)),
            (O, O, O, I) => unimplemented!(),
            (O, O, I, O) => unimplemented!(),
            (O, O, I, I) => unimplemented!(),
            (O, I, O, O) => unimplemented!(),
            (O, I, O, I) => unimplemented!(),
            (O, I, I, O) => unimplemented!(),
            (O, I, I, I) => unimplemented!(),
            (I, O, O, O) => unimplemented!(),
            (I, O, O, I) => unimplemented!(),
            (I, O, I, O) => unimplemented!(),
            (I, O, I, I) => unimplemented!(),
            (I, I, O, O) => unimplemented!(),
            (I, I, O, I) => unimplemented!(),
            (I, I, I, O) => unimplemented!(),
            (I, I, I, I) => {
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
                let (c1, error) = if tv1 < tu1 {
                    // enter through vertical side
                    let (cv1, error) = Self::cv1(v1, du, tu1, error);
                    ((Self::cu1_naive(clip), cv1), error)
                } else {
                    // enter through horizontal side
                    let (cu1, error) = Self::cu1(u1, half_du, dv, tv1, error);
                    ((cu1, Self::cv1_naive(clip)), error)
                };
                let tv2 = Self::tv2(tv2_naive, half_du);
                let end = if tu2 < tv2 {
                    // exit through vertical side
                    Self::end_naive(clip)
                } else {
                    // exit through horizontal side
                    Self::end(u1, dv, tv2)
                };
                (c1, error, end)
            }
        };
        Some((c1, error, end))
    }
}
