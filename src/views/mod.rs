use super::canvas::Canvas;
use super::gui_tk::{Gui,  Event, GuiState, Palette};
use super::state::{Mutation};
use std::sync::mpsc::{Sender, Receiver};
use super::hid::{HIDEvent, IOState};


use std::thread;
use std::thread::JoinHandle;

use std::time::{Duration};
use log::*;

/*
     ButtonInitializer {pin: 5, code: 0, key: "b"},
     ButtonInitializer {pin: 6, code: 1, key: "a"},
     ButtonInitializer {pin: 27, code: 2, key: "l"},
     ButtonInitializer {pin: 23, code: 3, key: "r"},
     ButtonInitializer {pin: 17, code: 4, key: "up"},
     ButtonInitializer {pin: 22, code: 5, key: "dn"},
     ButtonInitializer {pin: 4, code:  6, key: "hat"},
*/


enum InputMode {
    Navigate,
    Manipulate
}

pub fn run_view(mut root_view: RootView) -> JoinHandle<()>{
    root_view.initialize();
    root_view.activate_bar();
    thread::spawn(move || {
        loop {
            match root_view.input_receiver.try_recv() {
                Ok(button_actions) => {
                    for ba in &button_actions {
                        debug!("HIDEvent: {:#?}", ba);
                        match root_view.handle_button_action(ba) {
                            Some(action) => {
                                root_view.handle_action(action);                
                            },
                            None => ()
                        }
                    }
                    
                },
                Err(_) => ()
            }

            //match root_view.state_receiver.try_recv() {
            //    Ok(state) => {
                    if root_view.update_bar() || root_view.update_active_view() {
                        root_view.render();
                    }
             //       debug!("Root View Received State Update");
             //   },
             //   Err(_) => ()
            //};
            thread::sleep(Duration::from_millis(5));
        }
    })
}

pub fn gui_state_updater(object: &mut Box<dyn Gui + Send>, new_state: GuiState, canvas: &mut Canvas) {
    let current_state = object.get_gui_state();
    if let GuiState::Base = current_state {
        match new_state {
            GuiState::Base => (),
            _ => {
                object.set_gui_state(new_state.clone(), canvas);
            }
        }
    }
    if let GuiState::Clicked = current_state {
        match new_state {
            GuiState::Clicked => (),
            _ => {
                object.set_gui_state(new_state.clone(), canvas);
            }
        }
    }
    if let GuiState::Selected = current_state {
        match new_state {
            GuiState::Selected => (),
            _ => {
                object.set_gui_state(new_state.clone(), canvas);
            }
        }
    }


}

#[derive(Clone)]
pub struct ViewSettings {
    pub device: &'static str,
    pub palette: Palette,
    pub font_file: &'static str
}

pub struct RootView {
    bar: View,
    views: Vec<Box<View>>,
    active: usize,
    canvas: Canvas,
    input_receiver: Receiver<Vec<HIDEvent>>,
    action_sender: Sender<Event>
}

impl RootView {
    pub fn new(fbdev: &'static str,  input_receiver: Receiver<Vec<HIDEvent>>, action_sender: Sender<Event>, info_bar_view: View) -> RootView {
        let canvas: Canvas = Canvas::new(fbdev);
        RootView {
            bar: info_bar_view,
            views: vec![],
            canvas: canvas,
            active: 0,
            input_receiver,
            action_sender
        }
    }
    pub fn initialize(&mut self) {
        self.bar.initialize(&mut self.canvas);
        for i in 0..self.views.len() {
            self.views[i].initialize(&mut self.canvas);
        }
    }

    // input button handling
    pub fn handle_button_action(&mut self, ba: &HIDEvent) -> Option<Event>{
        if self.views.len() > self.active {
            match ba.code {
                0 => self.set_active_view(0), // go home
                _ => self.views[self.active].handle_button_action(ba)
            }
        } else {
            panic!("Cannot route input to non existent active view");
        }
    }

    pub fn handle_action(&mut self, gui_action: Event) {
        self.action_sender.send(gui_action).unwrap();
    }


    // draw it out
    pub fn render(&mut self) {
        self.canvas.render();
    }

    // update the top bar
    pub fn update_bar(&mut self) -> bool {
        self.bar.update(&mut self.canvas)
    }

