use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::{Duration};

//use chrono::format::strftime;
//use log::*;

pub fn run_state(mut root_state:  RootState) -> JoinHandle<()>{
        thread::spawn(move || {
            loop {

                // lisen for mutators
                match root_state.mutation_receiver.try_recv() {
                    Ok(mutator) => {
                        root_state.mutate(mutator);
                    },
                    Err(_) => ()
                };
                thread::sleep(Duration::from_millis(5));
            }
        })
}

pub type StateMutator = fn(&[u8], Mutator) -> Vec<u8>;

pub type StateSenderFilter = fn(&[u8], &Vec<u8>) -> bool;

pub struct FilteredStateSender {
    pub state_sender: Sender<Vec<u8>>,
    pub state_sender_filter: StateSenderFilter
}


pub struct RootState {
    pub state: Vec<u8>,
    pub filtered_state_senders: Vec<FilteredStateSender>,
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
            filtered_state_senders: vec![],
            mutation_receiver: receiver,
            mutation_sender: sender       
            
        }
    }

    pub fn reg_state_sender(&mut self, sender: Sender<Vec<u8>>, state_sender_filter: StateSenderFilter) { 
        let filtered_state_sender: FilteredStateSender = FilteredStateSender {
            state_sender: sender,
            state_sender_filter
        };      
        self.filtered_state_senders.push(filtered_state_sender);
    }


    pub fn get_mutation_sender(&self) -> Sender<Mutator> {
        self.mutation_sender.clone()
    }


    pub fn mutate(&mut self, mutator: Mutator) -> bool{
        //
        let mutated = match self.mutators.get(mutator.name) {
            Some(mutator_fn) =>  {
                let state_updater_fn: StateMutator = *mutator_fn;
                
                let new_state = state_updater_fn(&self.state[..], mutator);
                for filtered_state_sender in &self.filtered_state_senders {
                    let state_sender_filter_fn: StateSenderFilter  = filtered_state_sender.state_sender_filter;
                    if state_sender_filter_fn(&self.state[..], &new_state) {
                        filtered_state_sender.state_sender.send(new_state.clone()).unwrap();
                    }
                }
                self.state = new_state;
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


