// Crates
extern crate framebuffer;
extern crate image;
extern crate rusttype;
extern crate serde;
extern crate bincode;

// Modules
pub mod joy_pad;
pub mod fb;
pub mod canvas;
pub mod views;
pub mod gui_tk;
pub mod controller;
pub mod state;

/*

State Stuff
// State DEf
struct InfoBar {
    objects: Vec<Box<dyn Gui + Send>>, 
    pub update_fn :ViewStateUpdater 
}

impl InfoBar {
    pub fn new() -> InfoBar {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];

        InfoBar {
            objects: objects
        }
    }
    // since infobar is always in view initialize and activate functions are combined
    // there is no deactivate
    pub fn activate(&mut self,  canvas: &mut Canvas) -> bool {
         for i in (0 as usize)..(self.objects.len() as usize) {
            if !(self.objects[i].initialize(canvas) && self.objects[i].activate(canvas)) {
                println!("Could not get bar objects initialized and activated!");
                return false;
            }
        }
        true
    }
    pub fn update(&mut self, state: &State, canvas: &mut Canvas) -> bool{

        // update each object in the view with the correct state data
        // each view will likely have its own linkage to state data
        // so we let the author of the view provide their own updater_fn
        let update_fn_actor: ViewStateUpdater = self.update_fn;

        update_fn_actor(&mut self.objects, &state, canvas);
        true 


        //update each object in the view with the correct state data
        self.objects[0].set_text(state.time.current_time.clone(), canvas);
        self.objects[0].set_gui_state(state.views.get("bar").unwrap()[0].clone(), canvas);
        true
    }

    
}





        /*
        let mut views: HashMap<&str, Vec<GuiState>> = HashMap::new();
        views.insert("bar", vec![]);
        views.insert("boiler", vec![]);
        views.insert("steamer", vec![]);
        views.insert("settings", vec![]);
        */


                State {
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


*/

/* View Stuff
// InfoBar
        let button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 0, 140, 28, GuiAction::new("Time Click", None)));
        root_state.state.views.get_mut("bar").unwrap().push(button.gui_state.clone());
        objects.push(button);

// Generic add object to view
// adds a state tracker
        root_state.state.views.get_mut(&self.name[..]).unwrap().push(object.get_gui_state());
struct InfoBar {
    objects: Vec<Box<dyn Gui + Send>>, 
    pub update_fn :ViewStateUpdater 
}

impl InfoBar {
    pub fn new() -> InfoBar {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];

        InfoBar {
            objects: objects
        }
    }
    // since infobar is always in view initialize and activate functions are combined
    // there is no deactivate
    pub fn activate(&mut self,  canvas: &mut Canvas) -> bool {
         for i in (0 as usize)..(self.objects.len() as usize) {
            if !(self.objects[i].initialize(canvas) && self.objects[i].activate(canvas)) {
                println!("Could not get bar objects initialized and activated!");
                return false;
            }
        }
        true
    }
    pub fn update(&mut self, state: &State, canvas: &mut Canvas) -> bool{

        // update each object in the view with the correct state data
        // each view will likely have its own linkage to state data
        // so we let the author of the view provide their own updater_fn
        let update_fn_actor: ViewStateUpdater = self.update_fn;

        update_fn_actor(&mut self.objects, &state, canvas);
        true 


        //update each object in the view with the correct state data
        self.objects[0].set_text(state.time.current_time.clone(), canvas);
        self.objects[0].set_gui_state(state.views.get("bar").unwrap()[0].clone(), canvas);
        true
    }

    
}


*/
