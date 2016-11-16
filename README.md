# RESTMote Control

## What is this?
RESTMote Control is an application that allows you to expose REST services to
send keyboard events to a specified application. Inspired by Twitch Plays Pokemon,
I wanted to create an application that could achieve something similar while being
application agnostic.

## Tasks
- [x] Send keyboard events to a specified application on OSX with CG Framework
- [x] Create REST services to invoke keyboard events
- [x] Configurable with simple JSON formatted file
- [ ] Interface with Windows SendKeys API (Stretch)
- [ ] Linux compatible (Stretch)

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

| Key              | Values                                          |
| ---------------- |:-----------------------------------------------:|
| allowed_modifiers| SHIFT, CONTROL, COMMAND, OPTION, ALTERNATE, ALT |

