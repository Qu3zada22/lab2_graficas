mod framebuffer;
use framebuffer::Framebuffer;

use raylib::prelude::*;
use std::{thread, time::Duration};

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const CELL_SIZE: i32 = 6;

type Grid = [[bool; GRID_WIDTH]; GRID_HEIGHT];

fn main() {
    let (screen_width, screen_height) = (
        (GRID_WIDTH as i32) * CELL_SIZE,
        (GRID_HEIGHT as i32) * CELL_SIZE,
    );

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Juego de la Vida de Conway 🧪")
        .build();

    let mut framebuffer = Framebuffer::new(screen_width as u32, screen_height as u32);
    framebuffer.set_background_color(Color::BLACK);

    let mut grid = random_grid();

    while !rl.window_should_close() {
        grid = next_generation(&grid);

        framebuffer.clear();
        draw_grid(&mut framebuffer, &grid);
        framebuffer.swap_buffers(&mut rl, &thread);

        thread::sleep(Duration::from_millis(100)); // ~10 FPS para ver mejor los cambios
    }
}

fn random_grid() -> Grid {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            grid[y][x] = rng.gen_bool(0.2); // 20% chance de estar viva
        }
    }
    grid
}

fn count_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && nx < GRID_WIDTH as isize
                && ny < GRID_HEIGHT as isize
                && grid[ny as usize][nx as usize]
            {
                count += 1;
            }
        }
    }
    count
}

fn next_generation(current: &Grid) -> Grid {
    let mut new = [[false; GRID_WIDTH]; GRID_HEIGHT];
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let alive = current[y][x];
            let neighbors = count_neighbors(current, x, y);

            new[y][x] = match (alive, neighbors) {
                // Underpopulation or Overpopulation
                (true, n) if n < 2 || n > 3 => false,

                // Survival
                (true, 2) | (true, 3) => true,

                // Reproduction
                (false, 3) => true,

                // Otherwise
                _ => false,
            };
        }
    }
    new
}

fn draw_grid(fb: &mut Framebuffer, grid: &Grid) {
    fb.set_current_color(Color::WHITE);
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] {
                let px = (x as i32) * CELL_SIZE;
                let py = (y as i32) * CELL_SIZE;

                for dy in 0..CELL_SIZE {
                    for dx in 0..CELL_SIZE {
                        fb.draw_pixel(px + dx, py + dy);
                    }
                }
            }
        }
    }
}
