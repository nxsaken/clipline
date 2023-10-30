use core::cmp::{max, min};

pub type Point = (isize, isize);

/// Standardizes the line segment (such that `x1 < x2 && y1 < y2`).
pub fn standardize(
    xy1: isize,
    xy2: isize,
    wxy1: isize,
    wxy2: isize,
) -> Option<(isize, isize, isize, isize, isize)> {
    if xy1 < xy2 {
        (xy1 <= wxy2 && xy2 >= wxy1).then_some((1, xy1, xy2, wxy1, wxy2))
    } else {
        (xy2 <= wxy2 && xy1 >= wxy1).then_some((-1, -xy1, -xy2, -wxy2, -wxy1))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn vertical_line(
    x: isize,
    y1: isize,
    y2: isize,
    wx1: isize,
    wx2: isize,
    wy1: isize,
    wy2: isize,
    mut pixel_op: impl FnMut(isize, isize),
) -> Option<(isize, isize)> {
    if x < wx1 || x > wx2 {
        return None;
    }
    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
    if y1 > wy2 || y2 < wy1 {
        return None;
    }
    let (cy1, cy2) = (max(y1, wy1), min(y2, wy2));
    for y in cy1..=cy2 {
        pixel_op(x, y);
    }
    Some((cy1, cy2))
}

#[allow(clippy::too_many_arguments)]
pub fn horizontal_line(
    y: isize,
    x1: isize,
    x2: isize,
    wy1: isize,
    wy2: isize,
    wx1: isize,
    wx2: isize,
    mut pixel_op: impl FnMut(isize, isize),
) -> Option<(isize, isize)> {
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
    for x in cx1..=cx2 {
        pixel_op(x, y);
    }
    Some((cx1, cx2))
}

#[allow(clippy::too_many_arguments)]
pub fn clip_rect_entry(
    xy1: isize,
    yx1: isize,
    wxy1: isize,
    wxy2: isize,
    wyx1: isize,
    wyx2: isize,
    dyx: isize,
    dxy2: isize,
    dyx2: isize,
) -> Option<(isize, isize, isize)> {
    let (mut xyd, mut yxd) = (xy1, yx1);
    let mut err = dxy2 - dyx;

    if xy1 < wxy1 {
        let tmp = (2 * (wxy1 - xy1) - 1) * dyx;
        let msd = tmp / dxy2;
        yxd += msd;

        if yxd > wyx2 {
            return None;
        }

        if yxd >= wyx1 {
            let rem = tmp - msd * dxy2;
            xyd = wxy1;
            err -= rem + dyx;
            if rem > 0 {
                yxd += 1;
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
            xyd += 1;
            err -= dyx2;
        }
    }
    Some((xyd, yxd, err))
}

#[allow(clippy::too_many_arguments)]
pub fn clip_rect_exit(
    xy1: isize,
    xy2: isize,
    yx1: isize,
    yx2: isize,
    wxy2: isize,
    dyx: isize,
    dxy2: isize,
    dyx2: isize,
) -> isize {
    let mut term = yx2;
    if xy2 > wxy2 {
        let temp = dyx2 * (wxy2 - xy1) + dyx;
        let msd = temp / dxy2;
        term = yx1 + msd;

        if (temp - msd * dxy2) == 0 {
            term -= 1;
        }
    }
    term
}

#[allow(clippy::too_many_arguments)]
pub fn destandardize(
    mut term: isize,
    mut xyd: isize,
    mut yxd: isize,
    wxy2: isize,
    txy: isize,
    tyx: isize,
) -> (isize, isize, isize) {
    yxd *= tyx;
    xyd *= txy;
    term = txy * (min(term, wxy2) + 1);
    (xyd, yxd, term)
}

pub fn bresenham_step(
    mut err: isize,
    mut xyd: isize,
    mut yxd: isize,
    txy: isize,
    tyx: isize,
    dxy2: isize,
    dyx2: isize,
) -> (isize, isize, isize) {
    if err >= 0 {
        yxd += tyx;
        err -= dxy2;
    } else {
        err += dyx2;
    }
    xyd += txy;
    (err, xyd, yxd)
}
