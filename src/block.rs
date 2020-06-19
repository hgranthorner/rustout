use sdl2::rect::Rect;

use crate::models::{Colors, Rectangle};
use crate::utils::SafeSubtract;

#[derive(Debug, Clone)]
pub struct Block {
    pub shape: Rectangle,
    pub health: u8,
}

impl Block {
    pub fn default() -> Block {
        Block {
            health: 1,
            shape: Rectangle::new(Rect::new(10, 10, 10, 10), Colors::BLUE),
        }
    }

    pub fn new(shape: Rectangle, hits_left: u8) -> Block {
        Block {
            shape,
            health: hits_left,
        }
    }

    pub fn hit(&mut self, damage: u8) {
        match self.health.try_subtract(damage) {
            Ok(x) => { self.health = x; },
            Err(_) => { self.health = 0; },
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;

    impl Arbitrary for Block {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Block::new(Rectangle::arbitrary(g), u8::arbitrary(g))
        }
    }

    #[quickcheck]
    fn loses_health_on_hit(mut block: Block, damage: u8) -> bool {
        let health_before = block.health;
        block.hit(damage);
        block.health == match health_before.try_subtract(damage) {
            Ok(x) => { x },
            Err(_) => { 0 },
        }
    }
    
    // #[quickcheck]
    // fn width_is_always_foureenth_of_screen(block:Block) -> bool {
    //
    // }
}