    pub fn activate_bar(&mut self) -> bool {
        self.bar.activate(&mut self.canvas)
    }

    // this is a move operation
    pub fn add_view(&mut self, view: View) {
        if view.objects_len() > 0 {
            self.views.push(Box::new(view));
        } else {
            panic!("Cannot add a view with 0 objects")
        }
    }

    pub fn update_active_view(&mut self) -> bool{
        if self.views.len() > self.active {
            if self.views[self.active].update( &mut self.canvas) {
                true
            } else {
                false
            }
        } else {
            panic!("Cannot update a view which does not exist");
        }
   
    }

    // for user input routing
    pub fn set_active_view(&mut self, view: usize) -> Option<Event>{
        if self.views.len() <= view {
            panic!("Cannot activate a view which does not exist");
        }
        for i in (0 as usize)..self.views.len() {
            if i == view{
               self.active = view;
               self.views[self.active].activate(&mut self.canvas);
            } else {
                self.views[i].deactivate(&mut self.canvas);
            }
        }
        None
    }
}

//abstract trait to impl the view Trait while keeping it dry



pub struct View {
    objects: Vec<Box<dyn Gui + Send>>,
    input_mode: InputMode,
    nav_index: Vec<Vec<Vec<usize>>>,
    selected_row: usize,
    selected_column: usize,
    selected_object: usize,
    state_receiver: Receiver<Vec<u8>>,
    mutation_sender: Sender<Mutation>,
    name: String,
    update_fn: ViewStateUpdater
}
  
pub type ViewStateUpdater = fn(&mut  Vec<Box<dyn Gui + Send>>, &[u8], &mut Canvas );

