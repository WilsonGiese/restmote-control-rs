use error::RcError;

use keyboard;
use keyboard::{Keycode,Modifier,Pid,VirtualKeyboard};

use rustful::{Context,Handler,Response,Server,StatusCode,TreeRouter};

use std::collections::{HashSet,HashMap};
use std::io::Read;
use std::fs::File;
use std::str::FromStr;

use rustc_serialize::json;

pub struct KeyboardHandler {
    allowed_keys: HashSet<Keycode>,
    allowed_modifiers: HashMap<Keycode, Vec<Modifier>>,
    keyboard: VirtualKeyboard,
}

#[derive(RustcDecodable, RustcEncodable)]
struct Config  {
    pid: Pid,
    keypress_delay: u64,
    keys: Vec<Key>,
}

#[derive(RustcDecodable, RustcEncodable)]
struct Key {
    key: String,
    allowed_modifiers: Vec<String>,
}

impl KeyboardHandler {
    fn new(
        allowed_keys: HashSet<Keycode>,
        allowed_modifiers: HashMap<Keycode, Vec<Modifier>>,
        keyboard: VirtualKeyboard
    ) -> KeyboardHandler {
        KeyboardHandler {
            allowed_keys: allowed_keys,
            allowed_modifiers: allowed_modifiers,
            keyboard: keyboard,
        }
    }

    fn keyboard_handler_from_file(path: &str) -> Result<KeyboardHandler, RcError> {
        let mut config = try!(File::open(path));
        let mut json_str = String::new();
        try!(config.read_to_string(&mut json_str));

        KeyboardHandler::keyboard_handler_from_str(json_str.as_str())
    }

    fn keyboard_handler_from_str(s: &str) -> Result<KeyboardHandler, RcError> {
        let config: Config = try!(json::decode(s));

        let mut allowed_keys = HashSet::new();
        let mut allowed_modifiers = HashMap::new();
        for key in config.keys {
            // Add key to allowed_keys if it is a valid keycode
            if let Some(k) = keyboard::keycode_from_str(key.key.as_str()) {
                allowed_keys.insert(k);

                // Add modifiers to key map if they are valid modifiers
                let mut modifiers = Vec::with_capacity(key.allowed_modifiers.len());
                for modifier in key.allowed_modifiers {
                    if let Some(m) = keyboard::modifier_from_str(modifier.as_str()) {
                        modifiers.push(m);
                    } else {
                        return Err(RcError::Config(format!(
                            "Configuration Error: Unsupported modifier: {}", modifier)));
                    }
                }
                allowed_modifiers.insert(k, modifiers);
            } else {
                return Err(RcError::Config(format!(
                    "Configuration Error: Unsupported key: {}", key.key)));
            }
        }

        Ok(KeyboardHandler::new(allowed_keys, allowed_modifiers,
            VirtualKeyboard::new(config.pid, config.keypress_delay)))
    }
}

impl Handler for KeyboardHandler {

    /// Handle VirtualKeyboard POST Request
    ///
    /// Request Body Example:
    ///     {
    ///         key: "a"             (Required)
    ///         modifier: "SHIFT"    (Optional)
    ///         action: "DOWN"
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
                    response.send("Expected String for field: key");
                    return;
                },
            },
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send("Missing required field: key");
                return;
            }
        };

        let keycode = match keyboard::keycode_from_str(key) {
            Some(keycode) => keycode,
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send(format!("Invalid key: {}", key));
                return;
            }
        };

        let action = match json.find("action") {
            Some(action) => match action.as_string() {
                Some(action) => match keyboard::KeyboardAction::from_str(action) {
                    Ok(action) => action,
                    Err(e) => {
                        response.set_status(StatusCode::BadRequest);
                        response.send(e);
                        return;
                    }
                },
                None => {
                    response.set_status(StatusCode::BadRequest);
                    response.send("Expected String for field: action");
                    return;
                }
            },
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send("Missing required field: action");
                return;
            }
        };
        println!("{:?}", action);

        // Check if the key is allowed
        if !self.allowed_keys.contains(&keycode) {
            response.set_status(StatusCode::BadRequest);
            response.send(format!("Key not supported: {}", key));
            return;
        }

        let modifier = match json.find("modifier") {
            Some(modifier_str) => match modifier_str.as_string() {
                Some(modifier) => keyboard::modifier_from_str(modifier),
                None => None,
            },
            None => None,
        };

        // Check if the modifier is allowed
        if let Some(m) = modifier {
            if let Some(modifiers) = self.allowed_modifiers.get(&keycode) {
                if !modifiers.contains(&m) {
                    response.set_status(StatusCode::BadRequest);
                    response.send(format!("Modifier not supported: {:?}", m));
                    return;
                }
            }
        }

        if self.keyboard.press_key(keycode, modifier, action).is_err() {
            response.set_status(StatusCode::InternalServerError);
            response.send(format!("Failed to press key: {}", key));
        }
    }
}

pub fn run(config: &str) -> Result<(), RcError> {
    let handler = try!(KeyboardHandler::keyboard_handler_from_file(config));

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

#[cfg(test)]
mod tests {
    use keyboard;
    use std::time::Duration;
    use super::KeyboardHandler;

    #[test]
    fn keyboard_handler_from_str_test() {
        let handler = KeyboardHandler::keyboard_handler_from_str("{
            \"pid\":1234,
            \"keypress_delay\":77,
            \"keys\": [
                {
                  \"key\":\"a\",
                  \"allowed_modifiers\":[\"COMMAND\", \"SHIFT\", \"OPTION\", \"CONTROL\"]
                },
                {
                  \"key\":\"b\",
                  \"allowed_modifiers\":[\"CONTROL\"]
                },
                {
                  \"key\":\"ENTER\",
                  \"allowed_modifiers\":[]
                }
            ]
        }").unwrap();

        assert_eq!(handler.keyboard.pid, 1234);
        assert_eq!(handler.keyboard.delay_duration, Duration::from_millis(77));

        let a = keyboard::keycode_from_str("a").unwrap();
        let b = keyboard::keycode_from_str("b").unwrap();
        let d = keyboard::keycode_from_str("d").unwrap();
        let enter = keyboard::keycode_from_str("ENTER").unwrap();

        assert!(handler.allowed_keys.contains(&a));
        assert!(handler.allowed_keys.contains(&b));
        assert!(handler.allowed_keys.contains(&enter));
        assert!(!handler.allowed_keys.contains(&d));
    }
}
