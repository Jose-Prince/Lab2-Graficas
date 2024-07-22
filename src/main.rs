mod color;
mod framebuffer;
mod bmp;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use color::Color;
use line::Line;
use polygon::Polygon;
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;

const PIXEL_SIZE: usize = 10; // Tamaño de cada celda en píxeles

fn count_neighbors(state: &[bool], width: usize, height: usize, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dx in [-1, 0, 1].iter() {
        for dy in [-1, 0, 1].iter() {
            if *dx == 0 && *dy == 0 {
                continue;
            }
            let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
            if state[ny * width + nx] {
                count += 1;
            }
        }
    }
    count
}

fn update_state(state: &[bool], width: usize, height: usize) -> Vec<bool> {
    let mut new_state = vec![false; width * height];
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let alive = state[idx];
            let neighbors = count_neighbors(state, width, height, x, y);
            new_state[idx] = match (alive, neighbors) {
                (true, 2) | (_, 3) => true,
                _ => false,
            };
        }
    }
    new_state
}

fn place_pattern(state: &mut [bool], width: usize, pattern: &[(usize, usize)], offset_x: usize, offset_y: usize) {
    for &(x, y) in pattern.iter() {
        let idx = (y + offset_y) * width + (x + offset_x);
        if idx < state.len() {
            state[idx] = true;
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    // Calcula las dimensiones del tablero del Juego de la Vida
    let board_width = window_width / PIXEL_SIZE;
    let board_height = window_height / PIXEL_SIZE;

    let frame_delay = Duration::from_millis(100);

    let mut fb = Framebuffer::new(window_width, window_height);

    let mut window = match Window::new(
        "Rust Graphics - Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ) {
        Ok(window) => window,
        Err(e) => {
            eprintln!("Failed to create window: {}", e);
            return;
        }
    };

    // Definir patrones
    let block = vec![(1, 1), (1, 2), (2, 1), (2, 2)];
    let blinker = vec![(1, 0), (1, 1), (1, 2)];
    let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let pulsar = vec![
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    let lwss = vec![
        (0, 1), (3, 1),
        (4, 2),
        (0, 3), (4, 3),
        (1, 4), (2, 4), (3, 4), (4, 4),
    ];
    let toad = vec![
        (2, 1), (3, 1), (4, 1),
        (1, 2), (2, 2), (3, 2),
    ];
    let beacon = vec![
        (0, 0), (1, 0), (0, 1), (1, 1),
        (2, 2), (3, 2), (2, 3), (3, 3),
    ];
    let pentadecathlon = vec![
        (2, 1), (3, 1),
        (1, 2), (4, 2),
        (2, 3), (3, 3),
        (2, 5), (3, 5),
        (1, 6), (4, 6),
        (2, 7), (3, 7),
    ];
    let diehard = vec![
        (0, 1), (1, 1),
        (1, 2),
        (5, 2),
        (6, 0), (6, 2), (7, 2),
    ];
    let acorn = vec![
        (1, 0),
        (3, 1),
        (0, 2), (1, 2), (4, 2), (5, 2), (6, 2),
    ];
    let queen_bee_shuttle = vec![
        (0, 1), (1, 1), (2, 1),
        (2, 2),
        (1, 3),
        (5, 2), (6, 2), (7, 2),
        (4, 3), (4, 4),
    ];
    let r_pentomino = vec![
        (1, 0), (2, 0),
        (0, 1), (1, 1),
        (1, 2),
    ];
    let tumbler = vec![
        (1, 0), (2, 0), (4, 0), (5, 0),
        (0, 1), (1, 1), (2, 1), (4, 1), (5, 1), (6, 1),
        (0, 2), (2, 2), (4, 2), (6, 2),
        (0, 3), (1, 3), (2, 3), (4, 3), (5, 3), (6, 3),
        (1, 4), (2, 4), (4, 4), (5, 4),
    ];
    let gosper_glider_gun = vec![
        (24, 0),
        (22, 1), (24, 1),
        (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
        (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
        (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
        (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
        (10, 6), (11, 6), (16, 6), (17, 6),
        (11, 7), (15, 7),
    ];

    let patterns = vec![
        block, blinker, glider, pulsar, lwss, toad, beacon, pentadecathlon, diehard, acorn, queen_bee_shuttle, r_pentomino, tumbler, gosper_glider_gun,
    ];

    let mut state = vec![false; board_width * board_height];

    let max_pattern_width = patterns.iter().map(|p| p.iter().map(|&(x, _)| x).max().unwrap_or(0)).max().unwrap_or(0) + 1;
    let max_pattern_height = patterns.iter().map(|p| p.iter().map(|&(_, y)| y).max().unwrap_or(0)).max().unwrap_or(0) + 1;

    let num_patterns_x = (board_width as f64 / max_pattern_width as f64).ceil() as usize;
    let num_patterns_y = (board_height as f64 / max_pattern_height as f64).ceil() as usize;

    for y in 0..num_patterns_y {
        for x in 0..num_patterns_x {
            let pattern = &patterns[(y * num_patterns_x + x) % patterns.len()];
            let offset_x = x * max_pattern_width;
            let offset_y = y * max_pattern_height;
            place_pattern(&mut state, board_width, pattern, offset_x, offset_y);
        }
    }

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Actualiza el estado del juego de la vida
        state = update_state(&state, board_width, board_height);

        // Actualiza el framebuffer con el nuevo estado
        fb.clear();
        for y in 0..board_height {
            for x in 0..board_width {
                if state[y * board_width + x] {
                    for dy in 0..PIXEL_SIZE {
                        for dx in 0..PIXEL_SIZE {
                            fb.point((x * PIXEL_SIZE + dx) as isize, (y * PIXEL_SIZE + dy) as isize);
                        }
                    }
                }
            }
        }

        // Renderiza el framebuffer en la ventana
        window.update_with_buffer(&fb.get_buffer(), window_width, window_height).unwrap();

        std::thread::sleep(frame_delay);
    }

    // Guardar el framebuffer en un archivo BMP (opcional)
    if let Err(e) = fb.save_as_bmp("game_of_life.bmp") {
        eprintln!("Failed to write BMP file: {}", e);
    }
}
