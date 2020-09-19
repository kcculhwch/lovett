use framebuffer::{Framebuffer};
use image::{GenericImageView,  DynamicImage};

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {r, g, b, a: 255}
    }
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {r, g, b, a}
    }

    pub fn to_16b(&self) -> u16 {
        let r = u8_to_5b(self.r);
        let g = u8_to_6b(self.g);
        let b = u8_to_5b(self.b);
        let rgb565: u16 = ((r as u16) << 11) + ((g as u16) << 5) + b as u16;
        rgb565
    }
    pub fn from_16b(color: u16) -> Color{
        // |-----|---\---|-----| --- rbg 565 sliced on byte
        // |--|------\|-----|000| --- shift left and cut to lower byte then shift right again
        let b5: u8 = ((color << 3) as u8) >> 3;
        // |000-----\|------|--|  -- shift right 3 to allign, cut, then shift right 2
        let g6: u8 = ((color >> 3) as u8) >> 2;
        // |00000000\|-----|---| -- sifhr right 8 to align, cut, then shift right 3
        let r5: u8 = ((color >> 8) as u8) >> 3;
        
        let r = x_to_8b(r5, 31);
        let g = x_to_8b(g6, 63);
        let b = x_to_8b(b5, 31);
        Color::new(r,g,b)
    }
    pub fn add(&self, color: &Color) -> Color {
        // new color occludes any base 
        if color.a == 255 {
            return Color::new(color.r, color.g, color.b)
        }
        
        let r = add_c(self.r, color.r, color.a);
        let g = add_c(self.g, color.g, color.a);
        let b = add_c(self.b, color.b, color.a);

        Color::new_rgba(r, g, b, self.a)
    }

}

// helper functions
fn or_255(sp: u16) -> u8 {
    if sp > 255 {
        255
    } else {
        sp as u8
    }
}
fn u8_to_6b(c: u8) -> u8 {
    (63.0 * c as f32 / 255.0) as u8
}
fn u8_to_5b(c: u8) -> u8 {
    (31.0 * c as f32 / 255.0) as u8
}
fn x_to_8b(c: u8, factor: u8) -> u8 {
    (c as f32 * 255.0 / factor as f32) as u8
}

fn add_c(base: u8, addition: u8, alpha: u8) -> u8 {
    let alpha_p = alpha as f32 / 255.0;
    let c = ((base as f32 * (1.0 - alpha_p)) + (addition as f32 * alpha_p)) as u16;
    or_255(c)
}      

pub struct FB {
    fb: Framebuffer,
    pub w: u32,
    pub h: u32,
    ll: u32,
    bpp: u32,
    frame: Vec<u8>,
    background: Color
}

impl FB {
    pub fn new(dev: &str) -> FB {
    
        let framebuffer = Framebuffer::new(dev).unwrap();

        let w = framebuffer.var_screen_info.xres;
        let h = framebuffer.var_screen_info.yres;
        let line_length = framebuffer.fix_screen_info.line_length;
        let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

        let frame = vec![0u8; (line_length * h) as usize];
        FB {
            fb: framebuffer,
            w: w,
            h: h,
            ll: line_length,
            bpp: bytespp,
            frame: frame,
            background: Color::new(0,0,0)
        } 
    }

    pub fn flush(&mut self){
         let _ = self.fb.write_frame(&self.frame);
    }
    
    pub fn find_point(&self, x1: u32, y1: u32) -> usize{
         ((y1 * self.ll) + (x1 * self.bpp)) as usize
    }

    #[allow(dead_code)]
    pub fn set_background(&mut self, color: Color){
        self.background = color
    }

    fn check_x(&self, x: u32) -> u32 {
        if x >= self.w {
            return self.w - 1
        }
        return x
    }

    fn check_y(&self, y: u32) -> u32 {
        if y >= self.h {
            return  self.h - 1
        }
        return y
    }

    fn check_w(&self, x: u32, w: u32) -> u32 {
        if x + w > self.w {
            return self.w - x
        }
        return w
    }
    
    fn check_h(&self, y: u32, h: u32) -> u32 {
        if y + h > self.h {
            return self.h - y
        }
        return h
    }

