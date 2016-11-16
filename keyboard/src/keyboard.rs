///! Abstraction over the `CGEvents` lib to route virtual keyboard presses to an application using
///! its pid

use core_graphics::event::CGEvent;
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

use super::{Keycode,Modifier,Pid};

use std::thread;
use std::str::FromStr;
use std::time::Duration;

///! A keyboard action to be taken
#[derive(Copy,Clone,Debug)]
pub enum KeyboardAction {
    Up,     // Release key
    Down,   // Press key
    Cycle,  // Press then release key
}

impl FromStr for KeyboardAction {
    type Err = String;

    fn from_str(s: &str) -> Result<KeyboardAction, String> {
        match &*s.to_lowercase() {
            "up" => Ok(KeyboardAction::Up),
            "down" => Ok(KeyboardAction::Down),
            "cycle" => Ok(KeyboardAction::Cycle),
            _ =>  Err(format!("Invalid action: {}", s)),
        }
    }
}

pub struct VirtualKeyboard {
    /// Target application PID where keyboard events will be sent
    pid: Pid,

    /// Amount of time the thread will sleep before posting a keyboard event.
    /// This isn't required according to any documentation for the `CGEvents` API, but without this
    /// some events posted do not appear to make it to the target application (pid). Tuning seems
    /// required for individual applications; most times short durations like 10ms work, but other
    /// applications can take upwards of 50ms before events are reliably delivered
    delay_duration: Duration,
}

impl VirtualKeyboard {

    /// Create a new VirtualKeyboard connected to the target pid
    pub fn new(pid: Pid, delay_duration: u64) -> VirtualKeyboard {
        VirtualKeyboard {
            pid: pid,
            delay_duration: Duration::from_millis(delay_duration),
        }
    }

    /// Simulate a keyboard key press by sending a key-up then key-down event
    pub fn press_key(&self,
        keycode: Keycode,
        flags: Option<Modifier>,
        action: KeyboardAction
    ) -> Result<(), ()> {

        match action {
            KeyboardAction::Up => self.post_keyboard_event(keycode, flags, false),
            KeyboardAction::Down => self.post_keyboard_event(keycode, flags, true),
            KeyboardAction::Cycle => {
                try!(self.post_keyboard_event(keycode, flags, true));
                self.post_keyboard_event(keycode, flags, false)
            }
        }
    }

    /// Post a single keyboard event with optional flags for keycode with the current keydown state
    fn post_keyboard_event(&self,
        keycode: Keycode,
        flags: Option<Modifier>,
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