impl View {
    pub fn new(mutation_sender: Sender<Mutation>, name: String, update_fn: ViewStateUpdater, state_receiver: Receiver<Vec<u8>>) -> View {
        let objects: Vec<Box<dyn Gui + Send>> = vec![];
        let nav_index: Vec<Vec<Vec<usize>>> = vec![
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ],
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ],
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ],
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ]
                                                    ];
        let selected_row = 0;
        let selected_column = 0;
        let selected_object =  0;   
        View {
            objects,
            input_mode: InputMode::Navigate,
            nav_index,
            selected_row,
            selected_column,
            selected_object,
            mutation_sender,
            state_receiver,
            name,
            update_fn
        }
    }

    // add a navigable object to the view
    pub fn add_object(&mut self, object: Box<dyn Gui + Send>, row: usize, column: usize ) {
        let object_index = self.objects.len(); //
        self.objects.push(object);
        if self.nav_index.len() > row && self.nav_index[row].len() > column {
            self.nav_index[row][column].push(object_index);
        } else {
            panic!("Row and Column outside bounds");
        }
        
    }

    // add a static - non navigable object to the view
    #[allow(dead_code)]
    pub fn add_static_object(&mut self, object: Box<dyn Gui + Send>) {
        self.objects.push(object);
    }
    pub fn escape(&mut self) -> Option<Event>{
        None
    }



    pub fn nav(&mut self, ba: &HIDEvent) -> Option<Event> {
        match ba.code {
            2 => {
                    match ba.action { 
                        IOState::Pressed => self.h_move(-1),
                        _ => ()
                    }
                },
            3 => {
                    match ba.action { 
                        IOState::Pressed => self.h_move(1),
                        _ => ()
                    }

                }, 
            4 => {
                    match ba.action { 
                        IOState::Pressed => self.v_move(-1),
                        _ => ()
                    }
                },
            5 => {
                    match ba.action { 
                        IOState::Pressed => self.v_move(1),
                        _ => ()

                    }


                },
            _ => ()
        };
        None
    }

    pub fn v_move(&mut self, amount: isize) {
        // orig selected
        let original_selected_object = self.selected_object;

        // moving up and down
        let row_count = self.nav_index.len() as isize;
        let attempted_row = self.selected_row as isize + amount;
        debug!("Trying Row: {}", attempted_row);
        if attempted_row >= 0 && attempted_row < row_count {
            debug!(" - Row Exists");
            // the attempted row exists
            let attempted_row_length = self.nav_index[attempted_row as usize].len();
            // does the selected column exist?
            if self.selected_column < attempted_row_length && attempted_row_length > 0 {
                debug!("   - Current Column exists in this row");
                if self.nav_index[attempted_row as usize ][self.selected_column].len() > 0 {
                    debug!("     - Current Column Has Objects");
                    // is there anything in that column?
                    self.selected_row = attempted_row as usize;
                    self.selected_object = self.nav_index[self.selected_row][self.selected_column][0];
                } else {
                    debug!("      - Current Column has not objects - check the entire row");
                    // we move out adding and subtracting till we hit something
                    for offset in 1..attempted_row_length as isize { 
                        // pos
                        let pos_attempt = self.selected_column as isize + offset;
                        if pos_attempt >= 0 && (pos_attempt as usize) < attempted_row_length {
                            if self.nav_index[attempted_row as usize][pos_attempt as usize].len() > 0 {
                                self.selected_row = attempted_row as usize;
                                self.selected_column = pos_attempt as usize;
                                self.selected_object = self.nav_index[self.selected_row][self.selected_column][0];
                                break;
                            }
                        }
                        // neg
                        let neg_attempt = self.selected_column as isize - offset;
                        if neg_attempt >= 0 && (neg_attempt as usize) < attempted_row_length {
                            if self.nav_index[attempted_row as usize][neg_attempt as usize].len() > 0 {
                                self.selected_row = attempted_row as usize;
                                self.selected_column = neg_attempt as usize;
                                self.selected_object = self.nav_index[self.selected_row][self.selected_column][0];
                                break;
                            }
                        } 
                    }
                    if  original_selected_object == self.selected_object {
                        debug!("     - Row Appears Empty. Let's check the next row in this direction");
                        // we didn't hit anything recurse
                        if amount > 0 {
                            self.v_move(amount + 1)
                        } else if amount < 0 {
                            self.v_move(amount - 1);
                        }
                    }
                }

            } else if attempted_row_length > 0 { // there is stuff in here but we had hit a column with nothing in it
                debug!("     - We went to a row with existing columns, but exceeded the column count");
                // go to last column we can
                // if selected_column was out of bounds // try the greatest column on down to see if we could get anything
                    for attempted_column in (0..attempted_row_length).rev() {
                        if self.nav_index[attempted_row as usize][attempted_column as usize].len() > 0 {
                            self.selected_row = attempted_row as usize;
                            self.selected_column = attempted_column as usize;
                            self.selected_object = self.nav_index[self.selected_row][self.selected_column][0];
                            break;
                        }
                    }
                if  original_selected_object == self.selected_object {
                    debug!("     - There was still nothing in this row. Try the next row in this direction");
                    if original_selected_object != self.selected_object && amount > 0 {
                        self.v_move(amount + 1)
                    } else if original_selected_object != self.selected_object && amount < 0 {
                        self.v_move(amount - 1);
                    }
                }
            } else {// nothing in that row we could use... try another row up or down
                debug!("     - We hit a row with 0 columns, keep moving to the next row");
                if amount > 0 {
                    self.v_move(amount + 1)
                } else if amount < 0 {
                    self.v_move(amount - 1);
                }
            }
            // if change send mutator
            if original_selected_object != self.selected_object {
                self.mutation_sender.send( Mutation::new("[Move Selection To]", self.name.clone(), self.selected_object as isize) ).unwrap();
            }
            
        } // attempted row is out of bounds .. stay where we are
    }

    fn h_cell_move(&mut self, amount: isize) {
        let row_length = self.nav_index[self.selected_row].len() as isize;
        let attempted_column: isize =  (self.selected_column as isize) + amount;
        if attempted_column < row_length && attempted_column >= 0 {
            let cell_length =  self.nav_index[self.selected_row][attempted_column as usize].len();  
            if cell_length > 0 {
                // new column is good
                self.selected_column = attempted_column as usize; // set the selected_column
                if amount < 0 { // moving left
                    // selected = greatest
                    self.selected_object = self.nav_index[self.selected_row][self.selected_column][cell_length - 1];
                } else if amount > 0 { // moving right
                    // selected = smallest 
                    self.selected_object = self.nav_index[self.selected_row][self.selected_column][0];
                }
            } else {
                if amount > 0 {
                    self.h_cell_move(amount + 1);
                } else if amount < 0 {
                    self.h_cell_move( amount - 1);
                }
            }
        } // tried to move too far we bail and just stay where we are
    }

    pub fn h_move(&mut self, amount: isize) {
        // orig selected
        let original_selected_object = self.selected_object;
        // current cell
        let current_cell_length = self.nav_index[self.selected_row][self.selected_column].len();
        let current_cell_index: usize;
        match self.nav_index[self.selected_row][self.selected_column].iter().position(|x| *x == self.selected_object) {
            Some(x) => current_cell_index = x,
            None => {
                panic!("Could not find any objects matching selected_object: {}", self.selected_object);
            }
        }
        // move left one in cell
        if (current_cell_index as isize) >= 1 && amount < 0 {
            self.selected_object = self.nav_index[self.selected_row][self.selected_column][current_cell_index - 1];
        // move once cell left
        } else if (current_cell_index as isize) < 1 && amount < 0 {
            self.h_cell_move(-1);
        // move right one in cell
        } else if (current_cell_index as isize) + amount < current_cell_length as isize && amount > 0 {
            self.selected_object = self.nav_index[self.selected_row][self.selected_column][current_cell_index + 1];          
        } else if (current_cell_index as isize) + amount >= current_cell_length as isize && amount > 0 {
            self.h_cell_move(1);
        }
        if original_selected_object != self.selected_object {
            self.mutation_sender.send( Mutation::new("[Move Selection To]", self.name.clone(), self.selected_object as isize) ).unwrap();
        }
    }
    
    pub fn send_to_selected(&mut self, ba: &HIDEvent) -> Option<Event>{
        let (return_control, mutation, gui_action) = self.objects[self.selected_object].handle_button_action(ba);
        match mutation {
            Some(mutate) => self.mutation_sender.send(Mutation::new(mutate, self.name.clone(), self.selected_object as isize  )).unwrap(),
            None => ()
        };

        if return_control {
            self.input_mode = InputMode::Navigate;    
        } else {
            self.input_mode = InputMode::Manipulate;
        }
        gui_action
    }

    fn initialize(&mut self, canvas: &mut Canvas) -> bool {
         for i in (0 as usize)..self.objects.len() {
            if !self.objects[i].initialize(canvas) {
                return false;
            }
        }
        true
    }

    fn activate(&mut self, canvas: &mut Canvas) -> bool {
        for i in (0 as usize)..self.objects.len() {
            if !self.objects[i].activate(canvas) {
                return false;
            }
        }
        // set first object as selected upon activation
        if self.objects.len() > 0 {
            self.selected_object = 0;
            // find what cell that object is in
            self.selected_column = 0;
            self.selected_row = 0;
            self.mutation_sender.send( Mutation::new("[Move Selection To]", self.name.clone(), self.selected_object as isize) ).unwrap();
        }
        true
       // all objects 
    }
    fn update(&mut self, canvas: &mut Canvas) -> bool {
        // update each object in the view with the correct state data
        // each view will likely have its own linkage to state data
        // so we let the author of the view provide their own updater_fn
        let updated = match self.state_receiver.try_recv() {
            Ok(state) => {
                let update_fn_actor: ViewStateUpdater = self.update_fn;
                update_fn_actor(&mut self.objects, &state[..], canvas);
                true 
            },
            Err(_) => {
                false
            }
        };
        updated
    }
    
    fn deactivate(&mut self, canvas: &mut Canvas) -> bool {
        for i in (0 as usize)..self.objects.len() {
            if !self.objects[i].deactivate(canvas) {
                return false;
            }
        }
        true
    }

    fn handle_button_action(&mut self, ba: &HIDEvent) -> Option<Event> {
        // nav mode or manipulate mode?
        match self.input_mode {
            InputMode::Navigate => {
                match ba.code {
                    0 => None, // go home -- should be handled by root
                    1 => self.escape(),
                    6 => self.send_to_selected(ba),
                    _ => self.nav(ba)
                }
            }, // up / down / right / left will move the selection from widget to widget -- b = home, a = back == home
            InputMode::Manipulate => {
                self.send_to_selected(ba)
            } // the element will parse the the input mode for the input bus. -- b = home, a = back == navigate mode
        }
    }
    fn objects_len(&self) -> usize {
        self.objects.len()
    } 

}
