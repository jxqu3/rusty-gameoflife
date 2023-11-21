#![windows_subsystem = "windows"]

use std::{
    sync::
        Arc,
    thread,
    time::Duration,
};

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

    rl.set_target_fps(144);

    let g = Game::init(WIN_WIDTH, WIN_HEIGHT);

    let game = gc_pt(g);

    {
        let game = Arc::clone(&game);
        thread::spawn(move || loop {
            let mut game = game.write().unwrap();
            if !game.paused {
                game.grid.next_iter();
            }
            let ips = game.iterations_second.clone();
            drop(game);
            thread::sleep(Duration::from_nanos(1000_000_000 / ips as u64));
        });
    }
    

    while !rl.window_should_close() {
        let mut game_mut = game.write().unwrap();
        game_mut.handle_input(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        {
            let mut d = d.begin_mode2D(game_mut.camera);

            game_mut.draw(&mut d);
        }
        d.draw_text(
            &format!(
                "Iterations/Sec: {}\nFPS: {}\nZoom: {:.1}",
                game_mut.iterations_second,
                d.get_fps(),
                game_mut.camera.zoom
            ),
            12,
            12,
            20,
            Color::WHITE,
        );

        if game_mut.paused {
            d.draw_text(&"| |", WIN_WIDTH - 60, 12, 60, Color::WHITE);
        }
        drop(game_mut);
    }
    drop(game);
}
