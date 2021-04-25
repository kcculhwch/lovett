use serde::{Serialize, Deserialize};

use uuid::Uuid;

use super::canvas::{Rect, Layer, Draw, Canvas, Text};
use super::fb::Color;

use super::hid::HIDEvent; 
use super::hid::IOState;

mod button;
#[allow(unused_imports)]
pub use button::*;

mod block;
#[allow(unused_imports)]
pub use block::*;

mod text_block;
#[allow(unused_imports)]
pub use text_block::*;
use glyph_brush_layout::*;
use ab_glyph::*;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GuiState{
    Base,
    Clicked,
    Selected
}

//color pallete
#[derive(Clone)]
pub struct Palette {
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
    fn handle_hid_event(&mut self, ba: &HIDEvent) -> (bool, Option<GuiState>, Option<Event>)  {
        (true, None, None)
    }

}

#[allow(dead_code)]
pub struct Menu {
    pub items: Vec<MenuItem>,
    pub event: Event,
    pub name: &'static str
}

#[allow(dead_code)]
pub struct MenuItem {
    pub text: &'static str,
    pub event: Event,
    pub name: &'static str
}

#[allow(dead_code)]
pub struct GuiImage {
    pub path: &'static str,
    pub event: Event,
    pub name: &'static str
}


#[derive(Clone, Debug)]
pub struct Event {
    pub name: &'static str,
    pub values: Option<Vec<String>>
}

impl Event {
    pub fn new(name: &'static str, values: Option<Vec<String>>) -> Event {
        Event {
            name, values
        }
    }
}
