use crate::consts;
use crate::models::{Rectangle, GetRectangle};
use crate::paddle::Paddle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;
use crate::block::Block;

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", consts::WIDTH, consts::HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    game_loop(&mut canvas, &mut event_pump)
}

fn game_loop(canvas: &mut Canvas<Window>, mut event_pump: &mut EventPump) {
    let mut running = true;
    let mut blocks = Vec::new();
    let mut paddle = Paddle::new();
    let mut counter = 0;
    while running {
        if counter % 100 == 0 {
            blocks.push(Block::default());
        }
        canvas.set_draw_color((0, 0, 0));
        canvas.clear();
        draw_rectangle(canvas, paddle.get_rectangle());
        for block in &blocks {
            draw_rectangle(canvas, &block.shape);
        }
        handle_input(&mut event_pump, &mut running, &mut paddle);
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        counter += 1;
    }
}

fn handle_input(event_pump: &mut EventPump, running: &mut bool, paddle: &mut Paddle) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => *running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => paddle.mv(-10),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => paddle.mv(10),
            _ => {}
        }
    }
}

fn draw_rectangle(canvas: &mut Canvas<Window>, rect: &Rectangle) {
    canvas.set_draw_color(rect.color.to_rgb());
    let result = canvas.fill_rect(rect.rect);
    match result {
        Ok(_) => {}
        Err(e) => panic!(e),
    };
}
