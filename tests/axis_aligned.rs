//! ## Axis-aligned iterator tests

mod iterator {
    #[test]
    fn length_is_correct() {
        for v1 in -2..4 {
            for v2 in -2..4 {
                let length = i8::abs_diff(v1, v2);
                assert_eq!(clipline::Axis0::<i8>::new(0, v1, v2).length(), length);
                assert_eq!(clipline::Axis1::<i8>::new(0, v1, v2).length(), length);
            }
        }
    }

    #[test]
    fn coordinate_order_is_correct() {
        let points = [-2, -1, 0, 1];
        clipline::Axis0::<i8>::new(0, -2, 2).enumerate().for_each(|(i, (x, y))| {
            assert_eq!(x, points[i]);
            assert_eq!(y, 0);
        });
        clipline::Axis1::<i8>::new(0, -2, 2).enumerate().for_each(|(i, (x, y))| {
            assert_eq!(x, 0);
            assert_eq!(y, points[i]);
        });
    }

    #[test]
    fn direction_is_correct() {
        clipline::Axis0::<i8>::new(0, -2, 2)
            .enumerate()
            .for_each(|(i, (x, _))| assert_eq!(x, [-2, -1, 0, 1][i]));
        clipline::Axis0::<i8>::new(0, 2, -2)
            .enumerate()
            .for_each(|(i, (x, _))| assert_eq!(x, [2, 1, 0, -1][i]));
        clipline::Axis1::<i8>::new(0, -2, 2)
            .enumerate()
            .for_each(|(i, (_, y))| assert_eq!(y, [-2, -1, 0, 1][i]));
        clipline::Axis1::<i8>::new(0, 2, -2)
            .enumerate()
            .for_each(|(i, (_, y))| assert_eq!(y, [2, 1, 0, -1][i]));
    }

    #[test]
    fn positive_reverse_is_correct() {
        let mut rev = clipline::Axis0::<i8>::new(0, 0, 5).rev().collect::<Vec<_>>();
        rev.reverse();
        assert_eq!(clipline::Axis0::<i8>::new(0, 0, 5).collect::<Vec<_>>(), rev);
    }

    #[test]
    fn positive_double_ended_iteration_is_correct() {
        let mut line = clipline::Axis0::<i8>::new(0, 0, 2);
        assert_eq!(line.next_back(), Some((1, 0)));
        assert_eq!(line.next(), Some((0, 0)));
        assert!(line.is_done());
    }

    #[test]
    fn negative_reverse_is_correct() {
        let mut rev = clipline::Axis0::<i8>::new(0, 5, 0).rev().collect::<Vec<_>>();
        rev.reverse();
        assert_eq!(clipline::Axis0::<i8>::new(0, 5, 0).collect::<Vec<_>>(), rev);
    }

    #[test]
    fn negative_double_ended_iteration_is_correct() {
        let mut line = clipline::Axis0::<i8>::new(0, 0, -2);
        assert_eq!(line.next_back(), Some((-1, 0)));
        assert_eq!(line.next(), Some((0, 0)));
        assert!(line.is_done());
    }
}

mod clip {
    use clipline::Clip;

    const CLIP: Clip<i8> = match Clip::<i8>::new((0, 0), (63, 47)) {
        Some(clip) => clip,
        None => unreachable!(),
    };

    #[test]
    fn axis_0_correct() {
        let mut neg = CLIP.axis_0(32, 76, -23).unwrap();
        assert_eq!(neg.next(), Some((63, 32)));
        assert_eq!(neg.next_back(), Some((0, 32)));

        let mut neg = CLIP.axis_0(32, 63, 0).unwrap();
        assert_eq!(neg.next(), Some((63, 32)));
        assert_eq!(neg.next_back(), Some((1, 32)));

        let mut pos = CLIP.axis_0(32, -23, 76).unwrap();
        assert_eq!(pos.next(), Some((0, 32)));
        assert_eq!(pos.next_back(), Some((63, 32)));

        let mut pos = CLIP.axis_0(32, 0, 63).unwrap();
        assert_eq!(pos.next(), Some((0, 32)));
        assert_eq!(pos.next_back(), Some((62, 32)));
    }

    #[test]
    fn axis_1_correct() {
        let mut neg = CLIP.axis_1(32, 76, -23).unwrap();
        assert_eq!(neg.next(), Some((32, 47)));
        assert_eq!(neg.next_back(), Some((32, 0)));

        let mut neg = CLIP.axis_1(32, 47, 0).unwrap();
        assert_eq!(neg.next(), Some((32, 47)));
        assert_eq!(neg.next_back(), Some((32, 1)));

        let mut pos = CLIP.axis_1(32, -23, 76).unwrap();
        assert_eq!(pos.next(), Some((32, 0)));
        assert_eq!(pos.next_back(), Some((32, 47)));

        let mut pos = CLIP.axis_1(32, 0, 47).unwrap();
        assert_eq!(pos.next(), Some((32, 0)));
        assert_eq!(pos.next_back(), Some((32, 46)));
    }
}
