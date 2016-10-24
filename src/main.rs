extern crate core_graphics;
use core_graphics::event::{CGEvent,CGEventFlags,CGEventTapLocation,CGKeyCode};
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

extern crate getopts;
use getopts::Options;

extern crate libc;
use libc::pid_t;

use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;


const EVENT_POST_SLEEP_DURATION: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "pid", "set target pid were keyboard events will be sent", "<PROCESS ID>");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            println!("Error: {}", f);
            print_usage(&program, opts);
            return;
        }
    };

    let pid = match matches.opt_str("p") {
        Some(p) => {
            match p.parse::<pid_t>() {
                Ok(p) => p,
                Err(e) => panic!("{}", e)
            }
        },
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    press_key_with_flags(pid, 0x04, CGEventFlags::Shift); // H
    press_key(pid, 0x0E);                                 // e
    press_key(pid, 0x25);                                 // l
    press_key(pid, 0x25);                                 // l
    press_key(pid, 0x1F);                                 // o
    press_key_with_flags(pid, 0x12, CGEventFlags::Shift); // !
    press_key(pid, 0x34);                                 // ENTER (\n)
}

fn press_key(pid: pid_t, keycode: CGKeyCode) -> Result<(), ()> {

    try!(send_keyboard_event(pid, keycode, true));
    send_keyboard_event(pid, keycode, false)
}

fn press_key_with_flags(pid: pid_t, keycode: CGKeyCode, flags: CGEventFlags) -> Result<(), ()> {
    try!(send_keyboard_event_with_flags(pid, keycode, flags, true));
    send_keyboard_event_with_flags(pid, keycode, flags, false)
}

fn send_keyboard_event(pid: pid_t, keycode: CGKeyCode, keydown: bool) -> Result<(), ()> {
    let eventSource = try!(CGEventSource::new(CGEventSourceStateID::HIDSystemState));
    let event = try!(CGEvent::new(eventSource, keycode, keydown));

    thread::sleep_ms(EVENT_POST_SLEEP_DURATION);
    event.post_to_pid(pid);
    Ok(())
}

fn send_keyboard_event_with_flags(pid: pid_t, keycode: CGKeyCode, flags: CGEventFlags, keydown: bool) -> Result<(), ()>  {
    let eventSource = try!(CGEventSource::new(CGEventSourceStateID::HIDSystemState));
    let event = try!(CGEvent::new(eventSource, keycode, keydown));
    event.set_flags(flags);

    thread::sleep_ms(EVENT_POST_SLEEP_DURATION);
    event.post_to_pid(pid);
    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
