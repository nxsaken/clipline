use crate::clip::{Clip, Viewport, if_clip, if_clip_u};
use crate::line_d::{LineD, LineD2};
use crate::math::ops;

macro_rules! clip_line_d {
    ($U:ty | $I:ty) => {
        clip_line_d!(@impl Clip<unsigned $U>, $U);
        clip_line_d!(@impl Clip<signed $I>, $U);
        clip_line_d!(@pub impl Clip<$U>);
        clip_line_d!(@pub impl Clip<$I>);
        clip_line_d!(@pub impl Clip<$U, proj $U>);
        clip_line_d!(@pub impl Clip<$I, proj $U>);

        clip_line_d!(@impl Viewport<unsigned $U>, $U);
        clip_line_d!(@impl Viewport<signed $I>, $U);
        clip_line_d!(@pub impl Viewport<$U>);
        clip_line_d!(@pub impl Viewport<$I>);
        clip_line_d!(@pub impl Viewport<$U, proj $U>);
        clip_line_d!(@pub impl Viewport<$I, proj $U>);
    };
    (@impl $Self:ident<$signess:ident $UI:ty>, $U:ty) => {
        impl $Self<$UI> {
            const fn reject_bbox_half_open<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> bool {
                let (x_min, y_min) = if_clip!($Self { (0, 0) } else { (self.x_min, self.y_min) });
                let x1_le_min = if_clip_u!($signess $Self { x1 == x_min } else { x1 <= x_min });
                let x0_lt_min = if_clip_u!($signess $Self { false } else { x0 < x_min });
                let y1_le_min = if_clip_u!($signess $Self { y1 == y_min } else { y1 <= y_min });
                let y0_lt_min = if_clip_u!($signess $Self { false } else { y0 < y_min });
                !FX && (x1_le_min || self.x_max < x0)
                    || FX && (x0_lt_min || self.x_max <= x1)
                    || !FY && (y1_le_min || self.y_max < y0)
                    || FY && (y0_lt_min || self.y_max <= y1)
            }

            pub(super) const fn outcode<const YX: bool, const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> [bool; 4] {
                let (x_min, y_min) = if_clip!($Self { (0, 0) } else { (self.x_min, self.y_min) });
                let x0_lt_min = if_clip_u!($signess $Self { _ = x_min; false } else { x0 < x_min });
                let y0_lt_min = if_clip_u!($signess $Self { _ = y_min; false } else { y0 < y_min });
                let x1_lt_min = if_clip_u!($signess $Self { _ = x_min; false } else { x1 < x_min });
                let y1_lt_min = if_clip_u!($signess $Self { _ = y_min; false } else { y1 < y_min });
                let maybe_ix = if FX { self.x_max < x0 } else { x0_lt_min };
                let maybe_iy = if FY { self.y_max < y0 } else { y0_lt_min };
                let maybe_ox = if FX { x1_lt_min } else { self.x_max < x1 };
                let maybe_oy = if FY { y1_lt_min } else { self.y_max < y1 };
                [maybe_ix, maybe_iy, maybe_ox, maybe_oy]
            }

            const fn du<const OI: bool, const YX: bool, const FU: bool>(&self, u0: $UI) -> $U {
                let u_min = if_clip!($Self { 0 } else { if YX { self.y_min } else { self.x_min } });
                let u_max = if YX { self.y_max } else { self.x_max };
                let (u_min, u_max) = if OI { (u_max, u_min) } else { (u_min, u_max) };
                let (lhs, rhs) = if FU { (u0, u_max) } else { (u_min, u0) };
                ops::<$UI>::abs_diff(lhs, rhs)
            }

            const fn dx0<const FX: bool>(&self, x0: $UI) -> $U {
                self.du::<false, false, FX>(x0)
            }

            const fn dy0<const FY: bool>(&self, y0: $UI) -> $U {
                self.du::<false, true, FY>(y0)
            }

            const fn dx1<const FX: bool>(&self, x0: $UI) -> $U {
                self.du::<true, false, FX>(x0)
            }

            const fn dy1<const FY: bool>(&self, y0: $UI) -> $U {
                self.du::<true, true, FY>(y0)
            }

            const fn uv0_iu<const YX: bool, const FU: bool, const FV: bool>(
                &self,
                v0: $UI,
                du0: $U
            ) -> ($UI, $UI) {
                let u_min = if_clip!($Self { 0 } else { if YX { self.y_min } else { self.x_min } });
                let u_max = if YX { self.y_max } else { self.x_max };
                let u0 = if FU { u_max } else { u_min };
                let v0 = if FV { ops::<$UI>::sub_u(v0, du0) } else { ops::<$UI>::add_u(v0, du0) } ;
                (u0, v0)
            }

            const fn xy0_ix<const FX: bool, const FY: bool>(
                &self,
                y0: $UI,
                dx0: $U
            ) -> ($UI, $UI) {
                self.uv0_iu::<false, FX, FY>(y0, dx0)
            }

            const fn xy0_iy<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                dy0: $U
            ) -> ($UI, $UI) {
                let (y0, x0) = self.uv0_iu::<true, FY, FX>(x0, dy0);
                (x0, y0)
            }

            const fn xy0_ixy<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                dx0: $U,
                dy0: $U,
            ) -> ($UI, $UI) {
                if dy0 <= dx0 {
                    self.xy0_ix::<FX, FY>(y0, dx0)
                } else {
                    self.xy0_iy::<FX, FY>(x0, dy0)
                }
            }

            const fn x1_ox<const FX: bool>(&self) -> $UI {
                let x_min = if_clip!($Self { 0 } else { self.x_min });
                let exit = if FX { x_min } else { self.x_max };
                let sx = if FX { -1 } else { 1 };
                ops::<$UI>::add_i(exit, sx)
            }

            const fn x1_oy<const FX: bool>(x0: $UI, dy1: $U) -> $UI {
                let dy1_adj = dy1 + 1;
                if FX { ops::<$UI>::add_u(x0, dy1_adj) } else { ops::<$UI>::sub_u(x0, dy1_adj) }
            }

            const fn x1_oxy<const FX: bool>(&self, x0: $UI, dx1: $U, dy1: $U) -> $UI {
                if dx1 <= dy1 {
                    self.x1_ox::<FX>()
                } else {
                    Self::x1_oy::<FX>(x0, dy1)
                }
            }

            const fn raw_line_dq<const FX: bool, const FY: bool>(
                &self,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<($UI, $UI, $UI, i8, i8)> {
                let dx = ops::<$UI>::abs_diff_const_signed::<FX>(x1, x0);
                let dy = ops::<$UI>::abs_diff_const_signed::<FY>(y1, y0);
                if dx != dy || self.reject_bbox_half_open::<FX, FY>(x0, y0, y1, y1) {
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
                        let cx1 = Self::x1_oy::<FX>(x0, dy1);
                        (x0, y0, cx1)
                    },
                    [false, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 0 # 1
                        // ---+---+---
                        //    |   |
                        let cx1 = self.x1_ox::<FX>();
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
                        let cx1 = self.x1_oxy::<FX>(x0, dx1, dy1);
                        (x0, y0, cx1)
                    },
                    [false, true, false, false] => {
                        //    |   |
                        // ---+---+---
                        //    | 1 |
                        // ---+-@-+---
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        let (cx0, cy0) = self.xy0_iy::<FX, FY>(x0, dy0);
                        (cx0, cy0, x1)
                    },
                    [false, true, false, true] => {
                        //    | 1 |
                        // ---+-#-+---
                        //    |   |
                        // ---+-@-+---
                        //    | 0 |
                        let dy0 = self.dy0::<FY>(y0);
                        let (cx0, cy0) = self.xy0_iy::<FX, FY>(x0, dy0);
                        let dy1 = self.dy1::<FY>(y0);
                        let cx1 = Self::x1_oy::<FX>(x0, dy1);
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
                        let (cx0, cy0) = self.xy0_iy::<FX, FY>(x0, dy0);
                        let cx1 = self.x1_ox::<FX>();
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
                        let (cx0, cy0) = self.xy0_iy::<FX, FY>(x0, dy0);
                        let dy1 = self.dy1::<FY>(y0);
                        let cx1 = self.x1_oxy::<FX>(x0, dx1, dy1);
                        (cx0, cy0, cx1)
                    },
                    [true, false, false, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @ 1 |
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let (cx0, cy0) = self.xy0_ix::<FX, FY>(y0, dx0);
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
                        let (cx0, cy0) = self.xy0_ix::<FX, FY>(y0, dx0);
                        let cx1 = Self::x1_oy::<FX>(x0, dy1);
                        (cx0, cy0, cx1)
                    },
                    [true, false, true, false] => {
                        //    |   |
                        // ---+---+---
                        //  0 @   # 1
                        // ---+---+---
                        //    |   |
                        let dx0 = self.dx0::<FX>(x0);
                        let (cx0, cy0) = self.xy0_ix::<FX, FY>(y0, dx0);
                        let cx1 = self.x1_ox::<FX>();
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
                        let (cx0, cy0) = self.xy0_ix::<FX, FY>(y0, dx0);
                        let dx1 = self.dx1::<FX>(x0);
                        let cx1 = self.x1_oxy::<FX>(x0, dx1, dy1);
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
                        let (cx0, cy0) = self.xy0_ixy::<FX, FY>(x0, y0, dx0, dy0);
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
                        let (cx0, cy0) = self.xy0_ixy::<FX, FY>(x0, y0, dx0, dy0);
                        let cx1 = Self::x1_oy::<FX>(x0, dy1);
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
                        let (cx0, cy0) = self.xy0_ixy::<FX, FY>(x0, y0, dx0, dy0);
                        let cx1 = self.x1_ox::<FX>();
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
                        let (cx0, cy0) = self.xy0_ixy::<FX, FY>(x0, y0, dx0, dy0);
                        let cx1 = self.x1_oxy::<FX>(x0, dx1, dy1);
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
                    (false, false) => self.raw_line_dq::<false, false>(x0, y0, x1, y1),
                    (false, true) => self.raw_line_dq::<false, true>(x0, y0, x1, y1),
                    (true, false) => self.raw_line_dq::<true, false>(x0, y0, x1, y1),
                    (true, true) => self.raw_line_dq::<true, true>(x0, y0, x1, y1),
                }
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            pub const fn line_d(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$UI>> {
                let Some((x0, y0, x1, sx, sy)) = self.raw_line_d(x0, y0, x1, y1) else {
                    return None;
                };
                Some(LineD { x0, y0, x1, sx, sy })
            }

            pub const fn line_d2(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$UI>> {
                let Some(line_d) = self.line_d(x0, y0, x1, y1) else {
                    return None;
                };
                // todo: can this be optimized?
                Some(line_d.to_line_d2())
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            pub const fn line_d_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$U>> {
                let Some((x0, y0, x1, sx, sy)) = self.raw_line_d(x0, y0, x1, y1) else {
                    return None;
                };
                let (x0, y0, x1) = if_clip!($Self {
                    let x0 = x0 as $U;
                    let y0 = y0 as $U;
                    let x1 = x1 as $U;
                    (x0, y0, x1)
                } else {
                    let Self { x_min, y_min, .. } = *self;
                    let x0 = ops::<$UI>::abs_diff(x0, x_min);
                    let y0 = ops::<$UI>::abs_diff(y0, y_min);
                    let x1 = ops::<$UI>::abs_diff(x1, x_min);
                    (x0, y0, x1)
                });
                Some(LineD { x0, y0, x1, sx, sy })
            }

            pub const fn line_d2_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$U>> {
                let Some(line_d) = self.line_d_proj(x0, y0, x1, y1) else {
                    return None;
                };
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
