pub mod text;

pub const GLYPH_WIDTH: usize = 8;
pub const GLYPH_HEIGHT: usize = 16;


#[repr(transparent)]
pub struct TextBuffer<const W: usize, const H: usize>(pub [[u32; W]; H]);

impl<const W: usize, const H: usize> Default for TextBuffer<W, H> {
    fn default() -> Self { Self([[0; W]; H]) }
}

impl<const W: usize, const H: usize> TextBuffer<W, H> {
    pub fn get(&self, at: (f32, f32)) -> Option<char> {
        let (x, y) = at;
        if 0.0 <= x && x < W as f32 && 0.0 <= y && y < H as f32 {
            let rx = x.floor() as usize;
            let ry = y.floor() as usize;
            return char::from_u32(self.0[ry][rx]);
        }
        None
    }

    pub fn set(&mut self, at: (f32, f32), c: char) -> &mut Self {
        let (x, y) = at;
        if 0.0 <= x && x < W as f32 && 0.0 <= y && y < H as f32 {
            let rx = x.floor() as usize;
            let ry = y.floor() as usize;
            self.0[ry][rx] = c as u32;
        }
        self
    }

    pub fn clear(&mut self, c: char) -> &mut Self {
        for yi in 0..H {
            for xi in 0..W {
                self.0[yi][xi] = c as u32;
            }
        }
        self
    }
}

#[repr(transparent)]
pub struct PixelBuffer<const W: usize, const H: usize>(pub [[u8; W]; H]);

impl<const W: usize, const H: usize> Default for PixelBuffer<W, H> {
    fn default() -> Self { Self([[0; W]; H]) }
}

impl<const W: usize, const H: usize> PixelBuffer<W, H> {
    pub fn get(&self, at: (f32, f32)) -> Option<u8> {
        let (x, y) = at;
        if 0.0 <= x && x < W as f32 && 0.0 <= y && y < H as f32 {
            let rx = x.floor() as usize;
            let ry = y.floor() as usize;
            return Some(self.0[ry][rx]);
        }
        None
    }

    pub fn set(&mut self, at: (f32, f32), v: u8) {
        let (x, y) = at;
        if 0.0 <= x && x < W as f32 && 0.0 <= y && y < H as f32 {
            let rx = x.floor() as usize;
            let ry = y.floor() as usize;
            self.0[ry][rx] = v;
        }
    }

    pub fn clear(&mut self, v: u8) -> &mut Self {
        for yi in 0..H {
            for xi in 0..W {
                self.0[yi][xi] = v;
            }
        }
        self
    }
}

pub fn pixelspace_to_textspace(at: (f32, f32)) -> (f32, f32) {
    let (x, y) = at;

    ((x / GLYPH_WIDTH as f32),
     (y / GLYPH_HEIGHT as f32))
}

pub fn textspace_to_pixelspace(at: (f32, f32)) -> (f32, f32) {
    let (x, y) = at;

    ((x * GLYPH_WIDTH as f32),
     (y * GLYPH_HEIGHT as f32))
}

#[repr(u32)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch
}