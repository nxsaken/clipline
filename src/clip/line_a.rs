use crate::clip::{Clip, Viewport, if_clip, if_clip_u};
use crate::line_a::{LineA, LineAu, LineAx, LineAy};
use crate::math::ops;

macro_rules! clip_line_a {
    ($U:ty | $I:ty) => {
        clip_line_a!(@impl Clip<unsigned $U>);
        clip_line_a!(@impl Clip<signed $I>);
        clip_line_a!(@pub impl Clip<$U>);
        clip_line_a!(@pub impl Clip<$I>);
        clip_line_a!(@pub impl Clip<$U, proj $U>);
        clip_line_a!(@pub impl Clip<$I, proj $U>);

        clip_line_a!(@impl Viewport<unsigned $U>);
        clip_line_a!(@impl Viewport<signed $I>);
        clip_line_a!(@pub impl Viewport<$U>);
        clip_line_a!(@pub impl Viewport<$I>);
        clip_line_a!(@pub impl Viewport<$U, proj $U>);
        clip_line_a!(@pub impl Viewport<$I, proj $U>);
    };
    (@impl $Clip:ident<$signess:ident $UI:ty>) => {
        impl $Clip<$UI> {
            const fn raw_line_au<const YX: bool>(&self, v: $UI, u0: $UI, u1: $UI) -> Option<($UI, $UI, i8)> {
                let (u_min, v_min, u_max, v_max) = if_clip!($Clip {
                    let Self { x_max, y_max } = *self;
                    let (u_max, v_max) = if YX { (y_max, x_max) } else { (x_max, y_max) };
                    (0, 0, u_max, v_max)
                } else {
                    let Self { x_min, y_min, x_max, y_max } = *self;
                    let (u_min, v_min) = if YX { (y_min, x_min) } else { (x_min, y_min) };
                    let (u_max, v_max) = if YX { (y_max, x_max) } else { (x_max, y_max) };
                    (u_min, v_min, u_max, v_max)
                });
                let v_lt_min = if_clip_u!($signess $Clip { _ = v_min; false } else { v < v_min });
                if v_lt_min || v_max < v {
                    return None;
                }
                if u0 <= u1 {
                    let u1_le_min = if_clip_u!($signess $Clip { u1 == u_min } else { u1 <= u_min });
                    if u1_le_min || u_max < u0 {
                        return None;
                    }
                    let cu0 = if_clip_u!($signess $Clip { u0 } else { ops::<$UI>::max(u0, u_min) });
                    let cu1 = ops::<$UI>::min_adj(u_max, u1);
                    Some((cu0, cu1, 1))
                } else {
                    let u0_lt_min = if_clip_u!($signess $Clip { false } else { u0 < u_min });
                    if u_max <= u1 || u0_lt_min {
                        return None;
                    }
                    let cu0 = ops::<$UI>::min(u0, u_max);
                    let cu1 = if_clip_u!($signess $Clip { u1 } else { ops::<$UI>::max_adj(u_min, u1) });
                    Some((cu0, cu1, -1))
                }
            }
        }
    };
    (@pub impl $Clip:ident<$UI:ty>) => {
        impl $Clip<$UI> {
            pub const fn line_au<const YX: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<LineAu<YX, $UI>> {
                let Some((u0, u1, su)) = self.raw_line_au::<YX>(v, u0, u1) else {
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
    (@pub impl $Clip:ident<$UI:ty, proj $U:ty>) => {
        impl $Clip<$UI> {
            pub const fn line_au_proj<const YX: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<LineAu<YX, $U>> {
                let Some((u0, u1, su)) = self.raw_line_au::<YX>(v, u0, u1) else {
                    return None;
                };
                let (v, u0, u1) = if_clip!($Clip {
                    let v = v as $U;
                    let u0 = u0 as $U;
                    let u1 = u1 as $U;
                    (v, u0, u1)
                } else {
                    let Self { x_min, y_min, .. } = *self;
                    let (u_min, v_min) = if YX { (y_min, x_min) } else { (x_min, y_min) };
                    let v = ops::<$UI>::abs_diff(v, v_min);
                    let u0 = ops::<$UI>::abs_diff(u0, u_min);
                    let u1 = ops::<$UI>::abs_diff(u1, u_min);
                    (v, u0, u1)
                });
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
