use core_graphics::event::{CGEvent,CGEventFlags,CGKeyCode};
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

use libc::pid_t;

use std::thread;
use std::time::Duration;

pub struct VirtualKeyboard {
    pid: pid_t,

    /// Amount of time the thread will sleep before posting a keyboard event.
    /// This isn't required according to any documentation for the `CGEvents` API, but without this
    /// some events posted do not appear to make it to the target application (pid). Tuning seems
    /// required for individual applications
    delay_duration: Duration,
}

impl VirtualKeyboard {

    /// Create a new VirtualKeyboard connected to the target pid
    pub fn new(pid: pid_t, delay_duration: u64) -> VirtualKeyboard {
        VirtualKeyboard {
            pid: pid,
            delay_duration: Duration::from_millis(delay_duration),
        }
    }

    /// Simulate a keyboard key press by sending a key-up then key-down event
    pub fn press_key(&self, keycode: CGKeyCode, flags: Option<CGEventFlags>) -> Result<(), ()> {
        try!(self.post_keyboard_event(keycode, flags, true));
        self.post_keyboard_event(keycode, flags, false)
    }

    /// Post a single keyboard event with optional flags for keycode with the current keydown state
    fn post_keyboard_event(&self,
        keycode: CGKeyCode,
        flags: Option<CGEventFlags>,
        keydown: bool
    ) -> Result<(), ()> {
        let event_source = try!(CGEventSource::new(CGEventSourceStateID::HIDSystemState));
        let event = try!(CGEvent::new_keyboard_event(event_source, keycode, keydown));

        if let Some(f) = flags {
            event.set_flags(f)
        }

        thread::sleep(self.delay_duration);
        event.post_to_pid(self.pid);
        Ok(())
    }
}
