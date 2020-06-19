use std::convert::TryInto;

use sdl2::rect::Rect;

use crate::consts;
use crate::models::{Colors, GetRectangle};
use crate::models::Rectangle;

#[derive(Debug, Clone)]
pub struct Paddle {
    shape: Rectangle,
}

impl GetRectangle for Paddle {
    fn get_rectangle(&self) -> &Rectangle {
        &self.shape
    }
}

impl Paddle {
    pub fn new() -> Paddle {
        let width = consts::WIDTH / 10;
        let x = (consts::WIDTH / 2) - (width / 2);
        let y = (consts::HEIGHT / 10) * 9;
        Paddle {
            shape: Rectangle::new(Rect::new(x as i32, y as i32, width, 10), Colors::RED),
        }
    }

    // #[allow(dead_code)]
    // pub fn new(shape: Rectangle) -> Paddle {
    //     Paddle { shape }
    // }

    pub fn mv(&mut self, delta_x: i32) {
        let new_x = self.shape.rect.x() + delta_x;
        let cap = consts::WIDTH - self.shape.rect.width();
        if new_x > cap as i32 {
            self.shape.rect.x = cap
                .try_into()
                .unwrap();
        } else if new_x < 0 {
            self.shape.rect.x = 0;
        } else {
            self.shape.rect.x = new_x;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use quickcheck::{Arbitrary, Gen};

    use super::*;

    impl Arbitrary for Colors {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            match Colors::try_from(u32::arbitrary(g) % 5) {
                Ok(x) => x,
                Err(_) => panic!("Invalid color generated."),
            }
        }
    }

    impl Arbitrary for Rectangle {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Rectangle {
                rect: Rect::new(
                    i32::arbitrary(g).abs(),
                    i32::arbitrary(g).abs(),
                    u32::arbitrary(g),
                    u32::arbitrary(g),
                ),
                color: Colors::arbitrary(g),
            }
        }
    }

    impl Paddle {
        pub fn test_new(shape: Rectangle) -> Paddle {
            Paddle { shape }
        }
    }

    impl Arbitrary for Paddle {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Paddle::test_new(Rectangle::arbitrary(g))
        }
    }

    #[quickcheck]
    fn works() -> bool {
        true
    }

    #[quickcheck]
    fn paddle_can_move(mut paddle: Paddle, x: i32) -> bool {
        let x = x.abs() % consts::WIDTH as i32;
        let before_x = paddle.shape.rect.x();
        paddle.mv(x);
        let after_x = paddle.shape.rect.x();
        after_x - before_x == x
    }

    #[quickcheck]
    fn moves_to_edge_if_movement_overflows(mut paddle: Paddle, mut x: i32) -> bool {
        return if x >= 0 {
            x += consts::WIDTH as i32;
            paddle.mv(x);
            let b =
                paddle.shape.rect.x() == consts::WIDTH as i32 - paddle.shape.rect.width() as i32;
            b
        } else {
            x -= consts::WIDTH as i32;
            paddle.mv(x);
            let b = paddle.shape.rect.x() == 0;
            b
        };
    }
}
