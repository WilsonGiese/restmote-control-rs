use core_graphics::event::{CGEvent,CGEventFlags,CGKeyCode};
use core_graphics::event_source::{CGEventSource,CGEventSourceStateID};

use libc::pid_t;

use std::thread;
use std::time::Duration;

pub type Keycode = CGKeyCode;
pub type Modifier = CGEventFlags;

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
    pub fn press_key(&self, keycode: Keycode, flags: Option<Modifier>) -> Result<(), ()> {
        try!(self.post_keyboard_event(keycode, flags, true));
        self.post_keyboard_event(keycode, flags, false)
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

/// Map of ascii characters to their respective CG Keycodes
static ASCII_KEYCODE_MAP_LETTERS: &'static [Keycode] =
    &[
        // a    b    c    d    e    f    g    h    i    j    k    l    m
        0x00,0x0B,0x08,0x02,0x0E,0x03,0x05,0x04,0x22,0x26,0x28,0x25,0x2E,
        // n    o    p    q    r    s    t    u    v    w    x    y    z
        0x2D,0x1F,0x23,0x0C,0x0F,0x01,0x11,0x20,0x09,0x0D,0x07,0x10,0x06
    ];

/// Map of ascii digits to their respective CG Keycodes
static ASCII_KEYCODE_MAP_NUMBERS: &'static [Keycode] =
    &[
        // 0    1    2    3    4    5    6    7    8    9
        0x1D,0x12,0x13,0x14,0x15,0x17,0x16,0x1A,0x1C,0x19
    ];

pub fn keycode_from_char(c: char) -> Option<Keycode> {
    let i = c as usize;

    let mut keycode = match i {
        // Letters
        i if i >= 97 && i <= 122 => Some(ASCII_KEYCODE_MAP_LETTERS[i - 97]),
        // Numbers
        i if i >= 48 && i <= 57 => Some(ASCII_KEYCODE_MAP_NUMBERS[i - 48]),
        _ => None,
    };

    if keycode == None {
        keycode = match c {
            ' ' => Some(0x31),
            '=' => Some(0x18),
            '-' => Some(0x1B),
            ']' => Some(0x1E),
            '[' => Some(0x21),
            '/' => Some(0x2C),
            ';' => Some(0x29),
            ',' => Some(0x2B),
            '.' => Some(0x2F),
            '`' => Some(0x32),
            '"' => Some(0x27),
            '\\' => Some(0x2A),
            _ => None,
        };
    }

    keycode
}

pub fn keycode_from_str(s: &str) -> Option<Keycode> {
    let mut keycode = None;
    let s = s.to_lowercase();

    // Characters
    if s.len() == 1 {
        keycode = keycode_from_char(s.as_bytes()[0] as char);
    }

    // Command Keys
    if keycode == None {
        keycode = match &*s {
            // Command keys
            "return" => Some(0x24),
            "tab" => Some(0x30),
            "space" => Some(0x31),
            "delete" => Some(0x33),
            "escape" => Some(0x35),
            "capslock" => Some(0x39),
            "volumeup" => Some(0x48),
            "volumedown" => Some(0x49),
            "mute" => Some(0x4A),
            "help" => Some(0x72),
            "home" => Some(0x73),
            "pageup" => Some(0x74),
            "forwarddelete" => Some(0x75),
            "end" => Some(0x77),
            "pagedown" => Some(0x79),
            "leftarrow" => Some(0x7B),
            "rightarrow" => Some(0x7C),
            "downarrow" => Some(0x7D),
            "uparrow" => Some(0x7E),
            _ => None
        };
    }

    keycode
}

/// `CGEventFlags` from string. Case is ignored
pub fn modifier_from_str(s: &str) -> Option<Modifier> {
    match &*s.to_lowercase() {
        "shift" => Some(CGEventFlags::Shift),
        "control" => Some(CGEventFlags::Control),
        "command" => Some(CGEventFlags::Command),
        "option" | "alternate" | "alt" => Some(CGEventFlags::Alternate),
        _ => None,
    }
}

#[test]
fn keycode_from_str_test() {
    assert_eq!(keycode_from_str("a").unwrap(), 0x00);
    assert_eq!(keycode_from_str("b").unwrap(), 0x0B);
    assert_eq!(keycode_from_str("p").unwrap(), 0x23);
    assert_eq!(keycode_from_str("z").unwrap(), 0x06);
    assert_eq!(keycode_from_str("0").unwrap(), 0x1D);
    assert_eq!(keycode_from_str("5").unwrap(), 0x17);
    assert_eq!(keycode_from_str("9").unwrap(), 0x19);
    assert_eq!(keycode_from_str(";").unwrap(), 0x29);
    assert_eq!(keycode_from_str("return").unwrap(), 0x24);
    assert_eq!(keycode_from_str("escape").unwrap(), 0x35);
    assert_eq!(keycode_from_str("downarrow").unwrap(), 0x7D);
    assert_eq!(keycode_from_str("uparrow").unwrap(), 0x7E);
    assert!(keycode_from_str("!").is_none());
    assert!(keycode_from_str("foobar").is_none());
    assert!(keycode_from_str("").is_none());
}
