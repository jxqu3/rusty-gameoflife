use crate::structs::*;
use raylib::prelude::*;

impl Game {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(20, 20, 20, 255));

        let grid = self.grid.clone();

        for cell in self.grid.cells.iter_mut() {

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
        let mut cells = vec![Cell::new(0, 0, false); width * height];
        for (i, cell) in cells.iter_mut().enumerate() {
            cell.x = (i % width) as i32;
            cell.y = (i / width) as i32;
            cell.alive = false;
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn randomize(self: &mut Grid) {
        for c in &mut self.cells {
            c.alive = rand::random();
        }
    }

    pub fn clear(self: &mut Grid) {
        for c in &mut self.cells {
            c.alive = false;
        }
    }

    pub fn next_iter(self: &mut Grid) {
        let mut new_grid = Grid::new(self.width, self.height);

        for cell in &self.cells {
            let neighbors = cell.get_alive_neighbors(&self);

            if cell.alive && (neighbors == 2 || neighbors == 3) {
                new_grid.get_cell_mut(cell.x, cell.y).alive = true;
                continue;
            }
            if !cell.alive && neighbors == 3 {
                new_grid.get_cell_mut(cell.x, cell.y).alive = true;
                continue;
            }
            new_grid.get_cell_mut(cell.x, cell.y).alive = false;
        }

        self.cells = new_grid.cells;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> &Cell {
        let nx = (self.width as i32 + x ) % self.width as i32;
        let ny = (self.height as i32 + y) % self.height as i32;
        self.cells.get((nx + ny * self.width as i32) as usize).unwrap()
    }

    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> &mut Cell {
        let nx = (self.width as i32 + x ) % self.width as i32;
        let ny = (self.height as i32 + y) % self.height as i32;
        self.cells.get_mut((nx + ny * self.width as i32) as usize).unwrap()
    }

}
