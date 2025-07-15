use crate::clip::{Clip, Viewport};
use crate::line_b::{LineB, LineBu, LineBx, LineBy};
use crate::math::Coord;

struct Impl<C: Coord>(C);

macro_rules! clip_line_b {
    ($U:ty | $I:ty) => {
        clip_line_b!(common $U, $U);
        clip_line_b!(common $I, $U);
        clip_line_b!(signed $I, $U);
    };
    (common $UI:ty, $U:ty) => {
        impl Impl<$UI> {
            const fn reject_bbox_closed<const FX: bool, const FY: bool>(
                x_min: $UI,
                y_min: $UI,
                x_max: $UI,
                y_max: $UI,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> bool {
                !FX && (x1 < x_min || x_max < x0)
                    || FX && (x0 < x_min || x_max < x1)
                    || !FY && (y1 < y_min || y_max < y0)
                    || FY && (y0 < y_min || y_max < y1)
            }
        }

        impl Clip<$UI> {
            pub const fn line_bu<const YX: bool>(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBu<YX, $UI>> {
                None
            }

            pub const fn line_bx(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBx<$UI>> {
                None
            }
            pub const fn line_by(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBy<$UI>> {
                None
            }

            pub const fn line_b(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineB<$UI>> {
                None
            }
        }

        impl Viewport<$UI> {
            pub const fn line_bu<const YX: bool>(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBu<YX, $UI>> {
                None
            }

            pub const fn line_bx(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBx<$UI>> {
                None
            }
            pub const fn line_by(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBy<$UI>> {
                None
            }

            pub const fn line_b(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineB<$UI>> {
                None
            }

            pub const fn line_bu_o<const YX: bool>(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBu<YX, $U>> {
                None
            }

            pub const fn line_bx_o(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBx<$U>> {
                None
            }
            pub const fn line_by_o(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineBy<$U>> {
                None
            }

            pub const fn line_b_o(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineB<$U>> {
                None
            }
        }
    };
    (signed $I:ty, $U:ty) => {
        impl Clip<$I> {
            pub const fn line_bu_o<const YX: bool>(&self, x0: $I, y0: $I, x1: $I, y1: $I) -> Option<LineBu<YX, $U>> {
                None
            }

            pub const fn line_bx_o(&self, x0: $I, y0: $I, x1: $I, y1: $I) -> Option<LineBx<$U>> {
                None
            }
            pub const fn line_by_o(&self, x0: $I, y0: $I, x1: $I, y1: $I) -> Option<LineBy<$U>> {
                None
            }

            pub const fn line_b_o(&self, x0: $I, y0: $I, x1: $I, y1: $I) -> Option<LineB<$U>> {
                None
            }
        }
    };
}

clip_line_b!(u8 | i8);
clip_line_b!(u16 | i16);
clip_line_b!(u32 | i32);
clip_line_b!(u64 | i64);
clip_line_b!(usize | isize);
