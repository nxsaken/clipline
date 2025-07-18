use crate::clip::{Clip, Viewport};
use crate::line_b::{LineB, LineBu, LineBx, LineBy};
use crate::math::{Coord, ops};
use crate::util::try_opt;

macro_rules! clip_line_b {
    ($U:ty | $I:ty) => {
        clip_line_b!(@impl Clip<$U>, $U, <$U as Coord>::U2, <$U as Coord>::I2);
        clip_line_b!(@impl Clip<$I>, $U, <$U as Coord>::U2, <$U as Coord>::I2);
        clip_line_b!(@impl Viewport<$U>, $U, <$U as Coord>::U2, <$U as Coord>::I2);
        clip_line_b!(@impl Viewport<$I>, $U, <$U as Coord>::U2, <$U as Coord>::I2);

        clip_line_b!(@pub impl Clip<$U>);
        clip_line_b!(@pub impl Clip<$I>);
        clip_line_b!(@pub impl Viewport<$U>);
        clip_line_b!(@pub impl Viewport<$I>);

        clip_line_b!(@impl Clip<$I, proj $U>);
        clip_line_b!(@impl Viewport<$U, proj $U>);
        clip_line_b!(@impl Viewport<$I, proj $U>);

        clip_line_b!(@pub impl Clip<$I, proj $U>);
        clip_line_b!(@pub impl Viewport<$U, proj $U>);
        clip_line_b!(@pub impl Viewport<$I, proj $U>);
    };
    (@impl $Self:ident<$UI:ty>, $U:ty, $U2:ty, $I2:ty) => {
        impl $Self<$UI> {
            const fn reject_bbox_closed<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> bool {
                !FX && (x1 < self.x_min() || self.x_max < x0)
                    || FX && (x0 < self.x_min() || self.x_max < x1)
                    || !FY && (y1 < self.y_min() || self.y_max < y0)
                    || FY && (y0 < self.y_min() || self.y_max < y1)
            }

            pub(super) const fn outcode<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                u0: $UI,
                v0: $UI,
                u1: $UI,
                v1: $UI,
            ) -> [bool; 4] {
                let (u_min, v_min, u_max, v_max) = self.uv_min_max::<YX>();
                let maybe_iu = if FU { v_max < u0 } else { u0 < u_min };
                let maybe_iv = if FV { v_max < v0 } else { v0 < v_min };
                let maybe_ou = if FU { u1 < u_min } else { u_max < u1 };
                let maybe_ov = if FV { v1 < v_min } else { v_max < v1 };
                [maybe_iu, maybe_iv, maybe_ou, maybe_ov]
            }

            pub(super) const fn du<const YX: bool, const FU: bool, const OI: bool>(
                &self,
                u0: $UI,
            ) -> $U {
                let (lhs, rhs) = match (FU, OI) {
                    (false, false) => (self.u_min::<YX>(), u0),
                    (false, true) => (self.u_max::<YX>(), u0),
                    (true, false) => (u0, self.u_max::<YX>()),
                    (true, true) => (u0, self.u_min::<YX>()),
                };
                ops::<$UI>::abs_diff(lhs, rhs)
            }

            pub(super) const fn dv<const YX: bool, const FV: bool, const OI: bool>(
                &self,
                v0: $UI,
            ) -> $U {
                let (lhs, rhs) = match (FV, OI) {
                    (false, false) => (self.v_min::<YX>(), v0),
                    (false, true) => (self.v_max::<YX>(), v0),
                    (true, false) => (v0, self.v_max::<YX>()),
                    (true, true) => (v0, self.v_min::<YX>()),
                };
                ops::<$UI>::abs_diff(lhs, rhs)
            }

            const fn tu0<const YX: bool, const FU: bool>(
                &self,
                u0: $UI,
                dv: $U,
            ) -> $U2 {
                let du0 = self.du::<YX, FU, false>(u0);
                du0 as $U2 * dv as $U2
            }

            const fn tu1<const YX: bool, const FU: bool>(
                &self,
                u0: $UI,
                dv: $U,
            ) -> $U2 {
                let du1 = self.du::<YX, FU, true>(u0);
                du1 as $U2 * dv as $U2
            }

            const fn tv0<const YX: bool, const FV: bool>(
                &self,
                v0: $UI,
                du: $U,
                du_half: $U,
            ) -> $U2 {
                let dv0 = self.dv::<YX, FV, false>(v0);
                let tv0_raw = dv0 as $U2 * du as $U2;
                tv0_raw - du_half as $U2
            }

            const fn tv1<const YX: bool, const FV: bool>(
                &self,
                v0: $UI,
                du: $U,
                du_half: $U,
            ) -> $U2 {
                let dv1 = self.dv::<YX, FV, true>(v0);
                let tv1_raw = dv1 as $U2 * du as $U2;
                tv1_raw + du_half as $U2
            }

            const fn cuv0_iu_bu<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                v0: $UI,
                du: $U,
                tu0: $U2,
                mut err: $I2,
            ) -> ($UI, $UI, $I2) {
                let (mut dvc, dvc_rem) = {
                    // SAFETY: this is never called with du == 0.
                    unsafe { core::hint::assert_unchecked(du != 0) };
                    let (dvc, dvc_rem) = (tu0 / du as $U2, tu0 % du as $U2);
                    debug_assert!(dvc <= <$U>::MAX as $U2);
                    (dvc as $U, dvc_rem as $U)
                };
                err += dvc_rem as $I2;
                if du as $U2 <= dvc_rem as $U2 * 2 {
                    dvc += 1;
                    err -= du as $I2;
                }
                let cu0 = if FU { self.u_max::<YX>() } else { self.u_min::<YX>() };
                let cv0 = if FV { ops::<$UI>::sub_u(v0, dvc) } else { ops::<$UI>::add_u(v0, dvc) };
                (cu0, cv0, err)
            }

            const fn cuv0_iv_bu<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                u0: $UI,
                dv: $U,
                tv0: $U2,
                mut err: $I2,
                du_half: $U,
            ) -> ($UI, $UI, $I2) {
                let (mut duc, duc_rem) = {
                    // SAFETY: this is never called with dv == 0.
                    unsafe { core::hint::assert_unchecked(dv != 0) };
                    let (duc, duc_rem) = (tv0 / dv as $U2, tv0 % dv as $U2);
                    debug_assert!(duc <= <$U>::MAX as $U2);
                    (duc as $U, duc_rem as $U)
                };
                err -= du_half as $I2;
                err -= duc_rem as $I2;
                if 0 < duc_rem {
                    duc += 1;
                    err += dv as $I2;
                }
                let cu0 = if FU { ops::<$UI>::sub_u(u0, duc) } else { ops::<$UI>::add_u(u0, duc) };
                let cv0 = if FV { self.v_max::<YX>() } else { self.v_min::<YX>() };
                (cu0, cv0, err)
            }

            #[expect(clippy::too_many_arguments)]
            const fn cuv0_iuv_bu<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                u0: $UI,
                v0: $UI,
                du: $U,
                dv: $U,
                tu0: $U2,
                tv0: $U2,
                err: $I2,
                du_half: $U,
            ) -> ($UI, $UI, $I2) {
                if tv0 <= tu0 {
                    self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err)
                } else {
                    self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half)
                }
            }

            const fn cu1_ou_bu<const YX: bool, const FU: bool>(
                &self,
            ) -> $UI {
                let ou = if FU { self.u_min::<YX>() } else { self.u_max::<YX>() };
                let su = if FU { -1 } else { 1 };
                ops::<$UI>::add_i(ou, su)
            }

            const fn cu1_ov_bu<const FU: bool>(
                u0: $UI,
                dv: $U,
                tv1: $U2,
                du_odd: bool,
            ) -> $UI {
                let (mut duc, duc_rem) = {
                    // SAFETY: this is never called with dv == 0.
                    unsafe { core::hint::assert_unchecked(dv != 0) };
                    let (duc, duc_rem) = (tv1 / dv as $U2, tv1 % dv as $U2);
                    debug_assert!(duc <= <$U>::MAX as $U2);
                    (duc as $U, duc_rem as $U)
                };
                duc += (duc_rem != 0 || du_odd) as $U;
                if FU { ops::<$UI>::sub_u(u0, duc) } else { ops::<$UI>::add_u(u0, duc) }
            }

            const fn cu1_ouv_bu<const YX: bool, const FU: bool>(
                &self,
                u0: $UI,
                dv: $U,
                tu1: $U2,
                tv1: $U2,
                du_odd: bool,
            ) -> $UI {
                if tu1 <= tv1 {
                    self.cu1_ou_bu::<YX, FU>()
                } else {
                    Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd)
                }
            }

            const fn raw_line_bu_fufv<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                u0: $UI,
                v0: $UI,
                u1: $UI,
                v1: $UI,
                du: $U,
                dv: $U,
            ) -> Option<($UI, $UI, $I2, $UI, i8, i8)> {
                if !FU && u1 == self.u_min::<YX>() || FU && u1 == self.u_max::<YX>() {
                    // ends on the entry along the major axis
                    return None;
                }
                let (du_half, du_odd) = (du / 2, du % 2 != 0);
                let err = dv as $I2 - (du_half + du_odd as $U) as $I2;
                let (u0, v0, err, u1) = match self.outcode::<YX, FU, FV>(u0, v0, u1, v1) {
                    [false, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    |0 1|
                        // ---+---+---
                        //    |   |
                        (u0, v0, err, u1)
                    },
                    [false, false, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    | 0 |
                        // ---+---+---
                        //    |   |
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        let cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
                        (u0, v0, err, cu1)
                    },
                    [false, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 0 # 1
                        // ---+---+---
                        //    |   |
                        let cu1 = self.cu1_ou_bu::<YX, FU>();
                        (u0, v0, err, cu1)
                    },
                    [false, false, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    | 0 #
                        // ---+---+---
                        //    |   |
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        let cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                        (u0, v0, err, cu1)
                    },
                    [false, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 1 |
                        // ---+-@-+---
                        //    | 0 |
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        (cu0, cv0, err, u1)
                    },
                    [false, true, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    |   |
                        // ---+-@-+---
                        //    | 0 |
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        let cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
                        (cu0, cv0, err, cu1)
                    },
                    [false, true, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    |   # 1
                        // ---+-@-+-/-
                        //    | 0 |
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        if tu1 < tv0 {
                            return None;
                        }
                        let (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        let cu1 = self.cu1_ou_bu::<YX, FU>();
                        (cu0, cv0, err, cu1)
                    },
                    [false, true, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    |   #
                        // ---+-@-+-/-
                        //    | 0 |
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        if tu1 < tv0 {
                            return None;
                        }
                        let (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        let cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                        (cu0, cv0, err, cu1)
                    },
                    [true, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @ 1 |
                        // ---+---+---
                        //    |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let (cu0, cv0, err) = self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err);
                        (cu0, cv0, err, u1)
                    },
                    [true, false, false, true] => {
                        //    | 1 |
                        // -/-+-#-+---
                        //  0 @   |
                        // ---+---+---
                        //    |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        if tv1 < tu0 {
                            return None;
                        }
                        let (cu0, cv0, err) = self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err);
                        let cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
                        (cu0, cv0, err, cu1)
                    },
                    [true, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @   # 1
                        // ---+---+---
                        //    |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let (cu0, cv0, err) = self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err);
                        let cu1 = self.cu1_ou_bu::<YX, FU>();
                        (cu0, cv0, err, cu1)
                    },
                    [true, false, true, true] => {
                        //    |   | 1
                        // -/-+-#-+---
                        //  0 @   #
                        // ---+---+---
                        //    |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        if tv1 < tu0 {
                            return None;
                        }
                        let (cu0, cv0, err) = self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err);
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        let cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                        (cu0, cv0, err, cu1)
                    },
                    [true, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    @ 1 |
                        // ---+-@-+---
                        //  0 |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let (cu0, cv0, err) = self.cuv0_iuv_bu::<YX, FU, FV>(u0, v0, du, dv, tu0, tv0, err, du_half);
                        (cu0, cv0, err, u1)
                    },
                    [true, true, false, true] => {
                        //    | 1 |
                        // -/-+-#-+---
                        //    @   |
                        // ---+-@-+---
                        //  0 |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        if tv1 < tu0 {
                            return None;
                        }
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let (cu0, cv0, err) = self.cuv0_iuv_bu::<YX, FU, FV>(u0, v0, du, dv, tu0, tv0, err, du_half);
                        let cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
                        (cu0, cv0, err, cu1)
                    },
                    [true, true, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    @   # 1
                        // ---+-@-+-/-
                        //  0 |   |
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        if tu1 < tv0 {
                            return None;
                        }
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let (cu0, cv0, err) = self.cuv0_iuv_bu::<YX, FU, FV>(u0, v0, du, dv, tu0, tv0, err, du_half);
                        let cu1 = self.cu1_ou_bu::<YX, FU>();
                        (cu0, cv0, err, cu1)
                    },
                    [true, true, true, true] => {
                        //    |   | 1
                        // -/-+-#-+---
                        //    @   #
                        // ---+-@-+-/-
                        //  0 |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        if tv1 < tu0 {
                            return None;
                        }
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        if tu1 < tv0 {
                            return None;
                        }
                        let (cu0, cv0, err) = self.cuv0_iuv_bu::<YX, FU, FV>(u0, v0, du, dv, tu0, tv0, err, du_half);
                        let cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                        (cu0, cv0, err, cu1)
                    },
                };
                let su = if FU { -1 } else { 1 };
                let sv = if FV { -1 } else { 1 };
                Some((u0, v0, err, u1, su, sv))
            }

            const fn line_b_fxfy<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<LineB<$UI>> {
                if !FY && y0 == y1 {
                    let (x0, x1, sx) = try_opt!(self.raw_line_au_fu::<false, FX>(y0, x0, x1));
                    return Some(LineB::Bx(LineBx::<$UI>::new_au(y0, x0, x1, sx)))
                }
                if !FX && x0 == x1 {
                    let (y0, y1, sy) = try_opt!(self.raw_line_au_fu::<true, FY>(x0, y0, y1));
                    return Some(LineB::By(LineBy::<$UI>::new_au(x0, y0, y1, sy)))
                }
                if self.reject_bbox_closed::<FX, FY>(x0, y0, x1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::abs_diff_const_signed::<FX>(x1, x0);
                let dy = ops::<$UI>::abs_diff_const_signed::<FY>(y1, y0);
                if dy <= dx {
                    let (du, dv) = (dx, dy);
                    let (u0, v0, err, u1, su, sv) =
                        try_opt!(self.raw_line_bu_fufv::<false, FX, FY>(x0, y0, x1, y1, du, dv));
                    Some(LineB::Bx(LineBu { u0, v0, du, dv, err, u1, su, sv }))
                } else {
                    let (du, dv) = (dy, dx);
                    let (u0, v0, err, u1, su, sv) =
                        try_opt!(self.raw_line_bu_fufv::<true, FY, FX>(y0, x0, y1, x1, du, dv));
                    Some(LineB::By(LineBu { u0, v0, du, dv, err, u1, su, sv }))
                }
            }
        }
    };
    (@impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            const fn line_b_fxfy_proj<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<LineB<$U>> {
                if !FY && y0 == y1 {
                    let (y0, x0, x1, sx) = try_opt!(self.raw_line_au_fu_proj::<false, FX>(y0, x0, x1));
                    return Some(LineB::Bx(LineBx::<$U>::new_au(y0, x0, x1, sx)))
                }
                if !FX && x0 == x1 {
                    let (x0, y0, y1, sy) = try_opt!(self.raw_line_au_fu_proj::<true, FY>(x0, y0, y1));
                    return Some(LineB::By(LineBy::<$U>::new_au(x0, y0, y1, sy)))
                }
                if self.reject_bbox_closed::<FX, FY>(x0, y0, x1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::abs_diff_const_signed::<FX>(x1, x0);
                let dy = ops::<$UI>::abs_diff_const_signed::<FY>(y1, y0);
                if dy <= dx {
                    let (du, dv) = (dx, dy);
                    let (u0, v0, err, u1, su, sv) =
                        try_opt!(self.raw_line_bu_fufv::<false, FX, FY>(x0, y0, x1, y1, du, dv));
                    let u0 = ops::<$UI>::abs_diff(u0, self.x_min());
                    let v0 = ops::<$UI>::abs_diff(v0, self.y_min());
                    let u1 = ops::<$UI>::abs_diff(u1, self.x_min());
                    Some(LineB::Bx(LineBu { u0, v0, du, dv, err, u1, su, sv }))
                } else {
                    let (du, dv) = (dy, dx);
                    let (u0, v0, err, u1, su, sv) =
                        try_opt!(self.raw_line_bu_fufv::<true, FY, FX>(y0, x0, y1, x1, du, dv));
                    let u0 = ops::<$UI>::abs_diff(u0, self.y_min());
                    let v0 = ops::<$UI>::abs_diff(v0, self.x_min());
                    let u1 = ops::<$UI>::abs_diff(u1, self.y_min());
                    Some(LineB::By(LineBu { u0, v0, du, dv, err, u1, su, sv }))
                }
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            pub const fn line_b(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineB<$UI>> {
                let fx = x1 < x0;
                let fy = y1 < y0;
                match (fx, fy) {
                    (false, false) => self.line_b_fxfy::<false, false>(x0, y0, x1, y1),
                    (false, true) => self.line_b_fxfy::<false, true>(x0, y0, x1, y1),
                    (true, false) => self.line_b_fxfy::<true, false>(x0, y0, x1, y1),
                    (true, true) => self.line_b_fxfy::<true, true>(x0, y0, x1, y1),
                }
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            pub const fn line_b_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineB<$U>> {
                let fx = x1 < x0;
                let fy = y1 < y0;
                match (fx, fy) {
                    (false, false) => self.line_b_fxfy_proj::<false, false>(x0, y0, x1, y1),
                    (false, true) => self.line_b_fxfy_proj::<false, true>(x0, y0, x1, y1),
                    (true, false) => self.line_b_fxfy_proj::<true, false>(x0, y0, x1, y1),
                    (true, true) => self.line_b_fxfy_proj::<true, true>(x0, y0, x1, y1),
                }
            }
        }
    };
}

clip_line_b!(u8 | i8);
clip_line_b!(u16 | i16);
clip_line_b!(u32 | i32);
clip_line_b!(u64 | i64);
clip_line_b!(usize | isize);
