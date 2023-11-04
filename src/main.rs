use std::{sync::Arc, thread, time::Duration};

use crate::structs::*;
use raylib::prelude::*;
mod structs;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("CheckM4te - Game Of Life")
        .build();

    rl.set_target_fps(60);

    let g = Game {
        grid: Grid::new(80, 80),
        iterations_second: 5,
        paused: true,
        cell_size: 10,
    };

    let game = gc_pt(g);

    {
        let game = Arc::clone(&game);
        thread::spawn(move || loop {
            let mut game = game.lock().unwrap();
            let mut ips = game.iterations_second.clone() as u64;
            ips = if ips <= 1 { 1 } else { ips };
            if !game.paused{
                game.grid.next_iter();
            }
            thread::sleep(Duration::from_millis(1000 / ips));
        });
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mouse_wheel = d.get_mouse_wheel_move() as i32;
        
        let mut game_mut = game.lock().unwrap();

        if !game_mut.iterations_second - mouse_wheel <= 1 {
            game_mut.iterations_second += mouse_wheel as i32;
        } else {
            game_mut.iterations_second = 1;
        }
        if game_mut.iterations_second <= 1 {
            game_mut.iterations_second = 1;
        }

        if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && !game_mut.iterations_second - mouse_wheel <= 1 {
            game_mut.cell_size += mouse_wheel;
        }
        
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game_mut.paused = !game_mut.paused;
        }

        game_mut.draw(&mut d);
    }
}
