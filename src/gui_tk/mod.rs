use serde::{Serialize, Deserialize};

use uuid::Uuid;

use super::canvas::{Rect, Layer, Draw, Canvas, Text};
use super::fb::Color;

use super::joy_pad::ButtonAction;
use super::joy_pad::Action as JAction;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GuiState{
    Base,
    Clicked,
    Selected
}

//color pallete
struct Palette {
    base: Color,
    base_text: Color,
    base_background: Color,

    selected: Color,
    selected_text: Color,
    selected_background: Color,

    clicked: Color,
    clicked_text: Color,
    clicked_background: Color,

}

impl Palette {
    fn new() -> Palette {
        Palette {
            base: Color::new(78, 156, 183),
            base_text: Color::new(255, 255, 255),
            base_background: Color::new(30, 50, 50),

            selected: Color::new(78, 156, 183),
            selected_text: Color::new(255, 255, 255),
            selected_background: Color::new(60, 100, 100),

            clicked: Color::new(160, 255, 255),
            clicked_text: Color::new(0, 0, 0),
            clicked_background: Color::new(120, 200, 200),

        }
    }
}
// Gui Objects

// abstract trait for keeping it Dry

// Gui Trait
pub trait Gui {
/*    fn select(&mut self, canvas: &mut Canvas);
    fn deselect(&mut self, canvas: &mut Canvas);
    fn click(&mut self, canvas: &mut Canvas);
    fn lock(&mut self, canvas: &mut Canvas);
    fn unlock(&mut self, canvas: &mut Canvas);
    fn show(&mut self, canvas: &mut Canvas);
    fn hide(&mut self, canvas: &mut Canvas);
*/
//    fn draw(&self, canvas: &mut Canvas) -> bool;

    // move the layers over to the canvas
    #[allow(unused_variables)]
    fn initialize(&mut self, canvas: &mut Canvas) -> bool {
        true
    } 


    #[allow(unused_variables)]
    fn activate(&mut self, canvas: &mut Canvas) -> bool {
        // sets the current state layers to active on the canvas
        true
    }


    #[allow(unused_variables)]
    fn deactivate(&mut self, canvas: &mut Canvas) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn set_text(&mut self,  text: String, canvas: &mut Canvas){
        ()
    }

    #[allow(unused_variables)]
    fn get_text(&mut self) -> &str {
        ""
    }
   

    #[allow(unused_variables)]
    fn set_gui_state(&mut self, gui_state: GuiState, canvas: &mut Canvas){
        ()
    }
    fn get_gui_state(&self) -> GuiState {
        GuiState::Base
    }

    #[allow(unused_variables)]
    fn handle_button_action(&mut self, ba: &ButtonAction) -> (bool, Option<&'static str>, Option<GuiAction>)  {
        (true, None, None)
    }

}

//Base Gui Impl

pub struct Button {
    pub text: String,
    pub action: GuiAction,
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
    pub gui_state: GuiState
}

