mod keyboard;
mod server;

extern crate core_graphics;
extern crate getopts;
extern crate libc;
#[macro_use]
extern crate rustful;

use getopts::Options;

use libc::pid_t;

use std::env;
use std::fmt::Display;

const DEFAULT_KEYPRESS_DELAY: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Setup program opts
    let mut opts = Options::new();
    opts.optopt("p", "pid", "set target pid where keyboard events will be sent", "<PROCESS ID>");
    opts.optopt("d", "delay", "set the delay between up & down key presses in ms", "<DELAY MS>");
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

    // Parse out -d --delay flag
    let delay = match matches.opt_str("d") {
        Some(d) => {
            match d.parse::<usize>() {
                Ok(d) => d,
                Err(e) => {
                    print_usage(&program, opts, Some(&e));
                    return;
                }
            }
        },
        None => DEFAULT_KEYPRESS_DELAY,
    };

    server::run(pid);
}

/// Print program usage
fn print_usage(program: &str, opts: Options, error: Option<&Display>) {
    if let Some(e) = error {
        println!("Error: {}", e)
    }
    println!("{}", opts.usage(&format!("Usage: {} FILE [options]", program)));
}
