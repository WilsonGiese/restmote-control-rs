use core_graphics::event::{CGEventFlags,CGKeyCode};
use keyboard::VirtualKeyboard;
use libc::pid_t;
use rustful::{Context,Handler,Response,Server,StatusCode,TreeRouter};

use std::error::Error;

impl Handler for VirtualKeyboard {
    fn handle_request(&self, context: Context, mut response: Response) {
        let keycode_str = match context.variables.get("keycode") {
            Some(k) => k,
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send("Missing required keycode parameter");
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
                match &*m {
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

        println!("Pressing key: {:X}", keycode);
        match self.press_key(keycode, modifier) {
            Ok(_) => (),
            Err(_) => response.send(format!("Failed to press key: {}", keycode)),
        }
    }
}

pub fn run(pid: pid_t, delay_duration: u64) {
    let router = insert_routes! {
        TreeRouter::new() => {
            "press/:keycode" => Put: VirtualKeyboard::new(pid, delay_duration)
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
