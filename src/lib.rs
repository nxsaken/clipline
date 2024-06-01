#![no_std]

use core::cmp::{max, min};

type Point = (isize, isize);
type ClipPoint = (isize, isize);

/// Performs scan conversion of a line segment using Bresenham's algorithm,
/// while clipping it to a specified rectangle.
///
/// The function takes a line defined by two endpoints (inclusive)
/// and a clipping rectangle defined by its corners (inclusive).
/// The provided closure `pixel_op` is responsible for handling each pixel within the clipped region.
/// To ensure optimal performance, it is recommended that `pixel_op` does not perform any bounds checks.
///
/// # Arguments
///
/// * `line`: A tuple representing the endpoints of the line segment, defined as `(Point, Point)`.
/// The line segment will be drawn between these two points.
///
/// * `clip_rect`: A tuple representing the corners of the clipping rectangle, defined as `(Point, Point)`.
/// The line segment will be clipped to this rectangle, and only the visible portion will be drawn.
///
/// * `pixel_op`: A closure that takes the coordinates of a pixel within the clipped region.
/// It is invoked for each pixel along the line. You can use this closure to perform any
/// desired pixel-specific operations without the need for bounds checking.
///
/// # Returns
///
/// If any part of the line segment is visible within the clipping rectangle,
/// the function returns an `Option` containing a tuple of two `ClipPoint` values
/// representing the starting and ending points of the visible portion of the line segment.
/// If the line is entirely outside the clipping region, the function returns `None`.
///
/// # Example
///
/// ```rust
/// use clipline::clipline;
///
/// fn draw_pixel(x: isize, y: isize) {
///     // Your custom pixel drawing logic here
///     // No bounds checks necessary here
/// }
///
/// let line = ((0, 0), (10, 10));
/// let clip_rect = ((2, 2), (8, 8));
///
/// let (start, end) = clipline(line, clip_rect, draw_pixel)
///     .expect("line intersects the clipping rectangle");
/// assert_eq!(start, clip_rect.0);
/// assert_eq!(end, clip_rect.1);
/// ```
///
/// # Note
///
/// The line endpoints and clip rectangle corners are inclusive, i.e.
/// the pixels at the specified coordinates are part of the visible region.
pub fn clipline<F>(
    line: (Point, Point),
    clip_rect: (Point, Point),
    mut pixel_op: F,
) -> Option<(ClipPoint, ClipPoint)>
where
    F: FnMut(isize, isize),
{
    let ((x1, y1), (x2, y2)) = line;
    let ((wx1, wy1), (wx2, wy2)) = clip_rect;

    if x1 == x2 {
        return vertical_line(x1, y1, y2, wx1, wx2, wy1, wy2, pixel_op)
            .map(|()| ((x1, y1), (x1, y2)));
    }

    if y1 == y2 {
        return horizontal_line(y1, x1, x2, wy1, wy2, wx1, wx2, pixel_op)
            .map(|()| ((x1, y1), (x2, y1)));
    }

    // Implementation of the paper by YP Kuzmin:
    // "Bresenham's Line Generation Algorithm with Built‐in Clipping." (1995)

    let (tx, x1, x2, wx1, wx2) = standardize(x1, x2, wx1, wx2)?;
    let (ty, y1, y2, wy1, wy2) = standardize(y1, y2, wy1, wy2)?;

    let dx = x2 - x1;
    let dy = y2 - y1;

    let (dx2, dy2) = (2 * dx, 2 * dy);

    if dx >= dy {
        let (yd, xd, mut err) = clip_rect_entry(y1, x1, wy1, wy2, wx1, wx2, dx, dy2, dx2)?;
        let term = clip_rect_exit(y1, y2, x1, x2, wy2, dx, dy2, dx2);
        let (mut xd, mut yd, term, dx2) = destandardize(term, xd, yd, dx2, dy2, wx2, tx, ty);
        let (cx1, cy1) = (xd, yd);
        let (mut cx2, mut cy2) = (cx1, cy1);
        while xd != term {
            pixel_op(xd, yd);
            (cx2, cy2) = (xd, yd);
            (err, xd, yd) = bresenham_step(err, xd, yd, tx, ty, dx2, dy2);
        }
        Some(((cx1, cy1), (cx2, cy2)))
    } else {
        let (xd, yd, mut err) = clip_rect_entry(x1, y1, wx1, wx2, wy1, wy2, dy, dx2, dy2)?;
        let term = clip_rect_exit(x1, x2, y1, y2, wx2, dy, dx2, dy2);
        let (mut yd, mut xd, term, dy2) = destandardize(term, yd, xd, dy2, dx2, wy2, ty, tx);
        let (cx1, cy1) = (xd, yd);
        let (mut cx2, mut cy2) = (cx1, cy1);
        while yd != term {
            pixel_op(xd, yd);
            (cx2, cy2) = (xd, yd);
            (err, yd, xd) = bresenham_step(err, yd, xd, ty, tx, dy2, dx2);
        }
        Some(((cx1, cy1), (cx2, cy2)))
    }
}

