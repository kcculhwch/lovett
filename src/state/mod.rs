use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::collections::HashMap;
use std::time::{Instant,  Duration};

//use chrono::format::strftime;
use chrono::DateTime;
use chrono::Local;

use super::gui_tk::GuiState;

pub fn run_state(mut root_state:  RootState) {
        thread::spawn(move || {
            loop {

                // lisen for mutators
                let mutator: Mutator = match root_state.mutation_receiver.try_recv() {
                    Ok(mutator) => mutator,
                    Err(_) => Mutator::new("", "".to_string(), 0)

                };
                // process mutation
                if mutator.name != "".to_string() {

                        root_state.mutate(mutator);
                        println!("Got Mutator");
                        // send state clone
                        for sender in &root_state.state_senders {
                            sender.send(root_state.state.clone()).unwrap();
                        }
                }
                thread::sleep(Duration::from_millis(5));
            }
        });

}



pub struct RootState {
    pub state: State,
    pub state_senders: Vec<Sender<State>>,
    pub mutation_receiver: Receiver<Mutator>,
    mutation_sender: Sender<Mutator>   
}

#[derive(Clone, Debug)]
pub struct State {
    pub boiler: BoilerState,
    pub tank: TankState,
    pub time: TimeState,
    pub settings: SettingsState,
    pub views: HashMap<&'static str, Vec<GuiState>>
}

#[derive(Clone, Debug)]
pub struct BoilerState {
    pub element_on: bool,
    pub temperature: i32
}

#[derive(Clone, Debug)]
pub struct TankState {
    pub level: i32
}

#[derive(Clone, Debug)]
pub struct TimeState {
    pub turned_on:  Instant,
    pub current_time: String
}

#[derive(Clone, Debug)]
pub struct SettingsState {
    pub running: bool,
    pub p: u32,
    pub i: u32,
    pub d: u32 
}
#[derive(Clone, Debug)]
pub struct ViewsState {
    pub bar: Vec<GuiState>,
    pub boiler: Vec<GuiState>,
    pub steamer: Vec<GuiState>,
    pub settings: Vec<GuiState>
}

impl RootState {
    pub fn new() -> RootState {
        let (sender, receiver) = channel();
        let mut views: HashMap<&str, Vec<GuiState>> = HashMap::new();
        views.insert("bar", vec![]);
        views.insert("boiler", vec![]);
        views.insert("steamer", vec![]);
        views.insert("settings", vec![]);

        RootState {
            state: State {
                boiler: BoilerState {
                    element_on: false,
                    temperature: 0
                },
                tank: TankState {
                    level: 0                    
                },
                time: TimeState {
                    turned_on: Instant::now(),
                    current_time: "00:00:00 XX".to_string() 
                },
                settings: SettingsState {
                    running: false,
                    p: 0,
                    i: 0,
                    d: 0
                },
                views,
            },
            state_senders: vec![],
            mutation_receiver: receiver,
            mutation_sender: sender       
            
        }
    }

    pub fn reg_state_sender(&mut self, sender: Sender<State>) { 
        self.state_senders.push(sender);
    }


    pub fn get_mutation_sender(&self) -> Sender<Mutator> {
        self.mutation_sender.clone()
    }


    pub fn mutate(&mut self, mutator: Mutator) {
        match mutator.name {
            "[time.current_time]" => {
                self.state.time.current_time = mutator.value;
            },
            "[Move Selection To]" => {
                let current = self.state.views.get(mutator.value.as_str()).unwrap().iter().position(|x| match x { 
                    GuiState::Selected => true,
                    _ => false
                    });
                match current {
                    Some(position) => self.state.views.get_mut(mutator.value.as_str()).unwrap()[position] = GuiState::Base,
                    _ => ()
                };
                self.state.views.get_mut(mutator.value.as_str()).unwrap()[mutator.number as usize] = GuiState::Selected;
            },
            "[Clicked Button]" => {
                self.state.views.get_mut(mutator.value.as_str()).unwrap()[mutator.number as usize] = GuiState::Clicked;
            },
            "[Released Button]" => {
                self.state.views.get_mut(mutator.value.as_str()).unwrap()[mutator.number as usize] = GuiState::Selected;
            }
            _ => ()
        }        
    }
}

pub struct Mutator {
    name: &'static str,
    value: String,
    number: isize
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
            println!("Tick");
            thread::sleep(Duration::from_millis(1000));        
        };

    });
}
