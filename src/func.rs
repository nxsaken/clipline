use crate::util::{
    bresenham_step, clip_rect_entry, clip_rect_exit, destandardize, horizontal_line, standardize,
    vertical_line, Point,
};

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
/// * `line`: A tuple representing the endpoints of the line segment, inclusive.
/// The line segment will be drawn between these two points.
///
/// * `clip_rect`: A tuple representing the corners of the clipping rectangle, inclusive.
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
///     .expect("line intersects clip_rect");
/// assert_eq!(start, clip_rect.0);
/// assert_eq!(end, clip_rect.1);
/// ```
///
/// # Note
///
/// This is slightly more optimized than the iterator version, but uses internal iteration. Unlike
/// the iterator version, vertical and horizontal lines will always be traversed in an ascending order.
pub fn clipline<F>(
    line: (Point, Point),
    clip_rect: (Point, Point),
    mut pixel_op: F,
) -> Option<(Point, Point)>
where
    F: FnMut(isize, isize),
{
    let ((x1, y1), (x2, y2)) = line;
    let ((wx1, wy1), (wx2, wy2)) = clip_rect;

    if x1 == x2 {
        return vertical_line(x1, y1, y2, wx1, wx2, wy1, wy2, pixel_op)
            .map(|(cy1, cy2)| ((x1, cy1), (x1, cy2)));
    }

    if y1 == y2 {
        return horizontal_line(y1, x1, x2, wy1, wy2, wx1, wx2, pixel_op)
            .map(|(cx1, cx2)| ((cx1, y1), (cx2, y1)));
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
        let (mut xd, mut yd, term) = destandardize(term, xd, yd, wx2, tx, ty);
        let dx2 = dx2 - dy2;
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
        let (mut yd, mut xd, term) = destandardize(term, yd, xd, wy2, ty, tx);
        let dy2 = dy2 - dx2;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn draw_pixel(_x: isize, _y: isize) {}

    #[test]
    fn test_line_outside_clip() {
        let line = ((0, 0), (10, 10));
        let clip_rect = ((12, 12), (15, 15));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }

    #[test]
    fn test_line_inside_clip() {
        let line = ((5, 5), (8, 8));
        let clip_rect = ((2, 2), (10, 10));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((5, 5), (8, 8))));
    }

    #[test]
    fn test_line_crosses_top_boundary() {
        let line = ((3, 1), (7, 5));
        let clip_rect = ((2, 2), (8, 4));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((4, 2), (6, 4))));
    }

    #[test]
    fn test_line_crosses_right_boundary() {
        let line = ((6, 3), (12, 7));
        let clip_rect = ((5, 2), (10, 6));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((6, 3), (10, 6))));
    }

    #[test]
    fn test_line_crosses_top_and_right_boundaries() {
        let line = ((7, 1), (13, 6));
        let clip_rect = ((5, 2), (10, 5));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((8, 2), (10, 4))));
    }

    #[test]
    fn test_single_point_line_inside_clip() {
        let line = ((4, 4), (4, 4));
        let clip_rect = ((2, 2), (5, 5));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((4, 4), (4, 4))));
    }

    #[test]
    fn test_single_point_line_outside_clip() {
        let line = ((1, 1), (1, 1));
        let clip_rect = ((2, 2), (5, 5));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }

    #[test]
    fn test_diagonal_line_top_left_to_bottom_right() {
        let line = ((1, 1), (4, 4));
        let clip_rect = ((2, 2), (3, 3));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((2, 2), (3, 3))));
    }

    #[test]
    fn test_diagonal_line_bottom_left_to_top_right() {
        let line = ((2, 5), (6, 1));
        let clip_rect = ((3, 2), (5, 4));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((3, 4), (5, 2))));
    }

    #[test]
    fn test_diagonal_line_top_left_to_bottom_right_reversed() {
        let line = ((4, 4), (1, 1));
        let clip_rect = ((2, 2), (3, 3));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((3, 3), (2, 2))));
    }

    #[test]
    fn test_diagonal_line_bottom_left_to_top_right_reversed() {
        let line = ((6, 1), (2, 5));
        let clip_rect = ((3, 2), (5, 4));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((5, 2), (3, 4))));
    }

    #[test]
    fn test_line_outside_clip_with_negatives() {
        let line = ((-3, -3), (-1, -1));
        let clip_rect = ((0, 0), (2, 2));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }

    #[test]
    fn test_line_inside_clip_with_negatives() {
        let line = ((-1, -1), (1, 1));
        let clip_rect = ((-2, -2), (2, 2));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-1, -1), (1, 1))));
    }

    #[test]
    fn test_line_crosses_top_and_right_boundaries_with_negatives() {
        let line = ((-1, 1), (3, 3));
        let clip_rect = ((-2, 0), (2, 2));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-1, 1), (1, 2))));
    }

    #[test]
    fn test_line_crosses_bottom_and_left_boundaries_with_negatives() {
        let line = ((-3, -2), (1, -5));
        let clip_rect = ((-4, -4), (0, -1));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-3, -2), (0, -4))));
    }

    #[test]
    fn test_single_point_line_inside_clip_with_negatives() {
        let line = ((-1, -1), (-1, -1));
        let clip_rect = ((-2, -2), (0, 0));

        let result = clipline(line, clip_rect, draw_pixel);
        assert_eq!(result, Some(((-1, -1), (-1, -1))));
    }

    #[test]
    fn test_single_point_line_outside_clip_with_negatives() {
        let line = ((-3, -3), (-3, -3));
        let clip_rect = ((-2, -2), (0, 0));

        let result = clipline(line, clip_rect, draw_pixel);
        assert!(result.is_none());
    }

    #[test]
    fn test_clip_horizontal_fully_outside() {
        let line = ((0, 2), (10, 2));
        let clip_rect = ((3, 0), (7, 1));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, None);
    }

    #[test]
    fn test_clip_vertical_fully_outside() {
        let line = ((4, 0), (4, 5));
        let clip_rect = ((2, 2), (3, 4));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, None,);
    }

    #[test]
    fn test_clip_horizontal_fully_inside() {
        let line = ((2, 1), (8, 1));
        let clip_rect = ((1, 0), (9, 2));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, Some(((2, 1), (8, 1))),);
    }

    #[test]
    fn test_clip_vertical_fully_inside() {
        let line = ((4, 3), (4, 7));
        let clip_rect = ((3, 2), (5, 8));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, Some(((4, 3), (4, 7))),);
    }

    #[test]
    fn test_clip_horizontal_partial_from_negative() {
        let line = ((-2, 2), (8, 2));
        let clip_rect = ((0, 0), (6, 4));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, Some(((0, 2), (6, 2))),);
    }

    #[test]
    fn test_clip_vertical_partial_from_negative() {
        let line = ((4, -1), (4, 7));
        let clip_rect = ((2, 2), (5, 8));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, Some(((4, 2), (4, 7))),);
    }

    #[test]
    fn test_clip_horizontal_partial_from_positive() {
        let line = ((2, 2), (12, 2));
        let clip_rect = ((0, 0), (6, 4));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, Some(((2, 2), (6, 2))),);
    }

    #[test]
    fn test_clip_vertical_partial_from_positive() {
        let line = ((4, 3), (4, 12));
        let clip_rect = ((2, 2), (5, 8));

        let clipped_line = clipline(line, clip_rect, |_, _| {});
        assert_eq!(clipped_line, Some(((4, 3), (4, 8))),);
    }
}
