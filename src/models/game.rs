use crate::consts;
use crate::models::ball::Ball;
use crate::models::block::Block;
use crate::models::paddle::Paddle;
use sdl2::rect::Rect;

pub struct Game {
    pub running: bool,
    pub blocks: Vec<Block>,
    pub paddle: Paddle,
    pub ball: Ball,
    pub letters: Vec<Rect>,
    score: u32,
    lives: u8,
}

impl Game {
    pub fn new() -> Game {
        let paddle = Paddle::new();
        let ball = Ball::new(&paddle);
        Game {
            running: true,
            blocks: Vec::new(),
            paddle,
            ball,
            score: 0,
            letters: create_score_letters(),
            lives: 3,
        }
    }

    pub fn stop_running(&mut self) {
        self.running = false;
    }

    pub fn add_block_layer(&mut self) {
        for block in self.blocks.iter_mut() {
            block
                .shape
                .rect
                .set_y(1 + block.shape.rect.y() + block.shape.rect.height() as i32);
        }
        let mut new_blocks = (0..14)
            .map(|x| {
                let new_x = (x * consts::WIDTH as i32) / 14;
                Block::new(new_x)
            })
            .collect();
        self.blocks.append(&mut new_blocks);
    }
    pub fn display_score(&self) -> Vec<Rect> {
        fn f(x: i32) -> Rect {
            Rect::new(x, consts::HEIGHT as i32 - 20, 4, 12)
        }
        if self.score > 0 {
            (0..self.score).map(|x| f(60 + (x * 5) as i32)).collect()
        } else {
            Vec::new()
        }
    }
    pub fn increase_score(&mut self) {
        self.score += 1;
    }
    pub fn display_lives(&self) -> Vec<Rect> {
        (0..self.lives).flat_map(|x| life_to_heart(x)).collect()
    }
    pub fn lose_life(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
        }
    }
}

fn create_score_letters() -> Vec<Rect> {
    fn f(x: i32, y: i32) -> Rect {
        Rect::new(x, consts::HEIGHT as i32 - y, 2, 2)
    }
    vec![
        // S
        f(5, 20),
        f(7, 20),
        f(9, 20),
        f(3, 18),
        f(3, 16),
        f(5, 14),
        f(7, 14),
        f(9, 12),
        f(9, 10),
        f(5, 8),
        f(7, 8),
        f(3, 8),
        // C
        f(15, 20),
        f(17, 20),
        f(19, 20),
        f(13, 18),
        f(13, 16),
        f(13, 14),
        f(13, 12),
        f(13, 10),
        f(15, 8),
        f(17, 8),
        f(19, 8),
        // O
        f(25, 20),
        f(27, 20),
        f(23, 18),
        f(23, 16),
        f(23, 14),
        f(23, 12),
        f(23, 10),
        f(29, 18),
        f(29, 16),
        f(29, 14),
        f(29, 12),
        f(29, 10),
        f(25, 8),
        f(27, 8),
        // R
        f(33, 20),
        f(35, 20),
        f(37, 20),
        f(33, 18),
        f(33, 16),
        f(33, 14),
        f(33, 12),
        f(33, 10),
        f(33, 8),
        f(39, 18),
        f(39, 16),
        f(37, 14),
        f(35, 14),
        f(39, 12),
        f(39, 10),
        f(39, 8),
        // E
        f(43, 20),
        f(45, 20),
        f(47, 20),
        f(49, 20),
        f(43, 18),
        f(43, 16),
        f(43, 14),
        f(43, 12),
        f(43, 10),
        f(49, 14),
        f(47, 14),
        f(45, 14),
        f(43, 8),
        f(45, 8),
        f(47, 8),
        f(49, 8),
        // :
        f(55, 18),
        f(55, 10),
    ]
}

fn life_to_heart(num: u8) -> Vec<Rect> {
    let f = |x: i32, y: i32| {
        Rect::new(
            (consts::WIDTH as i32 - 50) + (num as i32 * 16) + x,
            consts::HEIGHT as i32 - y,
            2,
            2,
        )
    };

    vec![
        f(0, 18),
        f(0, 16),
        f(2, 20),
        f(2, 18),
        f(2, 16),
        f(2, 14),
        f(2, 12),
        f(4, 18),
        f(4, 16),
        f(4, 14),
        f(4, 12),
        f(4, 10),
        f(4, 8),
        f(6, 20),
        f(6, 18),
        f(6, 16),
        f(6, 14),
        f(6, 12),
        f(8, 18),
        f(8, 16),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn can_add_layer() -> bool {
        let mut game = Game::new();
        game.add_block_layer();
        true
    }
}
