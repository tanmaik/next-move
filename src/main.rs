mod predict;

use device_query::{DeviceQuery, DeviceState, MouseState};
use std::{thread, time::Duration};
use std::fs::{OpenOptions, File};
use std::io::{Write, BufWriter};

fn main() {
    let device_state = DeviceState::new();
    let mut action_count = 0;

    loop {
        println!("Enter an action name (or 'q' to quit, 'p' to predict):");
        let mut action_name = String::new();
        std::io::stdin().read_line(&mut action_name).unwrap();
        action_name = action_name.trim().to_string();

        if action_name == "q" {
            break;
        } else if action_name == "p" {
            if let Err(e) = predict::run_prediction() {
                eprintln!("Error during prediction: {}", e);
            }
            continue;
        }

        action_count += 1;
        let file_name = format!("action_{:03}_{}.csv", action_count, action_name);
        let file = File::create(&file_name).unwrap();
        let mut writer = BufWriter::new(file);

        println!("Perform the action '{}'. Press Enter when done.", action_name);
        std::io::stdin().read_line(&mut String::new()).unwrap();

        println!("Recording cursor movements. Press Enter to stop.");
        let mut cursor_positions = Vec::new();

        loop {
            let mouse: MouseState = device_state.get_mouse();
            let (x, y) = mouse.coords;
            cursor_positions.push((x, y));
            
            if std::io::stdin().read_line(&mut String::new()).is_ok() {
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }

        for (x, y) in cursor_positions {
            writeln!(writer, "{},{}", x, y).unwrap();
        }
        println!("Action '{}' recorded and saved to {}", action_name, file_name);
    }
}