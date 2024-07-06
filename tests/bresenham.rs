//! Bresenham line segment iterator tests.

mod octant_bounds {
    #[test]
    fn octant_0_excludes_0_and_excludes_45_degrees() {
        assert!(clipline::BresenhamOctant0::new((0, 0), (1, 0)).is_none());
        assert!(clipline::BresenhamOctant0::new((0, 0), (1, 1)).is_none());
    }

    #[test]
    fn octant_0_excludes_empty_line() {
        assert!(clipline::BresenhamOctant0::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_1_includes_45_degrees_and_excludes_90_degrees() {
        assert!(clipline::BresenhamOctant1::new((0, 0), (1, 1)).is_some());
        assert!(clipline::BresenhamOctant1::new((0, 0), (0, 1)).is_none());
    }

    #[test]
    fn octant_1_excludes_empty_line() {
        assert!(clipline::BresenhamOctant1::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_2_excludes_315_degrees_and_includes_0_degrees() {
        assert!(clipline::BresenhamOctant2::new((0, 0), (1, 1)).is_none());
        assert!(clipline::BresenhamOctant2::new((0, 0), (1, 0)).is_some());
    }

    #[test]
    fn octant_2_excludes_empty_line() {
        assert!(clipline::BresenhamOctant2::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_3_excludes_270_degrees_and_includes_315_degrees() {
        assert!(clipline::BresenhamOctant3::new((0, 0), (0, 1)).is_none());
        assert!(clipline::BresenhamOctant3::new((0, 0), (1, 1)).is_some());
    }

    #[test]
    fn octant_3_excludes_empty_line() {
        assert!(clipline::BresenhamOctant3::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_4_excludes_135_and_excludes_180_degrees() {
        assert!(clipline::BresenhamOctant4::new((0, 0), (1, 1)).is_none());
        assert!(clipline::BresenhamOctant4::new((0, 0), (1, 0)).is_none());
    }

    #[test]
    fn octant_4_excludes_empty_line() {
        assert!(clipline::BresenhamOctant4::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_5_includes_90_and_includes_135_degrees() {
        assert!(clipline::BresenhamOctant5::new((0, 0), (0, 1)).is_some());
        assert!(clipline::BresenhamOctant5::new((0, 0), (1, 1)).is_some());
    }

    #[test]
    fn octant_5_excludes_empty_line() {
        assert!(clipline::BresenhamOctant5::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_6_includes_180_degrees_and_excludes_225_degrees() {
        assert!(clipline::BresenhamOctant6::new((0, 0), (1, 0)).is_some());
        assert!(clipline::BresenhamOctant6::new((0, 0), (1, 1)).is_none());
    }

    #[test]
    fn octant_6_excludes_empty_line() {
        assert!(clipline::BresenhamOctant6::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_7_includes_225_degrees_and_includes_270_degrees() {
        assert!(clipline::BresenhamOctant7::new((0, 0), (1, 1)).is_some());
        assert!(clipline::BresenhamOctant7::new((0, 0), (0, 1)).is_some());
    }

    #[test]
    fn octant_7_includes_empty_line() {
        assert!(clipline::BresenhamOctant7::new((0, 0), (0, 0)).unwrap().is_done());
    }
}

mod rasterization {
    #[test]
    fn length_is_correct() {
        for x in -2..=2 {
            for y in -2..=2 {
                let length = isize::abs_diff(0, x).max(isize::abs_diff(0, y));
                assert_eq!(clipline::Bresenham::new((0, 0), (x, y)).len(), length);
            }
        }
    }

    #[test]
    fn octant_0_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant0::new((0, 0), (5, 2))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2)]
        );
    }

    #[test]
    fn octant_1_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant1::new((0, 0), (2, 5))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4)]
        );
    }

    #[test]
    fn octant_2_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant2::new((0, 0), (5, 2))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (2, -1), (3, -1), (4, -2)]
        );
    }

    #[test]
    fn octant_3_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant3::new((0, 0), (2, 5))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, -1), (1, -2), (1, -3), (2, -4)]
        );
    }

    #[test]
    fn octant_4_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant4::new((0, 0), (5, 2))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (-1, 0), (-2, 1), (-3, 1), (-4, 2)]
        );
    }

    #[test]
    fn octant_5_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant5::new((0, 0), (2, 5))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (-1, 2), (-1, 3), (-2, 4)]
        );
    }

    #[test]
    fn octant_6_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant6::new((0, 0), (5, 2))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (-1, -0), (-2, -1), (-3, -1), (-4, -2)]
        );
    }

    #[test]
    fn octant_7_produces_correct_points() {
        assert_eq!(
            clipline::BresenhamOctant7::new((0, 0), (2, 5))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, -1), (-1, -2), (-1, -3), (-2, -4)]
        );
    }
}
