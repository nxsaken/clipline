//! Axis-aligned iterator tests

mod bounds {
    use clipline::*;

    #[test]
    fn empty_line_segments_are_positive() {
        assert!(PositiveAxis0::<u8>::new(0, 0, 0).is_some_and(|me| me.is_done()));
        assert!(PositiveAxis1::<u8>::new(0, 0, 0).is_some_and(|me| me.is_done()));
        assert!(NegativeAxis0::<u8>::new(0, 0, 0).is_none());
        assert!(NegativeAxis1::<u8>::new(0, 0, 0).is_none());
    }
}

mod iter {
    use clipline::*;

    #[test]
    fn length_is_correct() {
        for v1 in -2..4 {
            for v2 in -2..4 {
                let length = i8::abs_diff(v1, v2);
                assert_eq!(Axis0::<i8>::new(0, v1, v2).length(), length);
                assert_eq!(Axis1::<i8>::new(0, v1, v2).length(), length);
            }
        }
    }

    #[test]
    fn coordinate_order_is_correct() {
        let points = [-2, -1, 0, 1];
        Axis0::<i8>::new(0, -2, 2).enumerate().for_each(|(i, (x, y))| {
            assert_eq!(x, points[i]);
            assert_eq!(y, 0);
        });
        Axis1::<i8>::new(0, -2, 2).enumerate().for_each(|(i, (x, y))| {
            assert_eq!(x, 0);
            assert_eq!(y, points[i]);
        });
    }

    #[test]
    fn direction_is_correct() {
        Axis0::<i8>::new(0, -2, 2)
            .enumerate()
            .for_each(|(i, (x, _))| assert_eq!(x, [-2, -1, 0, 1][i]));
        Axis0::<i8>::new(0, 2, -2)
            .enumerate()
            .for_each(|(i, (x, _))| assert_eq!(x, [2, 1, 0, -1][i]));
        Axis1::<i8>::new(0, -2, 2)
            .enumerate()
            .for_each(|(i, (_, y))| assert_eq!(y, [-2, -1, 0, 1][i]));
        Axis1::<i8>::new(0, 2, -2)
            .enumerate()
            .for_each(|(i, (_, y))| assert_eq!(y, [2, 1, 0, -1][i]));
    }

    #[test]
    fn positive_reverse_is_correct() {
        let mut rev = Axis0::<i8>::new(0, 0, 5).rev().collect::<Vec<_>>();
        rev.reverse();
        assert_eq!(Axis0::<i8>::new(0, 0, 5).collect::<Vec<_>>(), rev);
    }

    #[test]
    fn positive_double_ended_iteration_is_correct() {
        let mut line = Axis0::<i8>::new(0, 0, 2);
        assert_eq!(line.next_back(), Some((1, 0)));
        assert_eq!(line.next(), Some((0, 0)));
        assert!(line.is_done());
    }

    #[test]
    fn negative_reverse_is_correct() {
        let mut rev = Axis0::<i8>::new(0, 5, 0).rev().collect::<Vec<_>>();
        rev.reverse();
        assert_eq!(Axis0::<i8>::new(0, 5, 0).collect::<Vec<_>>(), rev);
    }

    #[test]
    fn negative_double_ended_iteration_is_correct() {
        let mut line = Axis0::<i8>::new(0, 0, -2);
        assert_eq!(line.next_back(), Some((-1, 0)));
        assert_eq!(line.next(), Some((0, 0)));
        assert!(line.is_done());
    }
}

mod clip {
    use clipline::Clip;

    const CLIP: Clip<i8> = Clip::<i8>::new((0, 0), (63, 47)).unwrap();

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

#[cfg(test)]
mod convert {
    use clipline::*;

    const PA0_SIGNED: PositiveAxis0<u8> = PositiveAxis0::<u8>::new(0, 0, 1).unwrap();
    const NA0_SIGNED: NegativeAxis0<u8> = NegativeAxis0::<u8>::new(0, 2, 0).unwrap();
    const PA1_SIGNED: PositiveAxis1<u8> = PositiveAxis1::<u8>::new(0, 0, 3).unwrap();
    const NA1_SIGNED: NegativeAxis1<u8> = NegativeAxis1::<u8>::new(0, 4, 0).unwrap();

    const PA0_AXIS: Axis0<u8> = Axis::Positive(PA0_SIGNED);
    const NA0_AXIS: Axis0<u8> = Axis::Negative(NA0_SIGNED);
    const PA1_AXIS: Axis1<u8> = Axis::Positive(PA1_SIGNED);
    const NA1_AXIS: Axis1<u8> = Axis::Negative(NA1_SIGNED);

