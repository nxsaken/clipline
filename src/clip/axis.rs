use crate::axis::{Axis, Axis0, Axis1};
use crate::clip::Clip;
use crate::math::{C, S};

impl Clip {
    /// Clips a half-open axis-aligned line segment to this region.
    ///
    /// Returns an [`Axis<V>`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    ///
    /// `V` determines the orientation of the line segment:
    /// - `false`: horizontal, from `(u0, v)` to `(u1, v)`.
    /// - `true`: vertical, from `(v, u0)` to `(v, u1)`.
    #[expect(clippy::similar_names)]
    #[inline]
    #[must_use]
    pub const fn axis<const V: bool>(&self, v: C, u0: C, u1: C) -> Option<Axis<V>> {
        let (wv0, wv1) = if V { (self.x0, self.x1) } else { (self.y0, self.y1) };
        let (wu0, wu1) = if V { (self.y0, self.y1) } else { (self.x0, self.x1) };
        if v < wv0 || wv1 < v {
            return None;
        }
        let (su, cu0, cu1) = if u0 <= u1 {
            if u1 <= wu0 || wu1 < u0 {
                return None;
            }
            let cu0 = if u0 < wu0 { wu0 } else { u0 };
            let cu1 = if wu1 < u1 { wu1.wrapping_add(1) } else { u1 };
            (S::P, cu0, cu1)
        } else {
            if u0 < wu0 || wu1 <= u1 {
                return None;
            }
            let cu0 = if wu1 < u0 { wu1 } else { u0 };
            let cu1 = if u1 < wu0 { wu0.wrapping_sub(1) } else { u1 };
            (S::N, cu0, cu1)
        };
        // SAFETY: su matches the direction from cu0 to cu1.
        let axis = unsafe { Axis::new_unchecked(v, cu0, cu1, su) };
        Some(axis)
    }

    /// Clips a half-open horizontal line segment to this region.
    ///
    /// Returns an [`Axis0`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    #[inline]
    #[must_use]
    pub const fn axis_0(&self, y: C, x0: C, x1: C) -> Option<Axis0> {
        self.axis::<false>(y, x0, x1)
    }

    /// Clips a half-open vertical line segment to this region.
    ///
    /// Returns an [`Axis1`] over the portion of the segment inside this
    /// clipping region, or [`None`] if the segment lies fully outside.
    #[inline]
    #[must_use]
    pub const fn axis_1(&self, x: C, y0: C, y1: C) -> Option<Axis1> {
        self.axis::<true>(x, y0, y1)
    }
}

#[cfg(test)]
mod tests {
    use crate::axis::Axis;
    use crate::clip::Clip;
    use crate::math::{C, S};

    const CLIP: Clip = Clip::from_min_max((-64, -48), (63, 47)).unwrap();

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

        [S::P, S::N]
            .map(|su| [thread::spawn(test_axis::<false>(su)), thread::spawn(test_axis::<true>(su))])
            .into_iter()
            .flatten()
            .try_for_each(JoinHandle::join)
            .unwrap();
    }
}
