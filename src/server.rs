use error::RcError;

use keyboard;
use keyboard::VirtualKeyboard;

use rustful::{Context,Handler,Response,Server,StatusCode,TreeRouter};

use std::collections::HashSet;
use std::io::Read;
use std::fs::File;

use rustc_serialize::json::Json;

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

    fn keyboard_handler_from_config(path: &str) -> Result<KeyboardHandler, RcError> {
        let mut config = try!(File::open(path));
        let mut json_str = String::new();
        try!(config.read_to_string(&mut json_str));

        let json = try!(Json::from_str(&json_str));

        let mut allowed_keys = HashSet::new();

        match json.as_array() {
            Some(array) => {
                for key in array {
                    if let Some(key) = key.as_string() {
                        allowed_keys.insert(key.to_string());
                    } else {
                        return Err(RcError::Config(String::from("Expected key as string")))
                    }
                }
            },
            None => return Err(RcError::Config(String::from("Expected keys as Array")))
        };

        Ok(KeyboardHandler::new(allowed_keys, VirtualKeyboard::new(0, 0)))
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

        if self.keyboard.press_key(keycode, modifier).is_err() {
            response.set_status(StatusCode::InternalServerError);
            response.send(format!("Failed to press key: {}", key));
        }
    }
}


pub fn run(config: &str) -> Result<(), RcError> {
    let handler = try!(KeyboardHandler::keyboard_handler_from_config(config));

    let router = insert_routes! {
        TreeRouter::new() => {
            "press" => Post: handler
        }
    };

    //Build and run the server.
    let server = Server {
        host: 8080.into(),
        handlers: router,
        ..Server::default()
    };

    if let Err(e) = server.run() {
        Err(RcError::Server(e))
    } else {
        Ok(())
    }
}
