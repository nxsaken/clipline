//! Axis-aligned line segment iterator tests.

mod general {
    #[test]
    fn length_is_correct() {
        for v1 in -2..4 {
            for v2 in -2..4 {
                let length = i8::abs_diff(v1, v2);
                assert_eq!(clipline::Horizontal::new(0, v1, v2).length(), length);
                assert_eq!(clipline::Vertical::new(0, v1, v2).length(), length);
            }
        }
    }

    #[test]
    fn coordinate_order_is_correct() {
        let points = [-2, -1, 0, 1];
        clipline::Horizontal::new(0, -2, 2).enumerate().for_each(|(i, (x, y))| {
            assert_eq!(x, points[i]);
            assert_eq!(y, 0);
        });
        clipline::Vertical::new(0, -2, 2).enumerate().for_each(|(i, (x, y))| {
            assert_eq!(x, 0);
            assert_eq!(y, points[i]);
        });
    }

    #[test]
    fn direction_is_correct() {
        clipline::Horizontal::new(0, -2, 2)
            .enumerate()
            .for_each(|(i, (x, _))| assert_eq!(x, [-2, -1, 0, 1][i]));
        clipline::Horizontal::new(0, 2, -2)
            .enumerate()
            .for_each(|(i, (x, _))| assert_eq!(x, [2, 1, 0, -1][i]));
        clipline::Vertical::new(0, -2, 2)
            .enumerate()
            .for_each(|(i, (_, y))| assert_eq!(y, [-2, -1, 0, 1][i]));
        clipline::Vertical::new(0, 2, -2)
            .enumerate()
            .for_each(|(i, (_, y))| assert_eq!(y, [2, 1, 0, -1][i]));
    }

    #[test]
    fn positive_reverse_is_correct() {
        let mut rev = clipline::Horizontal::new(0, 0, 5).rev().collect::<Vec<_>>();
        rev.reverse();
        assert_eq!(clipline::Horizontal::new(0, 0, 5).collect::<Vec<_>>(), rev);
    }

    #[test]
    fn positive_double_ended_iteration_is_correct() {
        let mut line = clipline::Horizontal::new(0, 0, 2);
        assert_eq!(line.next_back(), Some((1, 0)));
        assert_eq!(line.next(), Some((0, 0)));
        assert!(line.is_done());
    }

    #[test]
    fn negative_reverse_is_correct() {
        let mut rev = clipline::Horizontal::new(0, 5, 0).rev().collect::<Vec<_>>();
        rev.reverse();
        assert_eq!(clipline::Horizontal::new(0, 5, 0).collect::<Vec<_>>(), rev);
    }

    #[test]
    fn negative_double_ended_iteration_is_correct() {
        let mut line = clipline::Horizontal::new(0, 0, -2);
        assert_eq!(line.next_back(), Some((-1, 0)));
        assert_eq!(line.next(), Some((0, 0)));
        assert!(line.is_done());
    }
}
