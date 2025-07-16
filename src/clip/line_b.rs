use crate::clip::{Clip, Viewport, if_clip};
use crate::line_b::{LineB, LineBu, LineBx, LineBy};
use crate::math::{Coord, ops};

macro_rules! clip_line_b {
    ($U:ty | $I:ty) => {
        clip_line_b!(@impl Clip<$U>, $U, <$U as Coord>::U2, <$U as Coord>::I2);
        clip_line_b!(@impl Clip<$I>, $U, <$U as Coord>::U2, <$U as Coord>::I2);
        clip_line_b!(@impl Clip<$I, proj $U>);
        clip_line_b!(@pub impl Clip<$U>);
        clip_line_b!(@pub impl Clip<$I>);
        clip_line_b!(@pub impl Clip<$I, proj $U>);

        clip_line_b!(@impl Viewport<$U>, $U, <$U as Coord>::U2, <$U as Coord>::I2);
        clip_line_b!(@impl Viewport<$I>, $U, <$U as Coord>::U2, <$U as Coord>::I2);
        clip_line_b!(@impl Viewport<$U, proj $U>);
        clip_line_b!(@impl Viewport<$I, proj $U>);
        clip_line_b!(@pub impl Viewport<$U>);
        clip_line_b!(@pub impl Viewport<$I>);
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

            pub(super) const fn outcode<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> [bool; 4] {
                let maybe_ix = if FX { self.x_max < x0 } else { x0 < self.x_min() };
                let maybe_iy = if FY { self.y_max < y0 } else { y0 < self.y_min() };
                let maybe_ox = if FX { x1 < self.x_min() } else { self.x_max < x1 };
                let maybe_oy = if FY { y1 < self.y_min() } else { self.y_max < y1 };
                [maybe_ix, maybe_iy, maybe_ox, maybe_oy]
            }

            pub(super) const fn dx<const FX: bool, const OI: bool>(
                &self,
                x0: $UI,
            ) -> $U {
                let (x_min, x_max) = if OI { (self.x_max, self.x_min()) } else { (self.x_min(), self.x_max) };
                let (lhs, rhs) = if FX { (x0, x_max) } else { (x_min, x0) };
                ops::<$UI>::abs_diff(lhs, rhs)
            }

            pub(super) const fn dy<const FY: bool, const OI: bool>(
                &self,
                y0: $UI,
            ) -> $U {
                let (y_min, y_max) = if OI { (self.y_max, self.y_min()) } else { (self.y_min(), self.y_max) };
                let (lhs, rhs) = if FY { (y0, y_max) } else { (y_min, y0) };
                ops::<$UI>::abs_diff(lhs, rhs)
            }

            const fn tx0<const FX: bool>(
                &self,
                x0: $UI,
                dy: $U,
            ) -> $U2 {
                let dx0 = self.dx::<FX, false>(x0);
                dx0 as $U2 * dy as $U2
            }

            const fn tx1<const FX: bool>(
                &self,
                x0: $UI,
                dy: $U,
            ) -> $U2 {
                let dx1 = self.dx::<FX, true>(x0);
                dx1 as $U2 * dy as $U2
            }

            const fn ty0<const FY: bool>(
                &self,
                y0: $UI,
                dx: $U,
                dx_half: $U,
            ) -> $U2 {
                let dy0 = self.dy::<FY, false>(y0);
                let ty0_raw = dy0 as $U2 * dx as $U2;
                ty0_raw - dx_half as $U2
            }

            const fn ty1<const FY: bool>(
                &self,
                y0: $UI,
                dx: $U,
                dx_half: $U,
            ) -> $U2 {
                let dy1 = self.dy::<FY, true>(y0);
                let ty1_raw = dy1 as $U2 * dx as $U2;
                ty1_raw + dx_half as $U2
            }

            const fn cxy0_ix_bx<const FX: bool, const FY: bool>(
                &self,
                y0: $UI,
                dx: $U,
                tx0: $U2,
                mut err: $I2,
            ) -> ($UI, $UI, $I2) {
                let (mut dyc, dyc_rem) = {
                    // SAFETY: this is never called with dx == 0.
                    unsafe { core::hint::assert_unchecked(dx != 0) };
                    let (dyc, dyc_rem) = (tx0 / dx as $U2, tx0 % dx as $U2);
                    debug_assert!(dyc <= <$U>::MAX as $U2);
                    (dyc as $U, dyc_rem as $U)
                };
                err += dyc_rem as $I2;
                if dx as $U2 <= dyc_rem as $U2 * 2 {
                    dyc += 1;
                    err -= dx as $I2;
                }
                let cx0 = if FX { self.x_max } else { self.x_min() };
                let cy0 = if FY { ops::<$UI>::sub_u(y0, dyc) } else { ops::<$UI>::add_u(y0, dyc) };
                (cx0, cy0, err)
            }

            const fn cxy0_iy_bx<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                dy: $U,
                ty0: $U2,
                mut err: $I2,
                dx_half: $U,
            ) -> ($UI, $UI, $I2) {
                let (mut dxc, dxc_rem) = {
                    // SAFETY: this is never called with dy == 0.
                    unsafe { core::hint::assert_unchecked(dy != 0) };
                    let (dxc, dxc_rem) = (ty0 / dy as $U2, ty0 % dy as $U2);
                    debug_assert!(dxc <= <$U>::MAX as $U2);
                    (dxc as $U, dxc_rem as $U)
                };
                err -= dx_half as $I2;
                err -= dxc_rem as $I2;
                if 0 < dxc_rem {
                    dxc += 1;
                    err += dy as $I2;
                }
                let cx0 = if FX { ops::<$UI>::sub_u(x0, dxc) } else { ops::<$UI>::add_u(x0, dxc) };
                let cy0 = if FY { self.y_max } else { self.y_min() };
                (cx0, cy0, err)
            }

            const fn cxy0_ixy_bx<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                dx: $U,
                dy: $U,
                tx0: $U2,
                ty0: $U2,
                err: $I2,
                dx_half: $U,
            ) -> ($UI, $UI, $I2) {
                if ty0 <= tx0 {
                    self.cxy0_ix_bx::<FX, FY>(y0, dx, tx0, err)
                } else {
                    self.cxy0_iy_bx::<FX, FY>(x0, dy, ty0, err, dx_half)
                }
            }

            const fn cx1_ox_bx<const FX: bool>(
                &self,
            ) -> $UI {
                let exit = if FX { self.x_min() } else { self.x_max };
                let su = if FX { -1 } else { 1 };
                ops::<$UI>::add_i(exit, su)
            }

            const fn cx1_oy_bx<const FX: bool>(
                x0: $UI,
                dy: $U,
                ty1: $U2,
                dx_odd: bool,
            ) -> $UI {
                let (mut dxc, dxc_rem) = {
                    // SAFETY: this is never called with dy == 0.
                    unsafe { core::hint::assert_unchecked(dy != 0) };
                    let (dxc, dxc_rem) = (ty1 / dy as $U2, ty1 % dy as $U2);
                    debug_assert!(dxc <= <$U>::MAX as $U2);
                    (dxc as $U, dxc_rem as $U)
                };
                dxc += (dxc_rem != 0 || dx_odd) as $U;
                if FX { ops::<$UI>::sub_u(x0, dxc) } else { ops::<$UI>::add_u(x0, dxc) }
            }

            const fn cx1_oxy_bx<const FX: bool>(
                &self,
                x0: $UI,
                dy: $U,
                tx1: $U2,
                ty1: $U2,
                dx_odd: bool,
            ) -> $UI {
                if tx1 <= ty1 {
                    self.cx1_ox_bx::<FX>()
                } else {
                    Self::cx1_oy_bx::<FX>(x0, dy, ty1, dx_odd)
                }
            }

            const fn raw_line_bx<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
                dx: $U,
                dy: $U,
            ) -> Option<($UI, $UI, $I2, $UI, i8, i8)> {
                if !FX && x1 == self.x_min() || FX && x1 == self.x_max {
                    // ends on the entry along the major axis
                    return None;
                }
                let (dx_half, dx_odd) = (dx / 2, dx % 2 != 0);
                let err = dy as $I2 - (dx_half + dx_odd as $U) as $I2;
                let (x0, y0, err, x1) = match self.outcode::<FX, FY>(x0, y0, x1, y1) {
                    [false, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    |0 1|
                        // ---+---+---
                        //    |   |
                        (x0, y0, err, x1)
                    },
                    [false, false, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    | 0 |
                        // ---+---+---
                        //    |   |
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        let cx1 = Self::cx1_oy_bx::<FX>(x0, dy, ty1, dx_odd);
                        (x0, y0, err, cx1)
                    },
                    [false, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 0 # 1
                        // ---+---+---
                        //    |   |
                        let cx1 = self.cx1_ox_bx::<FX>();
                        (x0, y0, err, cx1)
                    },
                    [false, false, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    | 0 #
                        // ---+---+---
                        //    |   |
                        let tx1 = self.tx1::<FX>(x0, dy);
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        let cx1 = self.cx1_oxy_bx::<FX>(x0, dy, tx1, ty1, dx_odd);
                        (x0, y0, err, cx1)
                    },
                    [false, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 1 |
                        // ---+-@-+---
                        //    | 0 |
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let (cx0, cy0, err) = self.cxy0_iy_bx::<FX, FY>(x0, dy, ty0, err, dx_half);
                        (cx0, cy0, err, x1)
                    },
                    [false, true, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    |   |
                        // ---+-@-+---
                        //    | 0 |
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let (cx0, cy0, err) = self.cxy0_iy_bx::<FX, FY>(x0, dy, ty0, err, dx_half);
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        let cx1 = Self::cx1_oy_bx::<FX>(x0, dy, ty1, dx_odd);
                        (cx0, cy0, err, cx1)
                    },
                    [false, true, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    |   # 1
                        // ---+-@-+-/-
                        //    | 0 |
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let tx1 = self.tx1::<FX>(x0, dy);
                        if tx1 < ty0 {
                            return None;
                        }
                        let (cx0, cy0, err) = self.cxy0_iy_bx::<FX, FY>(x0, dy, ty0, err, dx_half);
                        let cx1 = self.cx1_ox_bx::<FX>();
                        (cx0, cy0, err, cx1)
                    },
                    [false, true, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    |   #
                        // ---+-@-+-/-
                        //    | 0 |
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let tx1 = self.tx1::<FX>(x0, dy);
                        if tx1 < ty0 {
                            return None;
                        }
                        let (cx0, cy0, err) = self.cxy0_iy_bx::<FX, FY>(x0, dy, ty0, err, dx_half);
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        let cx1 = self.cx1_oxy_bx::<FX>(x0, dy, tx1, ty1, dx_odd);
                        (cx0, cy0, err, cx1)
                    },
                    [true, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @ 1 |
                        // ---+---+---
                        //    |   |
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let (cx0, cy0, err) = self.cxy0_ix_bx::<FX, FY>(y0, dx, tx0, err);
                        (cx0, cy0, err, x1)
                    },
                    [true, false, false, true] => {
                        //    | 1 |
                        // -/-+-#-+---
                        //  0 @   |
                        // ---+---+---
                        //    |   |
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        if ty1 < tx0 {
                            return None;
                        }
                        let (cx0, cy0, err) = self.cxy0_ix_bx::<FX, FY>(y0, dx, tx0, err);
                        let cx1 = Self::cx1_oy_bx::<FX>(x0, dy, ty1, dx_odd);
                        (cx0, cy0, err, cx1)
                    },
                    [true, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @   # 1
                        // ---+---+---
                        //    |   |
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let (cx0, cy0, err) = self.cxy0_ix_bx::<FX, FY>(y0, dx, tx0, err);
                        let cx1 = self.cx1_ox_bx::<FX>();
                        (cx0, cy0, err, cx1)
                    },
                    [true, false, true, true] => {
                        //    |   | 1
                        // -/-+-#-+---
                        //  0 @   #
                        // ---+---+---
                        //    |   |
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        if ty1 < tx0 {
                            return None;
                        }
                        let (cx0, cy0, err) = self.cxy0_ix_bx::<FX, FY>(y0, dx, tx0, err);
                        let tx1 = self.tx1::<FX>(x0, dy);
                        let cx1 = self.cx1_oxy_bx::<FX>(x0, dy, tx1, ty1, dx_odd);
                        (cx0, cy0, err, cx1)
                    },
                    [true, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    @ 1 |
                        // ---+-@-+---
                        //  0 |   |
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let (cx0, cy0, err) = self.cxy0_ixy_bx::<FX, FY>(x0, y0, dx, dy, tx0, ty0, err, dx_half);
                        (cx0, cy0, err, x1)
                    },
                    [true, true, false, true] => {
                        //    | 1 |
                        // -/-+-#-+---
                        //    @   |
                        // ---+-@-+---
                        //  0 |   |
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        if ty1 < tx0 {
                            return None;
                        }
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let (cx0, cy0, err) = self.cxy0_ixy_bx::<FX, FY>(x0, y0, dx, dy, tx0, ty0, err, dx_half);
                        let cx1 = Self::cx1_oy_bx::<FX>(x0, dy, ty1, dx_odd);
                        (cx0, cy0, err, cx1)
                    },
                    [true, true, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    @   # 1
                        // ---+-@-+-/-
                        //  0 |   |
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let tx1 = self.tx1::<FX>(x0, dy);
                        if tx1 < ty0 {
                            return None;
                        }
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let (cx0, cy0, err) = self.cxy0_ixy_bx::<FX, FY>(x0, y0, dx, dy, tx0, ty0, err, dx_half);
                        let cx1 = self.cx1_ox_bx::<FX>();
                        (cx0, cy0, err, cx1)
                    },
                    [true, true, true, true] => {
                        //    |   | 1
                        // -/-+-#-+---
                        //    @   #
                        // ---+-@-+-/-
                        //  0 |   |
                        let tx0 = self.tx0::<FX>(x0, dy);
                        let ty1 = self.ty1::<FY>(y0, dx, dx_half);
                        if ty1 < tx0 {
                            return None;
                        }
                        let ty0 = self.ty0::<FY>(y0, dx, dx_half);
                        let tx1 = self.tx1::<FX>(x0, dy);
                        if tx1 < ty0 {
                            return None;
                        }
                        let (cx0, cy0, err) = self.cxy0_ixy_bx::<FX, FY>(x0, y0, dx, dy, tx0, ty0, err, dx_half);
                        let cx1 = self.cx1_oxy_bx::<FX>(x0, dy, tx1, ty1, dx_odd);
                        (cx0, cy0, err, cx1)
                    },
                };
                let sx = if FX { -1 } else { 1 };
                let sy = if FY { -1 } else { 1 };
                Some((x0, y0, err, x1, sx, sy))
            }

            const fn raw_line_bq<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<LineB<$UI>> {
                if !FY && y0 == y1 {}
                if !FX && x0 == x1 {}
                if self.reject_bbox_closed::<FX, FY>(x0, y0, x1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::abs_diff_const_signed::<FX>(x1, x0);
                let dy = ops::<$UI>::abs_diff_const_signed::<FY>(y1, y0);
                if dy <= dx {
                    let Some((u0, v0, err, u1, su, sv)) = self.raw_line_bx::<FX, FY>(x0, y0, x1, y1, dx, dy)
                    else {
                        return None;
                    };
                    let (du, dv) = (dx, dy);
                    Some(LineB::Bx(LineBx { u0, v0, du, dv, err, u1, su, sv }))
                } else {
                    let clip = self.yx();
                    let Some((u0, v0, err, u1, su, sv)) = clip.raw_line_bx::<FY, FX>(y0, x0, y1, x1, dy, dx)
                    else {
                        return None;
                    };
                    let (du, dv) = (dy, dx);
                    Some(LineB::By(LineBy { u0, v0, du, dv, err, u1, su, sv }))
                }
            }
        }
    };
    (@impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            const fn raw_line_bu_proj<const YX: bool, const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
                dx: $U,
                dy: $U,
            ) -> Option<LineBu<YX, $U>> {
                let Some((u0, v0, err, u1, su, sv)) = (if YX {
                    self.yx().raw_line_bx::<FY, FX>(y0, x0, y1, x1, dy, dx)
                } else {
                    self.raw_line_bx::<FX, FY>(x0, y0, x1, y1, dx, dy)
                }) else {
                    return None;
                };
                let (du, dv) = if YX { (dy, dx) } else { (dx, dy) };
                let (u0, v0, u1) = if_clip!($Self {
                    let u0 = u0 as $U;
                    let v0 = v0 as $U;
                    let u1 = u1 as $U;
                    (u0, v0, u1)
                } else {
                    let u0 = ops::<$UI>::abs_diff(u0, if YX { self.y_min() } else { self.x_min() });
                    let v0 = ops::<$UI>::abs_diff(v0, if YX { self.x_min() } else { self.y_min() });
                    let u1 = ops::<$UI>::abs_diff(u1, if YX { self.y_min() } else { self.x_min() });
                    (u0, v0, u1)
                });
                Some(LineBu { u0, v0, du, dv, err, u1, su, sv })
            }

            const fn raw_line_bq_proj<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<LineB<$U>> {
                if !FY && y0 == y1 {
                    let Some((v0, u0, u1, su)) = self.raw_line_axs_proj::<FX>(y0, x0, x1)
                    else {
                        return None;
                    };
                    return Some(LineB::Bx(LineBx { u0, v0, du: 0, dv: 0, err: -1, u1, su, sv: 0 }))
                }
                if !FX && x0 == x1 {
                    let Some((v0, u0, u1, su)) = self.yx().raw_line_axs_proj::<FY>(y0, x0, x1)
                    else {
                        return None;
                    };
                    return Some(LineB::By(LineBy { u0, v0, du: 0, dv: 0, err: -1, u1, su, sv: 0 }))
                }
                if self.reject_bbox_closed::<FX, FY>(x0, y0, x1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::abs_diff_const_signed::<FX>(x1, x0);
                let dy = ops::<$UI>::abs_diff_const_signed::<FY>(y1, y0);
                if dy <= dx {
                    let Some(line) = self.raw_line_bu_proj::<false, FX, FY>(x0, y0, x1, y1, dx, dy)
                    else {
                        return None;
                    };
                    Some(LineB::Bx(line))
                } else {
                    let Some(line) = self.raw_line_bu_proj::<true, FY, FX>(x0, y0, x1, y1, dx, dy)
                    else {
                        return None;
                    };
                    Some(LineB::By(line))
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
                    (false, false) => self.raw_line_bq::<false, false>(x0, y0, x1, y1),
                    (false, true) => self.raw_line_bq::<false, true>(x0, y0, x1, y1),
                    (true, false) => self.raw_line_bq::<true, false>(x0, y0, x1, y1),
                    (true, true) => self.raw_line_bq::<true, true>(x0, y0, x1, y1),
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
                    (false, false) => self.raw_line_bq_proj::<false, false>(x0, y0, x1, y1),
                    (false, true) => self.raw_line_bq_proj::<false, true>(x0, y0, x1, y1),
                    (true, false) => self.raw_line_bq_proj::<true, false>(x0, y0, x1, y1),
                    (true, true) => self.raw_line_bq_proj::<true, true>(x0, y0, x1, y1),
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
