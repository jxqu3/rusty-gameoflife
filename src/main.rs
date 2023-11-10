#![windows_subsystem = "windows"]

use std::{sync::Arc, thread, time::Duration};

use crate::structs::*;
use raylib::prelude::*;
mod impls;
mod structs;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title("CheckM4te - Game Of Life")
        .build();

    rl.set_target_fps(144);

    let g = Game {
        grid: Grid::new((WIN_WIDTH / 3) as usize, (WIN_HEIGHT / 3) as usize),
        iterations_second: 10,
        paused: true,
        brush_size: 1,
        draw_grid: true,
        camera: Camera2D {
            target: Vector2::new(0f32, 0f32),
            offset: Vector2::new(0f32, 0f32),
            rotation: 0f32,
            zoom: 3f32,
        },
        width: WIN_HEIGHT,
        height: WIN_HEIGHT,
    };

    let game = gc_pt(g);

    {
        let game = Arc::clone(&game);
        thread::spawn(move || loop {
            let mut game = game.lock().unwrap();
            let ips = game.iterations_second.clone();
            if !game.paused {
                game.grid.next_iter();
            }
            drop(game);
            thread::sleep(Duration::from_nanos(1000_000_000 / ips as u64));
        });
    }

    while !rl.window_should_close() {
        let mut game_mut = game.lock().unwrap();
        let mut d = rl.begin_drawing(&thread);
        game_mut.handle_input(&mut d);
        {
            let mut d = d.begin_mode2D(game_mut.camera);

            game_mut.draw(&mut d);
        }
        d.draw_text(
            &format!(
                "Iterations/Sec: {}\nFPS: {}\nZoom: {}",
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
            d.draw_text(&"| |", 740, 12, 60, Color::WHITE);
        }
        drop(game_mut);
    }
}
