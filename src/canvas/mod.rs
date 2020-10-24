#[allow(dead_code)]
use super::fb::FB;
use super::fb::Color;
use image::{DynamicImage, Rgba }; // rgba is used internally by rusttype and image
//use rusttype::{point, Font, Scale};
use std::fs::File;
use std::io::Read;
use glyph_brush_layout::*;
use ab_glyph::*;
use glyph_brush_draw_cache::{DrawCache, Rectangle};
// Layer
use log::*;

#[derive(Clone, Debug)]
pub struct Layer<T> {
    pub item: Box<T>,
    pub active: bool,
    pub group: String
}


impl<T> Layer<T> {
    pub fn new(item: T, active: bool, group: String) -> Layer<T>{
        Layer{item: Box::new(item), active, group}
    }
}

// Canvas
pub struct Canvas {
    screen: FB,
    pub layers: Vec<Layer<Box<dyn Draw + Send>>>
}

impl Canvas {
    pub fn new(dev: &'static str) -> Canvas{
        let fb = FB::new(dev);
        let layers: Vec<Layer<Box<dyn Draw + Send>>> = vec![];
        Canvas {
            screen: fb,
            layers    
        }
    }

    pub fn render(&mut self) {
        self.screen.clear();
        for layer in &self.layers {
            if layer.active{
                layer.item.draw(&mut self.screen);
            }
        }
        self.screen.flush();

    }

    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.screen.clear();
        self.screen.flush();
    }

    #[allow(dead_code)]
    pub fn slide_layer_group(&mut self, group: &'static str, x: i32, y: i32) {
        for layer in &mut self.layers {
            if &layer.group == group {
                layer.item.slide(x, y);
            }
        }
    }

    #[allow(dead_code)] 
    pub fn activate_layer_group(&mut self, group: String){
        for layer in &mut self.layers {
            if layer.group == group {
                layer.active = true;
            }
        }
    }

    #[allow(dead_code)] 
    pub fn deactivate_layer_group(&mut self, group: String){
        for layer in &mut self.layers {
            if layer.group == group {
                layer.active = false;
            }
        }
    }

    pub fn drop_layer_group(&mut self, group: String){
        let mut to_remove: Vec<usize> = vec![];
        for i in 0..self.layers.len() {
            if self.layers[i].group == group {
                to_remove.push(i);
            }
        }
        for i in to_remove.iter().rev() {
            self.layers.remove(*i);
        }
    }

    pub fn get_layer_group(&mut self, group: String) ->  Vec<Layer<Box<dyn Draw + Send>>>  {
        let mut to_return: Vec<usize> = vec![];
        let mut result: Vec<Layer<Box<dyn Draw + Send>>> = vec![];
 
        for i in 0..self.layers.len() {
            if self.layers[i].group == group {
                to_return.push(i);
            }
        }

        for i in to_return.iter().rev() {
            debug!("Remove Layer: {}", i);
            result.push(self.layers.remove(*i));
        }
       
        result.reverse();
        result
    }
}

pub trait Draw {
    fn draw(&self, fb: &mut FB);
    fn slide(&mut self, x: i32, y: i32);
    fn clipped(&self, fb: &FB) -> Option<(u32, u32, u32, u32)>;
    #[allow(unused_variables)]    
    fn update_text(&mut self, text: String){
        
    } 
}


// Rectangles
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub filled: bool,
    pub color: Color
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32, filled: bool, color: Color) -> Rect {
        Rect {x ,y, w, h, filled, color}
    }

    fn clipped(&self, fb: &FB) -> Option<(u32, u32, u32, u32)>{
        let x: u32;
        let y: u32;
        let mut w: u32;
        let mut h: u32;

        // determine how much if any of the object is on screen
        // x negative direction

        if self.x < 0 {
            if self.x + self.w <= 0 {
                return None
            } else {
                x = 0;
                w = (self.w + self.x) as u32;
            }
        } else if self.x >= fb.w as i32 {  // x positive direction
            return None
        } else {
            x = self.x as u32;
            w = self.w as u32;            
        }

        // width greater than display
        if x + w > fb.w {
            w = fb.w - x;
        } 

        // y negative direction
        if self.y < 0 {
            if self.y + self.h <= 0 {
                return None
            } else {
                y = 0;
                h = (self.h + self.y) as u32;
            }
        } else if self.y >= fb.h as i32 {  // y positive direction
            return None
        } else {
            y = self.y as u32;
            h = self.h as u32;            
        }
        // width greater than display
        if y + h > fb.h {
            h = fb.h - y;
        } 
        Some((x, y, w, h))
    }
}



