//! Bresenham line segment iterator tests.

use clipline::{
    Bresenham, BresenhamOctant0, BresenhamOctant1, BresenhamOctant2, BresenhamOctant3,
    BresenhamOctant4, BresenhamOctant5, BresenhamOctant6, BresenhamOctant7, Clip,
};

mod octant_bounds {
    use super::*;

    #[test]
    fn all_octants_exclude_empty_line() {
        assert!(BresenhamOctant0::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant1::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant2::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant3::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant4::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant5::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant6::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant7::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_0_excludes_0_and_includes_45_degrees() {
        assert!(BresenhamOctant0::new((0, 0), (1, 0)).is_none());
        assert!(BresenhamOctant0::new((0, 0), (1, 1)).is_some());
    }

    #[test]
    fn octant_1_excludes_45_degrees_and_excludes_90_degrees() {
        assert!(BresenhamOctant1::new((0, 0), (1, 1)).is_none());
        assert!(BresenhamOctant1::new((0, 0), (0, 1)).is_none());
    }

    #[test]
    fn octant_2_includes_315_degrees_and_excludes_0_degrees() {
        assert!(BresenhamOctant2::new((0, 0), (1, -1)).is_some());
        assert!(BresenhamOctant2::new((0, 0), (1, 0)).is_none());
    }

    #[test]
    fn octant_3_excludes_270_degrees_and_excludes_315_degrees() {
        assert!(BresenhamOctant3::new((0, 0), (0, -1)).is_none());
        assert!(BresenhamOctant3::new((0, 0), (1, -1)).is_none());
    }

    #[test]
    fn octant_4_includes_135_and_excludes_180_degrees() {
        assert!(BresenhamOctant4::new((0, 0), (-1, 1)).is_some());
        assert!(BresenhamOctant4::new((0, 0), (-1, 0)).is_none());
    }

    #[test]
    fn octant_5_excludes_90_and_excludes_135_degrees() {
        assert!(BresenhamOctant5::new((0, 0), (0, 1)).is_none());
        assert!(BresenhamOctant5::new((0, 0), (-1, 1)).is_none());
    }

    #[test]
    fn octant_6_excludes_180_degrees_and_includes_225_degrees() {
        assert!(BresenhamOctant6::new((0, 0), (-1, 0)).is_none());
        assert!(BresenhamOctant6::new((0, 0), (-1, -1)).is_some());
    }

    #[test]
    fn octant_7_excludes_225_degrees_and_excludes_270_degrees() {
        assert!(BresenhamOctant7::new((0, 0), (-1, -1)).is_none());
        assert!(BresenhamOctant7::new((0, 0), (0, -1)).is_none());
    }
}

mod rasterization {
    use super::*;

    #[test]
    fn length_is_correct() {
        let clip = Clip::new((-2, -2), (2, 2)).unwrap();
        for x in -2..=2 {
            for y in -2..=2 {
                let length = i8::abs_diff(0, x).max(i8::abs_diff(0, y));
                assert_eq!(Bresenham::new((0, 0), (x, y)).length(), length);
                assert_eq!(Bresenham::clip((0, 0), (x, y), clip).unwrap().length(), length);
            }
        }
    }

    #[test]
    fn octant_0_produces_correct_points() {
        let points = vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2)];
        assert_eq!(BresenhamOctant0::new((0, 0), (5, 2)).unwrap().collect::<Vec<_>>(), points);
        assert_eq!(
            Clip::new((0, 0), (5, 2))
                .unwrap()
                .bresenham((0, 0), (5, 2))
                .unwrap()
                .collect::<Vec<_>>(),
            points
        );
        assert_eq!(
            BresenhamOctant0::clip((0, 0), (5, 2), Clip::new((1, 0), (4, 1)).unwrap())
                .unwrap()
                .collect::<Vec<_>>(),
            points[1..3]
        );
        assert_eq!(
            Clip::new((0, 1), (3, 2))
                .unwrap()
                .bresenham((0, 0), (5, 2))
                .unwrap()
                .collect::<Vec<_>>(),
            points[2..3]
        );
    }

    #[test]
    fn octant_1_produces_correct_points() {
        let points = vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4)];
        assert_eq!(BresenhamOctant1::new((0, 0), (2, 5)).unwrap().collect::<Vec<_>>(), points);
        assert_eq!(
            Clip::new((0, 0), (2, 5))
                .unwrap()
                .bresenham((0, 0), (2, 5))
                .unwrap()
                .collect::<Vec<_>>(),
            points
        );
    }

    #[test]
    fn octant_2_produces_correct_points() {
        assert_eq!(
            BresenhamOctant2::new((0, 0), (5, -2))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (2, -1), (3, -1), (4, -2)]
        );
    }

    #[test]
    fn octant_3_produces_correct_points() {
        assert_eq!(
            BresenhamOctant3::new((0, 0), (2, -5))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, -1), (1, -2), (1, -3), (2, -4)]
        );
    }

    #[test]
    fn octant_4_produces_correct_points() {
        assert_eq!(
            BresenhamOctant4::new((0, 0), (-5, 2))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (-1, 0), (-2, 1), (-3, 1), (-4, 2)]
        );
    }

    #[test]
    fn octant_5_produces_correct_points() {
        assert_eq!(
            BresenhamOctant5::new((0, 0), (-2, 5))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (-1, 2), (-1, 3), (-2, 4)]
        );
    }

    #[test]
    fn octant_6_produces_correct_points() {
        assert_eq!(
            BresenhamOctant6::new((0, 0), (-5, -2))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (-1, -0), (-2, -1), (-3, -1), (-4, -2)]
        );
    }

    #[test]
    fn octant_7_produces_correct_points() {
        assert_eq!(
            BresenhamOctant7::new((0, 0), (-2, -5))
                .expect("octant should be correct")
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, -1), (-1, -2), (-1, -3), (-2, -4)]
        );
    }
}
