use super::gui_tk::Event;
use std::sync::mpsc::{Sender, Receiver, channel};
use super::store::Action as Action;

use std::thread;
use std::thread::JoinHandle;

pub struct Dispatcher {
    pub event_rx: Receiver<Event>,
    event_tx: Sender<Event>,
    action_tx: Sender<Action>,
    handler: Box<dyn Dispatch + Send>
}

pub fn run_dispatcher(mut dispatcher: Dispatcher) -> JoinHandle<()>{
    thread::spawn(move || {                       
            while let Ok(event) = dispatcher.event_rx.recv() {
               dispatcher.handle_event(event);
            } 
            println!("Dispatcher exited");
    })
}




impl Dispatcher {
    pub fn new(action_tx: Sender<Action>, handler: Box<dyn Dispatch + Send>) -> Dispatcher {
        let (sender, receiver) = channel();
        Dispatcher {
            event_tx: sender,
            event_rx: receiver,
            action_tx,
            handler
        }
    }

    pub fn handle_event(&mut self, event: Event) -> bool{
        match self.handler.handle_event(event) {
            Some(action) => {
                self.action_tx.send(action).unwrap();
                true
            },
            _ => false

        }
    }
    pub fn get_event_sender(&self) -> Sender<Event> {
        self.event_tx.clone()
    }

}


pub trait Dispatch {
    fn handle_event(&self, event: Event) -> Option<Action>;
}


