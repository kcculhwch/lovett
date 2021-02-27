use std::sync::mpsc::{Sender, Receiver, channel};
use super::gui_tk::Event;
use super::state::Mutation;

use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::{Duration};

pub fn run_controller(mut root_controller: RootController) -> JoinHandle<()>{
    thread::spawn(move || {
        loop {                
            let event: Event = match root_controller.event_rx.try_recv() {
                Ok(event) => event,
                Err(_) => Event::new("", None)
            };
            root_controller.handle_event(event);
            thread::sleep(Duration::from_millis(5));
        }
    })

}


#[allow(dead_code)]
pub struct RootController {
    controllers: HashMap<&'static str, Controller>,
    pub event_rx: Receiver<Event>,
    event_tx: Sender<Event>,
    mutation_tx: Sender<Mutation>,
    router: Box<dyn Route + Send>
}

impl RootController {
    pub fn new(mutation_tx: Sender<Mutation>, router: Box<dyn Route + Send>) -> RootController {
        let (sender, receiver) = channel();
        let controllers: HashMap<&'static str, Controller> = HashMap::new();
        RootController {
            controllers,
            event_tx: sender,
            event_rx: receiver,
            mutation_tx,
            router
        }
    }

    pub fn handle_event(&mut self, event: Event) -> bool{
        self.router.handle_event(event)
    }
    pub fn get_event_sender(&self) -> Sender<Event> {
        self.event_tx.clone()
    }

}

#[allow(dead_code)]
pub struct Controller {
    actions: HashMap<&'static str, Action>,
    models: HashMap<&'static str, Box<dyn Model + Send>>
}

#[allow(dead_code)]
impl Controller {
    pub fn new() -> Controller {
        let actions: HashMap<&'static str, Action> = HashMap::new();
        let models: HashMap<&'static str, Box<dyn Model + Send>> = HashMap::new();
        Controller {
            actions,
            models
        }
    } 
}

pub type Action = fn();

pub trait Model {}

pub trait Route {
    fn handle_event(&self, event: Event) -> bool;
}

/*

Event.controller = settings
Event.action = start_boiler
Event.args = vec["steam"]

root.controllers["settings"]["start_boiler"](vec[steam])

Action start_boiler([steam]) {
    models[boiler]
    
}



*/


// router  action loop

// recieve action events from gui

// send mutators to change state data

// 

