use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::consts;
use crate::models::game::Game;
use crate::models::rectangle::GetRectangle;

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

fn game_loop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump) {
    let mut game = Game::new();
    while game.running {
        handle_input(event_pump, &mut game);
        update_state(&mut game);
        render_game(canvas, &mut game);
    }
}

fn handle_input(event_pump: &mut EventPump, game: &mut Game) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => game.stop_running(),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => game.paddle.mv(-10),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => game.paddle.mv(10),
            _ => {}
        }
    }
}

fn update_state(game: &mut Game) {
    game.ball.try_bounce(&game.paddle);
    game.ball.mv();
}

fn render_game(canvas: &mut Canvas<Window>, game: &mut Game) {
    for block in &game.blocks {
        draw_rectangle(canvas, block)
    }

    draw_rectangle(canvas, &game.ball);
    draw_rectangle(canvas, &game.paddle);

    canvas.present();
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    canvas.set_draw_color((0, 0, 0));
    canvas.clear();
}

fn draw_rectangle(canvas: &mut Canvas<Window>, rect: &impl GetRectangle) {
    canvas.set_draw_color(rect.get_rectangle().color.to_rgb());
    let result = canvas.fill_rect(rect.get_rectangle().rect);
    match result {
        Ok(_) => {}
        Err(e) => panic!(e),
    };
}
