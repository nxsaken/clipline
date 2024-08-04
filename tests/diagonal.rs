//! Diagonal line segment iterator tests.

use clipline::{
    Diagonal, DiagonalQuadrant0, DiagonalQuadrant1, DiagonalQuadrant2, DiagonalQuadrant3,
};

mod quadrant {
    use super::*;

    #[test]
    fn all_quadrants_exclude_empty_lines() {
        assert!(DiagonalQuadrant0::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(DiagonalQuadrant1::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(DiagonalQuadrant2::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(DiagonalQuadrant3::<i8>::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn all_quadrants_include_correct_directions() {
        assert_eq!(DiagonalQuadrant0::<i8>::new((0, 0), (2, 2)).unwrap().nth(1), Some((1, 1)));
        assert_eq!(DiagonalQuadrant1::<i8>::new((0, 0), (2, -2)).unwrap().nth(1), Some((1, -1)));
        assert_eq!(DiagonalQuadrant2::<i8>::new((0, 0), (-2, 2)).unwrap().nth(1), Some((-1, 1)));
        assert_eq!(DiagonalQuadrant3::<i8>::new((0, 0), (-2, -2)).unwrap().nth(1), Some((-1, -1)));
    }
}

mod general {
    use super::*;

    #[test]
    fn length_is_correct() {
        for v1 in -2..4 {
            for v2 in -2..4 {
                let length = i8::abs_diff(v1, v2);
                assert_eq!(Diagonal::<i8>::new((v1, v1), (v2, v2)).unwrap().length(), length);
            }
        }
    }
}
