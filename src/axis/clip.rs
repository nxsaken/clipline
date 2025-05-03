//! ### Signed axis clipping

use super::SignedAxis;
use crate::clip::Clip;
use crate::macros::control_flow::return_if;
use crate::macros::derive::nums;
use crate::macros::symmetry::{f, v};
use crate::math::ops;

macro_rules! impl_clip_signed_axis {
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
                v!(
                    (u < wy1 || wy2 < u) || f!(v2 <= wx1 || wx2 < v1, v1 < wx1 || wx2 <= v2),
                    (u < wx1 || wx2 < u) || f!(v2 <= wy1 || wy2 < v1, v1 < wy1 || wy2 <= v2)
                )
            }

            #[inline(always)]
            #[must_use]
            const fn cv1(v1: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> $T {
                f! {
                    v! {
                        return_if!(v1 < wx1, wx1),
                        return_if!(v1 < wy1, wy1),
                    },
                    v! {
                        return_if!(wx2 < v1, wx2),
                        return_if!(wy2 < v1, wy2),
                    },
                };
                v1
            }

            #[inline(always)]
            #[must_use]
            const fn cv2(v2: $T, &Clip { wx1, wy1, wx2, wy2 }: &Clip<$T>) -> $T {
                f! {
                    v! {
                        return_if!(wx2 < v2, ops::<$T>::t_add_1(wx2)),
                        return_if!(wy2 < v2, ops::<$T>::t_add_1(wy2))
                    },
                    v! {
                        return_if!(v2 < wx1, ops::<$T>::t_sub_1(wx1)),
                        return_if!(v2 < wy1, ops::<$T>::t_sub_1(wy1))
                    },
                };
                v2
            }

            #[inline(always)]
            #[must_use]
            pub(super) const fn clip_inner(u: $T, v1: $T, v2: $T, clip: &Clip<$T>) -> Option<Self> {
                return_if!(Self::reject(u, v1, v2, clip));
                Some(Self::new_inner(u, Self::cv1(v1, clip), Self::cv2(v2, clip)))
            }
        }
    };
}

nums!(impl_clip_signed_axis);
