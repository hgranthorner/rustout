use std::convert::TryFrom;

use sdl2::pixels::Color;
use sdl2::rect::*;

#[derive(Debug, Clone)]
pub enum Colors {
    RED,
    GREEN,
    BLUE,
    WHITE,
    BLACK,
}

impl TryFrom<u32> for Colors {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == Colors::RED as u32 => Ok(Colors::RED),
            x if x == Colors::BLUE as u32 => Ok(Colors::BLUE),
            x if x == Colors::GREEN as u32 => Ok(Colors::GREEN),
            x if x == Colors::WHITE as u32 => Ok(Colors::WHITE),
            x if x == Colors::BLACK as u32 => Ok(Colors::BLACK),
            _ => Err(()),
        }
    }
}

impl Colors {
    pub fn to_rgb(&self) -> Color {
        match self {
            Colors::RED => Color::from((255, 0, 0)),
            Colors::GREEN => Color::from((0, 255, 0)),
            Colors::BLUE => Color::from((0, 0, 255)),
            Colors::WHITE => Color::from((255, 255, 255)),
            Colors::BLACK => Color::from((0, 0, 0)),
        }
    }
}

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

