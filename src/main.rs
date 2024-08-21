use device_query::{DeviceQuery, DeviceState, MouseState};
use std::{thread, time::Duration};

fn main() {
    let device_state = DeviceState::new();

    loop {
        let mouse: MouseState = device_state.get_mouse();
        let (x, y) = mouse.coords;
        println!("Cursor position: ({}, {})", x, y);

        thread::sleep(Duration::from_secs(1));
    }
}