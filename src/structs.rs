use std::sync::{Arc, Mutex};

use raylib::prelude::*;

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
    pub x: i32,
    pub y: i32,
    pub just_edited: bool,
}

impl Cell {
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell {
            alive,
            x,
            y,
            just_edited: false,
        }
    }

    pub fn get_alive_neighbors(&self, grid: &Grid) -> u8 {
        let mut count: u8 = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                if grid.get_cell(x + self.x, y + self.y).alive {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut cells = vec![vec![Cell::new(0, 0, false); height as usize]; width as usize];
        for (x, i) in cells.iter_mut().enumerate() {
            for (y, cell) in i.iter_mut().enumerate() {
                cell.x = x as i32;
                cell.y = y as i32;
                cell.alive = false;
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn next_iter(self: &mut Grid) {
        let mut new_grid = Grid::new(self.width, self.height);

        for i in &self.cells {
            for cell in i {
                let neighbors = cell.get_alive_neighbors(&self);

                if cell.alive && (neighbors == 2 || neighbors == 3) {
                    new_grid.cells[cell.x as usize][cell.y as usize].alive = true;
                    continue;
                }
                if !cell.alive && neighbors == 3 {
                    new_grid.cells[cell.x as usize][cell.y as usize].alive = true;
                    continue;
                }
                new_grid.cells[cell.x as usize][cell.y as usize].alive = false;
            }
        }

        self.cells = new_grid.cells;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> &Cell {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            &self.cells[0][0]
        } else {
            &self.cells[x as usize][y as usize]
        }
    }
}

pub struct Game {
    pub grid: Grid,
    pub cell_size: i32,
    pub iterations_second: i32,
    pub paused: bool,
}

pub fn gc_pt<T>(obj: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(obj))
}

impl Game {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(20, 20, 20, 255));

        for row in self.grid.cells.iter_mut() {
            for cell in row.iter_mut() {
                let mut color = if cell.alive {
                    Color::ORANGE
                } else {
                    Color::BLACK
                };

                if d.get_mouse_x() >= cell.x * self.cell_size as i32
                    && d.get_mouse_x() <= cell.x * self.cell_size + self.cell_size - 1 as i32
                    && d.get_mouse_y() >= cell.y * self.cell_size as i32
                    && d.get_mouse_y() <= cell.y * self.cell_size + self.cell_size - 1 as i32
                {
                    color = Color::YELLOW;
                    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) && !cell.just_edited {
                        cell.alive = !cell.alive;
                        cell.just_edited = true;
                    } else {
                        cell.just_edited = false;
                    }
                }
                d.draw_rectangle(
                    cell.x as i32 * self.cell_size + 1,
                    cell.y as i32 * self.cell_size + 1,
                    self.cell_size - 2,
                    self.cell_size - 2,
                    color,
                );
            }
        }

        d.draw_text(
            &format!(
                "Iterations/Sec: {}\nFPS: {}",
                self.iterations_second,
                d.get_fps()
            ),
            12,
            12,
            20,
            Color::WHITE,
        );
    }
}