    pub fn clear(&mut self) {
        self.draw_filled_rect(0, 0, 240, 240, &Color::new(self.background.r, self.background.g, self.background.b));
    }

    pub fn draw_rect(&mut self, x1: u32, y1: u32, width: u32,  height: u32, color: &Color) {
        self.draw_h_line(x1, y1, width, color);
        self.draw_h_line(x1, y1 + height - 1, width, color);
        self.draw_v_line(x1, y1, height, color);
        self.draw_v_line(x1 + width - 1, y1, height, color);
    }

    pub fn draw_filled_rect(&mut self, x1: u32, y1: u32, width: u32, height: u32, color: &Color) {
        for i in 0..(height as u32) {
            self.draw_h_line(x1, y1 + i, width, color);
        }
    }

    pub fn draw_h_line(&mut self, x1: u32, y1: u32, width: u32, color: &Color){
        let x = self.check_x(x1);
        let y = self.check_y(y1);
        let w = self.check_w(x, width);
        let index = self.find_point(x, y);
        if color.a == 255 {
            let color = color.to_16b();
            for i in 0..(w as usize) {
                self.frame[index + (2 * i)] = color as u8;
                self.frame[index + (2 * i) + 1] = (color >> 8) as u8;
            }
        } else {
            for i in 0..(w as usize) {
                let mut base_color: u16;

                base_color = self.frame[index + (2 * i)] as u16;
                base_color += (self.frame[index + (2 * i) + 1] as u16) << 8;
                let base = Color::from_16b(base_color);
                let added_color = base.add(color).to_16b();
                self.frame[index + (2 * i)] = added_color as u8;
                self.frame[index + (2 * i) + 1] = (added_color >> 8) as u8;

            }
        }
    }

    pub fn draw_v_line(&mut self, x1: u32, y1: u32, height: u32, color: &Color) {
        let x = self.check_x(x1);
        let y = self.check_y(y1);
        let h = self.check_h(y, height);
        let index = self.find_point(x, y);
        if color.a == 255 {
            let color = color.to_16b();
            for i in 0..(h  as usize) {
                self.frame[index + (480 * i)] = color as u8;
                self.frame[index + (480 * i) + 1] = (color >> 8) as u8;
            }
        } else {
            for i in 0..(h as usize) {
                let mut base_color: u16;
                base_color = self.frame[index + (480 * i)] as u16;
                base_color += (self.frame[index + (480 * i) + 1] as u16) << 8;
                let base = Color::from_16b(base_color);
                let added_color = base.add(color).to_16b();
                self.frame[index + (480 * i)] = added_color as u8;
                self.frame[index + (480 * i) + 1] = (added_color >> 8) as u8;


            }
        }
    }
                  
    pub fn render_image(&mut self, img: &DynamicImage, x1: u32, y1: u32, w1:u32, h1:u32, img_x:u32, img_y:u32){
        let start_x = self.check_x(x1);
        let start_y = self.check_y(y1);
        let w = self.check_w(start_x, w1);
        let h = self.check_h(start_y, h1);
        if w <= 0 || h <= 0 {
            return; //do not attempt to render a 0 w or 0 h box
        }
        for x in 0..(w - 1) {
            for y in 0..(h - 1) {
                let px = img.get_pixel(img_x + x, img_y + y);
                let index = self.find_point(x + start_x, y + start_y );
                let color = Color::new_rgba(px[0], px[1], px[2], px[3]);
                if color.a == 255 {
                    let rgb565 = color.to_16b();
                    self.frame[index] = rgb565 as u8;
                    self.frame[index + 1] = (rgb565 >> 8) as u8;
                } else {
                    let mut base_color: u16;
                    base_color = self.frame[index] as u16;
                    base_color += (self.frame[index + 1] as u16) << 8;
                    let base = Color::from_16b(base_color);
                    let added_color = base.add(&color).to_16b();
                    self.frame[index] = added_color as u8;
                    self.frame[index + 1] = (added_color >> 8) as u8;
                }
            }
        }
    }
}
