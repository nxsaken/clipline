use clipline::{Clip, Point};
use std::fmt::{Display, Formatter};

const WX1: i32 = 1022;
const WY1: i32 = 1024;
const WX2: i32 = 4032;
const WY2: i32 = 4096;

const W1: Point<i32> = (WX1, WY1);
const W2: Point<i32> = (WX2, WY2);

const CLIP: Clip<i32> = match Clip::<i32>::new(W1, W2) {
    Some(clip) => clip,
    None => unreachable!(),
};

type Line = (Point<i32>, Point<i32>);

#[derive(Copy, Clone)]
struct Labeled(&'static str, Line);

impl Display for Labeled {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {:?}", self.0, self.1))
    }
}

fn to_isize(((x1, y1), (x2, y2)): Line) -> (Point<isize>, Point<isize>) {
    ((x1 as isize, y1 as isize), (x2 as isize, y2 as isize))
}

const OUTSIDE_LEFT: Line = ((WX1 - 10, WY1), (WX1 - 1, WY2));
const OUTSIDE_RIGHT: Line = ((WX2 + 1, WY1), (WX2 + 10, WY2));
const OUTSIDE_ABOVE: Line = ((WX1, WY2 + 1), (WX2, WY2 + 10));
const OUTSIDE_BELOW: Line = ((WX1, WY1 - 10), (WX2, WY1 - 1));
const MISS_TOP_LEFT: Line = ((WX1 - 15, WY2 - 2), (WX1 + 2, WY2 + 20));
const MISS_BOTTOM_RIGHT: Line = ((WX2 - 10, WY1 - 8), (WX2 + 15, WY1 + 2));

const INSIDE_1: Point<i32> = (WX1, WY1);
const INSIDE_2: Point<i32> = (WX2, WY2);

const V_ENTRY: Point<i32> = (WX1 - 4, WY1);
const U_ENTRY: Point<i32> = (WX1, WY1 - 4);
const UV_ENTRY_1: Point<i32> = (WX1 - 9, WY1 - 3);

const V_EXIT: Point<i32> = (WX2 + 5, WY2);
const U_EXIT: Point<i32> = (WY2, WY2 + 5);
const UV_EXIT_1: Point<i32> = (WX2 + 16, WY2 + 2);

const INSIDE_INSIDE: Line = (INSIDE_1, INSIDE_2);
const INSIDE_V_EXIT: Line = (INSIDE_1, V_EXIT);
const INSIDE_U_EXIT: Line = (INSIDE_1, U_EXIT);
const INSIDE_UV_EXIT: Line = (INSIDE_1, UV_EXIT_1);
const V_ENTRY_INSIDE: Line = (V_ENTRY, INSIDE_2);
const V_ENTRY_V_EXIT: Line = (V_ENTRY, V_EXIT);
const V_ENTRY_U_EXIT: Line = (V_ENTRY, U_EXIT);
const V_ENTRY_UV_EXIT: Line = (V_ENTRY, UV_EXIT_1);
const U_ENTRY_INSIDE: Line = (U_ENTRY, INSIDE_2);
const U_ENTRY_V_EXIT: Line = (U_ENTRY, V_EXIT);
const U_ENTRY_U_EXIT: Line = (U_ENTRY, U_EXIT);
const U_ENTRY_UV_EXIT: Line = (U_ENTRY, UV_EXIT_1);
const UV_ENTRY_INSIDE: Line = (UV_ENTRY_1, INSIDE_2);
const UV_ENTRY_V_EXIT: Line = (UV_ENTRY_1, V_EXIT);
const UV_ENTRY_U_EXIT: Line = (UV_ENTRY_1, U_EXIT);
const UV_ENTRY_UV_EXIT: Line = (UV_ENTRY_1, UV_EXIT_1);

fn pixel<T>(x: T, y: T) {
    let _p = (divan::black_box(x), divan::black_box(y));
}

#[divan::bench_group]
mod rejects {
    use super::*;

    const REJECTS: [Labeled; 6] = [
        Labeled("OUTSIDE_LEFT", OUTSIDE_LEFT),
        Labeled("OUTSIDE_RIGHT", OUTSIDE_RIGHT),
        Labeled("OUTSIDE_ABOVE", OUTSIDE_ABOVE),
        Labeled("OUTSIDE_BELOW", OUTSIDE_BELOW),
        Labeled("MISS_TOP_LEFT", MISS_TOP_LEFT),
        Labeled("MISS_BOTTOM_RIGHT", MISS_BOTTOM_RIGHT),
    ];

    #[divan::bench(args = REJECTS)]
    fn clipline_1(line: Labeled) {
        let line = to_isize(line.1);
        let clip = to_isize((W1, W2));
        assert_eq!(
            clipline_1::clipline(divan::black_box(line), divan::black_box(clip), pixel),
            None
        );
    }

    #[divan::bench(args = REJECTS)]
    fn clipline_2(line: Labeled) {
        let line = line.1;
        assert_eq!(
            clipline_2::clipline(divan::black_box(line), divan::black_box((W1, W2)), pixel),
            None
        );
    }

