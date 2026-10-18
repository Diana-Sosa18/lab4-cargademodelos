mod framebuffer;
mod mesh;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const ANCHO: usize = 800;
const ALTO: usize = 600;

fn main() {
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
    let mut window = Window::new("Lab 3 - Nave", ANCHO, ALTO, WindowOptions::default()).unwrap();
    framebuffer.set_background_color(0x14141E);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        window.update_with_buffer(&framebuffer.buffer, ANCHO, ALTO).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}
