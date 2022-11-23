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

#[derive(Copy, Clone)]
pub enum BoxStyle {
    Thin,
    Thick,
    Double,
}

impl TextBuffer {
    pub fn get(&self, at: &TextPos) -> Option<char> {
        let x = at.vec.x;
        let y = at.vec.y;
        if 0.0 <= x && x < TEXT_WIDTH as f32 && 0.0 <= y && y < TEXT_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            return char::from_u32(self.data[ry][rx]);
        }
        None
    }

    pub fn set(&mut self, at: &TextPos, c: char) -> &mut Self {
        let x = at.vec.x;
        let y = at.vec.y;
        if 0.0 <= x && x < TEXT_WIDTH as f32 && 0.0 <= y && y < TEXT_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            self.data[ry][rx] = c as u32;
        }
        self
    }

    /*
    pub fn isInArea(topLeft: TextPos, bottomRight: TextPos) {

    }
    */

    pub fn draw_label(&mut self, left_edge: &TextPos, s: String) -> &Self {
        let mut p = left_edge.clone();
        for c in s.chars() {
            self.set(&p, c);
            p.right();
        }
        self
    }

    pub fn draw_box(&mut self, top_left: &TextPos, inner_width: usize, inner_height: usize, style: BoxStyle) -> &Self {
        let mut p = top_left.clone();

        self.set(&p, match style {
            BoxStyle::Thin => '┌',
            BoxStyle::Thick => '┏',
            BoxStyle::Double => '╔'
        });
        for _ in 0..inner_width {
            self.set(p.right(), match style {
                BoxStyle::Thin => '─',
                BoxStyle::Thick => '━',
                BoxStyle::Double => '═'
            });
        }
        self.set(p.right(), match style {
            BoxStyle::Thin => '┐',
            BoxStyle::Thick => '┓',
            BoxStyle::Double => '╗'
        });
        for _ in 0..inner_height {
            self.set(p.down(), match style {
                BoxStyle::Thin => '│',
                BoxStyle::Thick => '┃',
                BoxStyle::Double => '║'
            });
        }
        self.set(p.down(), match style {
            BoxStyle::Thin => '┘',
            BoxStyle::Thick => '┛',
            BoxStyle::Double => '╝'
        });
        for _ in 0..inner_width {
            self.set(p.left(), match style {
                BoxStyle::Thin => '─',
                BoxStyle::Thick => '━',
                BoxStyle::Double => '═'
            });
        }
        self.set(p.left(), match style {
            BoxStyle::Thin => '└',
            BoxStyle::Thick => '┗',
            BoxStyle::Double => '╚'
        });
        for _ in 0..inner_height {
            self.set(p.up(), match style {
                BoxStyle::Thin => '│',
                BoxStyle::Thick => '┃',
                BoxStyle::Double => '║'
            });
        }

        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TextPos{
    pub vec: Vec2
}

impl TextPos {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { vec: Vec2::new(x, y) }
    }

    pub fn up(&mut self) -> &mut Self {
        self.vec.y -= 1.0;
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.vec.x += 1.0;
        self
    }

    pub fn down(&mut self) -> &mut Self {
        self.vec.y += 1.0;
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.vec.x -= 1.0;
        self
    }
}

impl From<PixelPos> for TextPos {
    fn from(pos: PixelPos) -> Self {
        let x = pos.vec.x;
        let y = pos.vec.y;

        TextPos::new(
            (x / ((PIXEL_WIDTH as f32 / TEXT_WIDTH as f32) as f32)).round(),
            (y / ((PIXEL_HEIGHT as f32 / TEXT_HEIGHT as f32) as f32)).round())
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
    pub fn get(&self, at: &PixelPos) -> Option<u8> {
        let x = at.vec.x;
        let y = at.vec.y;
        if 0.0 <= x && x < PIXEL_WIDTH as f32 && 0.0 <= y && y < PIXEL_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            return Some(self.data[ry][rx]);
        }
        None
    }

    pub fn set(&mut self, at: &PixelPos, v: u8) {
        let x = at.vec.x;
        let y = at.vec.y;
        if 0.0 <= x && x < PIXEL_WIDTH as f32 && 0.0 <= y && y < PIXEL_HEIGHT as f32 {
            let rx = x.round() as usize;
            let ry = y.round() as usize;
            self.data[ry][rx] = v;
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PixelPos{
    pub vec: Vec2
}

impl PixelPos {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { vec: Vec2::new(x, y) }
    }
}

impl From<TextPos> for PixelPos  {
    fn from(pos: TextPos) -> Self {
        let x = pos.vec.x;
        let y = pos.vec.y;

        PixelPos::new(
            (x * ((PIXEL_WIDTH as f32 / TEXT_WIDTH as f32) as f32)).round(),
            (y * ((PIXEL_HEIGHT as f32 / TEXT_HEIGHT as f32) as f32)).round())
    }
}

#[repr(u32)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch
}