    #[divan::bench(args = REJECTS)]
    fn clipline_3(line: Labeled) {
        let (p1, p2) = line.1;
        assert_eq!(
            clipline::Bresenham::<i32>::clip(
                divan::black_box(p1),
                divan::black_box(p2),
                divan::black_box(CLIP),
            ),
            None
        );
    }
}

#[divan::bench_group]
mod accepts {
    use super::*;

    const ACCEPTS: [Labeled; 16] = [
        Labeled("INSIDE_INSIDE", INSIDE_INSIDE),
        Labeled("INSIDE_V_EXIT", INSIDE_V_EXIT),
        Labeled("INSIDE_U_EXIT", INSIDE_U_EXIT),
        Labeled("INSIDE_UV_EXIT", INSIDE_UV_EXIT),
        Labeled("V_ENTRY_INSIDE", V_ENTRY_INSIDE),
        Labeled("V_ENTRY_V_EXIT", V_ENTRY_V_EXIT),
        Labeled("V_ENTRY_U_EXIT", V_ENTRY_U_EXIT),
        Labeled("V_ENTRY_UV_EXIT", V_ENTRY_UV_EXIT),
        Labeled("U_ENTRY_INSIDE", U_ENTRY_INSIDE),
        Labeled("U_ENTRY_V_EXIT", U_ENTRY_V_EXIT),
        Labeled("U_ENTRY_U_EXIT", U_ENTRY_U_EXIT),
        Labeled("U_ENTRY_UV_EXIT", U_ENTRY_UV_EXIT),
        Labeled("UV_ENTRY_INSIDE", UV_ENTRY_INSIDE),
        Labeled("UV_ENTRY_V_EXIT", UV_ENTRY_V_EXIT),
        Labeled("UV_ENTRY_U_EXIT", UV_ENTRY_U_EXIT),
        Labeled("UV_ENTRY_UV_EXIT", UV_ENTRY_UV_EXIT),
    ];

    #[divan::bench(args = ACCEPTS)]
    fn clipline_1(line: Labeled) {
        let line = divan::black_box(line.1);
        let line = to_isize(line);
        let clip = to_isize((W1, W2));
        clipline_1::clipline(line, divan::black_box(clip), pixel).unwrap();
    }

    #[divan::bench(args = ACCEPTS)]
    fn clipline_2(line: Labeled) {
        let line = divan::black_box(line.1);
        clipline_2::clipline(line, divan::black_box((W1, W2)), pixel).unwrap();
    }

    #[divan::bench(args = ACCEPTS)]
    fn clipline_3(line: Labeled) {
        let (p1, p2) = divan::black_box(line.1);
        clipline::Bresenham::<i32>::clip(p1, p2, divan::black_box(CLIP))
            .unwrap()
            .for_each(|(x, y)| pixel(x, y));
    }
}

#[divan::bench_group]
mod unclipped {
    use super::*;

    const ACCEPTS: [Labeled; 16] = [
        Labeled("INSIDE_INSIDE", INSIDE_INSIDE),
        Labeled("INSIDE_V_EXIT", INSIDE_V_EXIT),
        Labeled("INSIDE_U_EXIT", INSIDE_U_EXIT),
        Labeled("INSIDE_UV_EXIT", INSIDE_UV_EXIT),
        Labeled("V_ENTRY_INSIDE", V_ENTRY_INSIDE),
        Labeled("V_ENTRY_V_EXIT", V_ENTRY_V_EXIT),
        Labeled("V_ENTRY_U_EXIT", V_ENTRY_U_EXIT),
        Labeled("V_ENTRY_UV_EXIT", V_ENTRY_UV_EXIT),
        Labeled("U_ENTRY_INSIDE", U_ENTRY_INSIDE),
        Labeled("U_ENTRY_V_EXIT", U_ENTRY_V_EXIT),
        Labeled("U_ENTRY_U_EXIT", U_ENTRY_U_EXIT),
        Labeled("U_ENTRY_UV_EXIT", U_ENTRY_UV_EXIT),
        Labeled("UV_ENTRY_INSIDE", UV_ENTRY_INSIDE),
        Labeled("UV_ENTRY_V_EXIT", UV_ENTRY_V_EXIT),
        Labeled("UV_ENTRY_U_EXIT", UV_ENTRY_U_EXIT),
        Labeled("UV_ENTRY_UV_EXIT", UV_ENTRY_UV_EXIT),
    ];

    #[divan::bench(args = ACCEPTS)]
    fn clipline_raw(line: Labeled) {
        let (p1, p2) = divan::black_box(line.1);
        clipline::Bresenham::<i32>::new(p1, p2).for_each(|(x, y)| pixel(x, y));
    }

    #[divan::bench(args = ACCEPTS)]
    fn line_drawing(line: Labeled) {
        let (p1, p2) = divan::black_box(line.1);
        line_drawing::Bresenham::new(p1, p2).for_each(|(x, y)| pixel(x, y));
    }
}

fn main() {
    divan::main();
}
