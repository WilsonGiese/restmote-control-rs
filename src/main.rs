extern crate core_graphics;
use core_graphics::event::{CGEvent,CGEventFlags,CGEventTapLocation,CGKeyCode};

extern crate getopts;
use getopts::Options;

use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("t", "target", "set target application name", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            println!("Error: {}", f);
            print_usage(&program, opts);
            return;
        }
    };

    let target = match matches.opt_str("t") {
        Some(t) => t,
        None    => {
            print_usage(&program, opts);
            return;
        }
    };

    // Send A down, then A up
    {
        send_keyboard_event(0x00, true);
        send_keyboard_event(0x00, false);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn send_string_as_keystrokes(s: &str) {
    Command::new("osascript")
            .arg("-e")
            .arg(format!("tell application \"System Events\" to keystroke \"{}\"", s))
            .output();
}
