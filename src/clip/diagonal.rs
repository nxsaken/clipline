use crate::clip::Clip;
use crate::diagonal::Diagonal;
use crate::math::{ops, CxC, C, S, U};

const O: bool = false;
const I: bool = true;

type Code = (bool, bool, bool, bool);

const L0000: Code = (O, O, O, O);
const L0001: Code = (O, O, O, I);
const L0010: Code = (O, O, I, O);
const L0011: Code = (O, O, I, I);
const L0100: Code = (O, I, O, O);
const L0101: Code = (O, I, O, I);
const L0110: Code = (O, I, I, O);
const L0111: Code = (O, I, I, I);
const L1000: Code = (I, O, O, O);
const L1001: Code = (I, O, O, I);
const L1010: Code = (I, O, I, O);
const L1011: Code = (I, O, I, I);
const L1100: Code = (I, I, O, O);
const L1101: Code = (I, I, O, I);
const L1110: Code = (I, I, I, O);
const L1111: Code = (I, I, I, I);

impl Clip {
    /// Checks if `x0` of the line segment lies before the x-entry
    /// of the region, and if `x1` lies after the x-exit.
    ///
    /// # Safety
    ///
    /// `sx` must match the direction from `x0` to `x1`.
    const unsafe fn maybe_iox(&self, x0: C, x1: C, sx: S) -> (bool, bool) {
        // SAFETY:
        // - self.x0 <= self.x1.
        // - sx matches the direction from x0 to x1.
        unsafe { Self::maybe_iou(self.x0, self.x1, x0, x1, sx) }
    }

    /// Checks if `y0` of the line segment lies before the y-entry
    /// of the region, and if `y1` lies after the y-exit.
    ///
    /// # Safety
    ///
    /// `sy` must match the direction from `y0` to `y1`.
    const unsafe fn maybe_ioy(&self, y0: C, y1: C, sy: S) -> (bool, bool) {
        // SAFETY:
        // - self.y0 <= self.y1.
        // - sy matches the direction from y0 to y1.
        unsafe { Self::maybe_iou(self.y0, self.y1, y0, y1, sy) }
    }

    /// Returns the offset between `x0` of the line segment
    /// and the x-entry of this clipping region.
    ///
    /// # Safety
    ///
    /// `x0` must lie before the x-entry.
    const unsafe fn dx0(&self, x0: C, sx: S) -> U {
        // SAFETY:
        // - self.x0 <= self.x1.
        // - x0 lies before the x-entry.
        unsafe { Self::du0(self.x0, self.x1, x0, sx) }
    }

    /// Returns the offset between `y0` of the line segment
    /// and the y-entry of this clipping region.
    ///
    /// # Safety
    ///
    /// `y0` must lie before the y-entry.
    const unsafe fn dy0(&self, y0: C, sy: S) -> U {
        // SAFETY:
        // - self.y0 <= self.y1.
        // - y0 lies before the y-entry.
        unsafe { Self::du0(self.y0, self.y1, y0, sy) }
    }

    /// Returns the offset between `x0` of the line segment
    /// and the x-exit of this clipping region.
    ///
    /// # Safety
    ///
    /// `x0` must lie before the x-exit.
    const unsafe fn dx1(&self, x0: C, sx: S) -> U {
        // SAFETY:
        // - self.x0 <= self.x1.
        // - x0 lies before the x-exit.
        unsafe { Self::du1(self.x0, self.x1, x0, sx) }
    }

    /// Returns the offset between `y0` of the line segment
    /// and the y-exit of this clipping region.
    ///
    /// # Safety
    ///
    /// `y0` must lie before the y-exit.
    const unsafe fn dy1(&self, y0: C, sy: S) -> U {
        // SAFETY:
        // - self.y0 <= self.y1.
        // - y0 lies before the y-exit.
        unsafe { Self::du1(self.y0, self.y1, y0, sy) }
    }

