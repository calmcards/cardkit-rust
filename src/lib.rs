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

    /*
    pub fn isInArea(topLeft: TextPos, bottomRight: TextPos) {

    }
    */

    pub fn draw_label(&mut self, left_edge: (f32, f32), s: String) -> &Self {
        let mut p: Vec2 = left_edge.into();
        for c in s.chars() {
            self.set(p.into(), c);
            p += (1.0, 0.0).into();
        }
        self
    }

    pub fn draw_box(&mut self, top_left: (f32, f32), inner_width: usize, inner_height: usize, style: BoxStyle) -> &Self {
        let mut p: Vec2 = top_left.into();

        self.set(p.into(), match style {
            BoxStyle::Thin => '┌',
            BoxStyle::Thick => '┏',
            BoxStyle::Double => '╔'
        });
        for _ in 0..inner_width {
            p += (1.0, 0.0).into();
            self.set(p.into(), match style {
                BoxStyle::Thin => '─',
                BoxStyle::Thick => '━',
                BoxStyle::Double => '═'
            });
        }
        p += (1.0, 0.0).into();
        self.set(p.into(), match style {
            BoxStyle::Thin => '┐',
            BoxStyle::Thick => '┓',
            BoxStyle::Double => '╗'
        });
        for _ in 0..inner_height {
            p += (0.0, 1.0).into();
            self.set(p.into(), match style {
                BoxStyle::Thin => '│',
                BoxStyle::Thick => '┃',
                BoxStyle::Double => '║'
            });
        }
        p += (0.0, 1.0).into();
        self.set(p.into(), match style {
            BoxStyle::Thin => '┘',
            BoxStyle::Thick => '┛',
            BoxStyle::Double => '╝'
        });
        for _ in 0..inner_width {
            p += (-1.0, 0.0).into();
            self.set(p.into(), match style {
                BoxStyle::Thin => '─',
                BoxStyle::Thick => '━',
                BoxStyle::Double => '═'
            });
        }
        p += (-1.0, 0.0).into();
        self.set(p.into(), match style {
            BoxStyle::Thin => '└',
            BoxStyle::Thick => '┗',
            BoxStyle::Double => '╚'
        });
        for _ in 0..inner_height {
            p += (0.0, -1.0).into();
            self.set(p.into(), match style {
                BoxStyle::Thin => '│',
                BoxStyle::Thick => '┃',
                BoxStyle::Double => '║'
            });
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
}

pub fn pixelspaceToTextspace(at: (f32, f32)) -> (f32, f32) {
    let (x, y) = at;

    ((x / ((PIXEL_WIDTH as f32 / TEXT_WIDTH as f32) as f32)),
     (y / ((PIXEL_HEIGHT as f32 / TEXT_HEIGHT as f32) as f32)))
}

pub fn textspaceToPixelspace(at: (f32, f32)) -> (f32, f32) {
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