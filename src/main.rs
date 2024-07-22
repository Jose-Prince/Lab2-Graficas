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

fn main() {
    let window_width = 800;
    let window_height = 800;

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

    // Inicializa el estado del juego de la vida con algunas células vivas
    let mut state = vec![false; window_width * window_height];
    state[1 * window_width + 2] = true;
    state[2 * window_width + 3] = true;
    state[3 * window_width + 1] = true;
    state[3 * window_width + 2] = true;
    state[3 * window_width + 3] = true;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Actualiza el estado del juego de la vida
        state = update_state(&state, window_width, window_height);

        // Actualiza el framebuffer con el nuevo estado
        fb.update_buffer(&state);

        // Renderiza el framebuffer en la ventana
        window.update_with_buffer(&fb.get_buffer(), window_width, window_height).unwrap();

        std::thread::sleep(frame_delay);
    }

    // Guardar el framebuffer en un archivo BMP (opcional)
    if let Err(e) = fb.save_as_bmp("game_of_life.bmp") {
        eprintln!("Failed to write BMP file: {}", e);
    }
}
