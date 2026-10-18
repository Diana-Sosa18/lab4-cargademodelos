mod bmp;
mod exportar;
mod framebuffer;
mod mesh;
mod nave;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{self as glm, Mat4, Vec3};
use obj::Obj;
use std::time::Duration;

const ANCHO: usize = 800;
const ALTO: usize = 600;

fn a_color(c: Vec3) -> u32 {
    let r = (c.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (c.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (c.z.clamp(0.0, 1.0) * 255.0) as u32;
    (r << 16) | (g << 8) | b
}

fn aplicar(m: &Mat4, v: &Vec3) -> Vec3 {
    let r = m * glm::vec4(v.x, v.y, v.z, 1.0);
    Vec3::new(r.x, r.y, r.z)
}

fn render(framebuffer: &mut Framebuffer, modelo: &Obj, rotacion: &Mat4, pantalla: &Mat4) {
    let luz = Vec3::new(0.4, 0.7, 1.0).normalize();
    let girados: Vec<Vec3> = modelo.vertices.iter().map(|v| aplicar(rotacion, v)).collect();
    let proyectados: Vec<Vec3> = girados.iter().map(|v| aplicar(pantalla, v)).collect();

    framebuffer.clear();
    for cara in &modelo.caras {
        let [i, j, k] = cara.indices;
        let normal = (girados[j] - girados[i]).cross(&(girados[k] - girados[i]));
        let intensidad = if normal.norm() > 0.0 { normal.normalize().dot(&luz).max(0.0) } else { 0.0 };
        framebuffer.set_current_color(a_color(cara.color * (0.25 + 0.75 * intensidad)));
        triangle::triangle(framebuffer, proyectados[i], proyectados[j], proyectados[k]);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--exportar") {
        let modelo = nave::construir();
        let (vertices, triangulos) = exportar::guardar(&modelo, "nave.obj").expect("no se pudo exportar");
        println!("nave.obj: {} vertices, {} triangulos", vertices, triangulos);
        return;
    }

    let modelo = Obj::cargar("nave.obj").expect("no se pudo cargar nave.obj");
    let (centro, tamano) = modelo.centro_y_tamano();
    let escala = ALTO as f32 * 0.75 / tamano;
    let pantalla = glm::translate(&Mat4::identity(), &Vec3::new(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, 0.0))
        * glm::scale(&Mat4::identity(), &Vec3::new(escala, -escala, escala));
    let rotacion = glm::rotate_x(&Mat4::identity(), 0.5)
        * glm::rotate_y(&Mat4::identity(), 0.8)
        * glm::translate(&Mat4::identity(), &-centro);

    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
    framebuffer.set_background_color(0x14141E);
    render(&mut framebuffer, &modelo, &rotacion, &pantalla);

    if args.iter().any(|a| a == "--captura") {
        bmp::guardar(&framebuffer, "captura.bmp").unwrap();
        return;
    }

    let mut window = Window::new("Lab 3 - Nave", ANCHO, ALTO, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&framebuffer.buffer, ANCHO, ALTO).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}
