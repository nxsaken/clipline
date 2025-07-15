use crate::clip::{Clip, Viewport};
use crate::line_d::{LineD, LineD2};
use crate::math::{Coord, ops};

struct Impl<C: Coord>(C);

macro_rules! clip_line_d {
    ($U:ty | $I:ty) => {
        clip_line_d!(common $U, $U);
        clip_line_d!(common $I, $U);
        clip_line_d!(signed $I, $U);
    };
    (common $UI:ty, $U:ty) => {
        impl Impl<$UI> {
            const fn reject_bbox<const FX: bool, const FY: bool>(
                x_min: $UI,
                y_min: $UI,
                x_max: $UI,
                y_max: $UI,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> bool {
                !FX && (x1 <= x_min || x_max < x0)
                    || FX && (x0 < x_min || x_max <= x1)
                    || !FY && (y1 <= y_min || y_max < y0)
                    || FY && (y0 < y_min || y_max <= y1)
            }

            const fn line_dq<const FX: bool, const FY: bool>(
                x_min: $UI,
                y_min: $UI,
                x_max: $UI,
                y_max: $UI,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<($UI, $UI, $UI, i8, i8)> {
                let dx = ops::<$UI>::abs_diff_const_signed::<FX>(x1, x0);
                let dy = ops::<$UI>::abs_diff_const_signed::<FY>(y1, y0);
                if dx != dy || Impl::<$UI>::reject_bbox::<FX, FY>(
                    x_min, y_min, x_max, y_max,
                    x0, y0, y1, y1
                ) {
                    return None;
                }
                None
            }

            const fn line_d(
                x_min: $UI,
                y_min: $UI,
                x_max: $UI,
                y_max: $UI,
                x0: $UI,
                y0: $UI,
                x1: $UI,
                y1: $UI,
            ) -> Option<($UI, $UI, $UI, i8, i8)> {
                let fx = x1 < x0;
                let fy = y1 < y0;
                match (fx, fy) {
                    (false, false) => Self::line_dq::<false, false>(x_min, y_min, x_max, y_max, x0, y0, x1, y1),
                    (false, true) => Self::line_dq::<false, true>(x_min, y_min, x_max, y_max, x0, y0, x1, y1),
                    (true, false) => Self::line_dq::<true, false>(x_min, y_min, x_max, y_max, x0, y0, x1, y1),
                    (true, true) => Self::line_dq::<true, true>(x_min, y_min, x_max, y_max, x0, y0, x1, y1),
                }
            }
        }

        impl Clip<$UI> {
            pub const fn line_d(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$UI>> {
                None
            }

            pub const fn line_d2(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$UI>> {
                None
            }
        }

        impl Viewport<$UI> {
            pub const fn line_d(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$UI>> {
                None
            }

            pub const fn line_d2(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$UI>> {
                None
            }

            pub const fn line_d_o(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD<$U>> {
                None
            }

            pub const fn line_d2_o(&self, x0: $UI, y0: $UI, x1: $UI, y1: $UI) -> Option<LineD2<$U>> {
                None
            }
        }
    };
    (signed $I:ty, $U:ty) => {
        impl Clip<$I> {
            pub const fn line_d_o(&self, x0: $I, y0: $I, x1: $I, y1: $I) -> Option<LineD<$U>> {
                None
            }

            pub const fn line_d2_o(&self, x0: $I, y0: $I, x1: $I, y1: $I) -> Option<LineD2<$U>> {
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
