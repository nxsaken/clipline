//! ### Diagonal clipping

use super::Diagonal;
use crate::clip::Clip;
use crate::macros::control_flow::return_if;
use crate::macros::derive::nums;
use crate::macros::symmetry::{fx, fy};
use crate::math::{Delta, Math, Num, Point};

const O: bool = false;
const I: bool = true;

type LineCode = (bool, bool, bool, bool);

const INSIDE_INSIDE: LineCode = (O, O, O, O);
const INSIDE_Y_EXIT: LineCode = (O, O, O, I);
const INSIDE_X_EXIT: LineCode = (O, O, I, O);
const INSIDE_XY_EXIT: LineCode = (O, O, I, I);
const Y_ENTRY_INSIDE: LineCode = (O, I, O, O);
const Y_ENTRY_Y_EXIT: LineCode = (O, I, O, I);
const Y_ENTRY_X_EXIT: LineCode = (O, I, I, O);
const Y_ENTRY_XY_EXIT: LineCode = (O, I, I, I);
const X_ENTRY_INSIDE: LineCode = (I, O, O, O);
const X_ENTRY_Y_EXIT: LineCode = (I, O, O, I);
const X_ENTRY_X_EXIT: LineCode = (I, O, I, O);
const X_ENTRY_XY_EXIT: LineCode = (I, O, I, I);
const XY_ENTRY_INSIDE: LineCode = (I, I, O, O);
const XY_ENTRY_Y_EXIT: LineCode = (I, I, O, I);
const XY_ENTRY_X_EXIT: LineCode = (I, I, I, O);
const XY_ENTRY_XY_EXIT: LineCode = (I, I, I, I);

