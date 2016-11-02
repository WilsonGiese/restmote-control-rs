use core_graphics::event::{CGEventFlags,CGKeyCode};
use keyboard;
use keyboard::VirtualKeyboard;
use libc::pid_t;
use rustful::{Context,Handler,Response,Server,StatusCode,TreeRouter};

use std::error::Error;

impl Handler for VirtualKeyboard {

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
            Some(key) => key,
            None => {
                response.set_status(StatusCode::BadRequest);
                response.send(format!("Missing required field: key"));
                return;
            }
        };
        println!("Key: {}", key);

        let modifier = match json.find("modifier") {
            Some(modifier) => match modifier.as_string() {
                Some(modifier) => keyboard::event_flags_from_str(modifier),
                None => None,
            },
            None => None,
        };
        println!("Modifier: {:?}", modifier);

        // if let Err(_) = self.press_key(keycode, modifier) {
        //     response.set_status(StatusCode::InternalServerError);
        //     response.send(format!("Failed to press key: {}", keycode));
        // }
    }
}

pub fn run(pid: pid_t, delay_duration: u64) {
    let router = insert_routes! {
        TreeRouter::new() => {
            "press" => Post: VirtualKeyboard::new(pid, delay_duration)
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
