//! Bresenham line segment iterator tests.

use clipline::{
    Bresenham, BresenhamOctant0, BresenhamOctant1, BresenhamOctant2, BresenhamOctant3,
    BresenhamOctant4, BresenhamOctant5, BresenhamOctant6, BresenhamOctant7,
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
    fn covers_all_domain() {
        for x1 in i8::MIN..=i8::MAX {
            for y1 in i8::MIN..=i8::MAX {
                for x2 in i8::MIN..=i8::MAX {
                    for y2 in i8::MIN..=i8::MAX {
                        let length = u8::max(x1.abs_diff(x2), y1.abs_diff(y2));
                        assert_eq!(Bresenham::new((x1, y1), (x2, y2)).length(), length);
                    }
                }
            }
        }
    }

    #[test]
    fn octant_0_produces_correct_points() {
        let points = vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2)];
        assert_eq!(BresenhamOctant0::new((0, 0), (5, 2)).unwrap().collect::<Vec<_>>(), points);
    }
}

mod clip {
    use clipline::Clip;

    mod octant_0 {
        use super::*;
        use clipline::BresenhamOctant0;

        #[test]
        fn line_1111() {
            let clip = Clip::new((16, 16), (25, 20)).unwrap();
            for (start, end, clipped) in [
                // top-right
                ((7, 11), (31, 22), Some(((17, 16), (24, 19)))),
                // left-bottom
                ((7, 13), (34, 25), Some(((16, 17), (22, 20)))),
                // top-bottom
                ((8, 9), (34, 27), Some(((18, 16), (23, 19)))),
                // left-right
                ((0, 15), (47, 21), Some(((16, 17), (24, 18)))),
                // rejects
                ((9, 6), (38, 22), None),
                ((2, 14), (30, 30), None),
            ] {
                println!("{start:?} -> {end:?}");
                let mut line1 = BresenhamOctant0::new(start, end).unwrap();
                println!("raw points: {:?}", line1.clone().collect::<Vec<_>>());
                let line2 = BresenhamOctant0::clip(start, end, clip);
                assert_eq!(line2.is_some(), clipped.is_some());
                if let Some((c_start, c_end)) = clipped {
                    let line_points = line2.clone().unwrap().collect::<Vec<_>>();
                    println!("clipped points: {line_points:?}");
                    assert_eq!(line_points.first().copied(), Some(c_start));
                    assert_eq!(line_points.last().copied(), Some(c_end));

                    line1.nth(isize::abs_diff(c_start.0 as _, start.0 as _) - 1);
                    println!("raw skipped: {line1:?}");
                    println!("clipped: {line2:?}");
                    println!();
                }
            }
        }
    }
}
