//! Octant iterator tests

mod bounds {
    use clipline::*;

    #[test]
    fn empty_line_segments_are_rejected() {
        assert!(Octant0::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Octant1::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Octant2::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Octant3::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Octant4::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Octant5::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Octant6::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Octant7::<i8>::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn axis_aligned_line_segments_are_rejected() {
        assert!(Octant0::<i8>::new((0, 0), (1, 0)).is_none());
        assert!(Octant1::<i8>::new((0, 0), (1, 0)).is_none());
        assert!(Octant2::<i8>::new((0, 0), (-1, 0)).is_none());
        assert!(Octant3::<i8>::new((0, 0), (-1, 0)).is_none());
        assert!(Octant4::<i8>::new((0, 0), (1, 0)).is_none());
        assert!(Octant5::<i8>::new((0, 0), (1, 0)).is_none());
        assert!(Octant6::<i8>::new((0, 0), (-1, 0)).is_none());
        assert!(Octant7::<i8>::new((0, 0), (-1, 0)).is_none());

        assert!(Octant0::<i8>::new((0, 0), (0, 1)).is_none());
        assert!(Octant1::<i8>::new((0, 0), (0, -1)).is_none());
        assert!(Octant2::<i8>::new((0, 0), (0, 1)).is_none());
        assert!(Octant3::<i8>::new((0, 0), (0, -1)).is_none());
        assert!(Octant4::<i8>::new((0, 0), (0, 1)).is_none());
        assert!(Octant5::<i8>::new((0, 0), (0, -1)).is_none());
        assert!(Octant6::<i8>::new((0, 0), (0, 1)).is_none());
        assert!(Octant7::<i8>::new((0, 0), (0, -1)).is_none());
    }

    #[test]
    fn diagonal_line_segments_are_rejected() {
        assert!(Octant0::<i8>::new((0, 0), (1, 1)).is_none());
        assert!(Octant1::<i8>::new((0, 0), (1, 1)).is_none());
        assert!(Octant2::<i8>::new((0, 0), (1, -1)).is_none());
        assert!(Octant3::<i8>::new((0, 0), (1, -1)).is_none());
        assert!(Octant4::<i8>::new((0, 0), (-1, 1)).is_none());
        assert!(Octant5::<i8>::new((0, 0), (-1, 1)).is_none());
        assert!(Octant6::<i8>::new((0, 0), (-1, -1)).is_none());
        assert!(Octant7::<i8>::new((0, 0), (-1, -1)).is_none());
    }
}

mod iter {
    use clipline::*;

    #[test]
    fn octant_0_produces_correct_points() {
        let points = vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2)];
        assert_eq!(Octant0::<i8>::new((0, 0), (5, 2)).unwrap().collect::<Vec<_>>(), points);
    }

    #[test]
    fn octant_1_produces_correct_points() {
        let points = vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4)];
        assert_eq!(Octant1::<i8>::new((0, 0), (2, 5)).unwrap().collect::<Vec<_>>(), points);
    }

    #[test]
    fn octant_2_produces_correct_points() {
        let points = vec![(0, 0), (1, 0), (2, -1), (3, -1), (4, -2)];
        assert_eq!(Octant2::<i8>::new((0, 0), (5, -2)).unwrap().collect::<Vec<_>>(), points);
    }

    #[test]
    fn octant_3_produces_correct_points() {
        let points = vec![(0, 0), (0, -1), (1, -2), (1, -3), (2, -4)];
        assert_eq!(Octant3::<i8>::new((0, 0), (2, -5)).unwrap().collect::<Vec<_>>(), points);
    }

    #[test]
    fn octant_4_produces_correct_points() {
        let points = vec![(0, 0), (-1, 0), (-2, 1), (-3, 1), (-4, 2)];
        assert_eq!(Octant4::<i8>::new((0, 0), (-5, 2)).unwrap().collect::<Vec<_>>(), points);
    }

    #[test]
    fn octant_5_produces_correct_points() {
        let points = vec![(0, 0), (0, 1), (-1, 2), (-1, 3), (-2, 4)];
        assert_eq!(Octant5::<i8>::new((0, 0), (-2, 5)).unwrap().collect::<Vec<_>>(), points);
    }

    #[test]
    fn octant_6_produces_correct_points() {
        let points = vec![(0, 0), (-1, 0), (-2, -1), (-3, -1), (-4, -2)];
        assert_eq!(Octant6::<i8>::new((0, 0), (-5, -2)).unwrap().collect::<Vec<_>>(), points);
    }

    #[test]
    fn octant_7_produces_correct_points() {
        let points = vec![(0, 0), (0, -1), (-1, -2), (-1, -3), (-2, -4)];
        assert_eq!(Octant7::<i8>::new((0, 0), (-2, -5)).unwrap().collect::<Vec<_>>(), points);
    }
}

mod proptest {
    use clipline::*;
    use proptest::prelude::*;

