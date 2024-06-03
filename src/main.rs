use commands::Command;
use minifb::{Window, WindowOptions};
use std::{
    process::ExitCode,
    sync::mpsc::{self, Sender},
};

mod cli;
mod commands;

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
    let (cli_tx, cli_rx) = mpsc::channel();
    cli::start(cli_tx);

    // Draw buffer
    while window.is_open() {
        while let Ok(command) = cli_rx.try_recv() {
            match command {
                Command::Exit => return,
                Command::SetCellData(x, y, color) => {
                    buffer[y * WIDTH + x] = color;
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
