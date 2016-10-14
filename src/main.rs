use std::process::Command;

fn main() {
    Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to keystroke \"cargo run\n\"")
            .output();
}
