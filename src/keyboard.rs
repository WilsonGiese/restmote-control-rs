use core_graphics::event::{CGEvent,CGEventFlags,CGKeyCode};
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

use libc::pid_t;

use std::thread;

/// Amount of time the thread will sleep before posting a keyboard event.
/// This isn't required according to any documentation for the `CGEvents` API, but without this some
/// events posted do not appear to make it to the target application (pid)
const EVENT_POST_SLEEP_DURATION: u32 = 10;

/// Simulate a keyboard key press by sending a key-up then key-down event to an application
/// specified by the pid
pub fn press_key(pid: pid_t, keycode: CGKeyCode, flags: Option<CGEventFlags>) -> Result<(), ()> {
    try!(post_keyboard_event(pid, keycode, flags, true));
    post_keyboard_event(pid, keycode, flags, false)
}

/// Post a single keyboard event with optional flags for keycode with the current keydown state.
/// keydown = true for Key Pressed, keydown = false for Key Released
pub fn post_keyboard_event(pid: pid_t, keycode: CGKeyCode, flags: Option<CGEventFlags>, keydown: bool) -> Result<(), ()> {
    let event_source = try!(CGEventSource::new(CGEventSourceStateID::HIDSystemState));
    let event = try!(CGEvent::new_keyboard_event(event_source, keycode, keydown));

    if let Some(f) = flags {
        event.set_flags(f)
    }

    thread::sleep_ms(EVENT_POST_SLEEP_DURATION);
    event.post_to_pid(pid);
    Ok(())
}
