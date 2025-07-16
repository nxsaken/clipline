use crate::clip::{Clip, Viewport, if_clip};
use crate::line_a::{LineA, LineAu, LineAx, LineAy};
use crate::math::ops;

macro_rules! clip_line_a {
    ($U:ty | $I:ty) => {
        clip_line_a!(@impl Clip<$U>);
        clip_line_a!(@impl Clip<$I>);
        clip_line_a!(@impl Clip<$I, proj $U>);
        clip_line_a!(@pub impl Clip<$U>);
        clip_line_a!(@pub impl Clip<$I>);
        clip_line_a!(@pub impl Clip<$I, proj $U>);

        clip_line_a!(@impl Viewport<$U>);
        clip_line_a!(@impl Viewport<$I>);
        clip_line_a!(@impl Viewport<$U, proj $U>);
        clip_line_a!(@impl Viewport<$I, proj $U>);
        clip_line_a!(@pub impl Viewport<$U>);
        clip_line_a!(@pub impl Viewport<$I>);
        clip_line_a!(@pub impl Viewport<$U, proj $U>);
        clip_line_a!(@pub impl Viewport<$I, proj $U>);
    };
    (@impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            pub(super) const fn raw_line_axs<const FX: bool>(
                &self,
                y: $UI,
                x0: $UI,
                x1: $UI,
            ) -> Option<($UI, $UI, i8)> {
                let reject_y = y < self.y_min() || self.y_max < y;
                if FX {
                    if reject_y || self.x_max <= x1 || x0 < self.x_min() {
                        return None;
                    }
                    let cx0 = ops::<$UI>::min(x0, self.x_max);
                    let cx1 = ops::<$UI>::max_adj(self.x_min(), x1);
                    Some((cx0, cx1, -1))
                } else {
                    if reject_y || x1 <= self.x_min() || self.x_max < x0 {
                        return None;
                    }
                    let cx0 = ops::<$UI>::max(x0, self.x_min());
                    let cx1 = ops::<$UI>::min_adj(self.x_max, x1);
                    Some((cx0, cx1, 1))
                }
            }

            const fn raw_line_ax(&self, y: $UI, x0: $UI, x1: $UI) -> Option<($UI, $UI, i8)> {
                if x0 <= x1 {
                    self.raw_line_axs::<false>(y, x0, x1)
                } else {
                    self.raw_line_axs::<true>(y, x0, x1)
                }
            }
        }
    };
    (@impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            pub(super) const fn raw_line_axs_proj<const FX: bool>(
                &self,
                y: $UI,
                x0: $UI,
                x1: $UI,
            ) -> Option<($U, $U, $U, i8)> {
                let Some((x0, x1, sx)) = self.raw_line_axs::<false>(y, x0, x1) else { return None };
                if_clip!($Self {
                    let y = y as $U;
                    let x0 = x0 as $U;
                    let x1 = x1 as $U;
                    Some((y, x0, x1, sx))
                } else {
                    let y = ops::<$UI>::abs_diff(y, self.y_min());
                    let x1 = ops::<$UI>::abs_diff(x1, self.x_min());
                    let x0 = ops::<$UI>::abs_diff(x0, self.x_min());
                    Some((y, x0, x1, sx))
                })
            }

            const fn raw_line_ax_proj(&self, y: $UI, x0: $UI, x1: $UI) -> Option<($U, $U, $U, i8)> {
                if x0 <= x1 {
                    self.raw_line_axs_proj::<false>(y, x0, x1)
                } else {
                    self.raw_line_axs_proj::<true>(y, x0, x1)
                }
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            pub const fn line_au<const YX: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<LineAu<YX, $UI>> {
                let Some((u0, u1, su)) = (if !YX {
                    self.raw_line_ax(v, u0, u1)
                } else {
                    self.yx().raw_line_ax(v, u0, u1)
                }) else {
                    return None;
                };
                Some(LineAu { u0, u1, v, su })
            }

            pub const fn line_ax(&self, y: $UI, x0: $UI, x1: $UI) -> Option<LineAx<$UI>> {
                self.line_au(y, x0, x1)
            }

            pub const fn line_ay(&self, x: $UI, y0: $UI, y1: $UI) -> Option<LineAy<$UI>> {
                self.line_au(x, y0, y1)
            }

            pub const fn line_a(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineA<$UI>> {
                if y0 == y1 {
                    let Some(line) = self.line_ax(y0, x0, x1) else { return None };
                    Some(LineA::Ax(line))
                } else if x0 == x1 {
                    let Some(line) = self.line_ay(x0, y0, y1) else { return None };
                    Some(LineA::Ay(line))
                } else {
                    None
                }
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            pub const fn line_au_proj<const YX: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<LineAu<YX, $U>> {
                let Some((v, u0, u1, su)) = (if !YX {
                    self.raw_line_ax_proj(v, u0, u1)
                } else {
                    self.yx().raw_line_ax_proj(v, u0, u1)
                }) else {
                    return None;
                };
                Some(LineAu { u0, u1, v, su })
            }

            pub const fn line_ax_proj(&self, y: $UI, x0: $UI, x1: $UI) -> Option<LineAx<$U>> {
                self.line_au_proj(y, x0, x1)
            }

            pub const fn line_ay_proj(&self, x: $UI, y0: $UI, y1: $UI) -> Option<LineAy<$U>> {
                self.line_au_proj(x, y0, y1)
            }

            pub const fn line_a_proj(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineA<$U>> {
                if y0 == y1 {
                    let Some(line) = self.line_ax_proj(y0, x0, x1) else { return None };
                    Some(LineA::Ax(line))
                } else if x0 == x1 {
                    let Some(line) = self.line_ay_proj(x0, y0, y1) else { return None };
                    Some(LineA::Ay(line))
                } else {
                    None
                }
            }
        }
    };
}

clip_line_a!(u8 | i8);
clip_line_a!(u16 | i16);
clip_line_a!(u32 | i32);
clip_line_a!(u64 | i64);
clip_line_a!(usize | isize);
