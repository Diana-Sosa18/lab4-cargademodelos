mod animacion;
mod bmp;
mod exportar;
mod framebuffer;
mod mesh;
mod nave;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
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

struct Camara {
    rot_x: f32,
    rot_y: f32,
    zoom: f32,
    wireframe: bool,
    girar: bool,
}

fn render(framebuffer: &mut Framebuffer, modelo: &Obj, camara: &Camara, centro: Vec3, tamano: f32) {
    let rotacion = glm::rotate_x(&Mat4::identity(), camara.rot_x)
        * glm::rotate_y(&Mat4::identity(), camara.rot_y)
        * glm::translate(&Mat4::identity(), &-centro);
    let escala = ALTO as f32 * 0.75 / tamano * camara.zoom;
    let pantalla = glm::translate(&Mat4::identity(), &Vec3::new(ANCHO as f32 / 2.0, ALTO as f32 / 2.0, 0.0))
        * glm::scale(&Mat4::identity(), &Vec3::new(escala, -escala, escala));

    let luz = Vec3::new(0.4, 0.7, 1.0).normalize();
    let girados: Vec<Vec3> = modelo.vertices.iter().map(|v| aplicar(&rotacion, v)).collect();
    let proyectados: Vec<Vec3> = girados.iter().map(|v| aplicar(&pantalla, v)).collect();

    framebuffer.clear();
    for cara in &modelo.caras {
        let [i, j, k] = cara.indices;
        let (a, b, c) = (proyectados[i], proyectados[j], proyectados[k]);
        if camara.wireframe {
            framebuffer.set_current_color(a_color(cara.color));
            triangle::wireframe(framebuffer, a, b, c);
        } else {
            let normal = (girados[j] - girados[i]).cross(&(girados[k] - girados[i]));
            let intensidad = if normal.norm() > 0.0 { normal.normalize().dot(&luz).max(0.0) } else { 0.0 };
            framebuffer.set_current_color(a_color(cara.color * (0.25 + 0.75 * intensidad)));
            triangle::triangle(framebuffer, a, b, c);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tiene = |opcion: &str| args.iter().any(|a| a == opcion);

    if tiene("--exportar") {
        let modelo = nave::construir();
        let (vertices, triangulos) = exportar::guardar(&modelo, "nave.obj").expect("no se pudo exportar");
        println!("nave.obj: {} vertices, {} triangulos", vertices, triangulos);
        return;
    }

    let modelo = Obj::cargar("nave.obj").expect("no se pudo cargar nave.obj");
    let (centro, tamano) = modelo.centro_y_tamano();
    let mut camara = Camara { rot_x: 0.5, rot_y: 0.8, zoom: 1.0, wireframe: tiene("--wireframe"), girar: false };
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
    framebuffer.set_background_color(0x14141E);

    if tiene("--captura") {
        render(&mut framebuffer, &modelo, &camara, centro, tamano);
        bmp::guardar(&framebuffer, "captura.bmp").unwrap();
        return;
    }

    if tiene("--gif") {
        let mut gif = animacion::Gif::crear("nave.gif", ANCHO, ALTO).unwrap();
        let cuadros = 60;
        for k in 0..cuadros {
            camara.rot_y = 0.8 + std::f32::consts::TAU * k as f32 / cuadros as f32;
            render(&mut framebuffer, &modelo, &camara, centro, tamano);
            gif.agregar(&framebuffer, 6).unwrap();
        }
        println!("nave.gif: {} cuadros", cuadros);
        return;
    }

    let mut window = Window::new(
        "Lab 3 - Nave | Flechas: rotar  W/S: zoom  Espacio: wireframe  R: girar  P: captura",
        ANCHO,
        ALTO,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let paso = 0.04;
        if window.is_key_down(Key::Left) {
            camara.rot_y -= paso;
        }
        if window.is_key_down(Key::Right) {
            camara.rot_y += paso;
        }
        if window.is_key_down(Key::Up) {
            camara.rot_x -= paso;
        }
        if window.is_key_down(Key::Down) {
            camara.rot_x += paso;
        }
        if window.is_key_down(Key::W) {
            camara.zoom = (camara.zoom * 1.02).min(4.0);
        }
        if window.is_key_down(Key::S) {
            camara.zoom = (camara.zoom / 1.02).max(0.2);
        }
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            camara.wireframe = !camara.wireframe;
        }
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            camara.girar = !camara.girar;
        }
        if camara.girar {
            camara.rot_y += 0.015;
        }

        render(&mut framebuffer, &modelo, &camara, centro, tamano);

        if window.is_key_pressed(Key::P, KeyRepeat::No) {
            bmp::guardar(&framebuffer, "captura.bmp").unwrap();
        }

        window.update_with_buffer(&framebuffer.buffer, ANCHO, ALTO).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}
