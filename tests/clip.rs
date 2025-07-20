use clipline::*;
use proptest::prelude::*;

prop_compose! {
    fn clip_u8()(
        x_max in 0..=u8::MAX,
        y_max in 0..=u8::MAX,
    ) -> Clip<u8> {
        Clip::<u8>::from_max(x_max, y_max)
    }
}

prop_compose! {
    fn clip_i8()(
        x_max in 0..=i8::MAX,
        y_max in 0..=i8::MAX,
    ) -> Clip<i8> {
        Clip::<i8>::from_max(x_max, y_max).unwrap()
    }
}

prop_compose! {
    fn viewport_u8()(
        x_min in u8::MIN..u8::MAX,
        y_min in u8::MIN..u8::MAX
    )(
        x_max in x_min..=u8::MAX,
        y_max in y_min..=u8::MAX,
        x_min in Just(x_min),
        y_min in Just(y_min)
    ) -> Viewport<u8> {
        Viewport::<u8>::from_min_max(x_min, y_min, x_max, y_max).unwrap()
    }
}

prop_compose! {
    fn viewport_i8()(
        x_min in i8::MIN..i8::MAX,
        y_min in i8::MIN..i8::MAX
    )(
        x_max in x_min..=i8::MAX,
        y_max in y_min..=i8::MAX,
        x_min in Just(x_min),
        y_min in Just(y_min)
    ) -> Viewport<i8> {
        Viewport::<i8>::from_min_max(x_min, y_min, x_max, y_max).unwrap()
    }
}

fn u8_add_u8(u0: u8, du: u8, su: i8) -> u8 {
    if su > 0 { u0 + du } else { u0 - du }
}

fn i8_add_u8(u0: i8, du: u8, su: i8) -> i8 {
    let v = if su > 0 { u0.checked_add_unsigned(du) } else { u0.checked_sub_unsigned(du) };
    v.unwrap()
}

macro_rules! line_d {
    ($($uiN:ident)|+) => {
        paste::paste! {
            $(fn [<line_d _ $uiN>]() -> impl Strategy<Value = [$uiN; 4]> {
                any::<($uiN, $uiN)>().prop_flat_map(|(x0, y0)| {
                    let directions = proptest::sample::select(vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]);
                    directions.prop_flat_map(move |(sx, sy)| {
                        let max_dx = if sx > 0 { <$uiN>::MAX.abs_diff(x0) } else { <$uiN>::MIN.abs_diff(x0) };
                        let max_dy = if sy > 0 { <$uiN>::MAX.abs_diff(y0) } else { <$uiN>::MIN.abs_diff(y0) };
                        let max_d = max_dx.min(max_dy);
                        (0..=max_d).prop_map(move |d| {
                            let x1 = [<$uiN _add_u8>](x0, d, sx);
                            let y1 = [<$uiN _add_u8>](y0, d, sy);
                            [x0, y0, x1, y1]
                        })
                    })
                })
            })+
        }
    };
}

line_d!(u8 | i8);

