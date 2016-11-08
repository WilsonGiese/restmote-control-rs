use keyboard;
use keyboard::VirtualKeyboard;
use libc::pid_t;
use rustful::{Context,Handler,Response,Server,StatusCode,TreeRouter};

use std::collections::HashSet;
use std::error::Error;

pub struct KeyboardHandler {
    allowed_keys: HashSet<String>,
    keyboard: VirtualKeyboard,
}

impl KeyboardHandler {
    fn new(allowed_keys: HashSet<String>, keyboard: VirtualKeyboard) -> KeyboardHandler {
        KeyboardHandler {
            allowed_keys: allowed_keys,
            keyboard: keyboard,
        }
    }
}

impl Handler for KeyboardHandler {
    /// Handle VirtualKeyboard POST Request
    ///
    /// Request Body Example:
    ///     {
    ///         key: "a"             (Required)
    ///         modifier: "SHIFT"    (Optional)
    ///     }
    fn handle_request(&self, mut context: Context, mut response: Response) {
        let json = match context.body.read_json_body() {
            Ok(json) => json,
            Err(e) => {
                response.set_status(StatusCode::BadRequest);
                response.send(format!("Invalid request body: {}", e));
                return;
            }
        };

        let key = match json.find("key") {
            Some(key) => match key.as_string() {
                Some(key) => key,
                None => {
                    response.set_status(StatusCode::BadRequest);
                    response.send("Unexpected type for field: key");
                    return;
                },
            },
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send("Missing required field: key");
                return;
            }
        };

        if !self.allowed_keys.contains(key) {
            response.set_status(StatusCode::BadRequest);
            response.send(format!("Key not available: {}", key));
            return;
        }

        let keycode = match keyboard::keycode_from_str(key) {
            Some(keycode) => keycode,
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send(format!("Invalid key: {}", key));
                return;
            }
        };

        let modifier = match json.find("modifier") {
            Some(modifier) => match modifier.as_string() {
                Some(modifier) => keyboard::event_flags_from_str(modifier),
                None => None,
            },
            None => None,
        };

        if let Err(_) = self.keyboard.press_key(keycode, modifier) {
            response.set_status(StatusCode::InternalServerError);
            response.send(format!("Failed to press key: {}", key));
        }
    }
}

pub fn run(pid: pid_t, delay_duration: u64) {
    let keyboard = VirtualKeyboard::new(pid, delay_duration);
    let mut allowed_keys = HashSet::new();
    allowed_keys.insert(String::from("a"));

    let router = insert_routes! {
        TreeRouter::new() => {
            "press" => Post: KeyboardHandler::new(allowed_keys, keyboard)
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
