use clipline::*;
use proptest::prelude::*;

prop_compose! {
    fn sample_clip_u8()(
        x_max in 0..=u8::MAX,
        y_max in 0..=u8::MAX,
    ) -> Clip<u8> {
        Clip::<u8>::from_max(x_max, y_max)
    }
}

prop_compose! {
    fn sample_clip_i8()(
        x_max in 0..=i8::MAX,
        y_max in 0..=i8::MAX,
    ) -> Clip<i8> {
        Clip::<i8>::from_max(x_max, y_max).unwrap()
    }
}

prop_compose! {
    fn sample_viewport_u8()(
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
    fn sample_viewport_i8()(
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

macro_rules! sample_line {
    (@line_d $($UI:ident)|+) => {
        paste::paste! {
            $(fn [<sample_line_d _ $UI>]() -> impl Strategy<Value = ($UI, $UI, $UI, $UI)> {
                any::<($UI, $UI)>().prop_flat_map(|(x0, y0)| {
                    let directions = proptest::sample::select(vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]);
                    directions.prop_flat_map(move |(sx, sy)| {
                        let max_dx = if sx > 0 { <$UI>::MAX.abs_diff(x0) } else { <$UI>::MIN.abs_diff(x0) };
                        let max_dy = if sy > 0 { <$UI>::MAX.abs_diff(y0) } else { <$UI>::MIN.abs_diff(y0) };
                        let max_d = max_dx.min(max_dy);
                        (0..=max_d).prop_map(move |d| {
                            let x1 = [<$UI _add_u8>](x0, d, sx);
                            let y1 = [<$UI _add_u8>](y0, d, sy);
                            (x0, y0, x1, y1)
                        })
                    })
                })
            }
            fn [<sample_line_d2 _ $UI>]() -> impl Strategy<Value = ($UI, $UI, $UI, $UI)> {
                [<sample_line_d _ $UI>]()
            })+
        }
    };
    (@line_a $($UI:ident)|+) => {
        paste::paste! {
            $(fn [<sample_line_ax _ $UI>]() -> impl Strategy<Value = ($UI, $UI, $UI, $UI)> {
                any::<($UI, $UI, $UI)>().prop_map(|(v, u0, u1)| {
                    (u0, v, u1, v)
                })
            }
            fn [<sample_line_ay _ $UI>]() -> impl Strategy<Value = ($UI, $UI, $UI, $UI)> {
                any::<($UI, $UI, $UI)>().prop_map(|(v, u0, u1)| {
                    (v, u0, v, u1)
                })
            })+
        }
    };
    (@line_b $($UI:ident)|+) => {
        paste::paste! {
            $(fn [<sample_line_b _ $UI>]() -> impl Strategy<Value = ($UI, $UI, $UI, $UI)> {
                any::<($UI, $UI, $UI, $UI)>()
            })+
        }
    };
}

sample_line!(@line_a u8 | i8);
sample_line!(@line_b u8 | i8);
sample_line!(@line_d u8 | i8);

macro_rules! test {
    (
        $Line:ident $(+ $unwrap:ident)?,
        $sample_line:ident,
        $line:ident,
        $line_proj:ident,
        $N:literal
    ) => {
        test!(@ clip, $Line<u8> $(+ $unwrap)?, $sample_line, $line, $N);
        test!(@ clip, $Line<i8> $(+ $unwrap)?, $sample_line, $line, $N);
        test!(@ clip proj, $Line<i8> $(+ $unwrap)?, $sample_line, $line_proj, $N);
        test!(@ viewport, $Line<u8> $(+ $unwrap)?, $sample_line, $line, $N);
        test!(@ viewport proj, $Line<u8> $(+ $unwrap)?, $sample_line, $line_proj, $N);
        test!(@ viewport, $Line<i8> $(+ $unwrap)?, $sample_line, $line, $N);
        test!(@ viewport proj, $Line<i8> $(+ $unwrap)?, $sample_line, $line_proj, $N);
    };
    (@
        $clip:ident,
        $Line:ident<$UI:ty> $(+ $unwrap:ident)?,
        $sample_line:ident,
        $line:ident,
        $N:literal
    ) => {
        paste::paste! {
            proptest! {
                #![proptest_config(ProptestConfig {
                    cases: $N,
                    failure_persistence: None,
                    ..ProptestConfig::default()
                })]
                #[test]
                fn [<$clip _ $sample_line _ $UI>](
                    clip in [<sample_ $clip _ $UI>](),
                    (x0, y0, x1, y1) in [<sample_ $sample_line _ $UI>](),
                ) {
                    let raw = $Line::<$UI>::new(x0, y0, x1, y1);
                    $(let raw = raw.$unwrap();)?
                    let is_empty = raw.is_empty();
                    let naive = raw.filter(|&(x, y)| clip.point(x, y));
                    let smart = clip.$line(x0, y0, x1, y1);
                    if let Some(smart) = smart {
                        prop_assert!(!smart.is_empty() || is_empty, "clipped != empty");
                        prop_assert!(naive.eq(smart), "naive != smart");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
            }
        }
    };
    (@
        $clip:ident proj,
        $Line:ident<$UI:ty> $(+ $unwrap:ident)?,
        $sample_line:ident,
        $line_proj:ident,
        $N:literal
    ) => {
        paste::paste! {
            proptest! {
                #![proptest_config(ProptestConfig {
                    cases: $N,
                    failure_persistence: None,
                    ..ProptestConfig::default()
                })]
                #[test]
                fn [<$clip _ $sample_line _proj_ $UI>](
                    clip in [<sample_ $clip _ $UI>](),
                    (x0, y0, x1, y1) in [<sample_ $sample_line _ $UI>](),
                ) {
                    let raw = $Line::<$UI>::new(x0, y0, x1, y1);
                    $(let raw = raw.$unwrap();)?
                    let is_empty = raw.is_empty();
                    let naive = raw.filter_map(|(x, y)| clip.point_proj(x, y));
                    let smart = clip.$line_proj(x0, y0, x1, y1);
                    if let Some(smart) = smart {
                        prop_assert!(!smart.is_empty() || is_empty, "clipped != empty");
                        prop_assert!(naive.eq(smart), "naive != smart");
                    } else {
                        prop_assert_eq!(naive.count(), 0);
                    }
                }
            }
        }
    };
}

test!(LineA + unwrap, line_ax, line_a, line_a_proj, 4_000_000);
test!(LineA + unwrap, line_ay, line_a, line_a_proj, 4_000_000);
test!(LineB, line_b, line_b, line_b_proj, 4_000_000);
test!(LineD + unwrap, line_d, line_d, line_d_proj, 4_000_000);
test!(LineD2 + unwrap, line_d2, line_d2, line_d2_proj, 4_000_000);