    /// Returns the clipped start point of the line segment
    /// when it crosses the u-entry of this clipping region.
    ///
    /// Crossing the u-entry is a stronger condition than starting before
    /// and ending after the u-entry. It is possible for a line segment
    /// to satisfy the latter while not satisfying the former.
    ///
    /// The returned point is relative to the u-axis
    /// and needs to be transposed accordingly.
    ///
    /// # Safety
    ///
    /// - `wu0 <= wu1`.
    /// - the segment must cross the u-entry.
    #[expect(clippy::similar_names)]
    const unsafe fn c0_iu(wu0: C, wu1: C, v0: C, du0: U, su: S, sv: S) -> CxC {
        let cu0 = match su {
            S::P => wu0,
            S::N => wu1,
        };
        let cv0 = match sv {
            // SAFETY:
            // -----+--+ Let A = (u0, v0), B = (wu0, cv0), C = (wu0, v0).
            //      B  | Consider the triangle ABC.
            //     /|  | ∠CAB = 45° => |BC| = |AC|.
            // ---/-+--+ |AC| = wu0 - u0 = du0.
            //   /  |  | cv0 = C.y + |BC| = v0 + du0.
            //  A---C  | cv0 <= wv1 => v0 + du0 <= wv1.
            //      |  | Therefore, v0 + du0 cannot overflow.
            S::P => unsafe { v0.checked_add_unsigned(du0).unwrap_unchecked() },
            // SAFETY:
            // |  |      Let A = (u0, v0), B = (wu1, cv0), C = (wu1, v0).
            // |  C---A  Consider the triangle ABC.
            // |  |  /   ∠CAB = 45° => |BC| = |AC|.
            // +--+-/--- |AC| = u0 - wu1 = du0.
            // |  |/     cv0 = C.y - |BC| = v0 - du0.
            // |  B      wv0 <= cv0 => wv0 <= v0 - du0.
            // +--+----- Therefore, v0 - du0 cannot underflow.
            S::N => unsafe { v0.checked_sub_unsigned(du0).unwrap_unchecked() },
        };
        (cu0, cv0)
    }

    /// Shortcut for [`Self::c0_iu`], with `u = x`.
    #[expect(clippy::similar_names)]
    const unsafe fn c0_ix(&self, y0: C, dx0: U, sx: S, sy: S) -> CxC {
        // SAFETY:
        // - self.x0 <= self.x1.
        // - the segment crosses the x-entry.
        let (cu0, cv0) = unsafe { Self::c0_iu(self.x0, self.x1, y0, dx0, sx, sy) };
        (cu0, cv0)
    }

    /// Shortcut for [`Self::c0_iu`], with `u = y`.
    #[expect(clippy::similar_names)]
    const unsafe fn c0_iy(&self, x0: C, dy0: U, sy: S, sx: S) -> CxC {
        // SAFETY:
        // - self.y0 <= self.y1.
        // - the segment crosses the y-entry.
        let (cu0, cv0) = unsafe { Self::c0_iu(self.y0, self.y1, x0, dy0, sy, sx) };
        (cv0, cu0)
    }

    /// Returns the clipped start point of the line segment
    /// when it crosses the x-entry or y-entry of this clipping region.
    ///
    /// If the exact entry is known, use [`Self::c0_iu`] instead.
    ///
    /// # Safety
    ///
    /// The segment must cross the x-entry or y-entry.
    #[expect(clippy::similar_names)]
    const unsafe fn c0_ixy(&self, x0: C, y0: C, dx0: U, dy0: U, sx: S, sy: S) -> CxC {
        if dy0 <= dx0 {
            // SAFETY:
            // ----+---+ Let A = (x0, y0), B = (x0, wy0), C = (wx0, wy0), D = (wx0, y0).
            //     |   | Suppose the segment AP crosses the y-entry at P = (cx0, wy0).
            // -B--C-P-+ Then, |CP| > 0. ∠PAB = 45° => |AB| = |BP|.
            //  |  |/  | Consider the rectangle ABCD:
            //  |  /   | |AB| = wy0 - y0 = dy0. |BC| = wx0 - x0 = dx0.
            //  | /|   | dy0 <= dx0 => |AB| <= |BC|.
            //  |/ |   | |BC| + |CP| = |BP| = |AB|. Since |CP| > 0, |BC| < |AB|.
            //  A--D   | |AB| <= |BC| and |BC| < |AB| is a contradiction.
            //     |   | Therefore, the segment does not cross the y-entry.
            //     |   | Since it must cross the x-entry or y-entry, it crosses the x-entry.
            unsafe { self.c0_ix(y0, dx0, sx, sy) }
        } else {
            // SAFETY: the segment crosses the y-entry.
            // The proof is symmetrical to the other case.
            unsafe { self.c0_iy(x0, dy0, sy, sx) }
        }
    }

