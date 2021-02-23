//! Lovett is a framework for developing simple GUI apps
//! on Raspberry PI systems.
//!
//! Currently it supports limited hardware configurations
//! but it is intended to be flexible but powerful.
//!
//! At its core there is a View, State, Event Bus architecture
//!
//! [`RootView`] Holds a collection of [`View`] objects which
//! can render on to the RootvView's [`Canvas`] and paint [`gui_tk`] elements.
//! Furthermore, the RootView receives input from an hid_event sender, [`hid`]
//! Each view can recieve updated copies of a State tree contained by
//! the [`RootState`] object.
//! Events from the HID layer trigger Events on GUI elements.
//! The State can be changed by signalling the RootState with a 
//! Mutator
//!
//! [`RootState`]: ./state/struct.RootState.html
//! [`RootView`]: ./views/struct.RootView.html
//! [`Canvas`]: ./canvas/struct.Canvas.html
//! [`View`]: ./views/struct.View.html
//! [`hid`]: ./hid/index.html
//! [`gui_tk`]: ./gui_tk/index.html
//!
//!
//!
//! It is recommended to include this crate in your Cargo.toml
//! and then setup the main components of your application
//!
//!
//!### Usage
//!
//!
//!Its not yet anywhere else on the net, so if you want to use the very early pre alpha, you will need to speciy the github location in your `Cargo.toml`
//!
//!```toml
//![dependencies]
//!lovett = { git = "https://github.com/kcculhwch/lovett" }
//!```
//!
//!#### Basic Suggested Layout
//!
//!In your `main.rs`
//!
//!```rust
//!
//!// Crates
//!extern crate lovett;      // The Framework
//!extern crate serde;       // If you use the root_state handler you will need serde and bincode to for the state
//!extern crate bincode;     // object generator.
//!extern crate env_logger;  // env_logger is just an easy to use logger for getting log values out of the Framework
//!mod app;                  // the app module which we will outline below
//!
//!use app::*;               // import the app exports for use here.
//!
//!fn main()  {
//!    env_logger::init();   // setup the looger 
//!    let app = App::new(); // construct the app
//!    run_app(app);         // spin up the app threads
//!}
//!
//!```
//!
//!Create a `./app/mod.rs`
//!
//!It should define the `App` `struct`, and `impl`, as well as the `run_app` thread starter.
//!
//!* `struct App`
//!```rust
//!pub struct App {
//!    pub root_controller: RootController,        // for handling GuiAction inputs on a mspc channel
//!    pub root_state: RootState,                  // for broadcasting state changes and receiving mutator requests
//!    pub root_view: RootView,                    // receives state updates, handles gui and hid interactions 
//!                                                // and ... send Events/ to the Controller
//!}
//!```
//!
//!* `impl App`
//!```rust
//!impl App {
//!
//!    pub fn new() -> App{
//!    ...
//!    }
//!}
//!```
//!
//!* `run_app`
//!```rust
//!pub fn run_app(app: App) {
//!    run_view(app.root_view);
//!    run_state(app.root_state);
//!    // join the last thread
//!    run_controller(app.root_controller).join().expect("Couldn't join on the associated thread");
//!}
//!```
//!
//!#### Setup an hid event sender 
//!
//! currently the only support sender is a ButtonPad
//! the button pat sends an array of hid_events whenever new data is available
//!
//!Create a Vector of ButtonInitializer objects (hid::ButtonInitializer)
//!
//!   * pin - the gpio pin number
//!   * code - the internal code number of the button
//!   * key - the user readable &'static str for the key
//!
//!Create input_rx and input_tx channels
//!
//!Create the button_pad object with the Vector and the input channel.
//!
//!```rust
//!        // setup hw buttons
//!        let button_initializers = vec![
//!            ButtonInitializer {pin: 5, code: 0, key: "b"}, 
//!            ButtonInitializer {pin: 6, code: 1, key: "a"},
//!            ButtonInitializer {pin: 27, code: 2, key: "l"},
//!            ButtonInitializer {pin: 23, code: 3, key: "r"},
//!            ButtonInitializer {pin: 17, code: 4, key: "up"},
//!            ButtonInitializer {pin: 22, code: 5, key: "dn"},
//!            ButtonInitializer {pin: 4, code:  6, key: "hat"},
//!        ];
//!
//!
//!        //create channesl for sending raw input buttons to the root_view
//!        let (input_tx, input_rx) = mpsc::channel();
//!
//!        // setup the button_pad
//!        // throw errors if cannot initialize gpio states
//!        let button_pad =  match ButtonPad::new(&button_initializers, input_tx) {
//!            Ok(pad) => pad,
//!            Err(x) => panic!("Error Starting Button Pad: {}", x)
//!        };
//!```
//!
//!#### setup State 
//!
//!Create the root_state holder. (This still has way to much specific stuff in it)
//!
//!* `state/mod.rd` Define the Struct that will represent your program state
//!
//!```rust
//!pub mod mutators;                       // Include mutators
//!use serde::{Serialize, Deserialize};    // make sure we have Serialize and Deserialize decorators
//!
//!use lovett::gui_tk::*;                  // we will likely need to reference some Gui properties
//!
//!pub fn state_decoder(state: &[u8]) -> State{  // helper function for decoding serialized state array
//!    bincode::deserialize(state).unwrap()
//!}
//!
//!
//!#[derive(Clone, Debug, Serialize, Deserialize)]
//!pub struct State {
//!    example: String,
//!    ...
//!}
//!
//!
//!impl State {
//!    pub fn new() -> State {
//!...
//!}
//!```
//!* state/mutators/mod.rs Define the mutator functions that will be triggered
//!```rust
//!use lovett::state::*;
//!use lovett::gui_tk::*;
//!use super::*
//!
//!;
//!pub fn setup(root_state: &mut RootState) {
//!        // create the mutator handlers
//!        let example_updater: StateMutator = |state, mutator_signal| {
//!            let mut decoded_state = state_decoder(state);
//!            decoded_state.example = mutator_signal.value;
//!            bincode::serialize(&decoded_state).unwrap()
//!        };
//!
//!        ...
//!
//!        root_state.mutators.insert("[Example Mutation]", example_updater);
//!
//!}
//!```
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!












// Crates
extern crate framebuffer;
extern crate image;
extern crate serde;
extern crate bincode;


// Modules
pub mod hid;
pub mod fb;
pub mod canvas;
pub mod views;
pub mod gui_tk;
pub mod controller;
pub mod state;


