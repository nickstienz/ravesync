use minifb::{Window, WindowOptions};
use std::sync::mpsc;
use std::thread;

const WIDTH: usize = 10;
const HEIGHT: usize = 4;
const CELL_SIZE: usize = 20;

fn main() {
    // Setup window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "RaveSync",
        WIDTH * CELL_SIZE,
        HEIGHT * CELL_SIZE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_target_fps(60);

    // Handle user input
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 3 {
            if let (Ok(x), Ok(y), Ok(color)) = (
                parts[0].parse::<usize>(),
                parts[1].parse::<usize>(),
                u32::from_str_radix(parts[2], 16),
            ) {
                if x < WIDTH && y < HEIGHT && tx.send((x, y, color)).is_err() {
                    break;
                }
            }
        }
    });

    // Draw buffer
    while window.is_open() {
        while let Ok((x, y, color)) = rx.try_recv() {
            buffer[y * WIDTH + x] = color;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