impl Draw for Rect {
    fn draw(&self, fb: &mut FB){
        //clip actual coordinates to render what is on screen or do nothing
        match self.clipped(fb) {
            Some((x, y, w, h)) => {
                if self.filled {
                    fb.draw_filled_rect(x, y, w,h, &self.color );
                } else {
                    //only if x y w and h can we draw_rect ... otherwise we must do lines
                    if self.x as u32 == x && self.y as u32 == y && self.w as u32 == w && self.h as u32 == h { 
                        fb.draw_rect(x, y, w, h, &self.color);
                        return ()
                    }
                    // check left
                    if self.x >= 0 && self.x < fb.w as i32 {
                        //render left
                        fb.draw_v_line(x, y, h, &self.color)
                    }
                    // check top
                    if self.y >= 0 && self.y < fb.h as i32 {
                        // render top
                        fb.draw_h_line(x, y, w, &self.color)
                    }
                    // check right
                    if self.x + self.w < fb.w as i32 && self.x + self.h >= 0 {
                        //render right
                        fb.draw_v_line(x + w - 1, y, h, &self.color)

                    }
                    // check bottom
                    if self.y + self.h < fb.h as i32 && self.y + self.h >= 0 {
                        //render bottom
                        fb.draw_h_line(x, y + h -1, w, &self.color)
                    }

                }
            },
            None => ()
        }

    }
    fn slide(&mut self, x: i32, y: i32) {
        //move x
        self.x = self.x + x;

        //move y
        self.y = self.y + y;
    }
    fn clipped(&self, fb: &FB) -> Option<(u32, u32, u32, u32)>{
       clipper(self.x, self.y, self.w, self.h, fb.w, fb.h) 
    }
}

// images
pub struct Image {
    img: DynamicImage,
    // where it goes on the canvas
    x: i32,
    y: i32,
    w: i32,
    h: i32, 
    // where we sample from the image
    // for now we assume a 1:1 pixal sample
    img_x: u32,
    img_y: u32
}

impl Image {
    #[allow(dead_code)] 
    pub fn new(path: &'static str, x: i32, y: i32, w: i32, h: i32, img_x: u32, img_y: u32 ) -> Image {
        let img =image::open(path).unwrap();
        Image {
            img, x, y, w, h, img_x, img_y
        }
    }

}

impl Draw for Image {
    fn draw(&self, fb: &mut FB){
        match self.clipped(fb) {
            Some((x, y, w, h)) => {
                    fb.render_image(&self.img, x, y, w, h, self.img_x, self.img_y)
                },
            None => ()
        }
    }
    fn slide(&mut self, x: i32, y: i32) {
        //move x
        self.x = self.x + x;

        //move y
        self.y = self.y + y;
    }

    fn clipped(&self, fb: &FB) -> Option<(u32, u32, u32, u32)>{
       clipper(self.x, self.y, self.w, self.h, fb.w, fb.h) 
    }

}

#[allow(dead_code)]
pub struct Text {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32, 
    content: String,
    font: FontVec,
    scale: PxScale,
    color: Color,
    img: DynamicImage, // we store the actual rasterized text here and enough to rerasterize later
    img_x: u32,
    img_y: u32,
    padding: u32,
    draw_cache: DrawCache,
    texture: Vec<u8>

}

