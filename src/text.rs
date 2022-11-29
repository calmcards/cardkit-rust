use crate::{TextBuffer};
use ultraviolet::vec::Vec2;

pub fn label(text: &mut TextBuffer, top_left: (f32, f32), s: String) {
    label_with_stride(text, top_left, (1.0, 0.0), s);
}

pub fn label_with_stride(text: &mut TextBuffer, top_left: (f32, f32), stride: (f32, f32), s: String) {
    let mut p: Vec2 = top_left.into();
    for c in s.chars() {
        text.set(p.into(), c);
        p += stride.into();
    }
}

#[derive(Copy, Clone)]
pub struct RectStyle {
    pub top: char,
    pub bottom: char,
    pub left: char,
    pub right: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char
}

impl RectStyle {
    pub const THIN: Self = Self {
        top: '─',
        bottom: '─',
        left: '│',
        right: '│',
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
    };
    pub const THICK: Self = Self {
        top: '━',
        bottom: '━',
        left: '┃',
        right: '┃',
        top_left: '┏',
        top_right: '┓',
        bottom_left: '┗',
        bottom_right: '┛',
    };
    pub const DOUBLE: Self = Self {
        top: '═',
        bottom: '═',
        left: '║',
        right: '║',
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
    };
}

pub fn rect(text: &mut TextBuffer, top_left: (f32, f32), inner_width: usize, inner_height: usize, style: RectStyle) {
    let mut p: Vec2 = top_left.into();

    text.set(p.into(), style.top_left);
    for _ in 0..inner_width {
        p += (1.0, 0.0).into();
        text.set(p.into(), style.top);
    }
    p += (1.0, 0.0).into();
    text.set(p.into(), style.top_right);
    for _ in 0..inner_height {
        p += (0.0, 1.0).into();
        text.set(p.into(), style.right);
    }
    p += (0.0, 1.0).into();
    text.set(p.into(), style.bottom_right);
    for _ in 0..inner_width {
        p += (-1.0, 0.0).into();
        text.set(p.into(), style.bottom);
    }
    p += (-1.0, 0.0).into();
    text.set(p.into(), style.bottom_left);
    for _ in 0..inner_height {
        p += (0.0, -1.0).into();
        text.set(p.into(), style.left);
    }
}