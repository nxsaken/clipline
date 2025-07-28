use crate::clip::{Clip, Viewport};
use crate::line_d::{LineD, LineD2};
use crate::macros::*;
use crate::math::ops;

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
            #[inline]
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

            #[inline]
            const fn dx0<const FX: bool>(&self, x0: $UI) -> $U {
                self.du::<false, FX, false>(x0)
            }

            #[inline]
            const fn dx1<const FX: bool>(&self, x0: $UI) -> $U {
                self.du::<false, FX, true>(x0)
            }

            #[inline]
            const fn dy0<const FY: bool>(&self, y0: $UI) -> $U {
                self.dv::<false, FY, false>(y0)
            }

            #[inline]
            const fn dy1<const FY: bool>(&self, y0: $UI) -> $U {
                self.dv::<false, FY, true>(y0)
            }

            #[inline]
            const fn cxy0_ix_d<const FX: bool, const FY: bool>(
                &self,
                y0: $UI,
                dx0: $U
            ) -> ($UI, $UI) {
                let cx0 = self.u_near::<false, FX>();
                let cy0 = ops::<$UI>::add_fu::<FY>(y0, dx0);
                (cx0, cy0)
            }

            #[inline]
            const fn cxy0_iy_d<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                dy0: $U
            ) -> ($UI, $UI) {
                let cy0 = self.v_near::<false, FY>();
                let cx0 = ops::<$UI>::add_fu::<FX>(x0, dy0);
                (cx0, cy0)
            }

            #[inline]
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

            #[inline]
            const fn cx1_ox_d<const FX: bool>(&self) -> $UI {
                let exit = self.u_far::<false, FX>();
                let sx = if FX { -1 } else { 1 };
                ops::<$UI>::wadd_i(exit, sx)
            }

            #[inline]
            const fn cx1_oy_d<const FX: bool>(x0: $UI, dy1: $U) -> $UI {
                let dy1_adj = dy1 + 1;
                ops::<$UI>::add_fu::<FX>(x0, dy1_adj)
            }

            #[inline]
            const fn cx1_oxy_d<const FX: bool>(&self, x0: $UI, dx1: $U, dy1: $U) -> $UI {
                if dx1 <= dy1 {
                    self.cx1_ox_d::<FX>()
                } else {
                    Self::cx1_oy_d::<FX>(x0, dy1)
                }
            }

            #[inline]
            const fn raw_line_d_fxfy<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<($UI, $UI, $UI, i8, i8)> {
                if self.reject_bbox_half_open::<FX, FY>(x0, y0, x1, y1) {
                    return None;
                }
                let dx = ops::<$UI>::usub_f::<FX>(x1, x0);
                let dy = ops::<$UI>::usub_f::<FY>(y1, y0);
                if dx != dy {
                    return None;
                }
                let (cx0, cy0, cx1);
                match self.outcode::<false, FX, FY>(x0, y0, x1, y1) {
                    [false, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    |0 1|
                        // ---+---+---
                        //    |   |
                        (cx0, cy0, cx1) = (x0, y0, x1);
                    },
                    [false, false, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    | 0 |
                        // ---+---+---
                        //    |   |
                        let dy1 = self.dy1::<FY>(y0);
                        cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
                        (cx0, cy0) = (x0, y0);
                    },
                    [false, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 0 # 1
                        // ---+---+---
                        //    |   |
                        cx1 = self.cx1_ox_d::<FX>();
                        (cx0, cy0) = (x0, y0);
                    },
                    [false, false, true, true] => {
                        //    |   | 1
                        // ---+-#-+---
                        //    | 0 #
                        // ---+---+---
                        //    |   |
                        let dx1 = self.dx1::<FX>(x0);
                        let dy1 = self.dy1::<FY>(y0);
                        cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                        (cx0, cy0) = (x0, y0);
                    },
                    [false, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 1 |
                        // ---+-@-+---
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        cx1 = x1;
                    },
                    [false, true, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    |   |
                        // ---+-@-+---
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        let dy1 = self.dy1::<FY>(y0);
                        cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
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
                        (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        cx1 = self.cx1_ox_d::<FX>();
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
                        (cx0, cy0) = self.cxy0_iy_d::<FX, FY>(x0, dy0);
                        let dy1 = self.dy1::<FY>(y0);
                        cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                    },
                    [true, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @ 1 |
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        cx1 = x1;
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
                        (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
                    },
                    [true, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @   # 1
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        cx1 = self.cx1_ox_d::<FX>();
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
                        (cx0, cy0) = self.cxy0_ix_d::<FX, FY>(y0, dx0);
                        let dx1 = self.dx1::<FX>(x0);
                        cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                    },
                    [true, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    @ 1 |
                        // ---+-@-+---
                        //  0 |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let dy0 = self.dy0::<FY>(y0);
                        (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        cx1 = x1;
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
                        (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        cx1 = Self::cx1_oy_d::<FX>(x0, dy1);
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
                        (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        cx1 = self.cx1_ox_d::<FX>();
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
                        (cx0, cy0) = self.cxy0_ixy_d::<FX, FY>(x0, y0, dx0, dy0);
                        cx1 = self.cx1_oxy_d::<FX>(x0, dx1, dy1);
                    },
                };
                let sx = if FX { -1 } else { 1 };
                let sy = if FY { -1 } else { 1 };
                Some((cx0, cy0, cx1, sx, sy))
            }

            #[inline]
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
            /// Clips the directed, half-open line segment `(x0, y0) -> (x1, y1)` to this region
            /// if it is diagonal.
            ///
            /// Returns a [`LineD`] over the portion of the segment inside this
            /// clipping region, or [`None`] if the segment is not diagonal or lies fully outside.
            #[inline]
            pub const fn line_d(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$UI>> {
                let (x0, y0, x1, sx, sy) = try_opt!(self.raw_line_d(x0, y0, x1, y1));
                Some(LineD { x0, y0, x1, sx, sy })
            }

            /// Clips the directed, half-open line segment `(x0, y0) -> (x1, y1)` to this region
            /// if it is diagonal.
            ///
            /// Returns a [`LineD2`] over the portion of the segment inside this
            /// clipping region, or [`None`] if the segment is not diagonal or lies fully outside.
            #[inline]
            pub const fn line_d2(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$UI>> {
                let line_d = try_opt!(self.line_d(x0, y0, x1, y1));
                Some(line_d.to_line_d2())
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            /// Clips and projects the directed, half-open line segment `(x0, y0) -> (x1, y1)`
            /// to this region if it is diagonal.
            ///
            /// Returns a [`LineD`] over the portion of the segment inside this
            /// clipping region relative to the region, or [`None`] if the segment
            /// is not diagonal or lies fully outside.
            #[inline]
            pub const fn line_d_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$U>> {
                let (x0, y0, x1, sx, sy) = try_opt!(self.raw_line_d(x0, y0, x1, y1));
                let x0 = ops::<$UI>::wusub(x0, self.x_min());
                let y0 = ops::<$UI>::wusub(y0, self.y_min());
                let x1 = ops::<$UI>::wusub(x1, self.x_min());
                Some(LineD { x0, y0, x1, sx, sy })
            }

            /// Clips and projects the directed, half-open line segment `(x0, y0) -> (x1, y1)`
            /// to this region if it is diagonal.
            ///
            /// Returns a [`LineD2`] over the portion of the segment inside this
            /// clipping region relative to the region, or [`None`] if the segment
            /// is not diagonal or lies fully outside.
            #[inline]
            pub const fn line_d2_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$U>> {
                let line_d = try_opt!(self.line_d_proj(x0, y0, x1, y1));
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
