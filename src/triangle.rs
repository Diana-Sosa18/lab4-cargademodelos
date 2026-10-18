use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub fn line(framebuffer: &mut Framebuffer, a: Vec3, b: Vec3) {
    let (mut x0, mut y0) = (a.x.round() as i32, a.y.round() as i32);
    let (x1, y1) = (b.x.round() as i32, b.y.round() as i32);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let pasos = dx.max(-dy).max(1) as f32;
    let mut err = dx + dy;
    let mut i = 0.0;
    loop {
        let z = a.z + (b.z - a.z) * (i / pasos) + 0.01;
        framebuffer.point(x0, y0, z);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
        i += 1.0;
    }
}

pub fn wireframe(framebuffer: &mut Framebuffer, a: Vec3, b: Vec3, c: Vec3) {
    line(framebuffer, a, b);
    line(framebuffer, b, c);
    line(framebuffer, c, a);
}

fn borde(a: Vec3, b: Vec3, x: f32, y: f32) -> f32 {
    (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
}

pub fn triangle(framebuffer: &mut Framebuffer, a: Vec3, b: Vec3, c: Vec3) {
    let area = borde(a, b, c.x, c.y);
    if area.abs() < 1e-6 {
        return;
    }
    let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil().min(framebuffer.width as f32 - 1.0) as i32;
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min(framebuffer.height as f32 - 1.0) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let w0 = borde(b, c, px, py) / area;
            let w1 = borde(c, a, px, py) / area;
            let w2 = borde(a, b, px, py) / area;
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                framebuffer.point(x, y, w0 * a.z + w1 * b.z + w2 * c.z);
            }
        }
    }
}