    const PA0_ANY: AnyAxis<u8> = AnyAxis::PositiveAxis0(PA0_SIGNED);
    const NA0_ANY: AnyAxis<u8> = AnyAxis::NegativeAxis0(NA0_SIGNED);
    const PA1_ANY: AnyAxis<u8> = AnyAxis::PositiveAxis1(PA1_SIGNED);
    const NA1_ANY: AnyAxis<u8> = AnyAxis::NegativeAxis1(NA1_SIGNED);

    #[test]
    fn signed_axis_into_axis() {
        assert_eq!(PA0_SIGNED.into_axis(), PA0_AXIS);
        assert_eq!(PA1_SIGNED.into_axis(), PA1_AXIS);
        assert_eq!(NA0_SIGNED.into_axis(), NA0_AXIS);
        assert_eq!(NA1_SIGNED.into_axis(), NA1_AXIS);
    }

    #[test]
    fn signed_axis_into_any_axis() {
        assert_eq!(PA0_SIGNED.into_any_axis(), PA0_ANY);
        assert_eq!(PA1_SIGNED.into_any_axis(), PA1_ANY);
        assert_eq!(NA0_SIGNED.into_any_axis(), NA0_ANY);
        assert_eq!(NA1_SIGNED.into_any_axis(), NA1_ANY);
    }

    #[test]
    fn axis_into_signed_axis() {
        assert_eq!(PA0_AXIS.try_into_signed_axis::<false>(), Some(PA0_SIGNED));
        assert_eq!(NA0_AXIS.try_into_signed_axis::<true>(), Some(NA0_SIGNED));
        assert_eq!(PA1_AXIS.try_into_signed_axis::<false>(), Some(PA1_SIGNED));
        assert_eq!(NA1_AXIS.try_into_signed_axis::<true>(), Some(NA1_SIGNED));

        assert_eq!(PA0_AXIS.try_into_signed_axis::<true>(), None);
        assert_eq!(NA0_AXIS.try_into_signed_axis::<false>(), None);
        assert_eq!(PA1_AXIS.try_into_signed_axis::<true>(), None);
        assert_eq!(NA1_AXIS.try_into_signed_axis::<false>(), None);
    }

    #[test]
    fn axis_into_any_axis() {
        assert_eq!(PA0_AXIS.into_any_axis(), PA0_ANY);
        assert_eq!(NA0_AXIS.into_any_axis(), NA0_ANY);
        assert_eq!(PA1_AXIS.into_any_axis(), PA1_ANY);
        assert_eq!(NA1_AXIS.into_any_axis(), NA1_ANY);
    }

    #[test]
    fn any_axis_into_signed_axis() {
        assert_eq!(PA0_ANY.try_into_signed_axis::<false, false>(), Some(PA0_SIGNED));
        assert_eq!(NA0_ANY.try_into_signed_axis::<true, false>(), Some(NA0_SIGNED));
        assert_eq!(PA1_ANY.try_into_signed_axis::<false, true>(), Some(PA1_SIGNED));
        assert_eq!(NA1_ANY.try_into_signed_axis::<true, true>(), Some(NA1_SIGNED));

        assert_eq!(PA0_ANY.try_into_signed_axis::<true, false>(), None);
        assert_eq!(NA0_ANY.try_into_signed_axis::<false, false>(), None);
        assert_eq!(PA1_ANY.try_into_signed_axis::<true, true>(), None);
        assert_eq!(NA1_ANY.try_into_signed_axis::<false, true>(), None);
    }

    #[test]
    fn any_axis_into_axis() {
        assert_eq!(PA0_ANY.try_into_axis::<false>(), Some(PA0_AXIS));
        assert_eq!(NA0_ANY.try_into_axis::<false>(), Some(NA0_AXIS));
        assert_eq!(PA1_ANY.try_into_axis::<true>(), Some(PA1_AXIS));
        assert_eq!(NA1_ANY.try_into_axis::<true>(), Some(NA1_AXIS));

        assert_eq!(PA0_ANY.try_into_axis::<true>(), None);
        assert_eq!(NA0_ANY.try_into_axis::<true>(), None);
        assert_eq!(PA1_ANY.try_into_axis::<false>(), None);
        assert_eq!(NA1_ANY.try_into_axis::<false>(), None);
    }

    #[test]
    fn try_from_impls() {
        assert_eq!(PositiveAxis0::<u8>::try_from(PA0_AXIS), Ok(PA0_SIGNED));
        assert_eq!(NegativeAxis0::<u8>::try_from(PA0_AXIS), Err(()));

        assert_eq!(Axis0::<u8>::try_from(PA0_ANY), Ok(PA0_AXIS));
        assert_eq!(Axis1::<u8>::try_from(PA0_ANY), Err(()));

        assert_eq!(PositiveAxis0::<u8>::try_from(PA0_ANY), Ok(PA0_SIGNED));
        assert_eq!(NegativeAxis0::<u8>::try_from(PA0_ANY), Err(()));
    }
}