impl Text {
    pub fn new(x: i32, y: i32, size: f32, content: String, font: &'static str, color: Color, padding: u32) -> Text {
        let mut file = File::open(font).expect("Font File Not Found");
        let mut font_data: Vec<u8> = vec![];
        file.read_to_end(&mut font_data).expect("Unable to Read Font File");
        let mut draw_cache = DrawCache::builder().build();
        let font = FontVec::try_from_vec(font_data).unwrap();
        let scale = PxScale::from(size);
        let mut texture: Vec<u8> = vec![0; 256 * 256];
        let (image, img_x, img_y, w, h) = Text::layout_string(x, y, &color, &font, &content, &scale, padding, &mut draw_cache, &mut texture);
        Text {
            x, y, w: w as i32, h: h as i32, content, scale, color, img: image, font, img_x, img_y, padding, draw_cache, texture
        }
    }
    
   
    pub fn layout_string(x: i32, y: i32, color: &Color, font: &FontVec, content: &String, scale: &PxScale,  padding: u32, draw_cache: &mut DrawCache, texture: &mut Vec<u8>) -> (DynamicImage, u32, u32, u32, u32 ){
        // Rgba 
        let colour = (color.r, color.g, color.b, color.a);

        let scaled_font = font.as_scaled(scale.clone());
        let fonts = [font];
        let glyphs = Layout::default().calculate_glyphs(
            &fonts,
            &SectionGeometry {
                screen_position: (0.0, 0.0),
                ..SectionGeometry::default()
            },
            &[
                SectionText {
                    text: content,
                    scale: scale.clone(),
                    font_id: FontId(0),
                },
            ],
        );

        
        // work out the layout size
        let g_h = scaled_font.height().ceil() as u32;
        let g_w = {
            let min_x = glyphs.first().unwrap().glyph.position.x;
            let last_glyph = glyphs.last().unwrap().glyph.clone();
            let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
            (max_x - min_x).ceil() as u32
        };
        // Create a new rgba image with some padding
        let w: u32 = g_w + padding;
        let h: u32 = g_h + padding;

        debug!("Image size: {} x {}", w, h);
        // Create a new rgba image with some padding
        let mut image = DynamicImage::new_rgba8(w, h).to_rgba();

        for glyph_w in glyphs.clone() {
            draw_cache.queue_glyph(glyph_w.font_id.0, glyph_w.glyph);
        }

        draw_cache.cache_queued(&fonts, |rect, tex_data| Text::update_texture(rect, tex_data, texture)).unwrap();
/*
        let mut output:String  = String::from( "");

        for i in 0..texture.len() {
            if texture[i] != 0 {
                output = format!("{}*", output);
            } else {
                output = format!("{} ", output);
            }
            if i % 256 == 0 {
                debug!("{}", output);
                output = String::from("");
            }
        }
*/
        // Loop through the glyphs in the text, positing each one on a line
        for glyph_w in glyphs {
            let glyph = glyph_w.glyph;
            if let Some(outlined) = scaled_font.outline_glyph(glyph.clone()) {
                let bounds = outlined.px_bounds();
                match draw_cache.rect_for(glyph_w.font_id.0, &glyph) {
                    Some((tex_coords, px_coords)) => {
                        // width 
                        let width = (tex_coords.max.x - tex_coords.min.x) as usize;
                        let height = (tex_coords.max.y - tex_coords.min.y) as usize;
                        debug!("Copy Glyph at ({}, {}) - ({}, {}) to ({}, {})", tex_coords.min.x, tex_coords.min.y, tex_coords.max.x, tex_coords.max.y,  px_coords.min.x, px_coords.min.y);
                        for y in 0..height {
                            for x in 0..width {
                                // texture value []
                                let alpha: u8 = texture[x + tex_coords.min.x as usize + ( (y + tex_coords.min.y as usize) * 256)];
                                let px = image.get_pixel_mut(x as u32 + px_coords.min.x as u32, y as u32 + px_coords.min.y as u32);
                                // Turn the coverage into an alpha value (blended with any previous)
                                *px = Rgba([
                                    colour.0,
                                    colour.1,
                                    colour.2,
                                    alpha
                                ]);
                               
                            }

                        }
                    

                    }
                    None => {/* The glyph has no outline, or wasn't queued up to be cached */}
                }

            }
        }

        let image = DynamicImage::ImageRgba8(image);
        let img_x = adjust_img_loc(x, 0, w);        

        let img_y = adjust_img_loc(y, 0, h);
        (image, img_x, img_y, w, h)
    }

    pub fn update_texture(rect: Rectangle<u32>, tex_data: &[u8], texture: &mut Vec<u8>) {       
        // rect == where in the texture we want to cache the data
        // tex_data alpha value for each pixel 
        // texture 256 x 256 inlined vec
        let width: usize = (rect.max[0] - rect.min[0]) as usize;
        let height: usize = (rect.max[1] - rect.min[1]) as usize;
/*
        let mut output:String  = String::from( "");

        for i in 0..tex_data.len() {
            output = format!("{}{}", output, tex_data[i]);
            if i % width == 0 {
                debug!("{}", output);
                output = String::from("");
            }
        }

*/
        for y in 0..height {
            for x in 0..width {
                texture[x + rect.min[0] as usize + ( (y + rect.min[1] as usize) * 256)] = tex_data[(y * width) + x];
                // copy data at y * width + x
                // to texture[x + rect.min[0] + ( y + rect.min[1] * 256)]   
            
            }
        }
    }
}



