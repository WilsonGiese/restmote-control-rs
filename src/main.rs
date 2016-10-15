use std::process::Command;

fn main() {
    activate_application("TextEdit");
    send_string_as_keystrokes("Hello, TextEdit!");
}

fn send_string_as_keystrokes(s: &str) {
    Command::new("osascript")
            .arg("-e")
            .arg(format!("tell application \"System Events\" to keystroke \"{}\"", s))
            .output();
}

fn activate_application(name: &str) {
    Command::new("osascript")
            .arg("-e")
            .arg(format!("activate application\"{}\"", name))
            .output();
}
