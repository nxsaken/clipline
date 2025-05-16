use crate::axis::{Axis, AxisH, AxisV};
use crate::clip::Clip;
use crate::math::{ops, C, S};

impl Clip {
    /// Clips a half-open axis-aligned line segment to this region.
    ///
    /// Returns an [`Axis<V>`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    ///
    /// `V` determines the orientation of the line segment:
    /// * `false`: horizontal, from `(u0, v)` to `(u1, v)`.
    /// * `true`: vertical, from `(v, u0)` to `(v, u1)`.
    #[expect(clippy::similar_names)]
    #[inline]
    #[must_use]
    pub const fn axis<const V: bool>(&self, v: C, u0: C, u1: C) -> Option<Axis<V>> {
        let (wv0, wv1) = if V { (self.x0, self.x1) } else { (self.y0, self.y1) };
        if v < wv0 || wv1 < v {
            return None;
        }
        let (wu0, wu1) = if V { (self.y0, self.y1) } else { (self.x0, self.x1) };
        let (su, in_0, in_1, ex_0, ex_1, iu_0, iu_1, iu, ou_0, ou_1, ou) = if u0 <= u1 {
            (S::Pos, u1, wu0, wu1, u0, u0, wu0, wu0, wu1, u1, wu1)
        } else {
            (S::Neg, wu1, u1, u0, wu0, wu1, u0, wu1, u1, wu0, wu0)
        };
        if in_0 <= in_1 || ex_0 < ex_1 {
            return None;
        }
        let cu0 = if iu_0 < iu_1 { iu } else { u0 };
        let cu1 = if ou_0 < ou_1 {
            // SAFETY:
            // su > 0 => wu1 < u1 => wu1 + 1 cannot overflow.
            // su < 0 => u1 < wu0 => wu0 - 1 cannot underflow.
            unsafe { ops::unchecked_add_sign(ou, su) }
        } else {
            u1
        };
        // SAFETY: su = sign(cu1 - cu0).
        let clipped = unsafe { Axis::new_unchecked(v, cu0, cu1, su) };
        Some(clipped)
    }

    /// Clips a half-open horizontal line segment to this region.
    ///
    /// Returns an [`AxisH`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    #[inline]
    #[must_use]
    pub const fn axis_h(&self, y: C, x0: C, x1: C) -> Option<AxisH> {
        self.axis::<false>(y, x0, x1)
    }

    /// Clips a half-open vertical line segment to this region.
    ///
    /// Returns an [`AxisV`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    #[inline]
    #[must_use]
    pub const fn axis_v(&self, x: C, y0: C, y1: C) -> Option<AxisV> {
        self.axis::<true>(x, y0, y1)
    }
}

#[cfg(test)]
mod tests {
    use crate::axis::Axis;
    use crate::clip::Clip;
    use crate::math::{C, S};

    const CLIP: Clip = Clip::with_min_max((-64, -48), (63, 47)).unwrap();

    /// Calls `f` on all possible line segments
    /// aligned to an axis with the direction `su`.
    fn for_every(su: S, mut f: impl FnMut(C, C, C)) {
        let su = su as C;
        for v in C::MIN..=C::MAX {
            for u0 in C::MIN..=C::MAX {
                let max_du = if su > 0 { u0.abs_diff(C::MAX) } else { u0.abs_diff(C::MIN) };
                let mut u1 = u0;
                for _ in 0..max_du {
                    f(v, u0, u1);
                    u1 = u1.wrapping_add(su);
                }
            }
        }
    }

    #[test]
    fn naive_clip_matches_fast_clip() {
        extern crate std;
        use std::thread::{self, JoinHandle};

        fn test_axis<const V: bool>(su: S) -> impl Fn() {
            move || {
                let clip = CLIP;
                for_every(su, |v, u0, u1| {
                    let naive = Axis::<V>::new(v, u0, u1).filter(|&it| clip.point(it));
                    let fast = clip.axis::<V>(v, u0, u1).into_iter().flatten();
                    assert!(naive.eq(fast), "naive != fast at V={V}, v={v}, u0={u0}, u1={u1}");
                });
            }
        }

        [S::Pos, S::Neg]
            .map(|su| [thread::spawn(test_axis::<false>(su)), thread::spawn(test_axis::<true>(su))])
            .into_iter()
            .flatten()
            .try_for_each(JoinHandle::join)
            .unwrap();
    }
}
