use std::io::{self, BufRead};
use std::sync::mpsc::Sender;
use std::thread;

use crate::commands::Command;
use crate::{HEIGHT, WIDTH};

pub fn start(cli_tx: Sender<Command>) {
    thread::spawn(move || {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let command = line.unwrap();
            match command.as_str() {
                "exit" => cli_tx.send(Command::Exit).unwrap(),
                set if set.starts_with("set") => {
                    let parts: Vec<&str> = command.split_whitespace().collect();
                    if parts.len() == 4 {
                        if let (Ok(x), Ok(y), Ok(color)) = (
                            parts[1].parse::<usize>(),
                            parts[2].parse::<usize>(),
                            u32::from_str_radix(parts[3], 16),
                        ) {
                            if x < WIDTH
                                && y < HEIGHT
                                && cli_tx.send(Command::SetCellData(x, y, color)).is_err()
                            {
                                break;
                            }
                        }
                    }
                }
                _ => println!("Unknown command"),
            }
        }
    });
}
