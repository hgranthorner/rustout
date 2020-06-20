use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::consts;
use crate::models::block::{Block, Destroyed};
use crate::models::colors::Colors;
use crate::models::paddle::Paddle;
use crate::models::rectangle::{Rectangle, Renderable};

pub enum Bounce {
    Horizontal,
    Vertical,
}

#[derive(PartialEq)]
pub enum BouncedAgainst {
    WallOrPaddle,
    Block,
    DestroyedBlock,
}

#[derive(Debug, Clone)]
pub struct Ball {
    pub shape: Rectangle,
    pub delta_x: i32,
    pub delta_y: i32,
}

impl Renderable for Ball {
    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.shape.color.to_rgb());
        let result = canvas.fill_rect(self.shape.rect);
        match result {
            Ok(_) => {}
            Err(e) => panic!(e),
        };
    }
}

impl Ball {
    pub fn mv(&mut self) {
        self.shape.rect.set_x(self.new_x());
        self.shape.rect.set_y(self.new_y());
    }

    fn new_x(&self) -> i32 {
        self.shape.rect.x() + self.delta_x
    }
    fn new_y(&self) -> i32 {
        self.shape.rect.y() + self.delta_y
    }
    fn right_edge(&self) -> i32 {
        self.shape.rect.x() + self.shape.rect.width() as i32
    }
    fn bottom(&self) -> i32 {
        self.shape.rect.y() + self.shape.rect.height() as i32
    }

    pub fn new(paddle: &Paddle) -> Ball {
        Ball {
            delta_y: -5,
            delta_x: 5,
            shape: Rectangle::new(
                Rect::new(
                    paddle.shape.rect.x() + (paddle.shape.rect.y() / 2),
                    paddle.shape.rect.y() - 10,
                    10,
                    10,
                ),
                Colors::GREEN,
            ),
        }
    }

    pub fn try_bounce(
        &mut self,
        paddle: &Paddle,
        blocks: &mut Vec<Block>,
    ) -> Option<BouncedAgainst> {
        let mut maybe_bounce = self.check_wall_or_paddle_bounce(paddle);
        if maybe_bounce.is_none() {
            maybe_bounce = self.check_block_bounce(blocks);
        }

        match maybe_bounce {
            Some((Bounce::Horizontal, bounced_against)) => {
                self.delta_x = -self.delta_x;
                Some(bounced_against)
            }
            Some((Bounce::Vertical, bounced_against)) => {
                self.delta_y = -self.delta_y;
                Some(bounced_against)
            }
            None => None,
        }
    }

    fn check_block_bounce(&self, blocks: &mut Vec<Block>) -> Option<(Bounce, BouncedAgainst)> {
        let mut maybe_bounce = None;
        let mut destroyed_block = false;
        let mut i = 0;
        for block in blocks.iter_mut() {
            if block.shape.rect.has_intersection(self.shape.rect) {
                maybe_bounce = Some(Bounce::Vertical);
                break;
            }
            i += 1;
        }
        if maybe_bounce.is_some() {
            if let Destroyed::IsDestroyed = blocks.get_mut(i).unwrap().hit(1) {
                destroyed_block = true;
                blocks.remove(i);
            }
        };

        match maybe_bounce {
            None => None,
            Some(bounce) => {
                let bounced_against = if destroyed_block {
                    BouncedAgainst::DestroyedBlock
                } else {
                    BouncedAgainst::Block
                };
                Some((bounce, bounced_against))
            }
        }
    }

    fn check_wall_or_paddle_bounce(&self, paddle: &Paddle) -> Option<(Bounce, BouncedAgainst)> {
        match (
            self.shape.rect.y(),
            self.bottom(),
            self.shape.rect.x(),
            self.right_edge(),
        ) {
            (_, _, l, r)
                if (l <= 0 && self.delta_x < 0)
                    || (r >= consts::WIDTH as i32 && self.delta_x > 0) =>
            {
                Some((Bounce::Horizontal, BouncedAgainst::WallOrPaddle))
            }
            (t, b, _, _)
                if (t <= 0 && self.delta_y < 0)
                    || (b >= consts::HEIGHT as i32 && self.delta_y > 0) =>
            {
                Some((Bounce::Vertical, BouncedAgainst::WallOrPaddle))
            }
            (_, _, _, _) if paddle.shape.rect.has_intersection(self.shape.rect) => {
                Some((Bounce::Vertical, BouncedAgainst::WallOrPaddle))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::colors::Colors;
    use quickcheck::{Arbitrary, Gen};

    impl Ball {
        pub fn test_new(shape: Rectangle, delta: i32) -> Ball {
            Ball {
                shape,
                delta_x: delta,
                delta_y: delta,
            }
        }
    }

    impl Arbitrary for Ball {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Ball::test_new(Rectangle::arbitrary(g), i32::arbitrary(g))
        }
    }

    #[quickcheck]
    fn bounces_properly(mut ball: Ball, paddle: Paddle) -> bool {
        let mut vec = Vec::new();
        if (ball.shape.rect.x() + ball.shape.rect.width() as i32 >= consts::WIDTH as i32
            && ball.delta_x > 0)
            || (ball.shape.rect.x() <= 0 && ball.delta_x < 0)
            || (ball.shape.rect.y() + ball.shape.rect.height() as i32 >= consts::HEIGHT as i32)
            || (ball.shape.rect.y() <= 0 && ball.delta_y < 0)
            || paddle.shape.rect.has_intersection(ball.shape.rect)
        {
            return ball.try_bounce(&paddle, &mut vec) == Some(BouncedAgainst::WallOrPaddle);
        } else {
            return ball.try_bounce(&paddle, &mut vec) == None;
        }
    }

    // #[quickcheck]
    // fn removes_block(mut ball: Ball, mut vec: Vec<Block>, block: Block) -> bool {
    //     if vec.is_empty() {
    //         vec.push(block);
    //     }
    //     let paddle = Paddle::test_new(Rectangle::new(
    //         Rect::new(i32::MAX, i32::MAX, 0, 0),
    //         Colors::RED,
    //     ));
    //     let before_len = vec.len();
    //     if let BouncedAgainst::DidBounce = ball.try_bounce(&paddle, &mut vec) {
    //         println!("BouncedAgainst!");
    //         println!("Before: {}. After: {}.", before_len, vec.len());
    //         vec.len() == before_len - 1
    //     } else {
    //         println!("Did not Bounced.");
    //         println!("Before: {}. After: {}.", before_len, vec.len());
    //         vec.len() == before_len
    //     }
    // }
}
