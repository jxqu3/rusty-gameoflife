use crate::{structs::*, utils::*};
use raylib::prelude::*;

const CELL_SIZE: i32 = 10;

impl Game {
    pub fn handle_input(&mut self, d: &mut RaylibHandle) {
        let mouse_wheel = d.get_mouse_wheel_move() as i32;

        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            self.camera.target =
                self.camera.target - d.get_mouse_delta() * (1f32 / self.camera.zoom);
        }

        if mouse_wheel != 0 {
            if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                let new_zoom = min_one_f(self.camera.zoom + d.get_mouse_wheel_move() * 0.1);
                self.camera.zoom = new_zoom;
            } else if d.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                self.brush_size = min_one(self.brush_size + mouse_wheel);
            } else {
                self.iterations_second = min_one(self.iterations_second + mouse_wheel);
            }
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

        let m = d.get_screen_to_world2D(d.get_mouse_position(), self.camera);
        let cam_world_pos = d.get_screen_to_world2D(
            Vector2 {
                x: (self.width + 2 * CELL_SIZE) as f32,
                y: (self.height + 2 * CELL_SIZE) as f32,
            },
            self.camera,
        );
        let start_world_pos = d.get_screen_to_world2D(
            Vector2 {
                x: -CELL_SIZE as f32,
                y: -CELL_SIZE as f32,
            },
            self.camera,
        );

        let brush_size = self.brush_size - 1;
        let low = -brush_size;
        let high = CELL_SIZE + brush_size - 1;
        let mut x;
        let mut y;
        let mut color;
        let mut visible;

        for cell in self.grid.cells.iter_mut() {
            x = cell.x * CELL_SIZE;
            y = cell.y * CELL_SIZE;
            let cell_world_pos = Vector2 {
                x: x as f32,
                y: y as f32,
            };

            visible = cell_world_pos.x < cam_world_pos.x
                && cell_world_pos.y < cam_world_pos.y
                && cell_world_pos.x > start_world_pos.x
                && cell_world_pos.y > start_world_pos.y;

            if !visible {
                continue;
            }

            let neighbors_r = (grid.get_alive_neighbors(&cell) as f32 / 8f32 * 255f32) as u8;
            color = if cell.alive {
                Color::new(neighbors_r, 127, 0, 255)
            } else {
                Color::BLACK
            };

            if m.x as i32 >= x + low
                && m.x as i32 <= x + high
                && m.y as i32 >= y + low
                && m.y as i32 <= y + high
            {
                color = if !cell.alive {
                    Color::new(255, 255, 200, 50)
                } else {
                    Color::new(neighbors_r, 200, 100, neighbors_r)
                };
                if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    cell.alive = true;
                } else if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
                    cell.alive = false;
                }
            }

            if self.draw_grid {
                d.draw_rectangle(
                    cell.x * CELL_SIZE,
                    cell.y * CELL_SIZE,
                    CELL_SIZE - 1,
                    CELL_SIZE - 1,
                    color,
                );
                continue;
            }
            d.draw_rectangle(
                cell.x * CELL_SIZE,
                cell.y * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
    }

    pub fn init(width: i32, height: i32) -> Game {
        Game {
            grid: Grid::new(1000, 1000),
            iterations_second: 10,
            paused: true,
            brush_size: 1,
            draw_grid: true,
            camera: Camera2D {
                target: Vector2::new(0f32, 0f32),
                offset: Vector2::new(0f32, 0f32),
                rotation: 0f32,
                zoom: 1f32,
            },
            width,
            height,
        }
    }
}

impl Cell {
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell { alive, x, y }
    }
}

impl Grid {
    pub fn get_alive_neighbors(&self, cell: &Cell) -> u8 {
        let mut count: u8 = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                if self.get_cell(x + cell.x, y + cell.y).alive {
                    count += 1;
                }
            }
        }
        count
    }

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

    pub fn next_iter(self: &Grid) -> Grid {
        let mut new_grid = Grid::new(self.width, self.height);

        for cell in &self.cells {
            let neighbors = self.get_alive_neighbors(&cell);

            new_grid.get_cell_mut(cell.x, cell.y).alive =
                (cell.alive && neighbors == 2 || neighbors == 3) || (!cell.alive && neighbors == 3);
        }

        new_grid
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
