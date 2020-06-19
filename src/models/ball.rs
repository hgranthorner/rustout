use sdl2::rect::Rect;

use crate::consts;
use crate::models::colors::Colors;
use crate::models::paddle::Paddle;
use crate::models::rectangle::{GetRectangle, Rectangle};

pub enum Bounce {
    Horizontal,
    Vertical,
}

pub struct Ball {
    pub shape: Rectangle,
    pub delta_x: i32,
    pub delta_y: i32,
}

impl GetRectangle for Ball {
    fn get_rectangle(&self) -> &Rectangle {
        &self.shape
    }
}

impl Ball {
    pub fn mv(&mut self) {
        self.shape.rect.set_x(self.new_x());
        self.shape.rect.set_y(self.new_y());
    }

    fn new_x(&self) -> i32 { self.shape.rect.x() + self.delta_x }
    fn new_y(&self) -> i32 { self.shape.rect.y() + self.delta_y }
    fn right_edge(&self) -> i32 { self.shape.rect.x() + self.shape.rect.width() as i32 }
    fn bottom(&self) -> i32 { self.shape.rect.y() + self.shape.rect.height() as i32 }

    pub fn new(paddle: &Paddle) -> Ball {
        Ball {
            delta_y: -5,
            delta_x: 5,
            shape: Rectangle::new(
                Rect::new(
                    paddle.get_rectangle().rect.x() + (paddle.get_rectangle().rect.y() / 2),
                    paddle.get_rectangle().rect.y() - 10,
                    10,
                    10,
                ),
                Colors::GREEN,
            ),
        }
    }

    pub fn try_bounce(&mut self, paddle: &Paddle) -> bool {
        let maybe_bounce = match (self.shape.rect.y(), self.bottom(), self.shape.rect.x(), self.right_edge()) {
            (_, _, l, r) if l <= 0 || r >= consts::WIDTH as i32 => Some(Bounce::Horizontal),
            (t, b, _, _) if t <= 0 || b >= consts::HEIGHT as i32 => Some(Bounce::Vertical),
            (_, _, _, _) if paddle.get_rectangle().rect.has_intersection(self.shape.rect) => Some(Bounce::Vertical),
            _ => None,
        };
        match maybe_bounce {
            None => { false }
            Some(bounce_enum) => {
                match bounce_enum {
                    Bounce::Horizontal => {
                        self.delta_x = -self.delta_x;
                        true
                    }
                    Bounce::Vertical => {
                        self.delta_y = -self.delta_y;
                        true
                    }
                }
            }
        }
    }
}