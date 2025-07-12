use raylib::prelude::*;
use raylib::core::texture::Image;

pub struct Framebuffer {
    width: u32,
    height: u32,
    background_color: Color,
    current_color: Color,
    pub color_buffer: Image,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let background_color = Color::BLACK;
        let color_buffer = Image::gen_image_color(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            background_color,
        );

        Framebuffer {
            width,
            height,
            background_color,
            current_color: Color::WHITE,
            color_buffer,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
            self.background_color,
        );
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(Color::BLACK);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }

    pub fn draw_line(&mut self, start: Vector2, end: Vector2) {
        let mut x0 = start.x as i32;
        let mut y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.draw_pixel(x0, y0);

            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }
}
