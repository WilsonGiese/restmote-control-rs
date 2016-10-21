extern crate core_graphics;
use core_graphics::event::{CGEvent,CGEventFlags,CGEventTapLocation,CGKeyCode};
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

extern crate getopts;
use getopts::Options;

use std::env;
use std::process::Command;

use std::{thread, time};

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

    // Send "Hello!""
    for i in 0..2 {
        send_keyboard_event(0x12, true);
        send_keyboard_event(0x12, false);
        // Sleep because if we don't the next event post seems to be missed, unless we post a lot
        // events, then they don't get missed, or most of them don't get missed. This API is kind
        // of buggy... Fun -_-
        thread::sleep_ms(25);
        send_keyboard_event_with_flags(0x12, CGEventFlags::Shift, true);
        send_keyboard_event_with_flags(0x12, CGEventFlags::Shift, false);
        thread::sleep_ms(25);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn send_keyboard_event(keycode: CGKeyCode, keydown: bool) -> Result<(), ()> {
    let eventSource = try!(CGEventSource::new(CGEventSourceStateID::HIDSystemState));
    let event = try!(CGEvent::new(eventSource, keycode, keydown));
    event.post_to_pid(1645);
    
    Ok(())
}

fn send_keyboard_event_with_flags(keycode: CGKeyCode, flags: CGEventFlags, keydown: bool) -> Result<(), ()>  {
    let eventSource = try!(CGEventSource::new(CGEventSourceStateID::HIDSystemState));
    let event = try!(CGEvent::new(eventSource, keycode, keydown));
    event.set_flags(flags);
    event.post_to_pid(1645);

    Ok(())
}
