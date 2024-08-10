use clipline::{Clip, Point};
use divan::black_box;
use std::fmt::{Display, Formatter};

type Num = i16;

const WX1: Num = 1022;
const WY1: Num = 1024;
const WX2: Num = 4032;
const WY2: Num = 4096;

const W1: Point<Num> = (WX1, WY1);
const W2: Point<Num> = (WX2, WY2);

const CLIP: Clip<Num> = match Clip::<Num>::new(W1, W2) {
    Some(clip) => clip,
    None => unreachable!(),
};

type Line = (Point<Num>, Point<Num>);

#[derive(Copy, Clone)]
struct Labeled(&'static str, Line);

impl Display for Labeled {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {:?}", self.0, self.1))
    }
}

const OUTSIDE_LEFT: Line = ((WX1 - 10, WY1), (WX1 - 1, WY2));
const OUTSIDE_RIGHT: Line = ((WX2 + 1, WY1), (WX2 + 10, WY2));
const OUTSIDE_ABOVE: Line = ((WX1, WY2 + 1), (WX2, WY2 + 10));
const OUTSIDE_BELOW: Line = ((WX1, WY1 - 10), (WX2, WY1 - 1));
const MISS_TOP_LEFT: Line = ((WX1 - 15, WY2 - 2), (WX1 + 2, WY2 + 20));
const MISS_BOTTOM_RIGHT: Line = ((WX2 - 10, WY1 - 8), (WX2 + 15, WY1 + 2));

const INSIDE_1: Point<Num> = (WX1, WY1);
const INSIDE_2: Point<Num> = (WX2, WY2);

const V_ENTRY: Point<Num> = (WX1 - 4, WY1);
const U_ENTRY: Point<Num> = (WX1, WY1 - 4);
const UV_ENTRY_1: Point<Num> = (WX1 - 9, WY1 - 3);

const V_EXIT: Point<Num> = (WX2 + 5, WY2);
const U_EXIT: Point<Num> = (WY2, WY2 + 5);
const UV_EXIT_1: Point<Num> = (WX2 + 16, WY2 + 2);

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

const REJECTS: [Labeled; 6] = [
    Labeled("OUTSIDE_LEFT", OUTSIDE_LEFT),
    Labeled("OUTSIDE_RIGHT", OUTSIDE_RIGHT),
    Labeled("OUTSIDE_ABOVE", OUTSIDE_ABOVE),
    Labeled("OUTSIDE_BELOW", OUTSIDE_BELOW),
    Labeled("MISS_TOP_LEFT", MISS_TOP_LEFT),
    Labeled("MISS_BOTTOM_RIGHT", MISS_BOTTOM_RIGHT),
];

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

fn draw_pixel_checked<T: Copy + Ord>(xy: Point<T>, clip: (Point<T>, Point<T>)) {
    let (x, y) = black_box(xy);
    let ((wx1, wy1), (wx2, wy2)) = black_box(clip);
    if x >= wx1 && x < wx2 && y >= wy1 && y < wy2 {
        black_box((x, y));
    }
}

fn draw_pixel_unchecked<T: Copy + Ord>(p: Point<T>) {
    black_box(p);
}

#[divan::bench_group]
mod rejects {
    use super::*;

    #[divan::bench(args = REJECTS)]
    fn clipline_012(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let line = black_box(((x1 as _, y1 as _), (x2 as _, y2 as _)));
        let clip = black_box(((WX1 as _, WY1 as _), (WX2 as _, WY2 as _)));
        clipline_1::clipline(line, clip, |x, y| draw_pixel_unchecked((x, y)));
    }

    #[divan::bench(args = REJECTS)]
    fn clipline_020(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let line = black_box(((x1, y1), (x2, y2)));
        let clip = black_box((W1, W2));
        clipline_2::clipline::<Num, _>(line, clip, |x, y| draw_pixel_unchecked((x, y)));
    }

    #[divan::bench(args = REJECTS)]
    fn clipline_030(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let p1 = black_box((x1, y1));
        let p2 = black_box((x2, y2));
        let clip = black_box(CLIP);
        if let Some(line) = clipline::AnyOctant::<Num>::clip(p1, p2, &clip) {
            line.for_each(draw_pixel_unchecked)
        }
    }

    #[divan::bench(args = REJECTS)]
    fn clipline_030_no_clip(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let p1 = black_box((x1, y1));
        let p2 = black_box((x2, y2));
        let clip = black_box((W1, W2));
        clipline::AnyOctant::<Num>::new(p1, p2).for_each(|(x, y)| draw_pixel_checked((x, y), clip));
    }

    #[divan::bench(args = REJECTS)]
    fn line_drawing(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let (p1, p2) = (black_box((x1, y1)), black_box((x2, y2)));
        let clip = black_box((W1, W2));
        line_drawing::Bresenham::new(p1, p2).for_each(|(x, y)| draw_pixel_checked((x, y), clip));
    }
}

#[divan::bench_group]
mod accepts {
    use super::*;

    #[divan::bench(args = ACCEPTS)]
    fn clipline_012(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let line = (black_box((x1 as _, y1 as _)), black_box((x2 as _, y2 as _)));
        let clip = (black_box((WX1 as _, WY1 as _)), black_box((WX2 as _, WY2 as _)));
        clipline_1::clipline(line, clip, |x, y| draw_pixel_unchecked((x, y)));
    }

    #[divan::bench(args = ACCEPTS)]
    fn clipline_020(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let line = (black_box((x1, y1)), black_box((x2, y2)));
        let clip = black_box((W1, W2));
        clipline_2::clipline::<Num, _>(line, clip, |x, y| draw_pixel_unchecked((x, y)));
    }

    #[divan::bench(args = ACCEPTS)]
    fn clipline_030(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let p1 = black_box((x1, y1));
        let p2 = black_box((x2, y2));
        let clip = black_box(CLIP);
        if let Some(line) = clipline::AnyOctant::<Num>::clip(p1, p2, &clip) {
            line.for_each(draw_pixel_unchecked)
        }
    }

    #[divan::bench(args = ACCEPTS)]
    fn clipline_030_no_clip(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let p1 = black_box((x1, y1));
        let p2 = black_box((x2, y2));
        let clip = black_box((W1, W2));
        clipline::AnyOctant::<Num>::new(p1, p2).for_each(|(x, y)| draw_pixel_checked((x, y), clip));
    }

    #[divan::bench(args = ACCEPTS)]
    fn line_drawing(Labeled(_, ((x1, y1), (x2, y2))): Labeled) {
        let (p1, p2) = (black_box((x1, y1)), black_box((x2, y2)));
        let clip = black_box((W1, W2));
        line_drawing::Bresenham::new(p1, p2).for_each(|(x, y)| draw_pixel_checked((x, y), clip));
    }
}

fn main() {
    divan::main();
}