    fn config() -> ProptestConfig {
        ProptestConfig { cases: 4000000, failure_persistence: None, ..ProptestConfig::default() }
    }

    mod u8 {
        use super::*;

        const MIN: u8 = 0;
        const MAX: u8 = 255;
        const HALF_0: u8 = 127;
        const HALF_1: u8 = 128;
        const QUARTER_0: u8 = 63;
        const QUARTER_1: u8 = 64;
        const QUARTER_2: u8 = 191;
        const QUARTER_3: u8 = 192;

        const CLIPS: [(&str, Point<u8>, Point<u8>); 37] = [
            ("FULL", (MIN, MIN), (MAX, MAX)),
            ("POINT", (HALF_0, HALF_0), (HALF_0, HALF_0)),
            ("SMALL", (HALF_0, HALF_0), (HALF_1, HALF_1)),
            ("CENTER HALF-SIZE 0", (QUARTER_0, QUARTER_1), (QUARTER_2, QUARTER_3)),
            ("CENTER HALF-SIZE 1", (QUARTER_1, QUARTER_0), (QUARTER_3, QUARTER_2)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 0", (QUARTER_0, QUARTER_0), (HALF_0, HALF_0)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 1", (QUARTER_0, QUARTER_1), (HALF_1, HALF_0)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 2", (QUARTER_1, QUARTER_0), (HALF_0, HALF_1)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 3", (QUARTER_1, QUARTER_1), (HALF_1, HALF_1)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 0", (HALF_0, QUARTER_0), (QUARTER_2, HALF_0)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 1", (HALF_0, QUARTER_1), (QUARTER_3, HALF_1)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 2", (HALF_1, QUARTER_0), (QUARTER_2, HALF_1)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 3", (HALF_1, QUARTER_1), (QUARTER_3, HALF_0)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 0", (QUARTER_0, HALF_0), (HALF_0, QUARTER_2)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 1", (QUARTER_0, HALF_1), (HALF_1, QUARTER_3)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 2", (QUARTER_1, HALF_0), (HALF_0, QUARTER_2)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 3", (QUARTER_1, HALF_1), (HALF_1, QUARTER_3)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 0", (HALF_0, HALF_0), (QUARTER_3, QUARTER_2)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 1", (HALF_0, HALF_1), (QUARTER_3, QUARTER_3)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 2", (HALF_1, HALF_0), (QUARTER_2, QUARTER_2)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 3", (HALF_1, HALF_1), (QUARTER_2, QUARTER_3)),
            ("TOP-LEFT QUARTER 0", (MIN, MIN), (QUARTER_0, QUARTER_0)),
            ("TOP-LEFT QUARTER 1", (MIN, MIN), (QUARTER_1, QUARTER_1)),
            ("TOP-RIGHT QUARTER 0", (QUARTER_2, MIN), (MAX, QUARTER_2)),
            ("TOP-RIGHT QUARTER 1", (QUARTER_3, MIN), (MAX, QUARTER_3)),
            ("BOTTOM-LEFT QUARTER 0", (MIN, QUARTER_2), (QUARTER_2, MAX)),
            ("BOTTOM-LEFT QUARTER 1", (MIN, QUARTER_3), (QUARTER_3, MAX)),
            ("BOTTOM-RIGHT QUARTER 0", (QUARTER_2, QUARTER_2), (MAX, MAX)),
            ("BOTTOM-RIGHT QUARTER 1", (QUARTER_3, QUARTER_3), (MAX, MAX)),
            ("TOP HALF 0", (MIN, MIN), (MAX, HALF_0)),
            ("TOP HALF 1", (MIN, MIN), (MAX, HALF_1)),
            ("BOTTOM HALF 0", (MIN, HALF_0), (MAX, MAX)),
            ("BOTTOM HALF 1", (MIN, HALF_1), (MAX, MAX)),
            ("LEFT HALF 0", (MIN, MIN), (HALF_0, MAX)),
            ("LEFT HALF 1", (MIN, MIN), (HALF_1, MAX)),
            ("RIGHT HALF 0", (HALF_0, MIN), (MAX, MAX)),
            ("RIGHT HALF 1", (HALF_1, MIN), (MAX, MAX)),
        ];

        proptest! {
            #![proptest_config(config())]
            #[test]
            fn clipped_matches_unclipped(
                clip in proptest::sample::select(&CLIPS),
                ((p1, p2)): (Point<u8>, Point<u8>)
            ) {
                let (_, w1, w2) = clip;
                let clip = Clip::<u8>::new(w1, w2).unwrap();
                let clipped = clip.any_octant(p1, p2).into_iter().flatten();
                let naive_clipped = AnyOctant::<u8>::new(p1, p2).filter(|&xy| clip.point(xy));
                prop_assert!(clipped.eq(naive_clipped));
            }
        }
    }

    mod i8 {
        use super::*;

