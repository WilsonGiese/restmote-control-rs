mod error;
mod server;

extern crate clap;
extern crate keyboard;
extern crate rustc_serialize;
#[macro_use]
extern crate rustful;

use clap::{Arg, App, AppSettings};

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

    if let Err(e) = server::run(config) {
        println!("Server Error: {}", e);
        std::process::exit(1);
    }
}
