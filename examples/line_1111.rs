use clipline::Clip;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::f32::consts::TAU;

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

const BPM: f32 = 80.0;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Line 1111",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");
    window.set_target_fps(60);
    window.set_background_color(0, 0, 0);

    let mut t: f32 = 0.0;

    let (center_x, center_y) = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let t0_sin = (t.sin() + 1.0) / 2.0;
        let t0_cos = (t.cos() + 1.0) / 2.0;
        let (x1, y1, x2, y2) = generate_line_segment_2(t, BPM, t0_sin * t0_cos * 0.25);
        let (x3, y3, x4, y4) = generate_line_segment_2(t, BPM, t0_sin * t0_cos * 0.5);
        let (x5, y5, x6, y6) = generate_line_segment_2(t, BPM, t0_sin * t0_cos * 1.00);

        let p1 = (x1, y1);
        let p2 = (x2, y2);
        let p3 = (x3, y3);
        let p4 = (x4, y4);
        let p5 = (x5, y5);
        let p6 = (x6, y6);

        buffer.fill(0);

        let c1 = (center_x - 32, center_y - 32);
        let c2 = (center_x + 32, center_y + 32);

        let clip = Clip::new(c1, c2).unwrap();
        draw_rectangle(&mut buffer, c1, c2);

        for (p, q) in [
            (p1, p2),
            (p2, p3),
            (p3, p4),
            (p4, p5),
            (p5, p6),
            (p6, p1),
            (p1, p3),
            (p2, p4),
            (p3, p5),
            (p4, p6),
            (p5, p1),
            (p6, p2),
            (p1, p4),
            (p2, p5),
            (p3, p6),
        ] {
            clipline::Bresenham::new(p, q).for_each(|(x, y)| {
                let x = i8::abs_diff(x, -64);
                let y = i8::abs_diff(y, -64);
                let index = y as usize * WIDTH + x as usize;
                buffer[index] = 0xFF0000;
            });
            let Some(line) = clipline::Bresenham::clip(p, q, clip) else {
                continue;
            };
            line.for_each(|(x, y)| {
                let x = i8::abs_diff(x, -64);
                let y = i8::abs_diff(y, -64);
                let index = y as usize * WIDTH + x as usize;
                buffer[index] = 0x00FF00;
            });
        }

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        t += 1.0 / 60.0;
    }
}

fn draw_rectangle(buffer: &mut [u32], c1: (i8, i8), c2: (i8, i8)) {
    let start_x = i8::abs_diff(c1.0, -64) as usize;
    let start_y = i8::abs_diff(c1.1, -64) as usize;
    let end_x = i8::abs_diff(c2.0, -64) as usize;
    let end_y = i8::abs_diff(c2.1, -64) as usize;

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            let index = y * WIDTH + x;
            if index < buffer.len() {
                buffer[index] = 0x00008F;
            }
        }
    }
}

fn generate_line_segment(time: f32) -> (i8, i8, i8, i8) {
    let sin_t = (time.sin() + 1.0) / 2.0;
    let cos_t = (time.cos() + 1.0) / 2.0;

    let x1 = -64;
    let y1 = (-64.0 + 30.0 * sin_t).floor() as i8;

    let x2 = 63;
    let y2 = (63.0 - 30.0 * cos_t).floor() as i8;

    (x1, y1, x2, y2)
}

fn generate_line_segment_2(time: f32, bpm: f32, phase: f32) -> (i8, i8, i8, i8) {
    let f = bpm / 60.0;
    let angular_f = TAU * f;
    let phase_shift = TAU * phase;

    let sin_t = (time * angular_f + phase_shift).sin();
    let cos_t = (time * angular_f + phase_shift).cos();

    let x1 = (63.9 * cos_t).floor() as i8;
    let y1 = (63.9 * sin_t).floor() as i8;

    let x2 = (63.9 * sin_t * cos_t).floor() as i8;
    let y2 = (63.9 * cos_t * cos_t).floor() as i8;

    (x1, y1, x2, y2)
}