    /// Returns the clipped `cx1` coordinate of the line segment
    /// when it crosses the x-exit of the region.
    ///
    /// # Safety
    ///
    /// The segment must cross the x-exit.
    const unsafe fn cx1_ox(&self, sx: S) -> C {
        match sx {
            // SAFETY: wx1 + 1 cannot overflow because crossing the x-exit implies wx1 < x1.
            S::P => unsafe { self.x1.unchecked_add(1) },
            // SAFETY: wx0 - 1 cannot underflow because crossing the x-exit implies x1 < wx0.
            S::N => unsafe { self.x0.unchecked_sub(1) },
        }
    }

    /// Returns the clipped `cx1` coordinate of the line segment
    /// when it crosses the y-exit of the region.
    ///
    /// # Safety
    ///
    /// The segment must cross the y-exit.
    const unsafe fn cx1_oy(x0: C, dy1: U, sx: S) -> C {
        // SAFETY:
        // Crossing the y-exit implies wy1 < y1.
        // Subtract y0 from both sides => wy1 - y0 < y1 - y0 => dy1 < dy.
        // dy1 + 1 cannot overflow because dy1 < dy.
        let dy1_inc = unsafe { dy1.unchecked_add(1) };
        match sx {
            // SAFETY:
            // Crossing the y-exit: dy1 < dy;
            // Add 1: dy1 + 1 < dy + 1;
            // Add x0: x0 + dy1 + 1 < x0 + dy + 1;
            // Replace dy = dx = x1 - x0:  x0 + dy1 + 1 < x0 + (x1 - x0) + 1;
            // Simplify: x0 + dy1 < x1;
            // x0 + dy1 + 1 cannot overflow because x0 + dy1 < x1.
            S::P => unsafe { x0.checked_add_unsigned(dy1_inc).unwrap_unchecked() },
            // SAFETY:
            // dy1 + 1 < dy + 1;
            // Negate both sides: -(dy + 1) < -(dy1 + 1);
            // Add x0: x0 - (dy + 1) < x0 - (dy1 + 1);
            // Replace dy = dx = x0 - x1: x0 - (x0 - x1 + 1) < x0 - (dy1 + 1);
            // Simplify: x1 < x0 - dy1;
            // x0 - dy1 - 1 cannot overflow because x0 + dy1 < x1.
            S::N => unsafe { x0.checked_sub_unsigned(dy1_inc).unwrap_unchecked() },
        }
    }

    /// Returns the clipped `cx1` coordinate of the line segment
    /// when it crosses the x-exit or y-exit of the region.
    ///
    /// If the exact exit is known, use [`Self::cx1_ox`] or [`Self::cx1_oy`] instead.
    ///
    /// # Safety
    ///
    /// The segment must cross the x-exit or y-exit.
    #[expect(clippy::similar_names)]
    const unsafe fn cx1_oxy(&self, x0: C, dx1: U, dy1: U, sx: S) -> C {
        if dx1 <= dy1 {
            // |     | /
            // +-----+/---
            // |     #
            // |    /|
            // +---/-+----
            // SAFETY: the segment crosses the x-exit. Proof similar to the one in c0_ixy.
            unsafe { self.cx1_ox(sx) }
        } else {
            // |  /  |
            // +-#---+----
            // |/    |
            // /     |
            // +-----+----
            // SAFETY: the segment crosses the y-exit. Proof similar to the one in c0_ixy.
            unsafe { Self::cx1_oy(x0, dy1, sx) }
        }
    }

