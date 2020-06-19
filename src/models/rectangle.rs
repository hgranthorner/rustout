use sdl2::rect::*;

use crate::models::colors::Colors;

pub trait GetRectangle {
    fn get_rectangle(&self) -> &Rectangle;
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub rect: Rect,
    pub color: Colors,
}

impl GetRectangle for Rectangle {
    fn get_rectangle(&self) -> &Rectangle {
        self
    }
}

impl Rectangle {
    pub fn new(shape: Rect, color: Colors) -> Rectangle {
        Rectangle { rect: shape, color }
    }
}
