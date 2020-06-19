use sdl2::rect::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::models::colors::Colors;

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>);
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub rect: Rect,
    pub color: Colors,
}

impl Renderable for Rectangle {
    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color.to_rgb());
        let result = canvas.fill_rect(self.rect);
        match result {
            Ok(_) => {}
            Err(e) => panic!(e),
        };
    }
}

impl Rectangle {
    pub fn new(shape: Rect, color: Colors) -> Rectangle {
        Rectangle { rect: shape, color }
    }
}
