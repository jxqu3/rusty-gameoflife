#![windows_subsystem = "windows"]

use std::{sync::Arc, thread, time::Duration};

use crate::structs::*;
use raylib::prelude::*;
mod structs;
mod impls;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title("CheckM4te - Game Of Life")
        .build();

    rl.set_target_fps(144);

    let g = Game {
        grid: Grid::new((WIN_WIDTH / 5) as usize, (WIN_HEIGHT / 5) as usize),
        iterations_second: 999_999_999,
        paused: true,
        cell_size: 5,
        brush_size: 1,
        draw_grid: true,
    };

    let game = gc_pt(g);

    {
        let game = Arc::clone(&game);
        thread::spawn(move || loop {
            let mut game = game.lock().unwrap();
            let mut ips = game.iterations_second as u64;
            ips = if ips <= 1 { 1 } else { ips };
            if !game.paused{
                game.grid.next_iter();
            }
            drop(game);
            thread::sleep(Duration::from_nanos(1000_000_000 / ips));
        });
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mouse_wheel = d.get_mouse_wheel_move().floor() as i32;
        
        let mut game_mut = game.lock().unwrap();

        if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            if game_mut.cell_size + mouse_wheel >= 1 {
                game_mut.cell_size += mouse_wheel;
            }
        } else if d.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            if game_mut.brush_size + mouse_wheel >= 1 {
                game_mut.brush_size += mouse_wheel;
            }
        } else {
            if game_mut.iterations_second + mouse_wheel >= 1 {
                game_mut.iterations_second += mouse_wheel;
            }
        }
        
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game_mut.paused = !game_mut.paused;
        }

        if d.is_key_pressed(KeyboardKey::KEY_C) {
            game_mut.grid.clear();
        }

        if d.is_key_pressed(KeyboardKey::KEY_G) {
            game_mut.draw_grid = !game_mut.draw_grid;
        }

        game_mut.draw(&mut d);
    }
}
