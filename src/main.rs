extern crate core_graphics;
use core_graphics::event::{CGEvent,CGEventFlags,CGEventTapLocation,CGKeyCode};
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

extern crate getopts;
use getopts::Options;

extern crate libc;
use libc::pid_t;

use std::env;
use std::fmt::Display;
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Amount of time the thread will sleep before posting a keyboard event.
/// This isn't required according to any documentation for the CGEvents API, but without this some
/// events posted do not appear to make it to the target application (pid)
const EVENT_POST_SLEEP_DURATION: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Setup program opts
    let mut opts = Options::new();
    opts.optopt("p", "pid", "set target pid were keyboard events will be sent", "<PROCESS ID>");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            print_usage(&program, opts, Some(&e));
            return;
        }
    };

    // Parse out -p --pid flag
    let pid = match matches.opt_str("p") {
        Some(p) => {
            match p.parse::<pid_t>() {
                Ok(p) => p,
                Err(e) => {
                    print_usage(&program, opts, Some(&e));
                    return;
                }
            }
        },
        None => {
            print_usage(&program, opts, None);
            return;
        }
    };

    run(pid);
}

/// Print program usage
fn print_usage(program: &str, opts: Options, error: Option<&Display>) {
    match error {
        Some(e) => println!("Error: {}", e),
        None    => (),
    }
    println!("{}", opts.usage(&format!("Usage: {} FILE [options]", program)));
}

/// Start of application
fn run(pid: pid_t) {
    press_key(pid, 0x04, Some(CGEventFlags::Shift)); // H
    press_key(pid, 0x0E, None);                      // e
    press_key(pid, 0x25, None);                      // l
    press_key(pid, 0x25, None);                      // l
    press_key(pid, 0x1F, None);                      // o
    press_key(pid, 0x12, Some(CGEventFlags::Shift)); // !
    press_key(pid, 0x34, None);                      // ENTER (\n)
}

/// Simulate a keyboard key press by sending a key-up then key-down event to an application
/// specified by the pid
fn press_key(pid: pid_t, keycode: CGKeyCode, flags: Option<CGEventFlags>) -> Result<(), ()> {
    try!(post_keyboard_event(pid, keycode, flags, true));
    post_keyboard_event(pid, keycode, flags, false)
}

/// Post a single keyboard event with optional flags for keycode with the current keydown state.
/// keydown = true for Key Pressed, keydown = false for Key Released
fn post_keyboard_event(pid: pid_t, keycode: CGKeyCode, flags: Option<CGEventFlags>, keydown: bool) -> Result<(), ()> {
    let eventSource = try!(CGEventSource::new(CGEventSourceStateID::HIDSystemState));
    let event = try!(CGEvent::new(eventSource, keycode, keydown));

    match flags {
        Some(f) => event.set_flags(f),
        _ => (),
    }

    thread::sleep_ms(EVENT_POST_SLEEP_DURATION);
    event.post_to_pid(pid);
    Ok(())
}
