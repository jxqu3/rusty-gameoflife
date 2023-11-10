use crate::structs::*;
use raylib::prelude::*;

impl Game {
    pub fn handle_input(&mut self, d: &mut RaylibDrawHandle<'_>) {
        let mouse_wheel = d.get_mouse_wheel_move() as i32;

        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            self.camera.offset = self.camera.offset + d.get_mouse_delta();
        }

        if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            self.camera.zoom =
                (self.camera.zoom + d.get_mouse_wheel_move() * 0.2).clamp(0.5, f32::MAX);
        } else if d.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            self.brush_size = (self.brush_size + mouse_wheel).clamp(1, i32::MAX);
        } else {
            self.iterations_second = (self.iterations_second + mouse_wheel).clamp(1, i32::MAX);
        }

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.paused = !self.paused;
        }

        if d.is_key_pressed(KeyboardKey::KEY_C) {
            self.grid.clear();
        }

        if d.is_key_pressed(KeyboardKey::KEY_R) {
            self.grid.randomize();
        }

        if d.is_key_pressed(KeyboardKey::KEY_G) {
            self.draw_grid = !self.draw_grid;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
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

            let m_x = d.get_mouse_x() as f32;
            let m_y = d.get_mouse_y() as f32;
            let o_x = self.camera.offset.x;
            let o_y = self.camera.offset.y;
            let zoom = self.camera.zoom;

            let pos_x = ((m_x - o_x) / zoom) as i32;
            let pos_y = ((m_y - o_y) / zoom) as i32;

            let highlighting_cell = pos_x as i32 - 1 >= cell.x * 5 as i32 - brush_size
                && pos_x as i32 - 1 <= cell.x * 5 + 5 as i32 + brush_size - 1
                && pos_y as i32 - 1 >= cell.y * 5 as i32 - brush_size
                && pos_y as i32 - 1 <= cell.y * 5 + 5 as i32 + brush_size - 1;

            if highlighting_cell {
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
                d.draw_rectangle(cell.x * 5 + 1, cell.y * 5 + 1, 4, 4, color);
                continue;
            }
            d.draw_rectangle(cell.x * 5, cell.y * 5, 5, 5, color);
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
        let nx = (self.width as i32 + x) % self.width as i32;
        let ny = (self.height as i32 + y) % self.height as i32;
        self.cells
            .get((nx + ny * self.width as i32) as usize)
            .unwrap()
    }

    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> &mut Cell {
        let nx = (self.width as i32 + x) % self.width as i32;
        let ny = (self.height as i32 + y) % self.height as i32;
        self.cells
            .get_mut((nx + ny * self.width as i32) as usize)
            .unwrap()
    }
}
