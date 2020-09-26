use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::{Duration};

//use chrono::format::strftime;
use chrono::DateTime;
use chrono::Local;

use log::*;

pub fn run_state(mut root_state:  RootState) -> JoinHandle<()>{
        thread::spawn(move || {
            loop {

                // lisen for mutators
                match root_state.mutation_receiver.try_recv() {
                    Ok(mutator) => {
                        if root_state.mutate(mutator) {
                            // send state clone
                            for sender in &root_state.state_senders {
                                sender.send(root_state.state.clone()).unwrap();
                            }
                        }
                    },
                    Err(_) => ()

                };
                thread::sleep(Duration::from_millis(5));
            }
        })
}

pub type StateMutator = fn(&[u8], Mutator) -> Vec<u8>;

pub struct RootState {
    pub state: Vec<u8>,
    pub state_senders: Vec<Sender<Vec<u8>>>,
    pub mutation_receiver: Receiver<Mutator>,
    mutation_sender: Sender<Mutator>,   
    pub mutators: HashMap<&'static str, StateMutator>
}

impl RootState {
    pub fn new(state: Vec<u8>) -> RootState {
        let (sender, receiver) = channel();
        let mutators: HashMap<&'static str, StateMutator > = HashMap::new();
        RootState {
            mutators,
            state,
            state_senders: vec![],
            mutation_receiver: receiver,
            mutation_sender: sender       
            
        }
    }

    pub fn reg_state_sender(&mut self, sender: Sender<Vec<u8>>) { 
        self.state_senders.push(sender);
    }


    pub fn get_mutation_sender(&self) -> Sender<Mutator> {
        self.mutation_sender.clone()
    }


    pub fn mutate(&mut self, mutator: Mutator) -> bool{
        //
        let mutated = match self.mutators.get(mutator.name) {
            Some(mutator_fn) =>  {
                let state_updater_fn: StateMutator = *mutator_fn;
                self.state = state_updater_fn(&self.state[..], mutator);
                true
            },
            None => {
                false
            }
            
        };
        mutated
    }
}

#[allow(dead_code)]
pub struct Mutator {
    pub name: &'static str,
    pub value: String,
    pub number: isize
}
impl Mutator {
    pub fn new(name: &'static str, value: String, number: isize) -> Mutator {
        Mutator {
            name,
            value,
            number
        }
    }

}


fn get_current_time() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%r").to_string()
}

pub fn time_keeper(mutation_sender: Sender<Mutator>) {
    thread::spawn( move|| {
        loop {
            mutation_sender.send(
                Mutator{
                    name: "[time.current_time]",
                    value: get_current_time(),
                    number: 0
                }
            ).unwrap();
            debug!("Clock Tick");
            thread::sleep(Duration::from_millis(1000));        
        };

    });
}
