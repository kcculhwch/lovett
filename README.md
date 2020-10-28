# Lovett
A Rusty Pi Application Framework
![Lovett Logo](/assets/lovett.png "Lovett")

## Lovett is a Framework

Lovett was designed to create small graphical applications using Rust, a framebuffer, and GPIO pins on a Rasbperry PI.

### Usage

Its not yet anywhere else on the net, so if you want to use the very early pre alpha, you will need to speciy the github location in your `Cargo.toml`

```toml
[dependencies]
lovett = { git = "https://github.com/kcculhwch/lovett" }
```

#### Basic Suggested Layout

In your `main.rs`

```rs
extern crate lovett;

mod app;

use app::*;

fn main()  {
    let app = App::new();
    run_app(app);
}
```

Create a `./app/mod.rs`

It should define the `App` `struct`, and `impl`, as well as the `run_app` thread starter.

* `struct App`
```rs
pub struct App {
    pub root_controller: RootController, // for handling GuiAction inputs on a mspc channel
    pub root_state: RootState, // for broadcasting state changes and receiving mutator requests
    pub root_view: RootView, // receives state updates, handles gui / input_pad interactions ... send GuiActions to the Controller
    pub input_pad: Pad // handles raw button_actions, sends them to the root_view
}
```

* `impl App
```rs
impl App {

    pub fn new() -> App{
    ...
    }
}
```

* `run_app`
```rs
pub fn run_app(app: App) {
    run_pad(app.input_pad);
    run_view(app.root_view);
    run_state(app.root_state);
    // join the last thread
    run_controller(app.root_controller).join().expect("Couldn't join on the associated thread");
}
```

#### Setup an input_pad

Create a Vector of ButtonInitializer objects (joy_pad::ButtonInitializer)

   * pin - the gpio pin number
   * code - the internal code number of the button
   * key - the user readable &'static str for the key

Create input_rx and input_tx channels

Create the input_pad object with the Vector and the input channel.

```rs
        // setup hw buttons
        let button_initializers = vec![
            ButtonInitializer {pin: 5, code: 0, key: "b"}, 
            ButtonInitializer {pin: 6, code: 1, key: "a"},
            ButtonInitializer {pin: 27, code: 2, key: "l"},
            ButtonInitializer {pin: 23, code: 3, key: "r"},
            ButtonInitializer {pin: 17, code: 4, key: "up"},
            ButtonInitializer {pin: 22, code: 5, key: "dn"},
            ButtonInitializer {pin: 4, code:  6, key: "hat"},
        ];


        //create channesl for sending raw input buttons to the root_view
        let (input_tx, input_rx) = mpsc::channel();

        // setup the input_pad
        // throw errors if cannot initialize gpio states
        let input_pad =  match Pad::new(&button_initializers, input_tx) {
            Ok(pad) => pad,
            Err(x) => panic!("Error Starting Input Pad: {}", x)
        };
```

#### setup State 

Create the root_state holder. (This still has way to much specific stuff in it)

```rs
        // setup the root state object
        let mut root_state = RootState::new();

        // get a state mutation sender for time keeping thread
        // we setup a time keeper thread on a 1 second resolution to trigger initial state senders
        let time_mutator = root_state.get_mutation_sender();
        time_keeper(time_mutator);
```

#### Setup View

Views are setup with a root view, to which you add View objects with their own gui_tk objects

#### Setup Controllers

This is WIP and I haven't finished thinking through how to implement it in a generic way.

## Lovett is used by SilviaPiPID

## Lovett is a big work in progress and needs lots of love

* Lovett needs a test suite
* Lovett needs some more architectural work
* Lovett needs optimization
* Lovett needs people with more experience in Rust to make it better
