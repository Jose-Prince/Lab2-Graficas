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

fn render(framebuffer: &mut Framebuffer, x: isize) {
    let background_color = Color::new(33, 33, 55);
    let main_color = Color::new(255, 221, 221);

    framebuffer.set_background_color(background_color);
    framebuffer.clear();

    framebuffer.set_current_color(main_color);
    framebuffer.point(x, 40);
}

fn main() {
    let window_width = 800;
    let window_height = 800;

    let frame_delay = Duration::from_millis(0);

    let mut fb = Framebuffer::new(window_width, window_height);

    let mut window = match Window::new(
        "Rust Graphics - Framebuffer Example",
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

    let mut x = 1 as isize;  // Cambiado a isize
    let mut speed = 1 as isize;  // Cambiado a isize

    let mut points: Vec<[isize; 2]> = vec![
        [100, 100],
        [400, 500],
        [700, 300],
    ];

    initialPoints(&mut fb, &points);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if x as usize == window_width {
            speed = -1;
        } else if x == 0 {
            speed = 1;
        }

        x += speed;

        // Aquí puedes agregar lógica adicional para actualizar tu framebuffer y dibujar cosas en él

        // render(&mut fb, x);

        window.update_with_buffer(&fb.get_buffer(), window_width, window_height).unwrap();

        std::thread::sleep(frame_delay);
    }

    // Guardar el framebuffer en un archivo BMP
    // if let Err(e) = fb.save_as_bmp("polygon.bmp") {
    //     eprintln!("Failed to write BMP file: {}", e);
    // }
}