/// Standardizes the line segment (such that `x1 < x2 && y1 < y2`).
fn standardize(
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
fn vertical_line(
    x: isize,
    y1: isize,
    y2: isize,
    wx1: isize,
    wx2: isize,
    wy1: isize,
    wy2: isize,
    mut pixel_op: impl FnMut(isize, isize),
) -> Option<()> {
    if x < wx1 || x > wx2 {
        return None;
    }
    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
    for y in max(y1, wy1)..=min(y2, wy2) {
        pixel_op(x, y);
    }
    Some(())
}

#[allow(clippy::too_many_arguments)]
fn horizontal_line(
    y: isize,
    x1: isize,
    x2: isize,
    wy1: isize,
    wy2: isize,
    wx1: isize,
    wx2: isize,
    mut pixel_op: impl FnMut(isize, isize),
) -> Option<()> {
    if y < wy1 || y > wy2 {
        return None;
    }
    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
    // in practice it's better to fill the whole row in one operation,
    // but to keep the API simple we do it pixel-wise
    for x in max(x1, wx1)..=min(x2, wx2) {
        pixel_op(x, y);
    }
    Some(())
}

#[allow(clippy::too_many_arguments)]
fn clip_rect_entry(
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
        let temp = (2 * (wxy1 - xy1) - 1) * dyx;
        let msd = temp / dxy2;
        yxd += msd;

        if yxd > wyx2 {
            return None;
        }

        if yxd >= wyx1 {
            let rem = temp - msd * dxy2;
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
        let temp = dxy2 * (wyx1 - yx1);
        xyd += temp / dyx2;
        let rem = temp % dyx2;

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
fn clip_rect_exit(
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
fn destandardize(
    mut term: isize,
    mut xyd: isize,
    mut yxd: isize,
    mut dxy2: isize,
    dyx2: isize,
    wxy2: isize,
    txy: isize,
    tyx: isize,
) -> (isize, isize, isize, isize) {
    yxd *= tyx;
    xyd *= txy;
    term = txy * (min(term, wxy2) + 1);
    dxy2 -= dyx2;
    (xyd, yxd, term, dxy2)
}

fn bresenham_step(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn draw_pixel(_x: isize, _y: isize) {}

    // Test when the line is entirely outside the clipping region
    #[test]
    fn test_line_outside_clip() {
        let line = ((0, 0), (10, 10));
        let clip_rect = ((12, 12), (15, 15));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }

    // Test when the line is entirely inside the clipping region
    #[test]
    fn test_line_inside_clip() {
        let line = ((5, 5), (8, 8));
        let clip_rect = ((2, 2), (10, 10));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((5, 5), (8, 8))));
    }

    // Test when the line crosses the top boundary of the clipping region
    #[test]
    fn test_line_crosses_top_boundary() {
        let line = ((3, 1), (7, 5));
        let clip_rect = ((2, 2), (8, 4));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((4, 2), (6, 4))));
    }

    // Test when the line crosses the right boundary of the clipping region
    #[test]
    fn test_line_crosses_right_boundary() {
        let line = ((6, 3), (12, 7));
        let clip_rect = ((5, 2), (10, 6));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((6, 3), (10, 6))));
    }

    // Test when the line crosses both top and right boundaries of the clipping region
    #[test]
    fn test_line_crosses_top_and_right_boundaries() {
        let line = ((7, 1), (13, 6));
        let clip_rect = ((5, 2), (10, 5));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((8, 2), (10, 4))));
    }

    // Test when the line is completely horizontal
    #[test]
    fn test_horizontal_line() {
        let line = ((3, 4), (7, 4));
        let clip_rect = ((2, 3), (9, 5));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((3, 4), (7, 4))));
    }

    // Test when the line is completely vertical
    #[test]
    fn test_vertical_line() {
        let line = ((5, 2), (5, 7));
        let clip_rect = ((4, 1), (6, 8));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((5, 2), (5, 7))));
    }

    // Test when the line is a single point and within the clipping region
    #[test]
    fn test_single_point_line_inside_clip() {
        let line = ((4, 4), (4, 4));
        let clip_rect = ((2, 2), (5, 5));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((4, 4), (4, 4))));
    }

    // Test when the line is a single point and outside the clipping region
    #[test]
    fn test_single_point_line_outside_clip() {
        let line = ((1, 1), (1, 1));
        let clip_rect = ((2, 2), (5, 5));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }

    // Test when the line is a diagonal line from top-left to bottom-right
    #[test]
    fn test_diagonal_line_top_left_to_bottom_right() {
        let line = ((1, 1), (4, 4));
        let clip_rect = ((2, 2), (3, 3));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((2, 2), (3, 3))));
    }

    // Test when the line is a diagonal line from bottom-left to top-right
    #[test]
    fn test_diagonal_line_bottom_left_to_top_right() {
        let line = ((2, 5), (6, 1));
        let clip_rect = ((3, 2), (5, 4));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((3, 4), (5, 2))));
    }

    // Test when the line is a diagonal line from top-left to bottom-right with reversed points
    #[test]
    fn test_diagonal_line_top_left_to_bottom_right_reversed() {
        let line = ((4, 4), (1, 1));
        let clip_rect = ((2, 2), (3, 3));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((3, 3), (2, 2))));
    }

    // Test when the line is a diagonal line from bottom-left to top-right with reversed points
    #[test]
    fn test_diagonal_line_bottom_left_to_top_right_reversed() {
        let line = ((6, 1), (2, 5));
        let clip_rect = ((3, 2), (5, 4));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((5, 2), (3, 4))));
    }

    // Test when the line is entirely outside the clipping region
    #[test]
    fn test_line_outside_clip_with_negatives() {
        let line = ((-3, -3), (-1, -1));
        let clip_rect = ((0, 0), (2, 2));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }

    // Test when the line is entirely inside the clipping region
    #[test]
    fn test_line_inside_clip_with_negatives() {
        let line = ((-1, -1), (1, 1));
        let clip_rect = ((-2, -2), (2, 2));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-1, -1), (1, 1))));
    }

    // Test when the line crosses both top and right boundaries of the clipping region
    #[test]
    fn test_line_crosses_top_and_right_boundaries_with_negatives() {
        let line = ((-1, 1), (3, 3));
        let clip_rect = ((-2, 0), (2, 2));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-1, 1), (1, 2))));
    }

    // Test when the line crosses the bottom and left boundaries of the clipping region
    #[test]
    fn test_line_crosses_bottom_and_left_boundaries_with_negatives() {
        let line = ((-3, -2), (1, -5));
        let clip_rect = ((-4, -4), (0, -1));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-3, -2), (0, -4))));
    }

    // Test when the line is a single point and within the clipping region
    #[test]
    fn test_single_point_line_inside_clip_with_negatives() {
        let line = ((-1, -1), (-1, -1));
        let clip_rect = ((-2, -2), (0, 0));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-1, -1), (-1, -1))));
    }

    // Test when the line is a single point and outside the clipping region
    #[test]
    fn test_single_point_line_outside_clip_with_negatives() {
        let line = ((-3, -3), (-3, -3));
        let clip_rect = ((-2, -2), (0, 0));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }
}
