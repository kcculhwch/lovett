use super::*;

//Base Gui Impl
// box with text + all GuiStates
pub struct TextBlock {
    pub text: String,
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
    pub config: &'static GuiConfig
}

#[allow(dead_code)]
impl TextBlock {
    pub fn new(text: String, x: i32, y: i32, w: i32, h: i32, event: Event, config: &'static GuiConfig) -> TextBlock {
        let uuid_string = Uuid::new_v4().to_hyphenated().to_string();
        let name = format!("TextBlock - {}", uuid_string); 

        let regular_name = format!("{} - regular", name);
        let selected_name = format!("{} - selected", name);
        let clicked_name = format!("{} - clicked", name);

        let gui_state =  GuiState::Base;


        let layers: Vec<Layer<Box<dyn Draw + Send>>> = vec![];
        let mut button = TextBlock {
            text,
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
        button.gen_layers();
        button
    }
    pub fn reinit(&mut self, canvas: &mut Canvas){
        canvas.drop_layer_group(self.regular_name.clone());
        canvas.drop_layer_group(self.selected_name.clone());
        canvas.drop_layer_group(self.clicked_name.clone());
 
        //gen new layers
        self.gen_layers();
        self.initialize(canvas);
        self.activate(canvas);

    }

    pub fn gen_layers(&mut self)  {
        
        // basic background box
        let mut text: Box<Text> = Box::new(
            Text::new(
                self.x, 
                self.y, 
                self.h as f32, 
                self.text.clone(), 
                &self.config.font,  
                self.config.palette.base_text.clone(), 
                2
            )
        );
        let text_width = text.w;
        if text_width < self.w {
            let x_offset = (self.w - text_width) / 2;
            text.x = self.x + x_offset;
        }
        let text_layer: Layer<Box<dyn Draw + Send>> = Layer::new(text, false, self.regular_name.clone()); 

        self.layers.push(text_layer);

        // Clicked background box
        let mut text: Box<Text> = Box::new(
            Text::new(
                self.x, 
                self.y, 
                self.h as f32, 
                self.text.clone(), 
                &self.config.font,  
                self.config.palette.clicked_text.clone(),
                2
            )
        );
        let text_width = text.w;
        if text_width < self.w {
            let x_offset = (self.w - text_width) / 2;
            text.x = self.x + x_offset;
        }
        let text_layer: Layer<Box<dyn Draw + Send>> = Layer::new(text, false, self.clicked_name.clone()); 

        self.layers.push(text_layer);

        // Selected background box
        let mut text: Box<Text> = Box::new(
            Text::new(
                self.x, 
                self.y, 
                self.h as f32, 
                self.text.clone(), 
                &self.config.font,  
                self.config.palette.selected_text.clone(), 
                2
            )
        );
        let text_width = text.w;
        if text_width < self.w {
            let x_offset = (self.w - text_width) / 2;
            text.x = self.x + x_offset;
        }
        let text_layer: Layer<Box<dyn Draw + Send>> = Layer::new(text, false, self.selected_name.clone()); 

        self.layers.push(text_layer);
    }
}
impl Gui for TextBlock {
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
            },
            GuiState::Clicked => {
                canvas.deactivate_layer_group(self.regular_name.clone());
                canvas.deactivate_layer_group(self.selected_name.clone());
                canvas.activate_layer_group(self.clicked_name.clone());
            },
            GuiState::Selected => {
                canvas.deactivate_layer_group(self.regular_name.clone());
                canvas.activate_layer_group(self.selected_name.clone());
                canvas.deactivate_layer_group(self.clicked_name.clone());
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

    fn set_text(&mut self, text: String, canvas: &mut Canvas) {
        self.text = text;
        // update regular_name
        let mut layers = canvas.get_layer_group(self.regular_name.clone());
        layers.append( &mut canvas.get_layer_group(self.selected_name.clone()));
        layers.append( &mut canvas.get_layer_group(self.clicked_name.clone()));
        // reverse here to preserve the order
        for layer in layers.iter_mut() {
            layer.item.update_text(self.text.clone())
        }
        canvas.layers.append(&mut layers);

    }

    fn get_text(&mut self) -> &str{
        &self.text[..]
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


