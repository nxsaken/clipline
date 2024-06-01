use bresenham::Bresenham as BresenhamA;
use clipline::{clipline, Clipline};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use line_drawing::Bresenham as BresenhamB;

type Point = (isize, isize);
type Line = (Point, Point);
type Rect = (Point, Point);

fn draw_pixel_checked(p: Point, clip: Rect) {
    let (x, y) = p;
    let ((wx1, wy1), (wx2, wy2)) = clip;
    if black_box(x >= wx1 && x < wx2 && y >= wy1 && y < wy2) {
        black_box((x, y));
    }
}

fn draw_pixel_unchecked(p: Point) {
    black_box(p);
}

fn line_a(line: Line, clip: Rect) {
    let (p1, p2) = line;
    for p in BresenhamA::new(black_box(p1), black_box(p2)) {
        draw_pixel_checked(p, clip)
    }
}

fn line_b(line: Line, clip: Rect) {
    let (p1, p2) = line;
    for p in BresenhamB::new(black_box(p1), black_box(p2)) {
        draw_pixel_checked(p, clip)
    }
}

fn clipline_a(line: Line, clip: Rect) {
    clipline(black_box(line), black_box(clip), |x, y| {
        draw_pixel_unchecked((x, y))
    });
}

fn clipline_b(line: Line, clip: Rect) {
    if let Some(clipline) = Clipline::new(black_box(line), black_box(clip)) {
        for p in clipline {
            draw_pixel_unchecked(p)
        }
    }
}

fn clipline_c(line: Line, clip: Rect) {
    if let Some(clipline) = Clipline::new(black_box(line), black_box(clip)) {
        match clipline {
            Clipline::Vlipline(ln) => ln.for_each(draw_pixel_unchecked),
            Clipline::Hlipline(ln) => ln.for_each(draw_pixel_unchecked),
            Clipline::Gentleham(ln) => ln.for_each(draw_pixel_unchecked),
            Clipline::Steepnham(ln) => ln.for_each(draw_pixel_unchecked),
        }
    }
}

fn bench_lines(c: &mut Criterion) {
    let cases = [1, 8, 32].iter().flat_map(|mult| {
        let (w, h) = (160 * mult, 160 * mult);
        [
            (
                format!("{w}x{h}_inside_vert"),
                (w, h),
                ((w / 2, 0), (w / 2, h - 1)),
            ),
            (
                format!("{w}x{h}_inside_hor"),
                (w, h),
                ((0, h / 2), (w - 1, h / 2)),
            ),
            (
                format!("{w}x{h}_inside_gentle"),
                (w, h),
                ((0, h - 1), (w / 2, 0)),
            ),
            (
                format!("{w}x{h}_inside_steep"),
                (w, h),
                ((0, h - 1), (w - 1, h / 2)),
            ),
            (format!("{w}x{h}_outside"), (w, h), ((0, h), (w - 1, 2 * h))),
        ]
    });
    for (case_name, clip_size, line) in cases {
        let mut group = c.benchmark_group(case_name);
        for num_lines in [1, 1024, 8192, 32768] {
            group.throughput(Throughput::Elements(num_lines));
            group.bench_with_input(
                BenchmarkId::new("bresenham", num_lines),
                &num_lines,
                |b, &num_lines| {
                    b.iter(|| {
                        for _ in 0..num_lines {
                            line_a(line, ((0, 0), clip_size));
                        }
                    });
                },
            );
            group.bench_with_input(
                BenchmarkId::new("line_drawing", num_lines),
                &num_lines,
                |b, &num_lines| {
                    b.iter(|| {
                        for _ in 0..num_lines {
                            line_b(line, ((0, 0), clip_size));
                        }
                    });
                },
            );
            group.bench_with_input(
                BenchmarkId::new("clipline(fn)", num_lines),
                &num_lines,
                |b, &num_lines| {
                    b.iter(|| {
                        for _ in 0..num_lines {
                            clipline_a(line, ((0, 0), clip_size));
                        }
                    });
                },
            );
            group.bench_with_input(
                BenchmarkId::new("Clipline(iter)", num_lines),
                &num_lines,
                |b, &num_lines| {
                    b.iter(|| {
                        for _ in 0..num_lines {
                            clipline_b(line, ((0, 0), clip_size));
                        }
                    });
                },
            );
            group.bench_with_input(
                BenchmarkId::new("Clipline(match-iter)", num_lines),
                &num_lines,
                |b, &num_lines| {
                    b.iter(|| {
                        for _ in 0..num_lines {
                            clipline_c(line, ((0, 0), clip_size));
                        }
                    });
                },
            );
        }
        group.finish()
    }
}

criterion_group!(benches, bench_lines);
criterion_main!(benches);
