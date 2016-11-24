# RESTMote Control

RESTMote Control is an application that allows you to expose REST services to
send keyboard events to a specified application. Inspired by Twitch Plays Pokemon,
I wanted to create an application that could achieve something similar while being
application agnostic.

## Configuration
The server requires a configuration file to be provided during start-up.

### Example config file
```json
{
  "pid":31277,
  "keypress_delay":10,
  "keys": [
    {
      "key":"a",
      "allowed_modifiers":["COMMAND", "SHIFT"]
    },
    {
      "key":"b",
      "allowed_modifiers":["CONTROL"]
    }
  ]
}
```

### Config fields

| Key              | Description                                     |
| ---------------- |:-----------------------------------------------:|
| pid              | The program identifier (PID) of the target application where keypress events will be sent. |            
| keypress_delay              | The number of milliseconds to wait between keypresses. |
| key              | The application supports generating keyboard events for most keys. Non-character keys like "tab" or "capslock" can also be used. |
| allowed_modifiers| The set of modifier keys which are allowed to be used on with the key. Requesting a modidifier that is not allowed with result in a 400 response code. |

## Development

What's going on...

### Tasks
- [x] Send keyboard events to a specified application on OSX with CG Framework
- [x] Create REST services to invoke keyboard events
- [x] Configurable with simple JSON formatted file
- [ ] Documentation 
- [ ] Interface with Windows SendKeys API (Stretch)
- [ ] Linux compatible (Stretch)
