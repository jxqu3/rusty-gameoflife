use std::sync::{Mutex, Arc};

use crate::{WIN_WIDTH, WIN_HEIGHT};

#[derive(Clone, Copy)]
pub struct Cell {
    pub alive: bool,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}


pub struct Game {
    pub paused: bool,
    pub grid: Grid,
    pub cell_size: i32,
    pub iterations_second: i32,
    pub brush_size: i32,
    pub draw_grid: bool,
}

pub fn gc_pt<T>(obj: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(obj))
}
