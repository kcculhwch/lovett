use super::*;
use log::*;
//Base Gui Impl
// box with text + all GuiStates
pub struct Block {
    pub event: Event,
    pub name: String,
    pub regular_name: String,
    pub selected_name: String,
    pub clicked_name: String,
    // cloned and appended to canvas
    pub layers: Vec<Layer<Box<dyn Draw + Send>>>,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub gui_state: GuiState,
    config: &'static GuiConfig
}

#[allow(dead_code)]
impl Block {
    pub fn new(x: i32, y: i32, w: i32, h: i32, event: Event, config: &'static GuiConfig) -> Block {
        let uuid_string = Uuid::new_v4().to_hyphenated().to_string();
        let name = format!("Block - {}", uuid_string); 

        let regular_name = format!("{} - regular", name);
        let selected_name = format!("{} - selected", name);
        let clicked_name = format!("{} - clicked", name);

        let gui_state =  GuiState::Base;


        let layers: Vec<Layer<Box<dyn Draw + Send>>> = vec![];
        let mut block = Block {
            event,
            name,
            regular_name,
            clicked_name,
            selected_name,
            layers,
            x,
            y,
            w,
            h,
            gui_state,
            config
        };
        block.gen_layers();
        block
    }
    pub fn reinit(&mut self, canvas: &mut Canvas){
        canvas.drop_layer_group(self.regular_name.clone());
        canvas.drop_layer_group(self.selected_name.clone());
        canvas.drop_layer_group(self.clicked_name.clone());
 
        //gen new layers
        self.gen_layers();
        self.initialize(canvas);


    }

    pub fn gen_layers(&mut self)  {
        
        // basic background box
        let bg: Layer<Box<dyn Draw + Send>> = Layer::new(
            Box::new(
                Rect::new(
                        self.x,
                        self.y, 
                        self.w, 
                        self.h, 
                        true, 
                        self.config.palette.base_background.clone()
                )
            ), 
            true, 
            self.regular_name.clone()
        );
        self.layers.push(bg);

        // Clicked background box
        let bg: Layer<Box<dyn Draw + Send>> = Layer::new(
            Box::new(
                Rect::new(
                    self.x, 
                    self.y, 
                    self.w, 
                    self.h, 
                    true, 
                    self.config.palette.clicked_background.clone()
                )
            ), 
            false, 
            self.clicked_name.clone()
        );
        self.layers.push(bg);

        // Selected background box
        let bg: Layer<Box<dyn Draw + Send>> = Layer::new(
            Box::new(
                Rect::new(
                    self.x, 
                    self.y, 
                    self.w, 
                    self.h, 
                    true, 
                    self.config.palette.selected_background.clone()
                )
            ), 
            false, 
            self.selected_name.clone()
        );
        self.layers.push(bg);
        debug!("Copied Layers to canvas for {}", self.name);
    }
}
impl Gui for Block {
    fn initialize(&mut self, canvas: &mut Canvas) -> bool {
        // add all layers to the canvas
        // self.layers is now empty
        canvas.layers.append(&mut self.layers);
        true
    }
    fn activate(&mut self, canvas: &mut Canvas) -> bool {
        //set the correct layers to active        
        match self.gui_state {
            GuiState::Base => {
                canvas.activate_layer_group(self.regular_name.clone());
                canvas.deactivate_layer_group(self.selected_name.clone());
                canvas.deactivate_layer_group(self.clicked_name.clone());
                debug!("Activate Base: {}", self.name);
            },
            GuiState::Clicked => {
                canvas.deactivate_layer_group(self.regular_name.clone());
                canvas.deactivate_layer_group(self.selected_name.clone());
                canvas.activate_layer_group(self.clicked_name.clone());
                debug!("Activate Clicked: {}", self.name);
            },
            GuiState::Selected => {
                canvas.deactivate_layer_group(self.regular_name.clone());
                canvas.activate_layer_group(self.selected_name.clone());
                canvas.deactivate_layer_group(self.clicked_name.clone());
                debug!("Activate Selected: {}", self.name);
            }
        };
        true
    }

    fn deactivate(&mut self, canvas: &mut Canvas) -> bool{
        canvas.deactivate_layer_group(self.regular_name.clone());
        canvas.deactivate_layer_group(self.selected_name.clone());
        canvas.deactivate_layer_group(self.clicked_name.clone());
        true 
    }

    #[allow(unused_variables)]
    fn set_text(&mut self, text: String, canvas: &mut Canvas) {
    }

    fn get_text(&mut self) -> &str{
        ""
    }

    fn set_gui_state(&mut self, gui_state: GuiState, canvas: &mut Canvas){
        self.gui_state = gui_state;
        self.activate(canvas);
    }
    fn get_gui_state(&self) -> GuiState {
        self.gui_state.clone()
    }

    fn handle_hid_event(&mut self, h_e: &HIDEvent) -> (bool, Option<GuiState>, Option<Event>) {
        match h_e.code {
            // handle hat press
            6 => {
                match h_e.io_state {
                    IOState::Pressed => {
                            (false, Some(GuiState::Clicked), Some(self.event.clone()))
                        },
                    IOState::Released => {
                            (true, Some(GuiState::Selected), None)
                        },
                    _ => (false, None, None)
                }
            },    
            _ => (false, None, None)
        }

        // true // returns back to view input handle
        // false // keeps input mode here
    }


}


