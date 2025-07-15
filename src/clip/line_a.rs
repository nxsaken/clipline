use crate::clip::{Clip, ClipV};
use crate::line_a::{LineA, LineAu, LineAx, LineAy};
use crate::math::{Coord, ops};

struct Impl<C: Coord>(C);

macro_rules! clip_line_a {
    ($Cu:ty | $Ci:ty) => {
        clip_line_a!(common $Cu, $Cu);
        clip_line_a!(common $Ci, $Cu);
        clip_line_a!(signed $Ci, $Cu);
    };
    (common $C:ty, $Cu:ty) => {
        impl Impl<$C> {
            #[inline]
            const fn line_au(
                u_min: $C,
                v_min: $C,
                u_max: $C,
                v_max: $C,
                v: $C,
                u0: $C,
                u1: $C,
            ) -> Option<($C, $C, i8)> {
                if v < v_min || v_max < v {
                    return None;
                }
                if u0 <= u1 {
                    if u1 <= u_min || u_max < u0 {
                        return None;
                    }
                    let cu0 = ops::<$C>::max(u0, u_min);
                    let cu1 = ops::<$C>::min_adj(u_max, u1);
                    Some((cu0, cu1, 1))
                } else {
                    if u_max <= u1 || u0 < u_min {
                        return None;
                    }
                    let cu0 = ops::<$C>::min(u0, u_max);
                    let cu1 = ops::<$C>::max_adj(u_min, u1);
                    Some((cu0, cu1, -1))
                }
            }
        }

        impl Clip<$C> {
            pub const fn line_au<const YX: bool>(
                &self,
                v: $C,
                u0: $C,
                u1: $C,
            ) -> Option<LineAu<YX, $C>> {
                let Self { x_max, y_max } = *self;
                let (u_max, v_max) = if YX { (y_max, x_max) } else { (x_max, y_max) };
                let Some((u0, u1, su)) = Impl::<$C>::line_au(0, 0, u_max, v_max, v, u0, u1) else {
                    return None;
                };
                Some(LineAu { u0, u1, v, su })
            }

            pub const fn line_ax(&self, y: $C, x0: $C, x1: $C) -> Option<LineAx<$C>> {
                self.line_au(y, x0, x1)
            }

            pub const fn line_ay(&self, x: $C, y0: $C, y1: $C) -> Option<LineAy<$C>> {
                self.line_au(x, y0, y1)
            }

            pub const fn line_a(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineA<$C>> {
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

        impl ClipV<$C> {
            pub const fn line_au<const YX: bool>(
                &self,
                v: $C,
                u0: $C,
                u1: $C,
            ) -> Option<LineAu<YX, $C>> {
                let Self { x_min, y_min, x_max, y_max } = *self;
                let (u_min, v_min) = if YX { (y_min, x_min) } else { (x_min, y_min) };
                let (u_max, v_max) = if YX { (y_max, x_max) } else { (x_max, y_max) };
                let Some((u0, u1, su)) = Impl::<$C>::line_au(u_min, v_min, u_max, v_max, v, u0, u1)
                else {
                    return None;
                };
                Some(LineAu { u0, u1, v, su })
            }

            pub const fn line_ax(&self, y: $C, x0: $C, x1: $C) -> Option<LineAx<$C>> {
                self.line_au::<false>(y, x0, x1)
            }

            pub const fn line_ay(&self, x: $C, y0: $C, y1: $C) -> Option<LineAy<$C>> {
                self.line_au::<true>(x, y0, y1)
            }

            pub const fn line_a(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineA<$C>> {
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

            pub const fn line_au_o<const YX: bool>(
                &self,
                v: $C,
                u0: $C,
                u1: $C,
            ) -> Option<LineAu<YX, $Cu>> {
                let Self { x_min, y_min, x_max, y_max } = *self;
                let (u_min, v_min) = if YX { (y_min, x_min) } else { (x_min, y_min) };
                let (u_max, v_max) = if YX { (y_max, x_max) } else { (x_max, y_max) };
                let Some((u0, u1, su)) = Impl::<$C>::line_au(u_min, v_min, u_max, v_max, v, u0, u1)
                else {
                    return None;
                };
                let v = ops::<$C>::abs_diff(v, v_min);
                let u0 = ops::<$C>::abs_diff(u0, u_min);
                let u1 = ops::<$C>::abs_diff(u1, u_min);
                Some(LineAu { u0, u1, v, su })
            }

            pub const fn line_ax_o(&self, y: $C, x0: $C, x1: $C) -> Option<LineAx<$Cu>> {
                self.line_au_o::<false>(y, x0, x1)
            }

            pub const fn line_ay_o(&self, x: $C, y0: $C, y1: $C) -> Option<LineAy<$Cu>> {
                self.line_au_o::<true>(x, y0, y1)
            }

            pub const fn line_a_o(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineA<$Cu>> {
                if y0 == y1 {
                    let Some(line) = self.line_ax_o(y0, x0, x1) else { return None };
                    Some(LineA::Ax(line))
                } else if x0 == x1 {
                    let Some(line) = self.line_ay_o(x0, y0, y1) else { return None };
                    Some(LineA::Ay(line))
                } else {
                    None
                }
            }
        }
    };
    (signed $C:ty, $Cu:ty) => {
        impl Clip<$C> {
            pub const fn line_au_o<const YX: bool>(
                &self,
                v: $C,
                u0: $C,
                u1: $C,
            ) -> Option<LineAu<YX, $Cu>> {
                let Self { x_max, y_max } = *self;
                let (u_max, v_max) = if YX { (y_max, x_max) } else { (x_max, y_max) };
                let Some((u0, u1, su)) = Impl::<$C>::line_au(0, 0, u_max, v_max, v, u0, u1) else {
                    return None;
                };
                let v = v as $Cu;
                let u0 = u0 as $Cu;
                let u1 = u1 as $Cu;
                Some(LineAu { u0, u1, v, su })
            }

            pub const fn line_ax_o(&self, y: $C, x0: $C, x1: $C) -> Option<LineAx<$Cu>> {
                self.line_au_o(y, x0, x1)
            }

            pub const fn line_ay_o(&self, x: $C, y0: $C, y1: $C) -> Option<LineAy<$Cu>> {
                self.line_au_o(x, y0, y1)
            }

            pub const fn line_a_o(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineA<$Cu>> {
                if y0 == y1 {
                    let Some(line) = self.line_ax_o(y0, x0, x1) else { return None };
                    Some(LineA::Ax(line))
                } else if x0 == x1 {
                    let Some(line) = self.line_ay_o(x0, y0, y1) else { return None };
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
