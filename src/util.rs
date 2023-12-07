use core::cmp::{max, min};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

pub type Point<T> = (T, T);

/// Standardizes the line segment (such that `x1 < x2 && y1 < y2`).
#[inline(always)]
pub fn standardize<T: Ord + Neg<Output = T> + TryFrom<u8>>(
    xy1: T,
    xy2: T,
    wxy1: T,
    wxy2: T,
) -> Option<(T, T, T, T, T)> {
    let one = T::try_from(1).unwrap_or_else(|_| unreachable!());
    if xy1 < xy2 {
        (xy1 <= wxy2 && xy2 >= wxy1).then_some((one, xy1, xy2, wxy1, wxy2))
    } else {
        (xy2 <= wxy2 && xy1 >= wxy1).then_some((-one, -xy1, -xy2, -wxy2, -wxy1))
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn vertical_line<T: Copy + Ord + Add<Output = T> + AddAssign + TryFrom<u8>>(
    x: T,
    y1: T,
    y2: T,
    wx1: T,
    wx2: T,
    wy1: T,
    wy2: T,
    mut pixel_op: impl FnMut(T, T),
) -> Option<(T, T)> {
    if x < wx1 || x > wx2 {
        return None;
    }
    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
    if y1 > wy2 || y2 < wy1 {
        return None;
    }
    let (cy1, cy2) = (max(y1, wy1), min(y2, wy2));
    let one = T::try_from(1).unwrap_or_else(|_| unreachable!());
    let mut y = cy1;
    while y <= cy2 {
        pixel_op(x, y);
        y += one;
    }
    Some((cy1, cy2))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn horizontal_line<T: Copy + Ord + Add<Output = T> + AddAssign + TryFrom<u8>>(
    y: T,
    x1: T,
    x2: T,
    wy1: T,
    wy2: T,
    wx1: T,
    wx2: T,
    mut pixel_op: impl FnMut(T, T),
) -> Option<(T, T)> {
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
    let one = T::try_from(1).unwrap_or_else(|_| unreachable!());
    let mut x = cx1;
    while x <= cx2 {
        pixel_op(x, y);
        x += one;
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
        + TryFrom<u8>,
{
    let [zero, one, two] = [0, 1, 2].map(|n| T::try_from(n).unwrap_or_else(|_| unreachable!()));

    let (mut xyd, mut yxd) = (xy1, yx1);
    let mut err = dxy2 - dyx;

    if xy1 < wxy1 {
        let tmp = (two * (wxy1 - xy1) - one) * dyx;
        let msd = tmp / dxy2;
        yxd += msd;

        if yxd > wyx2 {
            return None;
        }

        if yxd >= wyx1 {
            let rem = tmp - msd * dxy2;
            xyd = wxy1;
            err -= rem + dyx;
            if rem > zero {
                yxd += one;
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
            xyd += one;
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
        + TryFrom<u8>,
{
    let [zero, one] = [0, 1].map(|n| T::try_from(n).unwrap_or_else(|_| unreachable!()));
    let mut term = yx2;
    if xy2 > wxy2 {
        let temp = dyx2 * (wxy2 - xy1) + dyx;
        let msd = temp / dxy2;
        term = yx1 + msd;

        if (temp - msd * dxy2) == zero {
            term -= one;
        }
    }
    term
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn destandardize<T>(mut term: T, mut xyd: T, mut yxd: T, wxy2: T, txy: T, tyx: T) -> (T, T, T)
where
    T: Copy + Ord + Add<Output = T> + Mul<Output = T> + MulAssign + TryFrom<u8>,
{
    let one = T::try_from(1).unwrap_or_else(|_| unreachable!());
    yxd *= tyx;
    xyd *= txy;
    term = txy * (min(term, wxy2) + one);
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
    T: Ord + AddAssign + SubAssign + TryFrom<u8>,
{
    let zero = T::try_from(0).unwrap_or_else(|_| unreachable!());
    if err >= zero {
        yxd += tyx;
        err -= dxy2;
    } else {
        err += dyx2;
    }
    xyd += txy;
    (err, xyd, yxd)
}
