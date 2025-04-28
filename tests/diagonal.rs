//! Diagonal iterator tests

mod bounds {
    use clipline::*;

    #[test]
    fn empty_line_segments_are_in_quadrant_0() {
        assert!(Diagonal0::<i8>::new((0, 0), (0, 0)).is_some_and(|me| me.is_done()));
        assert!(Diagonal1::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Diagonal2::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(Diagonal3::<i8>::new((0, 0), (0, 0)).is_none());
        assert_eq!(
            AnyDiagonal::<i8>::new((0, 0), (0, 0)).and_then(|me| me.try_into_diagonal()),
            Diagonal0::<i8>::new((0, 0), (0, 0))
        );
        assert_eq!(
            Clip::<i8>::new((-1, -1), (1, 1))
                .unwrap()
                .any_diagonal((0, 0), (0, 0))
                .and_then(|me| me.try_into_diagonal()),
            Diagonal0::<i8>::new((0, 0), (0, 0))
        )
    }

    #[test]
    fn all_quadrants_include_correct_directions() {
        assert_eq!(Diagonal0::<i8>::new((0, 0), (2, 2)).unwrap().nth(1), Some((1, 1)));
        assert_eq!(Diagonal1::<i8>::new((0, 0), (2, -2)).unwrap().nth(1), Some((1, -1)));
        assert_eq!(Diagonal2::<i8>::new((0, 0), (-2, 2)).unwrap().nth(1), Some((-1, 1)));
        assert_eq!(Diagonal3::<i8>::new((0, 0), (-2, -2)).unwrap().nth(1), Some((-1, -1)));
    }
}

mod iter {
    use clipline::*;

    #[test]
    fn length_is_correct() {
        for v1 in -2..4 {
            for v2 in -2..4 {
                let length = i8::abs_diff(v1, v2);
                assert_eq!(AnyDiagonal::<i8>::new((v1, v1), (v2, v2)).unwrap().length(), length);
            }
        }
    }
}
