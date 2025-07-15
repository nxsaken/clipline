use crate::Clip;
use crate::clip::ClipV;
use crate::line_d::{LineD, LineD2};
use crate::math::Coord;

struct Impl<C: Coord>(C);

macro_rules! clip_line_d {
    ($Cu:ty | $Ci:ty) => {
        clip_line_d!(common $Cu, $Cu);
        clip_line_d!(common $Ci, $Cu);
        clip_line_d!(signed $Ci, $Cu);
    };
    (common $C:ty, $Cu:ty) => {
        impl Impl<$C> {
            const unsafe fn reject_bbox_half_open<const FX: bool, const FY: bool>(
                x_min: $C,
                y_min: $C,
                x_max: $C,
                y_max: $C,
                x0: $C,
                y0: $C,
                x1: $C,
                y1: $C,
            ) -> bool {
                !FX && (x1 <= x_min || x_max < x0)
                    || FX && (x0 < x_min || x_max <= x1)
                    || !FY && (y1 <= y_min || y_max < y0)
                    || FY && (y0 < y_min || y_max <= y1)
            }
        }

        impl Clip<$C> {
            pub const fn line_d(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD<$C>> {
                None
            }

            pub const fn line_d2(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD2<$C>> {
                None
            }
        }

        impl ClipV<$C> {
            pub const fn line_d(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD<$C>> {
                None
            }

            pub const fn line_d2(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD2<$C>> {
                None
            }

            pub const fn line_d_o(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD<$Cu>> {
                None
            }

            pub const fn line_d2_o(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD2<$Cu>> {
                None
            }
        }
    };
    (signed $C:ty, $Cu:ty) => {
        impl Clip<$C> {
            pub const fn line_d_o(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD<$Cu>> {
                None
            }

            pub const fn line_d2_o(&self, x0: $C, y0: $C, x1: $C, y1: $C) -> Option<LineD2<$Cu>> {
                None
            }
        }
    };
}

clip_line_d!(u8 | i8);
clip_line_d!(u16 | i16);
clip_line_d!(u32 | i32);
clip_line_d!(u64 | i64);
clip_line_d!(usize | isize);
