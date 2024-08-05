//! Bresenham line segment iterator tests.

use clipline::{
    BresenhamOctant0, BresenhamOctant1, BresenhamOctant2, BresenhamOctant3, BresenhamOctant4,
    BresenhamOctant5, BresenhamOctant6, BresenhamOctant7,
};

mod octant_bounds {
    use super::*;

    #[test]
    fn all_octants_exclude_empty_line() {
        assert!(BresenhamOctant0::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant1::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant2::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant3::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant4::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant5::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant6::<i8>::new((0, 0), (0, 0)).is_none());
        assert!(BresenhamOctant7::<i8>::new((0, 0), (0, 0)).is_none());
    }

    #[test]
    fn octant_0_excludes_0_and_includes_45_degrees() {
        assert!(BresenhamOctant0::<i8>::new((0, 0), (1, 0)).is_none());
        assert!(BresenhamOctant0::<i8>::new((0, 0), (1, 1)).is_some());
    }

    #[test]
    fn octant_1_excludes_45_degrees_and_excludes_90_degrees() {
        assert!(BresenhamOctant1::<i8>::new((0, 0), (1, 1)).is_none());
        assert!(BresenhamOctant1::<i8>::new((0, 0), (0, 1)).is_none());
    }

    #[test]
    fn octant_2_includes_315_degrees_and_excludes_0_degrees() {
        assert!(BresenhamOctant2::<i8>::new((0, 0), (1, -1)).is_some());
        assert!(BresenhamOctant2::<i8>::new((0, 0), (1, 0)).is_none());
    }

    #[test]
    fn octant_3_excludes_270_degrees_and_excludes_315_degrees() {
        assert!(BresenhamOctant3::<i8>::new((0, 0), (0, -1)).is_none());
        assert!(BresenhamOctant3::<i8>::new((0, 0), (1, -1)).is_none());
    }

    #[test]
    fn octant_4_includes_135_and_excludes_180_degrees() {
        assert!(BresenhamOctant4::<i8>::new((0, 0), (-1, 1)).is_some());
        assert!(BresenhamOctant4::<i8>::new((0, 0), (-1, 0)).is_none());
    }

    #[test]
    fn octant_5_excludes_90_and_excludes_135_degrees() {
        assert!(BresenhamOctant5::<i8>::new((0, 0), (0, 1)).is_none());
        assert!(BresenhamOctant5::<i8>::new((0, 0), (-1, 1)).is_none());
    }

    #[test]
    fn octant_6_excludes_180_degrees_and_includes_225_degrees() {
        assert!(BresenhamOctant6::<i8>::new((0, 0), (-1, 0)).is_none());
        assert!(BresenhamOctant6::<i8>::new((0, 0), (-1, -1)).is_some());
    }

    #[test]
    fn octant_7_excludes_225_degrees_and_excludes_270_degrees() {
        assert!(BresenhamOctant7::<i8>::new((0, 0), (-1, -1)).is_none());
        assert!(BresenhamOctant7::<i8>::new((0, 0), (0, -1)).is_none());
    }
}

mod rasterization {
    use super::*;

    #[test]
    fn octant_0_produces_correct_points() {
        let points = vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2)];
        assert_eq!(
            BresenhamOctant0::<i8>::new((0, 0), (5, 2)).unwrap().collect::<Vec<_>>(),
            points
        );
    }
}

mod clip {
    use clipline::Clip;

    mod octant_0 {
        use super::*;
        use clipline::BresenhamOctant0;

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
                let line_raw = BresenhamOctant0::<i8>::new(start, end).unwrap();
                let line_clip = BresenhamOctant0::<i8>::clip(start, end, &clip);
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
