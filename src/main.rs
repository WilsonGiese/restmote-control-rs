mod keyboard;
mod server;

extern crate core_graphics;
extern crate getopts;
extern crate libc;
#[macro_use]
extern crate rustful;

use getopts::Options;

use libc::pid_t;

use server::{KeyboardPress};

use std::env;
use std::fmt::Display;
use std::process::Command;

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

    server::run(pid);
}

/// Print program usage
fn print_usage(program: &str, opts: Options, error: Option<&Display>) {
    match error {
        Some(e) => println!("Error: {}", e),
        None    => (),
    }
    println!("{}", opts.usage(&format!("Usage: {} FILE [options]", program)));
}
