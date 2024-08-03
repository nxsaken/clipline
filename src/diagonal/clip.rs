//! ### Diagonal clipping
//!
//! This module provides [clipping](Clip) utilities for
//! [diagonal](crate::Diagonal) directed line segments.

use crate::clip::Clip;
use crate::diagonal::Quadrant;
use crate::math::{Delta, Math, Num, Point};
use crate::symmetry::{fx, fy};

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

#[allow(non_snake_case)]
impl<const FX: bool, const FY: bool> Quadrant<i8, FX, FY> {
    #[inline(always)]
    #[must_use]
    const fn trivial_reject(
        (x1, y1): Point<i8>,
        (x2, y2): Point<i8>,
        Clip { wx1, wy1, wx2, wy2 }: Clip<i8>,
    ) -> bool {
        fx!(x2 < wx1 || wx2 <= x1, x1 < wx1 || wx2 <= x2)
            || fy!(y2 < wy1 || wy2 <= y1, y1 < wy1 || wy2 <= y2)
    }

    #[inline(always)]
    #[must_use]
    const fn enters_x(x1: i8, Clip { wx1, wx2, .. }: Clip<i8>) -> bool {
        fx!(x1 < wx1, wx2 < x1)
    }

    #[inline(always)]
    #[must_use]
    const fn enters_y(y1: i8, Clip { wy1, wy2, .. }: Clip<i8>) -> bool {
        fy!(y1 < wy1, wy2 < y1)
    }

    #[inline(always)]
    #[must_use]
    const fn exits_x(x2: i8, Clip { wx1, wx2, .. }: Clip<i8>) -> bool {
        fx!(wx2 < x2, x2 < wx1)
    }

    #[inline(always)]
    #[must_use]
    const fn exits_y(y2: i8, Clip { wy1, wy2, .. }: Clip<i8>) -> bool {
        fy!(wy2 < y2, y2 < wy1)
    }

    #[inline(always)]
    #[must_use]
    const fn Dx1(x1: i8, Clip { wx1, wx2, .. }: Clip<i8>) -> <i8 as Num>::U {
        fx!(Math::<i8>::delta(wx1, x1), Math::<i8>::delta(x1, wx2))
    }

    #[inline(always)]
    #[must_use]
    const fn Dx2(x1: i8, Clip { wx1, wx2, .. }: Clip<i8>) -> <i8 as Num>::U {
        fx!(Math::<i8>::delta(wx2, x1), Math::<i8>::delta(x1, wx1))
    }

    #[inline(always)]
    #[must_use]
    const fn Dy1(y1: i8, Clip { wy1, wy2, .. }: Clip<i8>) -> <i8 as Num>::U {
        fy!(Math::<i8>::delta(wy1, y1), Math::<i8>::delta(y1, wy2))
    }

    #[inline(always)]
    #[must_use]
    const fn Dy2(y1: i8, Clip { wy1, wy2, .. }: Clip<i8>) -> <i8 as Num>::U {
        fy!(Math::<i8>::delta(wy2, y1), Math::<i8>::delta(y1, wy1))
    }

    #[inline(always)]
    #[must_use]
    const fn c1_x(y1: i8, Dx1: <i8 as Num>::U, Clip { wx1, wx2, .. }: Clip<i8>) -> Point<i8> {
        let cx1 = fx!(wx1, wx2);
        let cy1 = fy!(y1.wrapping_add_unsigned(Dx1), y1.wrapping_sub_unsigned(Dx1));
        (cx1, cy1)
    }

    #[inline(always)]
    #[must_use]
    const fn c1_y(x1: i8, Dy1: <i8 as Num>::U, Clip { wy1, wy2, .. }: Clip<i8>) -> Point<i8> {
        let cy1 = fy!(wy1, wy2);
        let cx1 = fx!(x1.wrapping_add_unsigned(Dy1), x1.wrapping_sub_unsigned(Dy1));
        (cx1, cy1)
    }

    #[inline(always)]
    #[must_use]
    const fn c1((x1, y1): Point<i8>, (Dx1, Dy1): Delta<i8>, clip: Clip<i8>) -> Point<i8> {
        if Dy1 < Dx1 {
            Self::c1_x(y1, Dx1, clip)
        } else {
            Self::c1_y(x1, Dy1, clip)
        }
    }

    #[inline(always)]
    #[must_use]
    const fn c2_x(Clip { wx1, wx2, .. }: Clip<i8>) -> i8 {
        fx!(wx2.wrapping_add(1), wx1.wrapping_sub(1))
    }

