use std::sync::mpsc::{Sender, Receiver, channel};
use super::gui_tk::Event;

use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::{Duration};

pub fn run_controller(mut root_controller: RootController) -> JoinHandle<()>{
    thread::spawn(move || {
        loop {                
            let event: Event = match root_controller.event_receiver.try_recv() {
                Ok(event) => event,
                Err(_) => Event::new("", None)
            };
            root_controller.handle_event(&event);
            thread::sleep(Duration::from_millis(5));
        }
    })

}


#[allow(dead_code)]
pub struct RootController {
    controllers: HashMap<&'static str, Controller>,
    pub event_receiver: Receiver<Event>,
    event_sender: Sender<Event>
}

impl RootController {
    pub fn new() -> RootController {
        let (sender, receiver) = channel();
        let controllers: HashMap<&'static str, Controller> = HashMap::new();
        RootController {
            controllers,
            event_sender: sender,
            event_receiver: receiver
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event.name {
            _ => ()
        }
    }
    pub fn get_event_sender(&self) -> Sender<Event> {
        self.event_sender.clone()
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

trait Model {}

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

