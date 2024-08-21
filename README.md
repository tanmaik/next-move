# Cursor Action Recorder and Replayer

This project consists of a Rust application for recording cursor actions and a Go application for replaying them.

## Prerequisites

- Rust (latest stable version)
- Go (1.16 or later)
- Python 3.7 or later
- Git

## Setup

1. Clone the repository:
   ```
   git clone <repository-url>
   cd <repository-name>
   ```

2. Set up the Rust environment:
   ```
   cargo build
   ```

3. Set up the Go environment:
   ```
   go mod init cursor_action_replayer
   go get github.com/eiannone/keyboard
   go get github.com/go-vgo/robotgo
   ```

4. Set up the Python environment:
   ```
   python3 -m venv venv
   source venv/bin/activate  # On Windows, use `venv\Scripts\activate`
   pip install -r requirements.txt
   ```

## Running the Application

1. To record cursor actions:
   ```
   cargo run
   ```
   Follow the prompts to record your actions.

2. To replay cursor actions:
   ```
   go run main.go
   ```
   Use arrow keys to select an action and press Enter to replay it.

3. To train the model (if implemented):
   After recording actions, choose the 'm' option when prompted in the Rust application.

## Notes

- Make sure you have the necessary permissions to control the mouse cursor on your system.
- On some systems, you might need to run the applications with elevated privileges.
- For the Go application, you may need to install additional dependencies based on your operating system. Refer to the `robotgo` documentation for more information.

## Troubleshooting

- If you encounter any issues with `robotgo` on Linux, you may need to install additional dependencies:
  ```
  sudo apt-get install libx11-dev xorg-dev libxtst-dev libpng++-dev
  ```
- On macOS, you might need to grant accessibility permissions to your terminal application.

## Contributing

Feel free to submit issues or pull requests if you find any bugs or have suggestions for improvements.

## License

MIT

