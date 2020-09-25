#[allow(dead_code)]
use super::fb::FB;
use super::fb::Color;
use image::{DynamicImage, Rgba }; // rgba is used internally by rusttype and image
use rusttype::{point, Font, Scale};
use std::fs::File;
use std::io::Read;

// Layer

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
}

pub trait Draw {
    fn draw(&self, fb: &mut FB);
    fn slide(&mut self, x: i32, y: i32);
    fn clipped(&self, fb: &FB) -> Option<(u32, u32, u32, u32)>;
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
    font: Font<'static>,
    scale: Scale,
    color: Color,
    img: DynamicImage, // we store the actual rasterized text here and enough to rerasterize later
    img_x: u32,
    img_y: u32

}

impl Text {
    pub fn new(x: i32, y: i32, size: f32, content: String, font: &'static str, color: Color, padding: u32) -> Text {
        let mut file = File::open(font).expect("Font File Not Found");
        let mut font_data: Vec<u8> = vec![];
        file.read_to_end(&mut font_data).expect("Unable to Read Font File");

        let font = Font::try_from_vec(font_data).expect("Error constructing Font");
        let scale = Scale::uniform(size);
        // Rgba 
        let colour = (color.r, color.g, color.b, color.a);

        let v_metrics = font.v_metrics(scale);
        
        // layout the glyphs in a line with 20 pixels padding
        let glyphs: Vec<_> = font
            .layout(&content, scale, point(0.0, 0.0 + v_metrics.ascent))
            .collect();

        // work out the layout size
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        // Create a new rgba image with some padding
        let w: u32 = glyphs_width + padding;
        let h: u32 = glyphs_height + padding;

        let mut img = DynamicImage::new_rgba8(w, h).to_rgba();
        
        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    let alpha: u8;
                    if colour.3  == 255 {
                        alpha = (v * 255.0) as u8;
                    } else {
                        alpha = ((colour.3 as f32 / 255.0 ) * (1.0 - v) + (v * 255.0)) as u8; //not quite right ... need to look up formula
                    }
                    img.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba([colour.0, colour.1, colour.2, alpha]),
                    )
                });
            }
        }

        let image = DynamicImage::ImageRgba8(img);
        // build the img
        let img_x = adjust_img_loc(x, 0, w);        

        let img_y = adjust_img_loc(y, 0, h);
        Text {
            x, y, w: w as i32, h: h as i32, content, scale, color, img: image, font, img_x, img_y
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

