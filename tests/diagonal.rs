//! Diagonal line segment iterator tests.

mod quadrant {
    #[test]
    fn all_quadrants_exclude_empty_lines() {
        assert!(clipline::DiagonalQuadrant0::new((0, 0), (0, 0)).is_none());
        assert!(clipline::DiagonalQuadrant1::new((0, 0), (0, 0)).is_none());
        assert!(clipline::DiagonalQuadrant2::new((0, 0), (0, 0)).is_none());
        assert!(clipline::DiagonalQuadrant3::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn all_quadrants_exclude_non_diagonal_lines() {
        assert!(clipline::DiagonalQuadrant0::new((0, 0), (1, 0)).is_none());
        assert!(clipline::DiagonalQuadrant1::new((0, 0), (1, 0)).is_none());
        assert!(clipline::DiagonalQuadrant2::new((0, 0), (1, 0)).is_none());
        assert!(clipline::DiagonalQuadrant3::new((0, 0), (1, 0)).is_none());
    }

    #[test]
    fn quadrant_0_includes_its_diagonal_and_excludes_rest() {
        assert!(!clipline::DiagonalQuadrant0::new((0, 0), (1, 1))
            .unwrap()
            .is_done());
        assert!(clipline::DiagonalQuadrant0::new((0, 0), (-1, -1)).is_none());
        assert!(clipline::DiagonalQuadrant0::new((0, 0), (1, -1)).is_none());
        assert!(clipline::DiagonalQuadrant0::new((0, 0), (-1, 1)).is_none());
    }

    #[test]
    fn quadrant_1_includes_its_diagonal_and_excludes_rest() {
        assert!(!clipline::DiagonalQuadrant1::new((0, 0), (-1, 1))
            .unwrap()
            .is_done());
        assert!(clipline::DiagonalQuadrant1::new((0, 0), (-1, -1)).is_none());
        assert!(clipline::DiagonalQuadrant1::new((0, 0), (1, -1)).is_none());
        assert!(clipline::DiagonalQuadrant1::new((0, 0), (1, 1)).is_none());
    }

    #[test]
    fn quadrant_2_includes_its_diagonal_and_excludes_rest() {
        assert!(!clipline::DiagonalQuadrant2::new((0, 0), (1, -1))
            .unwrap()
            .is_done());
        assert!(clipline::DiagonalQuadrant2::new((0, 0), (-1, -1)).is_none());
        assert!(clipline::DiagonalQuadrant2::new((0, 0), (-1, 1)).is_none());
        assert!(clipline::DiagonalQuadrant2::new((0, 0), (1, 1)).is_none());
    }

    #[test]
    fn quadrant_3_includes_its_diagonal_and_excludes_rest() {
        assert!(!clipline::DiagonalQuadrant3::new((0, 0), (-1, -1))
            .unwrap()
            .is_done());
        assert!(clipline::DiagonalQuadrant3::new((0, 0), (1, -1)).is_none());
        assert!(clipline::DiagonalQuadrant3::new((0, 0), (-1, 1)).is_none());
        assert!(clipline::DiagonalQuadrant3::new((0, 0), (1, 1)).is_none());
    }
}

mod general {
    #[test]
    fn length_is_correct() {
        for v1 in -2..4 {
            for v2 in -2..4 {
                let length = isize::abs_diff(v1, v2);
                assert_eq!(
                    clipline::Diagonal::new((v1, v1), (v2, v2)).unwrap().count(),
                    length
                );
            }
        }
    }
}