macro_rules! test {
    (@line_a, $clip:ident, $LineAu:ident, $line_au:ident, $($uiN:ident)|+, $(proj $uiN_p:ident)|+) => {
        paste::paste! {
            proptest! {
                #![proptest_config(ProptestConfig {
                    cases: 1_000_000,
                    failure_persistence: None,
                    ..ProptestConfig::default()
                })]
                $(
                #[test]
                fn [<$clip _ $line_au _ $uiN>](
                    clip in [<$clip _ $uiN>](),
                    (v, u0, u1) in any::<($uiN, $uiN, $uiN)>(),
                ) {
                    let naive = $LineAu::<$uiN>::new(v, u0, u1).filter(|&(x, y)| clip.point(x, y));
                    if let Some(smart) = clip.$line_au(v, u0, u1) {
                        prop_assert!(smart.len() != 0 || u0 == u1, "completely clipped line segment was not rejected");
                        prop_assert!(naive.eq(smart), "smart clip doesn't match naive clip");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
                )+

                $(
                #[test]
                fn [<$clip _ $line_au _ proj _ $uiN_p>](
                    clip in [<$clip _ $uiN_p>](),
                    (v, u0, u1) in any::<($uiN_p, $uiN_p, $uiN_p)>(),
                ) {
                    let naive = $LineAu::<$uiN_p>::new(v, u0, u1).filter_map(|(x, y)| clip.point_proj(x, y));
                    if let Some(smart) = clip.[<$line_au _ proj>](v, u0, u1) {
                        prop_assert!(smart.len() != 0 || u0 == u1, "completely clipped line segment was not rejected");
                        prop_assert!(naive.eq(smart), "smart clip projection doesn't match naive clip projection");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
                )+
            }
        }
    };
    (@line_b, $clip:ident, $($uiN:ident)|+, $(proj $uiN_p:ident)|+) => {
        paste::paste! {
            proptest! {
                #![proptest_config(ProptestConfig {
                    cases: 1_000_000,
                    failure_persistence: None,
                    ..ProptestConfig::default()
                })]
                $(
                #[test]
                fn [<$clip _ line_b _ $uiN>](
                    clip in [<$clip _ $uiN>](),
                    [x0, y0, x1, y1] in any::<[$uiN; 4]>(),
                ) {
                    let naive = LineB::<$uiN>::new(x0, y0, x1, y1).filter(|&(x, y)| clip.point(x, y));
                    if let Some(smart) = clip.line_b(x0, y0, x1, y1) {
                        prop_assert!(smart.len() != 0 || x0 == x1 || y0 == y1, "completely clipped line segment was not rejected");
                        prop_assert!(naive.eq(smart), "smart clip doesn't match naive clip");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
                )+

                $(
                #[test]
                fn [<$clip _ line_b_proj _ $uiN_p>](
                    clip in [<$clip _ $uiN_p>](),
                    [x0, y0, x1, y1] in any::<[$uiN_p; 4]>(),
                ) {
                    let naive = LineB::<$uiN_p>::new(x0, y0, x1, y1).filter_map(|(x, y)| clip.point_proj(x, y));
                    if let Some(smart) = clip.line_b_proj(x0, y0, x1, y1) {
                        prop_assert!(smart.len() != 0 || x0 == x1 || y0 == y1, "completely clipped line segment was not rejected");
                        prop_assert!(naive.eq(smart), "smart clip projection doesn't match naive clip projection");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
                )+
            }
        }
    };
    (@line_d, $clip:ident, $($uiN:ident)|+, $(proj $uiN_p:ident)|+) => {
        paste::paste! {
            proptest! {
                #![proptest_config(ProptestConfig {
                    cases: 1_000_000,
                    failure_persistence: None,
                    ..ProptestConfig::default()
                })]
                $(
                #[test]
                fn [<$clip _ line_d _ $uiN>](
                    clip in [<$clip _ $uiN>](),
                    [x0, y0, x1, y1] in [<line_d _ $uiN>](),
                ) {
                    let naive = LineD::<$uiN>::new(x0, y0, x1, y1)
                        .unwrap()
                        .filter(|&(x, y)| clip.point(x, y));
                    if let Some(smart) = clip.line_d(x0, y0, x1, y1) {
                        prop_assert!(smart.len() != 0 || x0 == x1, "completely clipped line segment was not rejected");
                        prop_assert!(naive.eq(smart), "smart clip doesn't match naive clip");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
                )+

                $(
                #[test]
                fn [<$clip _ line_d_proj _ $uiN_p>](
                    clip in [<$clip _ $uiN_p>](),
                    [x0, y0, x1, y1] in [<line_d _ $uiN_p>](),
                ) {
                    let naive = LineD::<$uiN_p>::new(x0, y0, x1, y1)
                        .unwrap()
                        .filter_map(|(x, y)| clip.point_proj(x, y));
                    if let Some(smart) = clip.line_d_proj(x0, y0, x1, y1) {
                        prop_assert!(smart.len() != 0 || x0 == x1, "completely clipped line segment was not rejected");
                        prop_assert!(naive.eq(smart), "smart clip projection doesn't match naive clip projection");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
                )+
            }
        }
    };
}

test!(@line_a, clip, LineAx, line_ax, u8 | i8, proj i8);
test!(@line_a, clip, LineAy, line_ay, u8 | i8, proj i8);
test!(@line_a, viewport, LineAx, line_ax, u8 | i8, proj u8 | proj i8);
test!(@line_a, viewport, LineAy, line_ay, u8 | i8, proj u8 | proj i8);

test!(@line_b, clip, u8 | i8, proj i8);
test!(@line_b, viewport, u8 | i8, proj u8 | proj i8);

test!(@line_d, clip, u8 | i8, proj i8);
test!(@line_d, viewport, u8 | i8, proj u8 | proj i8);
