#![feature(libc)]
extern crate core_graphics;
extern crate libc;

use core_graphics::event::{CGKeyCode,CGEventFlags};
use libc::pid_t;

mod keyboard;
mod util;

pub use keyboard::*;
pub use util::*;

pub type Keycode = CGKeyCode;
pub type Modifier = CGEventFlags;
pub type Pid = pid_t;