macro_rules! impl_clip_diagonal {
    ($T:ty) => {
        #[expect(non_snake_case)]
        impl<const FX: bool, const FY: bool> Diagonal<FX, FY, $T> {
            #[inline(always)]
            #[must_use]
            const fn enters_x(x1: $T, &Clip { wx1, wx2, .. }: &Clip<$T>) -> bool {
                fx!(x1 < wx1, wx2 < x1)
            }

            #[inline(always)]
            #[must_use]
            const fn enters_y(y1: $T, &Clip { wy1, wy2, .. }: &Clip<$T>) -> bool {
                fy!(y1 < wy1, wy2 < y1)
            }

            #[inline(always)]
            #[must_use]
            const fn exits_x(x2: $T, &Clip { wx1, wx2, .. }: &Clip<$T>) -> bool {
                fx!(wx2 < x2, x2 < wx1)
            }

            #[inline(always)]
            #[must_use]
            const fn exits_y(y2: $T, &Clip { wy1, wy2, .. }: &Clip<$T>) -> bool {
                fy!(wy2 < y2, y2 < wy1)
            }

            #[inline(always)]
            #[must_use]
            const fn Dx1(x1: $T, &Clip { wx1, wx2, .. }: &Clip<$T>) -> <$T as Num>::U {
                fx!(Math::<$T>::sub_tt(wx1, x1), Math::<$T>::sub_tt(x1, wx2))
            }

            #[inline(always)]
            #[must_use]
            const fn Dx2(x1: $T, &Clip { wx1, wx2, .. }: &Clip<$T>) -> <$T as Num>::U {
                fx!(Math::<$T>::sub_tt(wx2, x1), Math::<$T>::sub_tt(x1, wx1))
            }

            #[inline(always)]
            #[must_use]
            const fn Dy1(y1: $T, &Clip { wy1, wy2, .. }: &Clip<$T>) -> <$T as Num>::U {
                fy!(Math::<$T>::sub_tt(wy1, y1), Math::<$T>::sub_tt(y1, wy2))
            }

            #[inline(always)]
            #[must_use]
            const fn Dy2(y1: $T, &Clip { wy1, wy2, .. }: &Clip<$T>) -> <$T as Num>::U {
                fy!(Math::<$T>::sub_tt(wy2, y1), Math::<$T>::sub_tt(y1, wy1))
            }

            #[inline(always)]
            #[must_use]
            const fn c1_x(
                y1: $T,
                Dx1: <$T as Num>::U,
                &Clip { wx1, wx2, .. }: &Clip<$T>,
            ) -> Point<$T> {
                let cx1 = fx!(wx1, wx2);
                let cy1 = fy!(Math::<$T>::add_tu(y1, Dx1), Math::<$T>::sub_tu(y1, Dx1));
                (cx1, cy1)
            }

            #[inline(always)]
            #[must_use]
            const fn c1_y(
                x1: $T,
                Dy1: <$T as Num>::U,
                &Clip { wy1, wy2, .. }: &Clip<$T>,
            ) -> Point<$T> {
                let cy1 = fy!(wy1, wy2);
                let cx1 = fx!(Math::<$T>::add_tu(x1, Dy1), Math::<$T>::sub_tu(x1, Dy1));
                (cx1, cy1)
            }

            #[inline(always)]
            #[must_use]
            const fn c1((x1, y1): Point<$T>, (Dx1, Dy1): Delta<$T>, clip: &Clip<$T>) -> Point<$T> {
                if Dy1 < Dx1 {
                    Self::c1_x(y1, Dx1, clip)
                } else {
                    Self::c1_y(x1, Dy1, clip)
                }
            }

            #[inline(always)]
            #[must_use]
            const fn cx2_x(&Clip { wx1, wx2, .. }: &Clip<$T>) -> $T {
                fx!(wx2.wrapping_add(1), wx1.wrapping_sub(1))
            }

            #[inline(always)]
            #[must_use]
            const fn cx2_y(x1: $T, Dy2: <$T as Num>::U) -> $T {
                fx!(
                    Math::<$T>::add_tu(x1, Dy2).wrapping_add(1),
                    Math::<$T>::sub_tu(x1, Dy2).wrapping_sub(1)
                )
            }

            #[inline(always)]
            #[must_use]
            const fn cx2(x1: $T, (Dx2, Dy2): Delta<$T>, clip: &Clip<$T>) -> $T {
                if Dx2 < Dy2 {
                    Self::cx2_x(clip)
                } else {
                    Self::cx2_y(x1, Dy2)
                }
            }

            #[inline(always)]
            #[must_use]
            pub(crate) const fn clip_inner(
                (x1, y1): Point<$T>,
                (x2, y2): Point<$T>,
                clip: &Clip<$T>,
            ) -> Option<Self> {
                let (c1, cx2) = match (
                    Self::enters_x(x1, clip),
                    Self::enters_y(y1, clip),
                    Self::exits_x(x2, clip),
                    Self::exits_y(y2, clip),
                ) {
                    INSIDE_INSIDE => ((x1, y1), x2),
                    INSIDE_Y_EXIT => ((x1, y1), Self::cx2_y(x1, Self::Dy2(y1, clip))),
                    INSIDE_X_EXIT => ((x1, y1), Self::cx2_x(clip)),
                    INSIDE_XY_EXIT => {
                        ((x1, y1), Self::cx2(x1, (Self::Dx2(x1, clip), Self::Dy2(y1, clip)), clip))
                    }
                    Y_ENTRY_INSIDE => (Self::c1_y(x1, Self::Dy1(y1, clip), clip), x2),
                    Y_ENTRY_Y_EXIT => (
                        Self::c1_y(x1, Self::Dy1(y1, clip), clip),
                        Self::cx2_y(x1, Self::Dy2(y1, clip)),
                    ),
                    Y_ENTRY_X_EXIT => {
                        let Dy1 = Self::Dy1(y1, clip);
                        let Dx2 = Self::Dx2(x1, clip);
                        return_if!(Dx2 < Dy1);
                        (Self::c1_y(x1, Dy1, clip), Self::cx2_x(clip))
                    }
                    Y_ENTRY_XY_EXIT => {
                        let Dy1 = Self::Dy1(y1, clip);
                        let Dx2 = Self::Dx2(x1, clip);
                        return_if!(Dx2 < Dy1);
                        (Self::c1_y(x1, Dy1, clip), Self::cx2(x1, (Dx2, Self::Dy2(y1, clip)), clip))
                    }
                    X_ENTRY_INSIDE => (Self::c1_x(y1, Self::Dx1(x1, clip), clip), x2),
                    X_ENTRY_Y_EXIT => (
                        Self::c1_x(y1, Self::Dx1(x1, clip), clip),
                        Self::cx2_y(x1, Self::Dy2(y1, clip)),
                    ),
                    X_ENTRY_X_EXIT => {
                        (Self::c1_x(y1, Self::Dx1(x1, clip), clip), Self::cx2_x(clip))
                    }
                    X_ENTRY_XY_EXIT => {
                        let Dx1 = Self::Dx1(x1, clip);
                        let Dy2 = Self::Dy2(y1, clip);
                        return_if!(Dy2 < Dx1);
                        (Self::c1_x(y1, Dx1, clip), Self::cx2(x1, (Self::Dx2(x1, clip), Dy2), clip))
                    }
                    XY_ENTRY_INSIDE => {
                        (Self::c1((x1, y1), (Self::Dx1(x1, clip), Self::Dy1(y1, clip)), clip), x2)
                    }
                    XY_ENTRY_Y_EXIT => (
                        Self::c1((x1, y1), (Self::Dx1(x1, clip), Self::Dy1(y1, clip)), clip),
                        Self::cx2_y(x1, Self::Dy2(y1, clip)),
                    ),
                    XY_ENTRY_X_EXIT => (
                        Self::c1((x1, y1), (Self::Dx1(x1, clip), Self::Dy1(y1, clip)), clip),
                        Self::cx2_x(clip),
                    ),
                    XY_ENTRY_XY_EXIT => {
                        let Dy1 = Self::Dy1(y1, clip);
                        let Dx2 = Self::Dx2(x1, clip);
                        return_if!(Dx2 < Dy1);
                        let Dx1 = Self::Dx1(x1, clip);
                        let Dy2 = Self::Dy2(y1, clip);
                        return_if!(Dy2 < Dx1);
                        (Self::c1((x1, y1), (Dx1, Dy1), clip), Self::cx2(x1, (Dx2, Dy2), clip))
                    }
                };
                Some(Self::new_inner(c1, cx2))
            }
        }
    };
}

nums!(impl_clip_diagonal);
