use std::sync::mpsc::{Sender, Receiver, channel};

use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::{Duration};

pub fn run_scheduler(mut scheduler: ModelScheduler) -> JoinHandle<()>{
    thread::spawn(move || {
        loop {                
            scheduler.process_events();
            thread::sleep(Duration::from_millis(5));
        }
    })

}


#[allow(dead_code)]
pub struct ModelScheduler {
    models: HashMap<&'static str, Box<dyn Model + Send>>,
    state_tx: Sender<Vec<u8>>,
    state_rx: Receiver<Vec<u8>>
}

impl ModelScheduler {
    pub fn new() -> ModelScheduler {
        let (sender, receiver) = channel();
        let models: HashMap<&'static str, Box<dyn Model + Send>> = HashMap::new();
        ModelScheduler {
            models,
            state_tx: sender,
            state_rx: receiver
        }
    }

    pub fn process_events(&mut self) -> bool{
        true
    }
}


pub trait Model {}