// generic clipper to be called by draw's clipped
fn clipper(ix: i32, iy: i32, iw: i32, ih: i32, fw: u32, fh: u32) -> Option<(u32, u32, u32, u32)>{
        let x: i32;
        let y: i32;
        let mut w: i32;
        let mut h: i32;

        // determine how much if any of the object is on screen
        // x negative direction

        if ix < 0 {
            x = 0;
            w = iw + ix;
        } else if ix >= fw as i32{  // x positive direction
            return None
        } else {
            x = ix;
            w = iw;            
        }
        // width greater than display
        if x + w > fw as i32 {
            w = fw as i32 - x;
        } 

        // y negative direction
        if iy < 0 {
            y = 0;
            h = ih + iy;
        } else if iy >= fh as i32 {  // y positive direction
            return None
        } else {
            y = iy;
            h = ih;            
        }
        // width greater than display
        if y + h > fh as i32 {
            h = fh as i32 - y;
        }
        // w and h must be more than 0 in order to render anything
        // x and y cannot be less than zero coming out of the clipper - if so then likely say int overflow
        if w < 1 || h < 1 || x < 0 || y < 0 {
            return None;
        }
        
        Some((x as u32, y as u32, w as u32, h as u32))


}
impl Draw for Text {
    fn draw(&self, fb: &mut FB){
        match self.clipped(fb) {
            Some((x, y, w, h)) => {
                    fb.render_image(&self.img, x, y, w, h, 
                        adjust_img_loc(self.x, self.img_x, self.w as u32),
                        adjust_img_loc(self.y, self.img_y, self.h as u32)
                     )
                },
            None => ()
        }
    }
    fn slide(&mut self, x: i32, y: i32) {
        //move x
        self.x = self.x + x;
//        self.img_x = adjust_img_loc(self.x, self.img_x, self.w as u32);        
        //move y
        self.y = self.y + y;
//        self.img_y = adjust_img_loc(self.y, self.img_y, self.h as u32);         
    }


    fn clipped(&self, fb: &FB) -> Option<(u32, u32, u32, u32)>{
        clipper(self.x, self.y, self.w, self.h, fb.w, fb.h)
    }
    fn update_text(&mut self, new_content: String) {
        if new_content != self.content {
            let (image, img_x, img_y, w, h) = Text::layout_string(self.x, self.y, &self.color, &self.font, &self.content, &self.scale,  self.padding, &mut self.draw_cache, &mut self.texture);
            self.content = new_content;
            self.img_x = img_x;
            self.img_y = img_y;
            self.w = w as i32;
            self.h = h as i32;
            self.img = image; 
        }
    }
 
}

pub fn adjust_img_loc(pt: i32, img_pt: u32, max: u32) -> u32 {
    let mut new: u32;
        if pt < 0 {
            new = img_pt + (-1 * pt) as u32;
        } else {
            new = img_pt;
        }
        if new >= max {
            new = max - 1;
        }
        return new
}


/* old image code
    pub fn pan_image(&mut self, x: i32, y: i32){
         //move x
        let i32_x_total = self.offset_x as i32 + x;
    
        if i32_y_total < 0 {
            self.offset_y = 0;
        } else {    
            self.offset_y = i32_y_total as u32;
        }

        if self.offset_x > self.w {
            self.offset_x = self.w;
        }
        if self.offset_y > self.h {
            self.offset_y = self.h;
        }
        self.draw_image();
        self.flush(); 
    }

    pub fn draw_image(&mut self){
        for (x, y) in self.img.coordinates() {
            if x < 240 && y < 240 {
                let px = self.img.get_pixel(x + self.offset_x, y + self.offset_y);
                let start_index = ((y * self.ll) + (x * self.bpp)) as usize;
                let color = Color::new(px.r, px.g, px.b);
                let rgb565 = color.to_16b();
                self.frame[start_index] = rgb565 as u8;
                self.frame[start_index + 1] = (rgb565 >> 8) as u8;
            }
        }
    }
*/


