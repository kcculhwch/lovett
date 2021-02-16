use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::{Duration};
use super::gui_tk::GuiState;

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

pub type Mutator = fn(&[u8], Mutation) -> Vec<u8>;

pub type StateSenderFilter = fn(&[u8], &Vec<u8>) -> bool;

pub struct FilteredStateSender {
    pub state_sender: Sender<Vec<u8>>,
    pub state_sender_filter: StateSenderFilter
}


pub struct RootState {
    pub state: Vec<u8>,
    pub filtered_state_senders: Vec<FilteredStateSender>,
    pub mutation_receiver: Receiver<Mutation>,
    mutation_sender: Sender<Mutation>,   
    pub mutators: HashMap<&'static str, Mutator>
}

impl RootState {
    pub fn new(state: Vec<u8>) -> RootState {
        let (sender, receiver) = channel();
        let mutators: HashMap<&'static str, Mutator > = HashMap::new();
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


    pub fn get_mutation_sender(&self) -> Sender<Mutation> {
        self.mutation_sender.clone()
    }


    pub fn mutate(&mut self, mutator: Mutation) -> bool{
        //
        let mutated = match self.mutators.get(mutator.name) {
            Some(mutator_fn) =>  {
                let state_updater_fn: Mutator = *mutator_fn;
                
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
pub struct Mutation {
    pub name: &'static str,
    pub value: String,
    pub number: isize
}
impl Mutation {
    pub fn new(name: &'static str, value: String, number: isize) -> Mutation {
        Mutation {
            name,
            value,
            number
        }
    }

}

pub fn eq_gui_states(gui_states_1: &Vec<GuiState>, gui_states_2: &Vec<GuiState>) -> bool {
    if gui_states_1.len() == gui_states_2.len() {
        let mut eq = true;
        for i in 0..gui_states_1.len() {
            if !eq_gui_state(&gui_states_1[i], &gui_states_2[i]) {
                eq = false;
                break;        
            }
        }
        eq
    } else {
        false
    } 
}


pub fn eq_gui_state(gui_state_1: &GuiState, gui_state_2: &GuiState) -> bool {
    let mut eq = true;
    if let GuiState::Base = gui_state_1 {
        eq = match gui_state_2 {
            GuiState::Base => true,
            _ => false
        };
    }

    if let GuiState::Clicked = gui_state_1 {
        eq = match gui_state_2 {
            GuiState::Clicked => true,
            _ => false
        };
    }

    if let GuiState::Selected = gui_state_1 {
        eq = match gui_state_2 {
            GuiState::Selected => true,
            _ => false
        };
    }
    return eq
}



