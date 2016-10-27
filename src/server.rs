use core_graphics::event::{CGEventFlags,CGKeyCode};
use keyboard;
use libc::pid_t;
use rustful::{Context,Handler,Response,Server,StatusCode,TreeRouter};

use std::error::Error;

pub struct KeyboardPress {
    pid: pid_t,
}

impl Handler for KeyboardPress {
    fn handle_request(&self, context: Context, mut response: Response) {
        let keycode_str = match context.variables.get("keycode") {
            Some(k) => k,
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send(format!("Missing required keycode parameter!"));
                return;
            }
        };

        let keycode = match keycode_str.parse::<CGKeyCode>() {
            Ok(k) => k,
            Err(_) => {
                response.set_status(StatusCode::BadRequest);
                response.send(format!("Invalid keycode: {}", keycode_str));
                return;
            }
        };

        let modifier = match context.query.get("modifier") {
            Some(m) => {
                let m = m.into_owned();
                match m.as_str() {
                    "shift" => Some(CGEventFlags::Shift),
                    "control" => Some(CGEventFlags::Control),
                    "alternate" => Some(CGEventFlags::Alternate),
                    "command" => Some(CGEventFlags::Command),
                    _ => {
                        response.set_status(StatusCode::BadRequest);
                        response.send(format!("Invalid modifier: {}", m));
                        return;
                    }
                }
            },
            None => None
        };

        match keyboard::press_key(self.pid, keycode, modifier) {
            Ok(_) => (),
            Err(_) => response.send(format!("Failed to press key: {}", keycode)),
        }
    }
}

pub fn run(pid: pid_t) {
    let router = insert_routes! {
        TreeRouter::new() => {
            "press/:keycode" => Put: KeyboardPress { pid: pid }
        }
    };

    //Build and run the server.
    let server_result = Server {
        host: 8080.into(),
        handlers: router,
        ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}
