//! Octant line segment iterator tests.

use clipline::{Octant0, Octant1, Octant2, Octant3, Octant4, Octant5, Octant6, Octant7};

mod octant_bounds {
    use super::*;

    #[test]
    fn all_octants_exclude_empty_line() {
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
    fn octant_0_excludes_0_and_includes_45_degrees() {
        assert!(Octant0::<i8>::new((0, 0), (1, 0)).is_none());
        assert!(Octant0::<i8>::new((0, 0), (1, 1)).is_some());
    }

    #[test]
    fn octant_1_excludes_45_degrees_and_excludes_90_degrees() {
        assert!(Octant1::<i8>::new((0, 0), (1, 1)).is_none());
        assert!(Octant1::<i8>::new((0, 0), (0, 1)).is_none());
    }

    #[test]
    fn octant_2_includes_315_degrees_and_excludes_0_degrees() {
        assert!(Octant2::<i8>::new((0, 0), (1, -1)).is_some());
        assert!(Octant2::<i8>::new((0, 0), (1, 0)).is_none());
    }

    #[test]
    fn octant_3_excludes_270_degrees_and_excludes_315_degrees() {
        assert!(Octant3::<i8>::new((0, 0), (0, -1)).is_none());
        assert!(Octant3::<i8>::new((0, 0), (1, -1)).is_none());
    }

    #[test]
    fn octant_4_includes_135_and_excludes_180_degrees() {
        assert!(Octant4::<i8>::new((0, 0), (-1, 1)).is_some());
        assert!(Octant4::<i8>::new((0, 0), (-1, 0)).is_none());
    }

    #[test]
    fn octant_5_excludes_90_and_excludes_135_degrees() {
        assert!(Octant5::<i8>::new((0, 0), (0, 1)).is_none());
        assert!(Octant5::<i8>::new((0, 0), (-1, 1)).is_none());
    }

    #[test]
    fn octant_6_excludes_180_degrees_and_includes_225_degrees() {
        assert!(Octant6::<i8>::new((0, 0), (-1, 0)).is_none());
        assert!(Octant6::<i8>::new((0, 0), (-1, -1)).is_some());
    }

    #[test]
    fn octant_7_excludes_225_degrees_and_excludes_270_degrees() {
        assert!(Octant7::<i8>::new((0, 0), (-1, -1)).is_none());
        assert!(Octant7::<i8>::new((0, 0), (0, -1)).is_none());
    }
}

mod iterator {
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

mod clip {
    use clipline::{AnyOctant, Clip};

    mod octant_0 {
        use super::*;
        use clipline::Octant0;

        #[test]
        fn line_1111() {
            let clip = Clip::<i8>::new((16, 16), (25, 20)).unwrap();
            for (start, end, clipped) in [
                // top-right
                ((7, 11), (31, 22), Some(((17, 16), (25, 19)))),
                // left-bottom
                ((7, 13), (34, 25), Some(((16, 17), (23, 20)))),
                // top-bottom
                ((8, 9), (34, 27), Some(((18, 16), (24, 20)))),
                // left-right
                ((0, 15), (47, 21), Some(((16, 17), (25, 18)))),
                // rejects
                ((9, 6), (38, 22), None),
                ((2, 14), (30, 30), None),
            ] {
                let line_raw = Octant0::<i8>::new(start, end).unwrap();
                let line_clip = Octant0::<i8>::clip(start, end, &clip);
                if let Some((clip_start, clip_end)) = clipped {
                    let skip_len = i8::abs_diff(clip_start.0, start.0) as usize;
                    let line_raw = line_raw.skip(skip_len);
                    let line_clip = line_clip.unwrap();
                    let mut pair = line_raw.zip(line_clip);
                    let (actual_raw_start, actual_clip_start) = pair.next().unwrap();
                    assert_eq!(actual_raw_start, actual_clip_start);
                    assert_eq!(actual_clip_start, clip_start);
                    let mut pair = pair.peekable();
                    while let Some((raw, clip)) = pair.next() {
                        assert_eq!(raw, clip);
                        if pair.peek().is_none() {
                            assert_eq!(clip, clip_end);
                        }
                    }
                } else {
                    assert!(line_clip.is_none())
                }
            }
        }
    }
}
