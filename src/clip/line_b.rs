use crate::clip::{Clip, Viewport};
use crate::line_b::{LineB, LineBu, LineBx, LineBy};
use crate::macros::*;
use crate::math::{Coord, ops};

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
            #[inline]
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

            #[inline]
            pub(super) const fn outcode<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                u0: $UI,
                v0: $UI,
                u1: $UI,
                v1: $UI,
            ) -> [bool; 4] {
                let (u_min, v_min, u_max, v_max) = self.uv_min_max::<YX>();
                let maybe_iu = if FU { u_max < u0 } else { u0 < u_min };
                let maybe_iv = if FV { v_max < v0 } else { v0 < v_min };
                let maybe_ou = if FU { u1 < u_min } else { u_max < u1 };
                let maybe_ov = if FV { v1 < v_min } else { v_max < v1 };
                [maybe_iu, maybe_iv, maybe_ou, maybe_ov]
            }

            #[inline]
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
                ops::<$UI>::usub(lhs, rhs)
            }

            #[inline]
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
                ops::<$UI>::usub(lhs, rhs)
            }

            #[inline]
            const fn tu0<const YX: bool, const FU: bool>(
                &self,
                u0: $UI,
                dv: $U,
            ) -> $U2 {
                let du0 = self.du::<YX, FU, false>(u0);
                du0 as $U2 * dv as $U2
            }

            #[inline]
            const fn tu1<const YX: bool, const FU: bool>(
                &self,
                u0: $UI,
                dv: $U,
            ) -> $U2 {
                let du1 = self.du::<YX, FU, true>(u0);
                du1 as $U2 * dv as $U2
            }

            #[inline]
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

            #[inline]
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

            #[inline]
            const fn dc_rem(t0: $U2, d: $U) -> ($U, $U) {
                // SAFETY: this is never called with d == 0.
                unsafe { core::hint::assert_unchecked(d != 0) };
                let (dc, dc_rem) = (t0 / d as $U2, t0 % d as $U2);
                debug_assert!(dc <= <$U>::MAX as $U2);
                (dc as $U, dc_rem as $U)
            }

            #[inline]
            pub(super) const fn u_near<const YX: bool, const FU: bool>(&self) -> $UI {
                if FU { self.u_max::<YX>() } else { self.u_min::<YX>() }
            }

            #[inline]
            pub(super) const fn u_far<const YX: bool, const FU: bool>(&self) -> $UI {
                if FU { self.u_min::<YX>() } else { self.u_max::<YX>() }
            }

            #[inline]
            pub(super) const fn v_near<const YX: bool, const FV: bool>(&self) -> $UI {
                if FV { self.v_max::<YX>() } else { self.v_min::<YX>() }
            }

            #[inline]
            const fn cuv0_iu_bu<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                v0: $UI,
                du: $U,
                tu0: $U2,
                mut err: $I2,
                du_half_ceil: $U,
            ) -> ($UI, $UI, $I2) {
                let (mut dvc, dvc_rem) = Self::dc_rem(tu0, du);
                err += dvc_rem as $I2;
                if du_half_ceil <= dvc_rem {
                    dvc += 1;
                    err -= du as $I2;
                }
                let cu0 = self.u_near::<YX, FU>();
                let cv0 = ops::<$UI>::add_fu::<FV>(v0, dvc);
                (cu0, cv0, err)
            }

            #[inline]
            const fn cuv0_iu_bu_noadj<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                v0: $UI,
                dvc: $U,
                dvc_rem: $U,
                mut err: $I2,
            ) -> ($UI, $UI, $I2) {
                err += dvc_rem as $I2;
                let cu0 = self.u_near::<YX, FU>();
                let cv0 = ops::<$UI>::add_fu::<FV>(v0, dvc);
                (cu0, cv0, err)
            }

            #[inline]
            const fn cuv0_iv_bu<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                u0: $UI,
                dv: $U,
                tv0: $U2,
                mut err: $I2,
                du_half: $U,
            ) -> ($UI, $UI, $I2) {
                let (mut duc, duc_rem) = Self::dc_rem(tv0, dv);
                err -= du_half as $I2;
                err -= duc_rem as $I2;
                if 0 < duc_rem {
                    duc += 1;
                    err += dv as $I2;
                }
                let cu0 = ops::<$UI>::add_fu::<FU>(u0, duc);
                let cv0 = self.v_near::<YX, FV>();
                (cu0, cv0, err)
            }

            #[inline]
            const fn cu1_ou_bu<const YX: bool, const FU: bool>(
                &self,
            ) -> $UI {
                let ou = self.u_far::<YX, FU>();
                let su = if FU { -1 } else { 1 };
                ops::<$UI>::wadd_i(ou, su)
            }

            #[inline]
            const fn cu1_ov_bu<const FU: bool>(
                u0: $UI,
                dv: $U,
                tv1: $U2,
                du_odd: bool,
            ) -> $UI {
                let (mut duc, duc_rem) = Self::dc_rem(tv1, dv);
                duc += (duc_rem != 0 || du_odd) as $U;
                ops::<$UI>::add_fu::<FU>(u0, duc)
            }

            #[inline]
            const fn cu1_ouv_bu<const YX: bool, const FU: bool>(
                &self,
                u0: $UI,
                dv: $U,
                tu1: $U2,
                tv1: $U2,
                du_odd: bool,
            ) -> $UI {
                if tu1 < tv1 {
                    self.cu1_ou_bu::<YX, FU>()
                } else {
                    Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd)
                }
            }

            #[inline]
            const fn raw_line_bu_fufv<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                u0: $UI,
                v0: $UI,
                u1: $UI,
                v1: $UI,
                du: $U,
                dv: $U,
            ) -> Option<($UI, $UI, $I2, $UI, i8, i8)> {
                if u1 == self.u_near::<YX, FU>() {
                    // ends on the entry along the major axis
                    return None;
                }
                let (du_half, du_odd) = (du / 2, du % 2 != 0);
                if v1 == self.v_near::<YX, FV>() && du_half < dv {
                    return None;
                }
                let du_half_ceil = du_half + du_odd as $U;
                let mut err = dv as $I2 - du_half_ceil as $I2;
                let (cu0, cv0, cu1);
                match self.outcode::<YX, FU, FV>(u0, v0, u1, v1) {
                    [false, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    |0 1|
                        // ---+---+---
                        //    |   |
                        (cu0, cv0, cu1) = (u0, v0, u1);
                    },
                    [false, false, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    | 0 |
                        // ---+---+---
                        //    |   |
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
                        (cu0, cv0) = (u0, v0);
                    },
                    [false, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 0 # 1
                        // ---+---+---
                        //    |   |
                        cu1 = self.cu1_ou_bu::<YX, FU>();
                        (cu0, cv0) = (u0, v0);
                    },
                    [false, false, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    | 0 #
                        // ---+---+---
                        //    |   |
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                        (cu0, cv0) = (u0, v0);
                    },
                    [false, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 1 |
                        // ---+-@-+---
                        //    | 0 |
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        cu1 = u1;
                    },
                    [false, true, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    |   |
                        // ---+-@-+---
                        //    | 0 |
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
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
                        (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        cu1 = self.cu1_ou_bu::<YX, FU>();
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
                        (cu0, cv0, err) = self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                    },
                    [true, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @ 1 |
                        // ---+---+---
                        //    |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        (cu0, cv0, err) = self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil);
                        cu1 = u1;
                    },
                    [true, false, false, true] => {
                        //    | 1 |
                        // -/-+-#-+---
                        //  0 @   |
                        // ---+---+---
                        //    |   |
                        // u0 < u_min, u1 <= u_max, v_min <= v0, v_max < v1
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let tv1 = self.tv1::<YX, FV>(v0, du, du_half);
                        if tv1 < tu0 {
                            return None;
                        }
                        (cu0, cv0, err) = if tv1 == tu0 {
                            let (dvc, dvc_rem) = Self::dc_rem(tu0, du);
                            if du_half_ceil <= dvc_rem {
                                return None;
                            }
                            self.cuv0_iu_bu_noadj::<YX, FU, FV>(v0, dvc, dvc_rem, err)
                        } else {
                            self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil)
                        };
                        cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
                    },
                    [true, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @   # 1
                        // ---+---+---
                        //    |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        (cu0, cv0, err) = self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil);
                        cu1 = self.cu1_ou_bu::<YX, FU>();
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
                        (cu0, cv0, err) = if tv1 == tu0 {
                            let (dvc, dvc_rem) = Self::dc_rem(tu0, du);
                            if du_half_ceil <= dvc_rem {
                                return None;
                            }
                            self.cuv0_iu_bu_noadj::<YX, FU, FV>(v0, dvc, dvc_rem, err)
                        } else {
                            self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil)
                        };
                        let tu1 = self.tu1::<YX, FU>(u0, dv);
                        cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                    },
                    [true, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    @ 1 |
                        // ---+-@-+---
                        //  0 |   |
                        let tu0 = self.tu0::<YX, FU>(u0, dv);
                        let tv0 = self.tv0::<YX, FV>(v0, du, du_half);
                        (cu0, cv0, err) = if tv0 < tu0 {
                            self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil)
                        } else {
                            self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half)
                        };
                        cu1 = u1;
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
                        (cu0, cv0, err) = if tv0 < tu0 {
                            if tv1 == tu0 {
                                let (dvc, dvc_rem) = Self::dc_rem(tu0, du);
                                if du_half_ceil <= dvc_rem {
                                    return None;
                                }
                                self.cuv0_iu_bu_noadj::<YX, FU, FV>(v0, dvc, dvc_rem, err)
                            } else {
                                self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil)
                            }
                        } else {
                            self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half)
                        };
                        cu1 = Self::cu1_ov_bu::<FU>(u0, dv, tv1, du_odd);
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
                        (cu0, cv0, err) = if tv0 < tu0 {
                            self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil)
                        } else {
                            self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half)
                        };
                        cu1 = self.cu1_ou_bu::<YX, FU>();
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
                        (cu0, cv0, err) = if tv0 < tu0 {
                            if tv1 == tu0 {
                                let (dvc, dvc_rem) = Self::dc_rem(tu0, du);
                                if du_half_ceil <= dvc_rem {
                                    return None;
                                }
                                self.cuv0_iu_bu_noadj::<YX, FU, FV>(v0, dvc, dvc_rem, err)
                            } else {
                                self.cuv0_iu_bu::<YX, FU, FV>(v0, du, tu0, err, du_half_ceil)
                            }
                        } else {
                            self.cuv0_iv_bu::<YX, FU, FV>(u0, dv, tv0, err, du_half)
                        };
                        cu1 = self.cu1_ouv_bu::<YX, FU>(u0, dv, tu1, tv1, du_odd);
                    },
                };
                let su = if FU { -1 } else { 1 };
                let sv = if FV { -1 } else { 1 };
                Some((cu0, cv0, err, cu1, su, sv))
            }

            #[inline]
            const fn line_b_fxfy<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<LineB<$UI>> {
                if !FY && y0 == y1 {
                    let (x0, x1, sx) = try_opt!(self.raw_line_au_fu::<false, FX>(y0, x0, x1));
                    return Some(LineB::Bx(LineBx::<$UI>::from_line_au(y0, x0, x1, sx)))
                }
                if !FX && x0 == x1 {
                    let (y0, y1, sy) = try_opt!(self.raw_line_au_fu::<true, FY>(x0, y0, y1));
                    return Some(LineB::By(LineBy::<$UI>::from_line_au(x0, y0, y1, sy)))
                }
                if self.reject_bbox_closed::<FX, FY>(x0, y0, x1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::usub_f::<FX>(x1, x0);
                let dy = ops::<$UI>::usub_f::<FY>(y1, y0);
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
            #[inline]
            const fn line_b_fxfy_proj<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<LineB<$U>> {
                if !FY && y0 == y1 {
                    let (y0, x0, x1, sx) = try_opt!(self.raw_line_au_fu_proj::<false, FX>(y0, x0, x1));
                    return Some(LineB::Bx(LineBx::<$U>::from_line_au(y0, x0, x1, sx)))
                }
                if !FX && x0 == x1 {
                    let (x0, y0, y1, sy) = try_opt!(self.raw_line_au_fu_proj::<true, FY>(x0, y0, y1));
                    return Some(LineB::By(LineBy::<$U>::from_line_au(x0, y0, y1, sy)))
                }
                if self.reject_bbox_closed::<FX, FY>(x0, y0, x1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::usub_f::<FX>(x1, x0);
                let dy = ops::<$UI>::usub_f::<FY>(y1, y0);
                if dy <= dx {
                    let (du, dv) = (dx, dy);
                    let (u0, v0, err, u1, su, sv) =
                        try_opt!(self.raw_line_bu_fufv::<false, FX, FY>(x0, y0, x1, y1, du, dv));
                    let u0 = ops::<$UI>::wusub(u0, self.x_min());
                    let v0 = ops::<$UI>::wusub(v0, self.y_min());
                    let u1 = ops::<$UI>::wusub(u1, self.x_min());
                    Some(LineB::Bx(LineBu { u0, v0, du, dv, err, u1, su, sv }))
                } else {
                    let (du, dv) = (dy, dx);
                    let (u0, v0, err, u1, su, sv) =
                        try_opt!(self.raw_line_bu_fufv::<true, FY, FX>(y0, x0, y1, x1, du, dv));
                    let u0 = ops::<$UI>::wusub(u0, self.y_min());
                    let v0 = ops::<$UI>::wusub(v0, self.x_min());
                    let u1 = ops::<$UI>::wusub(u1, self.y_min());
                    Some(LineB::By(LineBu { u0, v0, du, dv, err, u1, su, sv }))
                }
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            /// Clips the directed, half-open line segment `(x0, y0) -> (x1, y1)` to this region.
            ///
            /// Returns a [`LineB`] over the portion of the segment inside this
            /// clipping region, or [`None`] if the segment lies fully outside.
            #[inline]
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
            /// Clips and projects the directed, half-open line segment `(x0, y0) -> (x1, y1)`
            /// to this region.
            ///
            /// Returns a [`LineB`] over the portion of the segment inside this
            /// clipping region relative to the region, or [`None`] if the segment
            /// lies fully outside.
            #[inline]
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
