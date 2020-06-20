use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::models::colors::Colors;
use crate::models::rectangle::{Rectangle, Renderable};
use crate::utils::SafeSubtract;

pub enum Destroyed {
    IsDestroyed,
    NotDestroyed,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub shape: Rectangle,
    pub health: u8,
}

impl Renderable for Block {
    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.shape.color.to_rgb());
        let result = canvas.fill_rect(self.shape.rect);
        match result {
            Ok(_) => {}
            Err(e) => panic!(e),
        };
    }
}

impl Block {
    pub fn new() -> Block {
        Block {
            health: 1,
            shape: Rectangle::new(Rect::new(10, 10, 400, 10), Colors::BLUE),
        }
    }

    pub fn hit(&mut self, damage: u8) -> Destroyed {
        match self.health.try_subtract(damage) {
            Ok(x) => {
                self.health = x;
            }
            Err(_) => {
                self.health = 0;
            }
        }

        if self.health == 0 {
            Destroyed::IsDestroyed
        } else {
            Destroyed::NotDestroyed
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;

    impl Block {
        pub fn test_new(shape: Rectangle) -> Block {
            Block { shape, health: 1 }
        }
    }

    impl Arbitrary for Block {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Block::test_new(Rectangle::arbitrary(g))
        }
    }

    #[quickcheck]
    fn loses_health_on_hit(mut block: Block, damage: u8) -> bool {
        let health_before = block.health;
        block.hit(damage);
        block.health
            == match health_before.try_subtract(damage) {
                Ok(x) => x,
                Err(_) => 0,
            }
    }

    // #[quickcheck]
    // fn width_is_always_foureenth_of_screen(block:Block) -> bool {
    //
    // }
}
