mod framebuffer;
use framebuffer::Framebuffer;

use raylib::prelude::*;
use std::{thread, time::Duration};

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 75;
const CELL_SIZE: usize = 8;

type Grid = [[bool; GRID_WIDTH]; GRID_HEIGHT];

fn count_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0
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
            let neighbors = count_neighbors(current, x, y);
            new[y][x] = match (current[y][x], neighbors) {
                (true, 2) | (_, 3) => true,
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
                for dy in 0..CELL_SIZE {
                    for dx in 0..CELL_SIZE {
                        fb.draw_pixel(
                            (x * CELL_SIZE + dx) as i32,
                            (y * CELL_SIZE + dy) as i32,
                        );
                    }
                }
            }
        }
    }
}

fn seed_grid() -> Grid {
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];

    // Glider
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;

    grid
}

fn main() {
    let (width, height) = (GRID_WIDTH * CELL_SIZE, GRID_HEIGHT * CELL_SIZE);
    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Conway's Game of Life 💥")
        .build();

    let mut framebuffer = Framebuffer::new(width as u32, height as u32);
    framebuffer.set_background_color(Color::new(20, 20, 40, 255));

    let mut grid = seed_grid();

    while !rl.window_should_close() {
        framebuffer.clear();
        draw_grid(&mut framebuffer, &grid);
        framebuffer.swap_buffers(&mut rl, &thread);

        grid = next_generation(&grid);

        thread::sleep(Duration::from_millis(100)); // 10 FPS
    }
}
