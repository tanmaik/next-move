use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn main() -> std::io::Result<()> {
    let device_state = DeviceState::new();
    let mut all_positions: Vec<(String, Position)> = Vec::new();

    loop {
        println!("Enter a name for the action you want to perform:");
        let mut action_name = String::new();
        std::io::stdin().read_line(&mut action_name)?;
        action_name = action_name.trim().to_string();

        println!("Start performing the action. Press 'd' when you're done.");

        let mut positions: Vec<Position> = Vec::new();
        let mut last_position: Option<Position> = None;

        loop {
            let mouse: MouseState = device_state.get_mouse();
            let keys: Vec<Keycode> = device_state.get_keys();

            let current_position = Position { x: mouse.coords.0, y: mouse.coords.1 };

            if last_position.as_ref() != Some(&current_position) {
                positions.push(current_position.clone());
                last_position = Some(current_position);
            }

            if keys.contains(&Keycode::D) {
                println!("Action completed. Processing data...");
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }

        for pos in positions {
            all_positions.push((action_name.clone(), pos));
        }

        let mut file = File::create("cursor_positions.csv")?;
        writeln!(file, "Action,X,Y")?;
        for (action, pos) in &all_positions {
            writeln!(file, "{},{},{}", action, pos.x, pos.y)?;
        }

        println!("Cursor positions saved to cursor_positions.csv");
        println!("Total positions recorded: {}", all_positions.len());

        println!("Press 'p' to record another action or 'q' to quit.");
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if keys.contains(&Keycode::Q) {
                println!("Quitting...");
                return Ok(());
            } else if keys.contains(&Keycode::P) {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}