    /// Clips a half-open diagonal line segment to this region.
    ///
    /// Returns a [`Diagonal`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment is not diagonal or is fully outside.
    #[expect(clippy::similar_names)]
    #[expect(clippy::too_many_lines)]
    #[inline]
    #[must_use]
    pub const fn diagonal(&self, (x0, y0): CxC, (x1, y1): CxC) -> Option<Diagonal> {
        let (sx, dx) = ops::sd(x0, x1);
        let (sy, dy) = ops::sd(y0, y1);
        if dx != dy {
            return None;
        }
        // SAFETY: sx and sy match the directions from x0 to x1 and from y0 to y1.
        if unsafe { self.rejects_bbox((x0, y0), (x1, y1), (sx, sy)) } {
            return None;
        }
        // SAFETY: sx matches the direction from x0 to x1.
        let (maybe_ix, maybe_ox) = unsafe { self.maybe_iox(x0, x1, sx) };
        // SAFETY: sy matches the direction from y0 to y1.
        let (maybe_iy, maybe_oy) = unsafe { self.maybe_ioy(y0, y1, sy) };
        //    |   | 1  [0] segment start
        // -/-+-#-+--- [1] segment end
        //    @   #    [@] possible entry (left – `x`, bottom – `y`)
        // ---+-@-+-/- [#] possible exit (right – `x`, top –`y`)
        //  0 |   |    [/] possible miss
        let (cx0, cy0, cx1) = match (maybe_ix, maybe_iy, maybe_ox, maybe_oy) {
            L0000 => {
                //    |   |
                // ---+---+---
                //    |0 1|
                // ---+---+---
                //    |   |
                (x0, y0, x1)
            }
            L0001 => {
                //    | 1 |
                // ---+-#-+---
                //    | 0 |
                // ---+---+---
                //    |   |
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                // SAFETY: the segment crosses the y-exit.
                let cx1 = unsafe { Self::cx1_oy(x0, dy1, sx) };
                (x0, y0, cx1)
            }
            L0010 => {
                //    |   |
                // ---+---+---
                //    | 0 # 1
                // ---+---+---
                //    |   |
                // SAFETY: the segment crosses the x-exit.
                let cx1 = unsafe { self.cx1_ox(sx) };
                (x0, y0, cx1)
            }
            L0011 => {
                //    |   | 1
                // ---+-#-+---
                //    | 0 #
                // ---+---+---
                //    |   |
                // SAFETY: x0 lies before the x-exit.
                let dx1 = unsafe { self.dx1(x0, sx) };
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                // SAFETY: the segment crosses the x-exit or y-exit.
                let cx1 = unsafe { self.cx1_oxy(x0, dx1, dy1, sx) };
                (x0, y0, cx1)
            }
            L0100 => {
                //    |   |
                // ---+---+---
                //    | 1 |
                // ---+-@-+---
                //    | 0 |
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: the segment crosses the y-entry.
                let (cx0, cy0) = unsafe { self.c0_iy(x0, dy0, sy, sx) };
                (cx0, cy0, x1)
            }
            L0101 => {
                //    | 1 |
                // ---+-#-+---
                //    |   |
                // ---+-@-+---
                //    | 0 |
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: the segment crosses the y-entry.
                let (cx0, cy0) = unsafe { self.c0_iy(x0, dy0, sy, sx) };
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                // SAFETY: the segment crosses the the y-exit.
                let cx1 = unsafe { Self::cx1_oy(x0, dy1, sx) };
                (cx0, cy0, cx1)
            }
            L0110 => {
                //    |   |
                // ---+---+---
                //    |   # 1
                // ---+-@-+-/-
                //    | 0 |
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: x0 lies before the x-exit.
                let dx1 = unsafe { self.dx1(x0, sx) };
                if dx1 < dy0 {
                    // REJECT: the segment misses the bottom-right corner.
                    return None;
                }
                // SAFETY: the segment crosses the y-entry.
                let (cx0, cy0) = unsafe { self.c0_iy(x0, dy0, sy, sx) };
                // SAFETY: the segment crosses the x-exit.
                let cx1 = unsafe { self.cx1_ox(sx) };
                (cx0, cy0, cx1)
            }
            L0111 => {
                //    |   | 1
                // ---+-#-+---
                //    |   #
                // ---+-@-+-/-
                //    | 0 |
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: x0 lies before the x-exit.
                let dx1 = unsafe { self.dx1(x0, sx) };
                if dx1 < dy0 {
                    // REJECT: the segment misses the bottom-right corner.
                    return None;
                }
                // SAFETY: the segment crosses the y-entry.
                let (cx0, cy0) = unsafe { self.c0_iy(x0, dy0, sy, sx) };
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                // SAFETY: the segment crosses the x-exit or y-exit.
                let cx1 = unsafe { self.cx1_oxy(x0, dx1, dy1, sx) };
                (cx0, cy0, cx1)
            }
            L1000 => {
                //    |   |
                // ---+---+---
                //  0 @ 1 |
                // ---+---+---
                //    |   |
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: the segment crosses the x-entry.
                let (cx0, cy0) = unsafe { self.c0_ix(y0, dx0, sx, sy) };
                (cx0, cy0, x1)
            }
            L1001 => {
                //    | 1 |
                // -/-+-#-+---
                //  0 @   |
                // ---+---+---
                //    |   |
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                if dy1 < dx0 {
                    // REJECT: the segment misses the top-left corner.
                    return None;
                }
                // SAFETY: the segment crosses the x-entry.
                let (cx0, cy0) = unsafe { self.c0_ix(y0, dx0, sx, sy) };
                // SAFETY: the segment crosses the y-exit.
                let cx1 = unsafe { Self::cx1_oy(x0, dy1, sx) };
                (cx0, cy0, cx1)
            }
            L1010 => {
                //    |   |
                // ---+---+---
                //  0 @   # 1
                // ---+---+---
                //    |   |
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: the segment crosses the x-entry.
                let (cx0, cy0) = unsafe { self.c0_ix(y0, dx0, sx, sy) };
                // SAFETY: the segment crosses the x-exit.
                let cx1 = unsafe { self.cx1_ox(sx) };
                (cx0, cy0, cx1)
            }
            L1011 => {
                //    |   | 1
                // -/-+-#-+---
                //  0 @   #
                // ---+---+---
                //    |   |
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                if dy1 < dx0 {
                    // REJECT: the segment misses the top-left corner.
                    return None;
                }
                // SAFETY: the segment crosses the x-entry.
                let (cx0, cy0) = unsafe { self.c0_ix(y0, dx0, sx, sy) };
                // SAFETY: x0 lies before the x-exit.
                let dx1 = unsafe { self.dx1(x0, sx) };
                // SAFETY: the segment crosses the x-exit or y-exit.
                let cx1 = unsafe { self.cx1_oxy(x0, dx1, dy1, sx) };
                (cx0, cy0, cx1)
            }
            L1100 => {
                //    |   |
                // ---+---+---
                //    @ 1 |
                // ---+-@-+---
                //  0 |   |
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: the segment crosses the x-entry or y-entry.
                let (cx0, cy0) = unsafe { self.c0_ixy(x0, y0, dx0, dy0, sx, sy) };
                (cx0, cy0, x1)
            }
            L1101 => {
                //    | 1 |
                // -/-+-#-+---
                //    @   |
                // ---+-@-+---
                //  0 |   |
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                if dy1 < dx0 {
                    // REJECT: the segment misses the top-left corner.
                    return None;
                }
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: the segment crosses the x-entry or y-entry.
                let (cx0, cy0) = unsafe { self.c0_ixy(x0, y0, dx0, dy0, sx, sy) };
                // SAFETY: the segment crosses the y-exit.
                let cx1 = unsafe { Self::cx1_oy(x0, dy1, sx) };
                (cx0, cy0, cx1)
            }
            L1110 => {
                //    |   |
                // ---+---+---
                //    @   # 1
                // ---+-@-+-/-
                //  0 |   |
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: x0 lies before the x-exit.
                let dx1 = unsafe { self.dx1(x0, sx) };
                if dx1 < dy0 {
                    // REJECT: the segment misses the bottom-right corner.
                    return None;
                }
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: the segment crosses the x-entry or y-entry.
                let (cx0, cy0) = unsafe { self.c0_ixy(x0, y0, dx0, dy0, sx, sy) };
                // SAFETY: the segment crosses the x-exit.
                let cx1 = unsafe { self.cx1_ox(sx) };
                (cx0, cy0, cx1)
            }
            L1111 => {
                //    |   | 1
                // -/-+-#-+---
                //    @   #
                // ---+-@-+-/-
                //  0 |   |
                // SAFETY: x0 lies before the x-entry.
                let dx0 = unsafe { self.dx0(x0, sx) };
                // SAFETY: y0 lies before the y-exit.
                let dy1 = unsafe { self.dy1(y0, sy) };
                if dy1 < dx0 {
                    // REJECT: the segment misses the top-left corner.
                    return None;
                }
                // SAFETY: y0 lies before the y-entry.
                let dy0 = unsafe { self.dy0(y0, sy) };
                // SAFETY: x0 lies before the x-exit.
                let dx1 = unsafe { self.dx1(x0, sx) };
                if dx1 < dy0 {
                    // REJECT: the segment misses the bottom-right corner.
                    return None;
                }
                // SAFETY: the segment crosses the x-entry or y-entry.
                let (cx0, cy0) = unsafe { self.c0_ixy(x0, y0, dx0, dy0, sx, sy) };
                // SAFETY: the segment crosses the x-exit or y-exit.
                let cx1 = unsafe { self.cx1_oxy(x0, dx1, dy1, sx) };
                (cx0, cy0, cx1)
            }
        };
        // SAFETY: sx matches the direction from cx0 to cx1.
        let diagonal = unsafe { Diagonal::new_unchecked((cx0, cy0), cx1, (sx, sy)) };
        Some(diagonal)
    }
}

