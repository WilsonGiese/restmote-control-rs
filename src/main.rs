mod error;
mod keyboard;
mod server;

extern crate clap;
extern crate core_graphics;
extern crate libc;
extern crate rustc_serialize;
#[macro_use]
extern crate rustful;

use clap::{Arg, App, AppSettings};

use std::fmt::Display;

const DEFAULT_KEYPRESS_DELAY: &'static str = "10";

fn main() {
    let app = App::new("RESTMote Control")
        .global_settings(&[AppSettings::ColoredHelp])
        .version("0.1")
        .author("Wilson Giese <giese.wilson@gmail.com>")
        .about("Control an application with a REST service")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG-FILE PATH")
            .required(true)
            .takes_value(true)
            .help("Application configuration file"))
        .get_matches();

    let config = app.value_of("config").unwrap();

    println!("{}", config);

    if let Err(e) = server::run(config) {
        println!("Failure: {}", e);
    }
}

// Print a message and error then exit with status 1
fn error(msg: &str, error: &Display) {
    println!("{}: {}", msg, error);
    std::process::exit(1);
}
