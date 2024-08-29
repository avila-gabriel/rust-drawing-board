use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut last_position: Option<(usize, usize)> = None;

    let mut window = Window::new(
        "Drawing Board - Click to draw",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut last_draw_position: Option<(usize, usize)> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&window, &mut last_position);

        if window.get_mouse_down(MouseButton::Left) {
            if let Some((last_x, last_y)) = last_position {
                if let Some((last_draw_x, last_draw_y)) = last_draw_position {
                    draw_line(&mut buffer, last_draw_x, last_draw_y, last_x, last_y);
                } else {
                    buffer[last_y * WIDTH + last_x] = 0xFFFFFF;
                }
                last_draw_position = Some((last_x, last_y));
            }
        } else {
            last_draw_position = None;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn handle_input(window: &Window, last_position: &mut Option<(usize, usize)>) {
    if window.get_mouse_down(MouseButton::Left) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let x = x as usize;
            let y = y as usize;
            if x < WIDTH && y < HEIGHT {
                *last_position = Some((x, y));
            }
        }
    }
}

fn draw_line(buffer: &mut Vec<u32>, x0: usize, y0: usize, x1: usize, y1: usize) {
    let dx = (x1 as isize - x0 as isize).abs();
    let dy = (y1 as isize - y0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;

    loop {
        if x0 >= 0 && x0 < WIDTH as isize && y0 >= 0 && y0 < HEIGHT as isize {
            buffer[y0 as usize * WIDTH + x0 as usize] = 0xFFFFFF;
        }

        if x0 == x1 as isize && y0 == y1 as isize { break; }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}