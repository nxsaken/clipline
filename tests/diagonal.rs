//! Diagonal line segment iterator tests.

mod quadrant {
    #[test]
    fn all_quadrants_exclude_empty_lines() {
        assert!(clipline::DiagonalQuadrant0::new((0, 0), 0).is_done());
        assert!(clipline::DiagonalQuadrant1::new((0, 0), 0).is_done());
        assert!(clipline::DiagonalQuadrant2::new((0, 0), 0).is_done());
        assert!(clipline::DiagonalQuadrant3::new((0, 0), 0).is_done());
    }

    #[test]
    fn all_quadrants_iterate_in_correct_direction() {
        assert_eq!(clipline::DiagonalQuadrant0::new((0, 0), 2).nth(1), Some((1, 1)));
        assert_eq!(clipline::DiagonalQuadrant1::new((0, 0), 2).nth(1), Some((1, -1)));
        assert_eq!(clipline::DiagonalQuadrant2::new((0, 0), 2).nth(1), Some((-1, 1)));
        assert_eq!(clipline::DiagonalQuadrant3::new((0, 0), 2).nth(1), Some((-1, -1)));
    }
}

mod general {
    #[test]
    fn length_is_correct() {
        for v1 in -2..4 {
            for v2 in -2..4 {
                let length = i8::abs_diff(v1, v2);
                assert_eq!(clipline::Diagonal::new((v1, v1), (v2, v2)).unwrap().length(), length);
            }
        }
    }
}
