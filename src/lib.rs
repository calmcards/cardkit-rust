pub mod text;

use ultraviolet::vec::Vec2;

pub const TEXT_WIDTH: usize = 80;
pub const TEXT_HEIGHT: usize = 30;

pub const PIXEL_WIDTH: usize = 640;
pub const PIXEL_HEIGHT: usize = 480;

#[repr(transparent)]
pub struct TextBuffer {
    pub data: [[u32; TEXT_WIDTH]; TEXT_HEIGHT],
}

impl Default for TextBuffer {
    fn default() -> Self { Self{data: [[0; TEXT_WIDTH]; TEXT_HEIGHT]} }
}

impl TextBuffer {
    pub fn get(&self, at: (f32, f32)) -> Option<char> {
        let (x, y) = at;
        if 0.0 <= x && x < TEXT_WIDTH as f32 && 0.0 <= y && y < TEXT_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            return char::from_u32(self.data[ry][rx]);
        }
        None
    }

    pub fn set(&mut self, at: (f32, f32), c: char) -> &mut Self {
        let (x, y) = at;
        if 0.0 <= x && x < TEXT_WIDTH as f32 && 0.0 <= y && y < TEXT_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            self.data[ry][rx] = c as u32;
        }
        self
    }

    pub fn clear(&mut self, c: char) -> &mut Self {
        for yi in 0..TEXT_HEIGHT {
            for xi in 0..TEXT_WIDTH {
                self.data[yi][xi] = c as u32;
            }
        }
        self
    }
}

#[repr(transparent)]
pub struct PixelBuffer {
    pub data: [[u8; PIXEL_WIDTH]; PIXEL_HEIGHT],
}

impl Default for PixelBuffer {
    fn default() -> Self { Self{data: [[0; PIXEL_WIDTH]; PIXEL_HEIGHT]} }
}

impl PixelBuffer {
    pub fn get(&self, at: (f32, f32)) -> Option<u8> {
        let (x, y) = at;
        if 0.0 <= x && x < PIXEL_WIDTH as f32 && 0.0 <= y && y < PIXEL_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            return Some(self.data[ry][rx]);
        }
        None
    }

    pub fn set(&mut self, at: (f32, f32), v: u8) {
        let (x, y) = at;
        if 0.0 <= x && x < PIXEL_WIDTH as f32 && 0.0 <= y && y < PIXEL_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            self.data[ry][rx] = v;
        }
    }

    pub fn clear(&mut self, v: u8) -> &mut Self {
        for yi in 0..PIXEL_HEIGHT {
            for xi in 0..PIXEL_WIDTH {
                self.data[yi][xi] = v;
            }
        }
        self
    }
}

pub fn pixelspace_to_textspace(at: (f32, f32)) -> (f32, f32) {
    let (x, y) = at;

    ((x / ((PIXEL_WIDTH as f32 / TEXT_WIDTH as f32) as f32)),
     (y / ((PIXEL_HEIGHT as f32 / TEXT_HEIGHT as f32) as f32)))
}

pub fn textspace_to_pixelspace(at: (f32, f32)) -> (f32, f32) {
    let (x, y) = at;

    ((x * ((PIXEL_WIDTH as f32 / TEXT_WIDTH as f32) as f32)),
     (y * ((PIXEL_HEIGHT as f32 / TEXT_HEIGHT as f32) as f32)))
}

#[repr(u32)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch
}