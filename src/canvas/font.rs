use std::fs::File;
use std::io::Read;

use super::ab_glyph::*;

#[allow(dead_code)]
pub fn get_font(font: &'static str) -> FontVec {
        let mut file = File::open(font).expect("Font File Not Found");
        let mut font_data: Vec<u8> = vec![];
        file.read_to_end(&mut font_data).expect("Unable to Read Font File");
        FontVec::try_from_vec(font_data).unwrap()
}
