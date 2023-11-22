use raylib::prelude::Camera2D;

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
    pub iterations_second: i32,
    pub brush_size: i32,
    pub draw_grid: bool,
    pub camera: Camera2D,
    pub width: i32,
    pub height: i32,
}

