mod keyboard;
mod server;

extern crate clap;
extern crate core_graphics;
extern crate libc;
#[macro_use]
extern crate rustful;

use clap::{Arg, App, AppSettings};

use libc::pid_t;

use std::fmt::Display;

const DEFAULT_KEYPRESS_DELAY: &'static str = "10";

fn main() {
    let app = App::new("RESTMote Control")
        .global_settings(&[AppSettings::ColoredHelp])
        .version("0.1")
        .author("Wilson Giese <giese.wilson@gmail.com>")
        .about("Control an application with a REST service")
        .arg(Arg::with_name("pid")
            .short("p")
            .long("pid")
            .value_name("PID")
            .required(true)
            .takes_value(true)
            .help("Target application PID where events will be sent"))
        .arg(Arg::with_name("d")
            .short("d")
            .long("delay")
            .value_name("MS")
            .required(false)
            .takes_value(true)
            .help("Delay between key up & down events in milliseconds"))
        .get_matches();

    let pid = match app.value_of("pid") {
        Some(pid) => {
            match pid.parse::<pid_t>() {
                Ok(pid) => pid,
                Err(e) => {
                    error("Invalid pid", &e);
                    return;
                },
            }
        }
        None => 0,
    };

    let delay = match app.value_of("delay").unwrap_or(DEFAULT_KEYPRESS_DELAY).parse::<u64>() {
        Ok(delay) => delay,
        Err(e) => {
            error("Invalid pid", &e);
            return;
        },
    };

    server::run(pid, delay)
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let program = args[0].clone();
//
//     // Setup program opts
//     let mut opts = Options::new();
//     opts.optopt("p", "pid", "set target pid where keyboard events will be sent", "<PROCESS ID>");
//     opts.optopt("d", "delay", "set the delay between up & down key presses in ms", "<DELAY MS>");
//     let matches = match opts.parse(&args[1..]) {
//         Ok(m) => m,
//         Err(e) => {
//             print_usage(&program, opts, Some(&e));
//             return;
//         }
//     };
//
//     // Parse out -p --pid flag
//     let pid = match matches.opt_str("p") {
//         Some(p) => {
//             match p.parse::<pid_t>() {
//                 Ok(p) => p,
//                 Err(e) => {
//                     print_usage(&program, opts, Some(&e));
//                     return;
//                 }
//             }
//         },
//         None => {
//             print_usage(&program, opts, None);
//             return;
//         }
//     };
//
//     // Parse out -d --delay flag
//     let delay = match matches.opt_str("d") {
//         Some(d) => {
//             match d.parse::<u64>() {
//                 Ok(d) => d,
//                 Err(e) => {
//                     print_usage(&program, opts, Some(&e));
//                     return;
//                 }
//             }
//         },
//         None => DEFAULT_KEYPRESS_DELAY,
//     };
//
//     server::run(pid, delay);
// }

// Print a message and error then exit with status 1
fn error(msg: &str, error: &Display) {
    println!("{}: {}", msg, error);
    std::process::exit(1);
}
