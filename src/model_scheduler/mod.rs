use std::sync::mpsc::{Sender, Receiver, channel};

use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::{Duration};

pub fn run_model_scheduler(mut scheduler: ModelScheduler) -> JoinHandle<()>{
    thread::spawn(move || {
        loop {                
            scheduler.process_state();
            thread::sleep(Duration::from_millis(5));
        }
    })

}


#[allow(dead_code)]
pub struct ModelScheduler {
    models: HashMap<&'static str, Box<dyn Model + Send>>,
    state_tx: Sender<Vec<u8>>,
    state_rx: Receiver<Vec<u8>>,
    schedule: Schedule
}

impl ModelScheduler {
    pub fn new(schedule: Schedule) -> ModelScheduler {
        let (sender, receiver) = channel();
        let models: HashMap<&'static str, Box<dyn Model + Send>> = HashMap::new();
        ModelScheduler {
            models,
            state_tx: sender,
            state_rx: receiver,
            schedule
        }
    }

    pub fn process_state(&mut self) -> bool{
        let schedule_fn = self.schedule;
        match self.state_rx.try_recv() { 
            Ok(state_vec) => {
                schedule_fn(&state_vec, &mut self.models);
                true
            },
            Err(_) => (false)


        }
    }

    pub fn register_model(&mut self, name: &'static str, model: Box<dyn Model + Send>) {
        self.models.insert(name, model);
    }
}


pub trait Model {
    fn handler(&mut self) {

    }
}
#[allow(dead_code)]
type Schedule = fn(&[u8], &HashMap<&'static str, Box<dyn Model + Send>>);