#[cfg(test)]
mod tests {
    use crate::clip::Clip;
    use crate::diagonal::Diagonal;
    use crate::math::{CxC, SxS, C, S, U};

    const CLIP: Clip = Clip::with_min_max((-64, -48), (63, 47)).unwrap();

    /// Calls `f` on all possible diagonal line segments
    /// with the directions `sx` and `sy`.
    #[expect(clippy::similar_names)]
    fn for_every((sx, sy): SxS, mut f: impl FnMut(CxC, CxC)) {
        let (sx, sy) = (sx as C, sy as C);
        for y0 in C::MIN..=C::MAX {
            for x0 in C::MIN..=C::MAX {
                let max_dx = if sx > 0 { x0.abs_diff(C::MAX) } else { x0.abs_diff(C::MIN) };
                let max_dy = if sy > 0 { y0.abs_diff(C::MAX) } else { y0.abs_diff(C::MIN) };
                let max_d = U::min(max_dx, max_dy);

                let (mut x1, mut y1) = (x0, y0);
                for _ in 0..max_d {
                    f((x0, y0), (x1, y1));
                    x1 = x1.wrapping_add(sx);
                    y1 = y1.wrapping_add(sy);
                }
            }
        }
    }

    #[test]
    fn naive_clip_matches_fast_clip() {
        extern crate std;
        use std::thread::{self, JoinHandle};

        [(S::P, S::P), (S::P, S::N), (S::N, S::P), (S::N, S::N)]
            .map(|(sx, sy)| {
                thread::spawn(move || {
                    let clip = CLIP;
                    for_every((sx, sy), |p, q| {
                        let naive = Diagonal::new(p, q).unwrap().filter(|&it| clip.point(it));
                        let fast = clip.diagonal(p, q).into_iter().flatten();
                        assert!(naive.eq(fast));
                    });
                })
            })
            .into_iter()
            .try_for_each(JoinHandle::join)
            .unwrap();
    }
}
