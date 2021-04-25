use super::*;
use std::fs::File;
use std::io::Read;


#[allow(dead_code)]
fn get_font(font: &'static str) -> FontVec {
        let mut file = File::open(font).expect("Font File Not Found");
        let mut font_data: Vec<u8> = vec![];
        file.read_to_end(&mut font_data).expect("Unable to Read Font File");
        FontVec::try_from_vec(font_data).unwrap()
}

pub struct GuiConfig {
    pub palette: Palette,
    pub font: FontVec

}

#[allow(dead_code)]
impl GuiConfig {
    pub fn new(palette: Palette, font_path: &'static str) -> GuiConfig {
        let font = get_font(font_path);
        GuiConfig {
            palette, font
        }
    }
}
