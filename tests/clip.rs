use clipline::*;
use proptest::prelude::*;

// todo: make into proptest
#[test]
fn test_proj_bounds() {
    let clip = Viewport::<i8>::from_min_max(-128, -128, 127, 127).unwrap();
    let (y, x0, x1) = (0, 127, -128);

    let naive = LineAx::<i8>::new(y, x0, x1).filter(|&(x, y)| clip.point(x, y));
    let fast = clip.line_ax(y, x0, x1).unwrap();
    let naive_proj = LineAx::<i8>::new(y, x0, x1).filter_map(|(x, y)| clip.point_proj(x, y));
    let fast_proj = clip.line_ax_proj(y, x0, x1).unwrap();

    let counts = [naive.count(), fast.count(), naive_proj.count(), fast_proj.count()];
    assert_eq!(counts, [counts[0]; 4]);
}

prop_compose! {
    pub fn viewport_u_min_max_u8()
        (u_min in u8::MIN..u8::MAX)
        (u_max in u_min..=u8::MAX, u_min in Just(u_min))
    -> (u8, u8) {
        (u_min, u_max)
    }
}

prop_compose! {
    pub fn viewport_u_min_max_i8()
        (u_min in i8::MIN..i8::MAX)
        (u_max in u_min..=i8::MAX, u_min in Just(u_min))
    -> (i8, i8) {
        (u_min, u_max)
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1_000_000,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn viewport_line_ax_u8(
        (x_min, x_max) in viewport_u_min_max_u8(),
        (y_min, y_max) in viewport_u_min_max_u8(),
        (y, x0, x1) in any::<(u8, u8, u8)>(),
    ) {
        let clip = Viewport::<u8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAx::<u8>::new(y, x0, x1).filter(|&(x, y)| clip.point(x, y));
        let fast = clip.line_ax(y, x0, x1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAx(u8, y={y}, x0={x0}, x1={x1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }

    #[test]
    fn viewport_line_ax_i8(
        (x_min, x_max) in viewport_u_min_max_i8(),
        (y_min, y_max) in viewport_u_min_max_i8(),
        (y, x0, x1) in any::<(i8, i8, i8)>(),
    ) {
        let clip = Viewport::<i8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAx::<i8>::new(y, x0, x1).filter(|&(x, y)| clip.point(x, y));
        let fast = clip.line_ax(y, x0, x1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAx(i8, y={y}, x0={x0}, x1={x1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }

    #[test]
    fn viewport_line_ay_u8(
        (x_min, x_max) in viewport_u_min_max_u8(),
        (y_min, y_max) in viewport_u_min_max_u8(),
        (x, y0, y1) in any::<(u8, u8, u8)>(),
    ) {
        let clip = Viewport::<u8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAy::<u8>::new(x, y0, y1).filter(|&(x, y)| clip.point(x, y));
        let fast = clip.line_ay(x, y0, y1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAy(u8, x={x}, y0={y0}, y1={y1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }

    #[test]
    fn viewport_line_ay_i8(
        (x_min, x_max) in viewport_u_min_max_i8(),
        (y_min, y_max) in viewport_u_min_max_i8(),
        (x, y0, y1) in any::<(i8, i8, i8)>(),
    ) {
        let clip = Viewport::<i8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAy::<i8>::new(x, y0, y1).filter(|&(x, y)| clip.point(x, y));
        let fast = clip.line_ay(x, y0, y1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAy(i8, x={x}, y0={y0}, y1={y1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }

    #[test]
    fn viewport_line_ax_proj_u8(
        (x_min, x_max) in viewport_u_min_max_u8(),
        (y_min, y_max) in viewport_u_min_max_u8(),
        (y, x0, x1) in any::<(u8, u8, u8)>(),
    ) {
        let clip = Viewport::<u8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAx::<u8>::new(y, x0, x1).filter_map(|(x, y)| clip.point_proj(x, y));
        let fast = clip.line_ax_proj(y, x0, x1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAx(u8, y={y}, x0={x0}, x1={x1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }

    #[test]
    fn viewport_line_ax_proj_i8(
        (x_min, x_max) in viewport_u_min_max_i8(),
        (y_min, y_max) in viewport_u_min_max_i8(),
        (y, x0, x1) in any::<(i8, i8, i8)>(),
    ) {
        let clip = Viewport::<i8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAx::<i8>::new(y, x0, x1).filter_map(|(x, y)| clip.point_proj(x, y));
        let fast = clip.line_ax_proj(y, x0, x1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAx(i8, y={y}, x0={x0}, x1={x1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }

    #[test]
    fn viewport_line_ay_proj_u8(
        (x_min, x_max) in viewport_u_min_max_u8(),
        (y_min, y_max) in viewport_u_min_max_u8(),
        (x, y0, y1) in any::<(u8, u8, u8)>(),
    ) {
        let clip = Viewport::<u8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAy::<u8>::new(x, y0, y1).filter_map(|(x, y)| clip.point_proj(x, y));
        let fast = clip.line_ay_proj(x, y0, y1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAy(u8, x={x}, y0={y0}, y1={y1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }

    #[test]
    fn viewport_line_ay_proj_i8(
        (x_min, x_max) in viewport_u_min_max_i8(),
        (y_min, y_max) in viewport_u_min_max_i8(),
        (x, y0, y1) in any::<(i8, i8, i8)>(),
    ) {
        let clip = Viewport::<i8>::from_min_max(x_min, y_min, x_max, y_max).unwrap();
        let naive = LineAy::<i8>::new(x, y0, y1).filter_map(|(x, y)| clip.point_proj(x, y));
        let fast = clip.line_ay_proj(x, y0, y1).into_iter().flatten();
        prop_assert!(
            naive.eq(fast),
            "LineAy(i8, x={x}, y0={y0}, y1={y1});\
            Viewport(x_min={x_min}, x_max={x_max}, y_min={y_min}, y_max={y_max})"
        );
    }
}
