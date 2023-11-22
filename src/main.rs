#![windows_subsystem = "windows"]

use std::thread;

use crate::structs::*;
use raylib::prelude::*;
mod impls;
mod structs;
mod utils;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title("CheckM4te - Game Of Life")
        .build();

    rl.set_target_fps(300);

    let mut game = Game::init(WIN_WIDTH, WIN_HEIGHT);

    let mut iter_time = 0f32;

    let (tx, sx) = std::sync::mpsc::sync_channel::<Grid>(1);

    while !rl.window_should_close() {
        iter_time += rl.get_frame_time();
        if let Ok(grid) = sx.try_recv() {
            game.grid = grid;
        }
        if iter_time >= 1. / game.iterations_second as f32 && !game.paused {
            iter_time = 0.;
            let tx = tx.clone();
            let grid = game.grid.clone();
            thread::spawn(move || {
                tx.try_send(grid.next_iter()).unwrap();
            });
        }
        game.handle_input(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        {
            let mut d = d.begin_mode2D(game.camera);

            game.draw(&mut d);
        }
        d.draw_text(
            &format!(
                "Iterations/Sec: {}\nFPS: {}\nZoom: {:.1}\nGrid Size: {s}x{s}",
                game.iterations_second,
                d.get_fps(),
                game.camera.zoom,
                s = game.grid.width
            ),
            12,
            12,
            20,
            Color::WHITE,
        );

        if game.paused {
            d.draw_text(&"| |", WIN_WIDTH - 60, 12, 60, Color::WHITE);
        }
    }
}
