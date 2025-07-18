use crate::clip::{Clip, Viewport};
use crate::math::ops;

macro_rules! clip_point {
    ($U:ty|$I:ty) => {
        clip_point!(@pub impl Clip<$U>);
        clip_point!(@pub impl Clip<$I>);
        clip_point!(@pub impl Viewport<$U>);
        clip_point!(@pub impl Viewport<$I>);

        clip_point!(@pub impl Clip<$I, proj $U>);
        clip_point!(@pub impl Viewport<$U, proj $U>);
        clip_point!(@pub impl Viewport<$I, proj $U>);
    };
    (@pub impl $Self:ident<$UI:ty>) => {
        impl $Self<$UI> {
            pub const fn point(&self, x: $UI, y: $UI) -> bool {
                self.x_min() <= x && x <= self.x_max && self.y_min() <= y && y <= self.y_max
            }
        }
    };
    (@pub impl $Self:ident<$UI:ty, proj $U:ty>) => {
        impl $Self<$UI> {
            pub const fn point_proj(&self, x: $UI, y: $UI) -> Option<($U, $U)> {
                if !self.point(x, y) {
                    return None;
                }
                let x = ops::<$UI>::abs_diff(x, self.x_min());
                let y = ops::<$UI>::abs_diff(y, self.y_min());
                Some((x, y))
            }
        }
    };
}

clip_point!(u8 | i8);
clip_point!(u16 | i16);
clip_point!(u32 | i32);
clip_point!(u64 | i64);
clip_point!(usize | isize);
