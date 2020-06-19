use crate::models::block::Block;
use crate::models::paddle::Paddle;
use crate::models::ball::Ball;

pub struct Game {
    pub running: bool,
    pub blocks: Vec<Block>,
    pub paddle: Paddle,
    pub ball: Ball,
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
        }
    }

    pub fn stop_running(&mut self) {
        self.running = false;
    }
}
