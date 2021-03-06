use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::consts;
use crate::models::ball::BouncedAgainst;
use crate::models::colors::Colors;
use crate::models::game::Game;
use crate::models::rectangle::Renderable;

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
    // Held keys need to be handled separately
    if event_pump
        .keyboard_state()
        .is_scancode_pressed(Scancode::Right)
    {
        game.paddle.mv(10);
    } else if event_pump
        .keyboard_state()
        .is_scancode_pressed(Scancode::Left)
    {
        game.paddle.mv(-10);
    }
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => game.stop_running(),
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                // Only for debugging
                game.ball.delta_x *= 2;
                game.ball.delta_y *= 2;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                // Only for debugging
                game.add_block_layer();
            }

            _ => {}
        }
    }
}

fn update_state(game: &mut Game) {
    match game.ball.try_bounce(&game.paddle, &mut game.blocks) {
        Some(BouncedAgainst::DestroyedBlock) => game.increase_score(),
        Some(BouncedAgainst::Floor) => game.lose_life(),
        _ => {}
    }

    // if let Some(BouncedAgainst::DestroyedBlock) =
    //     game.ball.try_bounce(&game.paddle, &mut game.blocks)
    // {
    //     game.increase_score();
    // };
    game.ball.mv();
}

fn render_game(canvas: &mut Canvas<Window>, game: &mut Game) {
    for block in &game.blocks {
        block.render(canvas);
    }

    game.ball.render(canvas);
    game.paddle.render(canvas);
    canvas.set_draw_color(Colors::WHITE.to_rgb());
    match canvas.fill_rects(&game.letters) {
        Ok(_) => {}
        Err(e) => panic!(e),
    }

    match canvas.fill_rects(&game.display_score()) {
        Ok(_) => {}
        Err(e) => panic!(e),
    }

    canvas.set_draw_color(Colors::RED.to_rgb());
    match canvas.fill_rects(&game.display_lives()) {
        Ok(_) => {}
        Err(e) => panic!(e),
    }

    canvas.present();
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    canvas.set_draw_color((0, 0, 0));
    canvas.clear();
}
