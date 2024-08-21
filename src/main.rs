use device_query::{DeviceQuery, DeviceState, MouseState};
use std::{thread, time::Duration};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let device_state = DeviceState::new();
    let mut cursor_positions = Vec::new();

    loop {
        let mouse: MouseState = device_state.get_mouse();
        let (x, y) = mouse.coords;
        println!("Cursor position: ({}, {})", x, y);
        cursor_positions.push((x, y));

        // Write to CSV file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("cursor_positions.csv")
            .unwrap();

        for (x, y) in &cursor_positions {
            writeln!(file, "{},{}", x, y).unwrap();
        }

        thread::sleep(Duration::from_millis(10));
    }
}