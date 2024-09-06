use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let (sender, receiver) = mpsc::channel::<Option<(usize, usize)>>();
    let buffer = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));

    let mut window = Window::new(
        "Drawing Board - Click to draw",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let buffer_clone = Arc::clone(&buffer);
    let drawing_thread_handle = thread::spawn(move || {
        let mut last_draw_position: Option<(usize, usize)> = None;
        while let Ok(pos) = receiver.recv() {
            if pos.is_none() {
                last_draw_position = None;
                continue;
            }

            let (x, y) = pos.unwrap();
            let mut buffer = buffer_clone.lock().unwrap();
            if let Some((last_x, last_y)) = last_draw_position {
                draw_line(&mut buffer, last_x, last_y, x, y);
            } else {
                buffer[y * WIDTH + x] = 0xFFFFFF;
            }
            last_draw_position = Some((x, y));
        }
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(MouseButton::Left) {
            handle_input(&window, &sender);
        } else {
            sender.send(None).unwrap();
        }

        window.update_with_buffer(&buffer.lock().unwrap(), WIDTH, HEIGHT).unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    drop(sender);
    drawing_thread_handle.join().unwrap();
}

fn handle_input(window: &Window, sender: &mpsc::Sender<Option<(usize, usize)>>) {
    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        let x = x as usize;
        let y = y as usize;
        if x < WIDTH && y < HEIGHT {
            sender.send(Some((x, y))).unwrap();
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
