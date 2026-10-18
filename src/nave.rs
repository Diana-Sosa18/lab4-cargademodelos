use crate::mesh::Mesh;
use nalgebra_glm::Vec3;

pub const CASCO: usize = 0;
pub const CABINA: usize = 1;
pub const MOTOR: usize = 2;
pub const DETALLE: usize = 3;

pub const MATERIALES: [(&str, [f32; 3]); 4] = [
    ("Casco", [0.72, 0.74, 0.78]),
    ("Cabina", [0.1, 0.45, 0.85]),
    ("Motor", [1.0, 0.45, 0.05]),
    ("Detalle", [0.75, 0.12, 0.12]),
];

fn v(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

fn casco(m: &mut Mesh) {
    let todos: Vec<usize> = (0..m.vertices.len()).collect();
    m.escalar(&todos, v(0.9, 1.4, 0.45), Vec3::zeros());
    let frente = m.buscar(v(0.0, 1.0, 0.0), |_| true);
    m.inset(frente, 0.08);
    m.extruir(frente, 1.1, v(0.75, 1.0, 0.7), v(0.0, 0.0, -0.05));
    m.extruir(frente, 0.9, v(0.45, 1.0, 0.5), v(0.0, 0.0, -0.08));
    let punta = m.subdividir(frente);
    m.mover(&[punta], v(0.0, 0.35, 0.0));
    let atras = m.buscar(v(0.0, -1.0, 0.0), |_| true);
    m.extruir(atras, 0.5, v(1.1, 1.0, 1.15), Vec3::zeros());
}

fn ala(m: &mut Mesh, lado: f32) {
    let f = m.buscar(v(lado, 0.0, 0.0), |c| c.y > -1.4 && c.y < 1.4);
    m.extruir(f, 0.25, v(1.0, 0.75, 0.6), v(0.0, -0.2, -0.05));
    m.extruir(f, 1.6, v(1.0, 0.45, 0.25), v(0.0, -0.9, 0.0));
    m.extruir(f, 0.05, v(1.0, 1.8, 2.6), Vec3::zeros());
    m.extruir(f, 0.35, v(1.0, 1.0, 1.0), Vec3::zeros());
    m.extruir(f, 0.05, v(1.0, 0.7, 0.7), Vec3::zeros());

    let tope = m.buscar(v(0.0, 0.0, 1.0), |c| c.x * lado > 2.5);
    let base = m.centro(tope);
    m.material = DETALLE;
    m.extruir(tope, 0.6, v(0.5, 0.45, 1.0), v(0.0, -0.35, 0.0));
    let medio = m.caras[tope].indices.clone();
    m.extruir(tope, 0.05, v(0.8, 0.8, 1.0), v(0.0, -0.05, 0.0));
    let mut aleta = medio;
    aleta.extend(m.caras[tope].indices.iter());
    m.rotar(&aleta, v(0.0, 1.0, 0.0), lado * 0.2, base);

    let tobera = m.buscar(v(0.0, -1.0, 0.0), |c| c.x * lado > 2.5);
    m.material = CASCO;
    m.inset(tobera, 0.04);
    m.material = MOTOR;
    m.extruir(tobera, -0.12, v(0.85, 1.0, 0.85), Vec3::zeros());
    m.material = CASCO;
}

fn cabina(m: &mut Mesh) {
    let techo = m.buscar(v(0.0, 0.0, 1.0), |c| c.y > -1.4 && c.y < 1.4 && c.x.abs() < 0.5);
    m.inset(techo, 0.15);
    m.material = CABINA;
    m.extruir(techo, 0.28, v(0.7, 0.6, 1.0), v(0.0, 0.4, 0.0));
    m.biselar(techo, 0.07);
    m.material = CASCO;
}

fn motores(m: &mut Mesh) {
    let atras = m.buscar(v(0.0, -1.0, 0.0), |c| c.x.abs() < 0.5);
    m.inset(atras, 0.12);
    m.material = MOTOR;
    m.extruir(atras, -0.3, v(0.85, 1.0, 0.85), Vec3::zeros());
    m.inset(atras, 0.08);
    m.material = CASCO;
}

fn aleta_dorsal(m: &mut Mesh) {
    let tope = m.buscar(v(0.0, 0.0, 1.0), |c| c.y < -1.4 && c.x.abs() < 0.6);
    m.material = DETALLE;
    m.extruir(tope, 0.06, v(0.18, 0.9, 1.0), Vec3::zeros());
    m.extruir(tope, 1.1, v(0.6, 0.45, 1.0), v(0.0, -0.5, 0.0));
    m.material = CASCO;
}

pub fn construir() -> Mesh {
    let mut m = Mesh::cubo(2.0);
    casco(&mut m);
    ala(&mut m, 1.0);
    ala(&mut m, -1.0);
    cabina(&mut m);
    motores(&mut m);
    aleta_dorsal(&mut m);
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nave_cerrada_y_sin_triangulos_degenerados() {
        let m = construir();
        assert_eq!(m.bordes_abiertos(), 0);
        for (t, _) in m.triangulos() {
            let [a, b, c] = t.map(|i| m.vertices[i]);
            assert!((b - a).cross(&(c - a)).norm() > 1e-6);
        }
        for material in [CASCO, CABINA, MOTOR, DETALLE] {
            assert!(m.caras.iter().any(|c| c.material == material));
        }
    }
}