    #[inline(always)]
    #[must_use]
    const fn c2_y(x1: i8, Dy2: <i8 as Num>::U) -> i8 {
        fx!(
            x1.wrapping_add_unsigned(Dy2).wrapping_add(1),
            x1.wrapping_sub_unsigned(Dy2).wrapping_sub(1)
        )
    }

    #[inline(always)]
    #[must_use]
    const fn c2(x1: i8, (Dx2, Dy2): Delta<i8>, clip: Clip<i8>) -> i8 {
        if Dx2 < Dy2 {
            Self::c2_x(clip)
        } else {
            Self::c2_y(x1, Dy2)
        }
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn clip_inner(
        (x1, y1): Point<i8>,
        (x2, y2): Point<i8>,
        clip: Clip<i8>,
    ) -> Option<Self> {
        if Self::trivial_reject((x1, y1), (x2, y2), clip) {
            return None;
        }
        let (c1, cx2) = match (
            Self::enters_x(x1, clip),
            Self::enters_y(y1, clip),
            Self::exits_x(x2, clip),
            Self::exits_y(y2, clip),
        ) {
            INSIDE_INSIDE => ((x1, y1), x2),
            INSIDE_Y_EXIT => ((x1, y1), Self::c2_y(x1, Self::Dy2(y1, clip))),
            INSIDE_X_EXIT => ((x1, y1), Self::c2_x(clip)),
            INSIDE_XY_EXIT => {
                ((x1, y1), Self::c2(x1, (Self::Dx2(x1, clip), Self::Dy2(y1, clip)), clip))
            }
            Y_ENTRY_INSIDE => (Self::c1_y(x1, Self::Dy1(y1, clip), clip), x2),
            Y_ENTRY_Y_EXIT => {
                (Self::c1_y(x1, Self::Dy1(y1, clip), clip), Self::c2_y(x1, Self::Dy2(y1, clip)))
            }
            Y_ENTRY_X_EXIT => {
                let Dy1 = Self::Dy1(y1, clip);
                let Dx2 = Self::Dx2(x1, clip);
                if Dx2 < Dy1 {
                    return None;
                }
                (Self::c1_y(x1, Dy1, clip), Self::c2_x(clip))
            }
            Y_ENTRY_XY_EXIT => {
                let Dy1 = Self::Dy1(y1, clip);
                let Dx2 = Self::Dx2(x1, clip);
                if Dx2 < Dy1 {
                    return None;
                }
                (Self::c1_y(x1, Dy1, clip), Self::c2(x1, (Dx2, Self::Dy2(y1, clip)), clip))
            }
            X_ENTRY_INSIDE => (Self::c1_x(y1, Self::Dx1(x1, clip), clip), x2),
            X_ENTRY_Y_EXIT => {
                (Self::c1_x(y1, Self::Dx1(x1, clip), clip), Self::c2_y(x1, Self::Dy2(y1, clip)))
            }
            X_ENTRY_X_EXIT => (Self::c1_x(y1, Self::Dx1(x1, clip), clip), Self::c2_x(clip)),
            X_ENTRY_XY_EXIT => {
                let Dx1 = Self::Dx1(x1, clip);
                let Dy2 = Self::Dy2(y1, clip);
                if Dy2 < Dx1 {
                    return None;
                }
                (Self::c1_x(y1, Dx1, clip), Self::c2(x1, (Self::Dx2(x1, clip), Dy2), clip))
            }
            XY_ENTRY_INSIDE => {
                (Self::c1((x1, y1), (Self::Dx1(x1, clip), Self::Dy1(y1, clip)), clip), x2)
            }
            XY_ENTRY_Y_EXIT => (
                Self::c1((x1, y1), (Self::Dx1(x1, clip), Self::Dy1(y1, clip)), clip),
                Self::c2_y(x1, Self::Dy2(y1, clip)),
            ),
            XY_ENTRY_X_EXIT => (
                Self::c1((x1, y1), (Self::Dx1(x1, clip), Self::Dy1(y1, clip)), clip),
                Self::c2_x(clip),
            ),
            XY_ENTRY_XY_EXIT => {
                let Dy1 = Self::Dy1(y1, clip);
                let Dx2 = Self::Dx2(x1, clip);
                if Dx2 < Dy1 {
                    return None;
                }
                let Dx1 = Self::Dx1(x1, clip);
                let Dy2 = Self::Dy2(y1, clip);
                if Dy2 < Dx1 {
                    return None;
                }
                (Self::c1((x1, y1), (Dx1, Dy1), clip), Self::c2(x1, (Dx2, Dy2), clip))
            }
        };
        Some(Self::new_inner(c1, cx2))
    }
}
