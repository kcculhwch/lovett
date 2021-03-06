use std::sync::mpsc::{Receiver};

use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;

pub fn run_model_scheduler(mut scheduler: ModelScheduler) -> JoinHandle<()>{
    thread::spawn(move || {
        while let Ok(state_vec) = scheduler.state_rx.recv() {
            scheduler.process_state(state_vec);
        }
    })
}


#[allow(dead_code)]
pub struct ModelScheduler {
    models: HashMap<&'static str, Box<dyn Model + Send>>,
    threads: HashMap<&'static str, Option<JoinHandle<()>>>,
    pub state_rx: Receiver<Vec<u8>>,
    schedule: Schedule
}

impl ModelScheduler {
    pub fn new(state_rx: Receiver<Vec<u8>>, schedule: Schedule) -> ModelScheduler {
        let models: HashMap<&'static str, Box<dyn Model + Send>> = HashMap::new();
        let threads: HashMap<&'static str, Option<JoinHandle<()>>> = HashMap::new();
        ModelScheduler {
            models,
            state_rx,
            schedule,
            threads
        }
    }

    pub fn process_state(&mut self, state_vec: Vec<u8>){
        let schedule_fn = self.schedule;
    }

    pub fn register_model(&mut self,  name: &'static str, model: Box<dyn Model + Send>) {
        self.models.insert(name, model);
    }


}
pub trait Model {
    fn handler(&mut self) {

    }

}
#[allow(dead_code)]
pub type Schedule = fn(&[u8], &mut HashMap<&'static str, Box<dyn Model + Send>>, &mut HashMap<&'static str, Option<JoinHandle<()>>>);
