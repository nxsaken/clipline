use crate::clip::{Clip, Viewport};
use crate::line_d::{LineD, LineD2};
use crate::math::ops;
use crate::util::try_opt;

macro_rules! clip_line_d {
    ($U:ty | $I:ty) => {
        clip_line_d!(@impl Clip<$U>, $U);
        clip_line_d!(@impl Clip<$I>, $U);
        clip_line_d!(@impl Viewport<$U>, $U);
        clip_line_d!(@impl Viewport<$I>, $U);

        clip_line_d!(@pub impl Clip<$U>);
        clip_line_d!(@pub impl Clip<$I>);
        clip_line_d!(@pub impl Viewport<$U>);
        clip_line_d!(@pub impl Viewport<$I>);

        clip_line_d!(@pub impl Clip<$I, proj $U>);
        clip_line_d!(@pub impl Viewport<$U, proj $U>);
        clip_line_d!(@pub impl Viewport<$I, proj $U>);
    };
    (@impl $Self:ident<$UI:ty>, $U:ty) => {
        impl $Self<$UI> {
            const fn reject_bbox_half_open<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> bool {
                !FX && (x1 <= self.x_min() || self.x_max < x0)
                    || FX && (x0 < self.x_min() || self.x_max <= x1)
                    || !FY && (y1 <= self.y_min() || self.y_max < y0)
                    || FY && (y0 < self.y_min() || self.y_max <= y1)
            }

            const fn dx0<const FX: bool>(&self, x0: $UI) -> $U {
                self.du::<false, FX, false>(x0)
            }

            const fn dx1<const FX: bool>(&self, x0: $UI) -> $U {
                self.du::<false, FX, true>(x0)
            }

            const fn dy0<const FY: bool>(&self, y0: $UI) -> $U {
                self.dv::<false, FY, false>(y0)
            }

            const fn dy1<const FY: bool>(&self, y0: $UI) -> $U {
                self.dv::<false, FY, true>(y0)
            }

            const fn cxy0_ix_d<const FX: bool, const FY: bool>(
                &self,
                y0: $UI,
                dx0: $U
            ) -> ($UI, $UI) {
                let cx0 = if FX { self.x_max } else { self.x_min() };
                let cy0 = if FY { ops::<$UI>::sub_u(y0, dx0) } else { ops::<$UI>::add_u(y0, dx0) };
                (cx0, cy0)
            }

            const fn cxy0_iy_d<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                dy0: $U
            ) -> ($UI, $UI) {
                let cy0 = if FY { self.y_max } else { self.y_min() };
                let cx0 = if FX { ops::<$UI>::sub_u(x0, dy0) } else { ops::<$UI>::add_u(x0, dy0) };
                (cx0, cy0)
            }

            const fn cxy0_ixy_d<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                dx0: $U,
                dy0: $U,
            ) -> ($UI, $UI) {
                if dy0 <= dx0 {
                    self.cxy0_ix_d::<FX, FY>(y0, dx0)
                } else {
                    self.cxy0_iy_d::<FX, FY>(x0, dy0)
                }
            }

            const fn cx1_ox_d<const FX: bool>(&self) -> $UI {
                let exit = if FX { self.x_min() } else { self.x_max };
                let sx = if FX { -1 } else { 1 };
                ops::<$UI>::add_i(exit, sx)
            }

            const fn cx1_oy_d<const FX: bool>(x0: $UI, dy1: $U) -> $UI {
                let dy1_adj = dy1 + 1;
                if FX { ops::<$UI>::add_u(x0, dy1_adj) } else { ops::<$UI>::sub_u(x0, dy1_adj) }
            }

            const fn cx1_oxy_d<const FX: bool>(&self, x0: $UI, dx1: $U, dy1: $U) -> $UI {
                if dx1 <= dy1 {
                    self.cx1_ox_d::<FX>()
                } else {
                    Self::cx1_oy_d::<FX>(x0, dy1)
                }
            }

            const fn raw_line_d_fxfy<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<($UI, $UI, $UI, i8, i8)> {
                if self.reject_bbox_half_open::<FX, FY>(x0, y0, y1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::abs_diff_const_signed::<FX>(x1, x0);
                let dy = ops::<$UI>::abs_diff_const_signed::<FY>(y1, y0);
                if dx != dy {
                    return None;
                }
                let (x0, y0, x1) = match self.outcode::<false, FX, FY>(x0, y0, x1, y1) {
                    [false, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    |0 1|
                        // ---+---+---
                        //    |   |
                        (x0, y0, x1)
                    },
                    [false, false, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    | 0 |
                        // ---+---+---
                        //    |   |
                        let dy1 = self.dy1::<FY>(y0);
                        let cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
                        (x0, y0, cx1)
                    },
                    [false, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 0 # 1
                        // ---+---+---
                        //    |   |
                        let cx1 = self.cx1_ox_d::<FX>();
                        (x0, y0, cx1)
                    },
                    [false, false, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    | 0 #
                        // ---+---+---
                        //    |   |
                        let dx1 = self.dx1::<FX>(x0);
                        let dy1 = self.dy1::<FY>(y0);
                        let cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                        (x0, y0, cx1)
                    },
                    [false, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 1 |
                        // ---+-@-+---
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        let (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        (cx0, cy0, x1)
                    },
                    [false, true, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    |   |
                        // ---+-@-+---
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        let (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        let dy1 = self.dy1::<FY>(y0);
                        let cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
                        (cx0, cy0, cx1)
                    },
                    [false, true, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    |   # 1
                        // ---+-@-+-/-
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        let dx1 = self.dx1::<FX>(x0);
                        if dx1 < dy0 {
                            return None;
                        }
                        let (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        let cx1 = self.cx1_ox_d::<FX>();
                        (cx0, cy0, cx1)
                    },
                    [false, true, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    |   #
                        // ---+-@-+-/-
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        let dx1 = self.dx1::<FX>(x0);
                        if dx1 < dy0 {
                            return None;
                        }
                        let (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        let dy1 = self.dy1::<FY>(y0);
                        let cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                        (cx0, cy0, cx1)
                    },
                    [true, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @ 1 |
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        (cx0, cy0, x1)
                    },
                    [true, false, false, true] => {
                        //    | 1 |
                        // -/-+-#-+---
                        //  0 @   |
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let dy1 = self.dy1::<FY>(y0);
                        if dy1 < dx0 {
                            return None;
                        }
                        let (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        let cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
                        (cx0, cy0, cx1)
                    },
                    [true, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @   # 1
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        let cx1 = self.cx1_ox_d::<FX>();
                        (cx0, cy0, cx1)
                    },
                    [true, false, true, true] => {
                        //    |   | 1
                        // -/-+-#-+---
                        //  0 @   #
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let dy1 = self.dy1::<FY>(y0);
                        if dy1 < dx0 {
                            return None;
                        }
                        let (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        let dx1 = self.dx1::<FX>(x0);
                        let cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                        (cx0, cy0, cx1)
                    },
                    [true, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    @ 1 |
                        // ---+-@-+---
                        //  0 |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let dy0 = self.dy0::<FY>(y0);
                        let (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        (cx0, cy0, x1)
                    },
                    [true, true, false, true] => {
                        //    | 1 |
                        // -/-+-#-+---
                        //    @   |
                        // ---+-@-+---
                        //  0 |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let dy1 = self.dy1::<FY>(y0);
                        if dy1 < dx0 {
                            return None;
                        }
                        let dy0 = self.dy0::<FY>(y0);
                        let (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        let cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
                        (cx0, cy0, cx1)
                    },
                    [true, true, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    @   # 1
                        // ---+-@-+-/-
                        //  0 |   |
                        let dy0 = self.dy0::<FY>(y0);
                        let dx1 = self.dx1::<FX>(x0);
                        if dx1 < dy0 {
                            return None;
                        }
                        let dx0 = self.dx0::<FX>(x0);
                        let (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        let cx1 = self.cx1_ox_d::<FX>();
                        (cx0, cy0, cx1)
                    },
                    [true, true, true, true] => {
                        //    |   | 1
                        // -/-+-#-+---
                        //    @   #
                        // ---+-@-+-/-
                        //  0 |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let dy1 = self.dy1::<FY>(y0);
                        if dy1 < dx0 {
                            return None;
                        }
                        let dy0 = self.dy0::<FY>(y0);
                        let dx1 = self.dx1::<FX>(x0);
                        if dx1 < dy0 {
                            return None;
                        }
                        let (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        let cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                        (cx0, cy0, cx1)
                    },
                };
                let sx = if FX { -1 } else { 1 };
                let sy = if FY { -1 } else { 1 };
                Some((x0, y0, x1, sx, sy))
            }

            const fn raw_line_d(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<($UI, $UI, $UI, i8, i8)> {
                let fx = x1 < x0;
                let fy = y1 < y0;
                match (fx, fy) {
                    (false, false) => self.raw_line_d_fxfy::<false, false>(x0, y0, x1, y1),
                    (false, true) => self.raw_line_d_fxfy::<false, true>(x0, y0, x1, y1),
                    (true, false) => self.raw_line_d_fxfy::<true, false>(x0, y0, x1, y1),
                    (true, true) => self.raw_line_d_fxfy::<true, true>(x0, y0, x1, y1),
                }
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            pub const fn line_d(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$UI>> {
                let (x0, y0, x1, sx, sy) = try_opt!(self.raw_line_d(x0, y0, x1, y1));
                Some(LineD { x0, y0, x1, sx, sy })
            }

            pub const fn line_d2(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$UI>> {
                let line_d = try_opt!(self.line_d(x0, y0, x1, y1));
                // todo: can this be optimized?
                Some(line_d.to_line_d2())
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            pub const fn line_d_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$U>> {
                let (x0, y0, x1, sx, sy) = try_opt!(self.raw_line_d(x0, y0, x1, y1));
                let x0 = ops::<$UI>::proj(x0, self.x_min());
                let y0 = ops::<$UI>::proj(y0, self.y_min());
                let x1 = ops::<$UI>::proj(x1, self.x_min());
                Some(LineD { x0, y0, x1, sx, sy })
            }

            pub const fn line_d2_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$U>> {
                let line_d = try_opt!(self.line_d_proj(x0, y0, x1, y1));
                // todo: can this be optimized?
                Some(line_d.to_line_d2())
            }
        }
    };
}

clip_line_d!(u8 | i8);
clip_line_d!(u16 | i16);
clip_line_d!(u32 | i32);
clip_line_d!(u64 | i64);
clip_line_d!(usize | isize);
