use clipline::Clip;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

const CLIP: Clip<i8> = match Clip::new((-32, -32), (32, 32)) {
    Some(clip) => clip,
    None => panic!(""),
};

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

    let mut t = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (x1, y1, x2, y2) = generate_line_segment(t);
        let (x3, y3, x4, y4) = generate_line_segment(t - 15.0);

        let p1 = (x1, y1);
        let p2 = (x2, y2);
        let p3 = (x3, y3);
        let p4 = (x4, y4);

        buffer.fill(0);

        draw_rectangle(&mut buffer);

        for (p, q) in [(p1, p2), (p2, p3), (p3, p4), (p4, p1)] {
            clipline::Bresenham::new(p, q).for_each(|(x, y)| {
                let x = i8::abs_diff(x, -64);
                let y = i8::abs_diff(y, -64);
                let index = y as usize * WIDTH + x as usize;
                buffer[index] = 0xFF0000;
            });
            let Some(line) = clipline::BresenhamOctant0::clip(p, q, CLIP) else {
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

        t += 4.0 / 64.0;
    }
}

fn draw_rectangle(buffer: &mut [u32]) {
    let start_x = (-32 + 64) as usize;
    let start_y = (-32 + 64) as usize;
    let end_x = (32 + 64) as usize;
    let end_y = (32 + 64) as usize;

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            let index = y * WIDTH + x;
            if index < buffer.len() {
                buffer[index] = 0x0000FF;
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
