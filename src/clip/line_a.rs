use crate::clip::{Clip, Viewport};
use crate::line_a::{LineA, LineAu, LineAx, LineAy};
use crate::math::ops;
use crate::util::try_opt;

macro_rules! clip_line_a {
    ($U:ty | $I:ty) => {
        clip_line_a!(@impl Clip<$U>);
        clip_line_a!(@impl Clip<$I>);
        clip_line_a!(@impl Viewport<$U>);
        clip_line_a!(@impl Viewport<$I>);

        clip_line_a!(@pub impl Clip<$U>);
        clip_line_a!(@pub impl Clip<$I>);
        clip_line_a!(@pub impl Viewport<$U>);
        clip_line_a!(@pub impl Viewport<$I>);

        clip_line_a!(@impl Clip<$I, proj $U>);
        clip_line_a!(@impl Viewport<$U, proj $U>);
        clip_line_a!(@impl Viewport<$I, proj $U>);

        clip_line_a!(@pub impl Clip<$I, proj $U>);
        clip_line_a!(@pub impl Viewport<$U, proj $U>);
        clip_line_a!(@pub impl Viewport<$I, proj $U>);
    };
    (@impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            pub(super) const fn raw_line_au_fu<const YX: bool, const FU: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<($UI, $UI, i8)> {
                let (u_min, v_min, u_max, v_max) = self.uv_min_max::<YX>();
                let reject_v = v < v_min || v_max < v;
                if FU {
                    if reject_v || u0 < u_min || u_max <= u1 {
                        return None;
                    }
                    let cu0 = ops::<$UI>::min(u0, u_max);
                    // fixme: projection breaks when cu1 = u_min - 1
                    //  if u1 < u_min { u_min - 1 } else { u1 }
                    //  u1_proj = if u1 < u_min { u_min - 1 - u_min } else { u1 - u_min }
                    //  u1_proj = if u1 < u_min { -1 (doesn't fit into uN) } else { u1 - u_min }
                    //  try wrapping? – tried, it works???
                    let cu1 = ops::<$UI>::max_adj(u_min, u1);
                    Some((cu0, cu1, -1))
                } else {
                    if reject_v || u1 <= u_min || u_max < u0 {
                        return None;
                    }
                    let cu0 = ops::<$UI>::max(u0, u_min);
                    let cu1 = ops::<$UI>::min_adj(u_max, u1);
                    Some((cu0, cu1, 1))
                }
            }

            const fn raw_line_au<const YX: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<($UI, $UI, i8)> {
                if u0 <= u1 {
                    self.raw_line_au_fu::<YX, false>(v, u0, u1)
                } else {
                    self.raw_line_au_fu::<YX, true>(v, u0, u1)
                }
            }
        }
    };
    (@impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            pub(super) const fn raw_line_au_fu_proj<const YX: bool, const FU: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<($U, $U, $U, i8)> {
                let (u0, u1, su) = try_opt!(self.raw_line_au_fu::<YX, FU>(v, u0, u1));
                let v = ops::<$UI>::proj(v, self.v_min::<YX>());
                let u0 = ops::<$UI>::proj(u0, self.u_min::<YX>());
                let u1 = ops::<$UI>::proj(u1, self.u_min::<YX>());
                Some((v, u0, u1, su))
            }

            const fn raw_line_au_proj<const YX: bool>(
                &self,
                v: $UI,
                u0: $UI,
                u1: $UI,
            ) -> Option<($U, $U, $U, i8)> {
                let (u0, u1, su) = try_opt!(self.raw_line_au::<YX>(v, u0, u1));
                let v = ops::<$UI>::proj(v, self.v_min::<YX>());
                let u0 = ops::<$UI>::proj(u0, self.u_min::<YX>());
                let u1 = ops::<$UI>::proj(u1, self.u_min::<YX>());
                Some((v, u0, u1, su))
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
                let (u0, u1, su) = try_opt!(self.raw_line_au::<YX>(v, u0, u1));
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
                    let line = try_opt!(self.line_ax(y0, x0, x1));
                    Some(LineA::Ax(line))
                } else if x0 == x1 {
                    let line = try_opt!(self.line_ay(x0, y0, y1));
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
                let (v, u0, u1, su) = try_opt!(self.raw_line_au_proj::<YX>(v, u0, u1));
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
                    let line = try_opt!(self.line_ax_proj(y0, x0, x1));
                    Some(LineA::Ax(line))
                } else if x0 == x1 {
                    let line = try_opt!(self.line_ay_proj(x0, y0, y1));
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