impl Button {
    pub fn new(text: String, x: i32, y: i32, w: i32, h: i32, action: GuiAction) -> Button {
        let uuid_string = Uuid::new_v4().to_hyphenated().to_string();
        let name = format!("Button - {}", uuid_string); 

        let regular_name = format!("{} - regular", name);
        let selected_name = format!("{} - selected", name);
        let clicked_name = format!("{} - clicked", name);

        let gui_state =  GuiState::Base;


        let layers: Vec<Layer<Box<dyn Draw + Send>>> = vec![];
        let mut button = Button {
            text,
            action,
            name,
            regular_name,
            clicked_name,
            selected_name,
            layers,
            x,
            y,
            w,
            h,
            gui_state
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


    }

    pub fn gen_layers(&mut self)  {
        let palette = Palette::new();
        
        // basic background box
        let bg: Layer<Box<dyn Draw + Send>> = Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, true, palette.base_background.clone())), true, self.regular_name.clone());
        let outline: Layer<Box<dyn Draw + Send>> = Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, false, palette.base.clone())), true, self.regular_name.clone());
        let mut text: Box<Text> = Box::new(Text::new(self.x, self.y, self.h as f32, self.text.clone(), "./assets/fonts/Antic_Slab/AnticSlab-Regular.ttf",  palette.base_text.clone(), 2),);
        let text_width = text.w;
        if text_width < self.w {
            let x_offset = (self.w - text_width) / 2;
            text.x = self.x + x_offset;
        }
        let text_layer: Layer<Box<dyn Draw + Send>> = Layer::new(text, true, self.regular_name.clone()); 

        self.layers.push(bg);
        self.layers.push(outline);
        self.layers.push(text_layer);

        // Clicked background box
        let bg: Layer<Box<dyn Draw + Send>> = Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, true, palette.clicked_background.clone())), false, self.clicked_name.clone());
        let outline: Layer<Box<dyn Draw + Send>> = Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, false, palette.clicked.clone())), false, self.clicked_name.clone());
        let mut text: Box<Text> = Box::new(Text::new(self.x, self.y, self.h as f32, self.text.clone(), "./assets/fonts/Antic_Slab/AnticSlab-Regular.ttf",  palette.clicked_text.clone(), 2),);
        let text_width = text.w;
        if text_width < self.w {
            let x_offset = (self.w - text_width) / 2;
            text.x = self.x + x_offset;
        }
        let text_layer: Layer<Box<dyn Draw + Send>> = Layer::new(text, false, self.clicked_name.clone()); 

        self.layers.push(bg);
        self.layers.push(outline);
        self.layers.push(text_layer);

        // Selected background box
        let bg: Layer<Box<dyn Draw + Send>> = Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, true, palette.selected_background.clone())), false, self.selected_name.clone());
        let outline: Layer<Box<dyn Draw + Send>> = Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, false, palette.selected.clone())), false, self.selected_name.clone());
        let mut text: Box<Text> = Box::new(Text::new(self.x, self.y, self.h as f32, self.text.clone(), "./assets/fonts/Antic_Slab/AnticSlab-Regular.ttf",  palette.selected_text.clone(), 2),);
        let text_width = text.w;
        if text_width < self.w {
            let x_offset = (self.w - text_width) / 2;
            text.x = self.x + x_offset;
        }
        let text_layer: Layer<Box<dyn Draw + Send>> = Layer::new(text, false, self.selected_name.clone()); 

        self.layers.push(bg);
        self.layers.push(outline);
        self.layers.push(text_layer);
    }
}
impl Gui for Button {
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
        self.reinit(canvas);
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

    fn handle_button_action(&mut self, ba: &ButtonAction) -> (bool, Option<&'static str>, Option<GuiAction>) {
        match ba.code {
            6 => {
                match ba.action {
                    JAction::Pressed => {
                            (false, Some("[Clicked Button]"), Some(self.action.clone()))
                        },
                    JAction::Released => {
                            (true, Some("[Released Button]"), None)
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

#[allow(dead_code)]
pub struct Menu {
    pub items: Vec<MenuItem>,
    pub action: GuiAction,
    pub name: &'static str
}

#[allow(dead_code)]
pub struct MenuItem {
    pub text: &'static str,
    pub action: GuiAction,
    pub name: &'static str
}

#[allow(dead_code)]
pub struct GuiImage {
    pub path: &'static str,
    pub action: GuiAction,
    pub name: &'static str
}

#[allow(dead_code)]
pub struct TextBox {
    pub text: &'static str,
    pub action: GuiAction,
    pub name: &'static str
}

#[derive(Clone, Debug)]
pub struct GuiAction {
    pub name: &'static str,
    pub values: Option<Vec<&'static str>>
}

impl GuiAction {
    pub fn new(name: &'static str, values: Option<Vec<&'static str>>) -> GuiAction {
        GuiAction {
            name, values
        }
    }
}
