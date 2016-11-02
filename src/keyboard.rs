use core_graphics::event::{CGEvent,CGEventFlags,CGKeyCode};
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

use libc::pid_t;

use std::thread;
use std::time::Duration;
use std::str::FromStr;

pub struct VirtualKeyboard {
    /// Target application PID where keyboard events will be sent
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

/// CGEventFlags from string. Case is ignored
pub fn event_flags_from_str(s: &str) -> Option<CGEventFlags> {
    match &*s.to_lowercase() {
        "shift" => Some(CGEventFlags::Shift),
        "control" => Some(CGEventFlags::Control),
        "command" => Some(CGEventFlags::Command),
        "option" => Some(CGEventFlags::Alternate),
        "alternate" => Some(CGEventFlags::Alternate),
        "alt" => Some(CGEventFlags::Alternate),
        _ => None,
    }
}

/// ASCII to CGKeyCodes

/// Get CGCodeCode for coressponding ASCII code (QWERTY Keyboard)
pub fn keycode_from_ascii(c: char) -> Option<CGKeyCode> {
    let mut i = c as usize;

    match i {
        // Letters
        i if i >= 97 && i <= 122 => Some(ASCII_KEYCODE_MAP_LETTERS[i - 97]),
        // Numbers
        i if i >= 48 && i <= 57 => Some(ASCII_KEYCODE_MAP_NUMBERS[i - 48]),
        _ => None,
    }
}

#[test]
fn keycode_from_ascii_test() {
    assert_eq!(keycode_from_ascii('a').unwrap(), 0x00);
    assert_eq!(keycode_from_ascii('b').unwrap(), 0x0B);
    assert_eq!(keycode_from_ascii('p').unwrap(), 0x23);
    assert_eq!(keycode_from_ascii('z').unwrap(), 0x06);
    assert_eq!(keycode_from_ascii('0').unwrap(), 0x1D);
    assert_eq!(keycode_from_ascii('5').unwrap(), 0x17);
    assert_eq!(keycode_from_ascii('9').unwrap(), 0x19);
    assert!(keycode_from_ascii('.').is_none());
    assert!(keycode_from_ascii(';').is_none());
    assert!(keycode_from_ascii('!').is_none());
    assert!(keycode_from_ascii('~').is_none());
}

static ASCII_KEYCODE_MAP_LETTERS: &'static [CGKeyCode] =
    &[
        0x00, // a
        0x0B, // b
        0x08, // c
        0x02, // d
        0x0E, // e
        0x03, // f
        0x05, // g
        0x04, // h
        0x22, // i
        0x26, // j
        0x28, // k
        0x25, // l
        0x2E, // m
        0x2D, // n
        0x1F, // o
        0x23, // p
        0x0C, // q
        0x0F, // r
        0x01, // s
        0x11, // t
        0x20, // u
        0x09, // v
        0x0D, // w
        0x07, // x
        0x10, // y
        0x06, // z
    ];

static ASCII_KEYCODE_MAP_NUMBERS: &'static [CGKeyCode] =
    &[
        0x1D, // 0
        0x12, // 1
        0x13, // 2
        0x14, // 3
        0x15, // 4
        0x17, // 5
        0x16, // 6
        0x1A, // 7
        0x1C, // 8
        0x19, // 9
    ];