        const MIN: i8 = -128;
        const MAX: i8 = 127;
        const HALF_0: i8 = 0;
        const HALF_1: i8 = 1;
        const QUARTER_0: i8 = -64;
        const QUARTER_1: i8 = -63;
        const QUARTER_2: i8 = 64;
        const QUARTER_3: i8 = 65;

        const CLIPS: [(&str, Point<i8>, Point<i8>); 37] = [
            ("FULL", (MIN, MIN), (MAX, MAX)),
            ("POINT", (HALF_0, HALF_0), (HALF_0, HALF_0)),
            ("SMALL", (HALF_0, HALF_0), (HALF_1, HALF_1)),
            ("CENTER HALF-SIZE 0", (QUARTER_0, QUARTER_1), (QUARTER_2, QUARTER_3)),
            ("CENTER HALF-SIZE 1", (QUARTER_1, QUARTER_0), (QUARTER_3, QUARTER_2)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 0", (QUARTER_0, QUARTER_0), (HALF_0, HALF_0)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 1", (QUARTER_0, QUARTER_1), (HALF_1, HALF_0)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 2", (QUARTER_1, QUARTER_0), (HALF_0, HALF_1)),
            ("TOP-LEFT-CENTER QUARTER-SIZE 3", (QUARTER_1, QUARTER_1), (HALF_1, HALF_1)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 0", (HALF_0, QUARTER_0), (QUARTER_2, HALF_0)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 1", (HALF_0, QUARTER_1), (QUARTER_3, HALF_1)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 2", (HALF_1, QUARTER_0), (QUARTER_2, HALF_1)),
            ("TOP-RIGHT-CENTER QUARTER-SIZE 3", (HALF_1, QUARTER_1), (QUARTER_3, HALF_0)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 0", (QUARTER_0, HALF_0), (HALF_0, QUARTER_2)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 1", (QUARTER_0, HALF_1), (HALF_1, QUARTER_3)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 2", (QUARTER_1, HALF_0), (HALF_0, QUARTER_2)),
            ("BOTTOM-LEFT-CENTER QUARTER-SIZE 3", (QUARTER_1, HALF_1), (HALF_1, QUARTER_3)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 0", (HALF_0, HALF_0), (QUARTER_3, QUARTER_2)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 1", (HALF_0, HALF_1), (QUARTER_3, QUARTER_3)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 2", (HALF_1, HALF_0), (QUARTER_2, QUARTER_2)),
            ("BOTTOM-RIGHT-CENTER QUARTER-SIZE 3", (HALF_1, HALF_1), (QUARTER_2, QUARTER_3)),
            ("TOP-LEFT QUARTER 0", (MIN, MIN), (QUARTER_0, QUARTER_0)),
            ("TOP-LEFT QUARTER 1", (MIN, MIN), (QUARTER_1, QUARTER_1)),
            ("TOP-RIGHT QUARTER 0", (QUARTER_2, MIN), (MAX, QUARTER_2)),
            ("TOP-RIGHT QUARTER 1", (QUARTER_3, MIN), (MAX, QUARTER_3)),
            ("BOTTOM-LEFT QUARTER 0", (MIN, QUARTER_2), (QUARTER_2, MAX)),
            ("BOTTOM-LEFT QUARTER 1", (MIN, QUARTER_3), (QUARTER_3, MAX)),
            ("BOTTOM-RIGHT QUARTER 0", (QUARTER_2, QUARTER_2), (MAX, MAX)),
            ("BOTTOM-RIGHT QUARTER 1", (QUARTER_3, QUARTER_3), (MAX, MAX)),
            ("TOP HALF 0", (MIN, MIN), (MAX, HALF_0)),
            ("TOP HALF 1", (MIN, MIN), (MAX, HALF_1)),
            ("BOTTOM HALF 0", (MIN, HALF_0), (MAX, MAX)),
            ("BOTTOM HALF 1", (MIN, HALF_1), (MAX, MAX)),
            ("LEFT HALF 0", (MIN, MIN), (HALF_0, MAX)),
            ("LEFT HALF 1", (MIN, MIN), (HALF_1, MAX)),
            ("RIGHT HALF 0", (HALF_0, MIN), (MAX, MAX)),
            ("RIGHT HALF 1", (HALF_1, MIN), (MAX, MAX)),
        ];

        proptest! {
            #![proptest_config(config())]
            #[test]
            fn clipped_matches_unclipped(
                clip in proptest::sample::select(&CLIPS),
                ((p1, p2)): (Point<i8>, Point<i8>)
            ) {
                let (_, w1, w2) = clip;
                let clip = Clip::<i8>::new(w1, w2).unwrap();
                let clipped = clip.any_octant(p1, p2).into_iter().flatten();
                let naive_clipped = AnyOctant::<i8>::new(p1, p2).filter(|&xy| clip.point(xy));
                prop_assert!(clipped.eq(naive_clipped));
            }
        }
    }
}
