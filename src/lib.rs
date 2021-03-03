//! Lovett is a framework for developing simple GUI apps
//! on Raspberry PI systems.
//!
//! Currently it supports limited hardware configurations
//! but it is intended to be flexible but powerful.
//!
//! At its core there is loosely based on a Redux data store architecture,
//! where state data is fully centralized in the store, and can only be 
//! reduced by the reducer functions which do not mutate the state.
//!
//! [`WindowViewer`] Holds a collection of [`View`] objects which
//! can render on to the WindowViewer's [`Canvas`] and paint [`gui_tk`] elements.
//! Furthermore, the WindowViewer receives input from an hid_event sender, [`hid`]
//! Each view can recieve updated copies of a State tree contained by
//! the [`Store`] object.
//! Events from the HID layer trigger Events on GUI elements.
//! The State can be changed by signalling the Store with a 
//! Reducer
//!
//! [`Store`]: ./store/struct.Store.html
//! [`WindowViewer`]: ./window_viewer/struct.WindowViewer.html
//! [`Canvas`]: ./canvas/struct.Canvas.html
//! [`View`]: ./window_viewer/struct.View.html
//! [`hid`]: ./hid/index.html
//! [`gui_tk`]: ./gui_tk/index.html
//!
//! ## Architecture Diagram for an applicaiton
//! 
//! ![Architecture of a Lovett Program](../Architecture.png "Architecture of a Lovett Program")
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
//!extern crate serde;       // If you use the store handler you will need serde and bincode to for the state
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
//!    pub model_scheduler: ModelScheduler,        // for handling GuiAction inputs on a mspc channel
//!    pub store: Store,                  // for broadcasting state changes and receiving reducer requests
//!    pub window_viewer: WindowViewer,                    // receives state updates, handles gui and hid interactions 
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
//!    run_view(app.window_viewer);
//!    run_state(app.store);
//!    // join the last thread
//!    run_controller(app.model_scheduler).join().expect("Couldn't join on the associated thread");
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
//!        //create channesl for sending raw input buttons to the window_viewer
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
//!Create the store holder. (This still has way to much specific stuff in it)
//!
//!* `state/mod.rd` Define the Struct that will represent your program state
//!
//!```rust
//!pub mod reducers;                       // Include reducers
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
//!* state/reducers/mod.rs Define the reducer functions that will be triggered
//!```rust
//!use lovett::state::*;
//!use lovett::gui_tk::*;
//!use super::*
//!
//!;
//!pub fn setup(store: &mut Store) {
//!        // create the reducer handlers
//!        let example_reducer: Reducer = |state, action| {
//!            let mut decoded_state = state_decoder(state);
//!            decoded_state.example = reducer_signal.value;
//!            bincode::serialize(&decoded_state).unwrap()
//!        };
//!
//!        ...
//!
//!        store.reducers.insert("[Example Action]", example_reducer);
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
pub mod window_viewer;
pub mod gui_tk;
pub mod model_scheduler;
pub mod store;
pub mod dispatcher;

