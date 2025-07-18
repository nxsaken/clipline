use clipline::*;
use proptest::prelude::*;

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
    fn clip_line_ax_u8(
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
    fn clip_line_ax_i8(
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
}
