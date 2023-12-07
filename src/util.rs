use core::cmp::{max, min};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Range, Rem, Sub, SubAssign};

pub type Point<T> = (T, T);

/// Standardizes the line segment (such that `x1 < x2 && y1 < y2`).
#[inline(always)]
pub fn standardize<T: Ord + Neg<Output = T> + Constant<Output = T>>(
    xy1: T,
    xy2: T,
    wxy1: T,
    wxy2: T,
) -> Option<(T, T, T, T, T)> {
    if xy1 < xy2 {
        (xy1 <= wxy2 && xy2 >= wxy1).then_some((T::ONE, xy1, xy2, wxy1, wxy2))
    } else {
        (xy2 <= wxy2 && xy1 >= wxy1).then_some((-T::ONE, -xy1, -xy2, -wxy2, -wxy1))
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn vertical_line<T>(
    x: T,
    y1: T,
    y2: T,
    wx1: T,
    wx2: T,
    wy1: T,
    wy2: T,
    mut pixel_op: impl FnMut(T, T),
) -> Option<(T, T)>
where
    T: Copy + Ord + Add<Output = T> + AddAssign + Constant<Output = T>,
    Range<T>: Iterator<Item = T>,
{
    if x < wx1 || x > wx2 {
        return None;
    }
    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
    if y1 > wy2 || y2 < wy1 {
        return None;
    }
    let (cy1, cy2) = (max(y1, wy1), min(y2, wy2));
    for y in cy1..(cy2 + T::ONE) {
        pixel_op(x, y);
    }
    Some((cy1, cy2))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn horizontal_line<T>(
    y: T,
    x1: T,
    x2: T,
    wy1: T,
    wy2: T,
    wx1: T,
    wx2: T,
    mut pixel_op: impl FnMut(T, T),
) -> Option<(T, T)>
where
    T: Copy + Ord + Add<Output = T> + AddAssign + Constant<Output = T>,
    Range<T>: Iterator<Item = T>,
{
    if y < wy1 || y > wy2 {
        return None;
    }
    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
    if x1 > wx2 || x2 < wx1 {
        return None;
    }
    let (cx1, cx2) = (max(x1, wx1), min(x2, wx2));
    // in practice it's better to fill the whole row in one operation,
    // but to keep the API simple we do it pixel-wise
    for x in cx1..(cx2 + T::ONE) {
        pixel_op(x, y);
    }
    Some((cx1, cx2))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn clip_rect_entry<T>(
    xy1: T,
    yx1: T,
    wxy1: T,
    wxy2: T,
    wyx1: T,
    wyx2: T,
    dyx: T,
    dxy2: T,
    dyx2: T,
) -> Option<(T, T, T)>
where
    T: Copy
        + Ord
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Constant<Output = T>,
{
    let (mut xyd, mut yxd) = (xy1, yx1);
    let mut err = dxy2 - dyx;

    if xy1 < wxy1 {
        let tmp = (T::TWO * (wxy1 - xy1) - T::ONE) * dyx;
        let msd = tmp / dxy2;
        yxd += msd;

        if yxd > wyx2 {
            return None;
        }

        if yxd >= wyx1 {
            let rem = tmp - msd * dxy2;
            xyd = wxy1;
            err -= rem + dyx;
            if rem > T::ZERO {
                yxd += T::ONE;
                err += dxy2;
            }
            return Some((xyd, yxd, err));
        }
    }

    if yx1 < wyx1 {
        let tmp = dxy2 * (wyx1 - yx1);
        xyd += tmp / dyx2;
        let rem = tmp % dyx2;

        if xyd > wxy2 || (xyd == wxy2 && rem >= dyx) {
            return None;
        }

        yxd = wyx1;
        err += rem;
        if rem >= dyx {
            xyd += T::ONE;
            err -= dyx2;
        }
    }
    Some((xyd, yxd, err))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn clip_rect_exit<T>(xy1: T, xy2: T, yx1: T, yx2: T, wxy2: T, dyx: T, dxy2: T, dyx2: T) -> T
where
    T: Copy
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + Div<Output = T>
        + Constant<Output = T>,
{
    let mut term = yx2;
    if xy2 > wxy2 {
        let temp = dyx2 * (wxy2 - xy1) + dyx;
        let msd = temp / dxy2;
        term = yx1 + msd;

        if (temp - msd * dxy2) == T::ZERO {
            term -= T::ONE;
        }
    }
    term
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn destandardize<T>(mut term: T, mut xyd: T, mut yxd: T, wxy2: T, txy: T, tyx: T) -> (T, T, T)
where
    T: Copy + Ord + Add<Output = T> + Mul<Output = T> + MulAssign + Constant<Output = T>,
{
    yxd *= tyx;
    xyd *= txy;
    term = txy * (min(term, wxy2) + T::ONE);
    (xyd, yxd, term)
}

#[inline(always)]
pub fn bresenham_step<T>(
    mut err: T,
    mut xyd: T,
    mut yxd: T,
    txy: T,
    tyx: T,
    dxy2: T,
    dyx2: T,
) -> (T, T, T)
where
    T: Ord + AddAssign + SubAssign + Constant<Output = T>,
{
    if err < T::ZERO {
        err += dyx2;
    } else {
        err -= dxy2;
        yxd += tyx;
    }
    xyd += txy;
    (err, xyd, yxd)
}

pub(crate) trait Constant {
    type Output;

    const ZERO: Self::Output;
    const ONE: Self::Output;
    const TWO: Self::Output;
}

macro_rules! impl_constant {
    ($num:ty) => {
        impl Constant for $num {
            type Output = $num;
            const ZERO: $num = 0;
            const ONE: $num = 1;
            const TWO: $num = 2;
        }
    };
}

impl_constant!(i8);
impl_constant!(i16);
impl_constant!(i32);
impl_constant!(i64);
impl_constant!(i128);
impl_constant!(isize);

impl_constant!(u8);
impl_constant!(u16);
impl_constant!(u32);
impl_constant!(u64);
impl_constant!(u128);
impl_constant!(usize);
