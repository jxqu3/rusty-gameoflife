use crate::structs::*;
use raylib::prelude::*;

impl Game {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(20, 20, 20, 255));

        let grid = self.grid.clone();

        for row in self.grid.cells.iter_mut() {
            for cell in row.iter_mut() {
                let mut color = if cell.alive {
                    Color::new(
                        (cell.get_alive_neighbors(&grid) as f32 / 8f32 * 255f32) as u8,
                        127,
                        0,
                        255,
                    )
                } else {
                    Color::BLACK
                };

                let mut brush_size = self.brush_size * 2 - (self.brush_size / 2);

                if self.brush_size == 1 {
                    brush_size = 0
                };

                let highlighting_cell = d.get_mouse_x() >= cell.x * self.cell_size - brush_size as i32
                && d.get_mouse_y() >= cell.y * self.cell_size - brush_size as i32
                && d.get_mouse_x()
                    <= cell.x * self.cell_size + self.cell_size + brush_size - 1 as i32
                && d.get_mouse_y()
                    <= cell.y * self.cell_size + self.cell_size + brush_size - 1 as i32;

                if highlighting_cell
                {
                    color = if !cell.alive {
                        Color::new(255, 255, 200, 50)
                    } else {
                        let c = (cell.get_alive_neighbors(&grid) as f32 / 8f32 * 255f32) as u8;
                        Color::new(c, 200, 100, c)
                    };
                    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                        cell.alive = true;
                    } else if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
                        cell.alive = false;
                    }
                }
                if self.draw_grid {
                    d.draw_rectangle(
                        cell.x as i32 * self.cell_size + 1,
                        cell.y as i32 * self.cell_size + 1,
                        self.cell_size - 2,
                        self.cell_size - 2,
                        color,
                    );
                    continue;
                }
                if cell.alive || highlighting_cell {
                    d.draw_rectangle(
                        cell.x as i32 * self.cell_size,
                        cell.y as i32 * self.cell_size,
                        self.cell_size,
                        self.cell_size,
                        color,
                    );
                }
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

        if self.paused {
            d.draw_text(&"| |", 740, 12, 60, Color::WHITE);
        }
    }
}

impl Cell {
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell { alive, x, y }
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

    pub fn clear(self: &mut Grid) {
        for i in &mut self.cells {
            for cell in i {
                cell.alive = false;
            }
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
