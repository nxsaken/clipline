//! ### Signed axis clipping

use super::SignedAxis;
use crate::clip::Clip;
use crate::macros::{f, hv, none_if};

macro_rules! clip_impl {
    ($T:ty) => {
        impl<const F: bool, const V: bool> SignedAxis<F, V, $T> {
            #[inline(always)]
            #[must_use]
            const fn reject(
                u: $T,
                v1: $T,
                v2: $T,
                &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>,
            ) -> bool {
                // TODO: strict comparison for closed line segments
                hv!(
                    (u < wy1 || wy2 < u) || f!(v2 <= wx1 || wx2 < v1, v1 < wx1 || wx2 <= v2),
                    (u < wx1 || wx2 < u) || f!(v2 <= wy1 || wy2 < v1, v1 < wy1 || wy2 <= v2)
                )
            }

            #[inline(always)]
            #[must_use]
            const fn cv1(v1: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> $T {
                match (F, V) {
                    (false, false) if v1 < wx1 => wx1,
                    (false, true) if v1 < wy1 => wy1,
                    (true, false) if wx2 < v1 => wx2,
                    (true, true) if wy2 < v1 => wy2,
                    _ => v1,
                }
            }

            #[inline(always)]
            #[must_use]
            const fn cv2(v2: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> $T {
                match (F, V) {
                    (false, false) if wx2 < v2 => wx2.wrapping_add(1),
                    (false, true) if wy2 < v2 => wy2.wrapping_add(1),
                    (true, false) if v2 < wx1 => wx1.wrapping_sub(1),
                    (true, true) if v2 < wy1 => wy1.wrapping_sub(1),
                    _ => v2,
                }
            }

            #[inline(always)]
            #[must_use]
            pub(super) const fn clip_inner(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                none_if!(Self::reject(u, v1, v2, clip));
                Some(Self::new_inner(u, Self::cv1(v1, clip), Self::cv2(v2, clip)))
            }
        }
    };
}

clip_impl!(i8);
clip_impl!(u8);
clip_impl!(i16);
clip_impl!(u16);
clip_impl!(i32);
clip_impl!(u32);
clip_impl!(i64);
clip_impl!(u64);
clip_impl!(isize);
clip_impl!(usize);
