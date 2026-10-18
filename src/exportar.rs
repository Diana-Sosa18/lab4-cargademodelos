use crate::mesh::Mesh;
use crate::nave::MATERIALES;
use nalgebra_glm::Vec3;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::Path;

fn a_obj(p: Vec3) -> Vec3 {
    Vec3::new(p.x, p.z, -p.y)
}

pub fn guardar(mesh: &Mesh, ruta_obj: &str) -> io::Result<(usize, usize)> {
    let ruta = Path::new(ruta_obj);
    let nombre_mtl = ruta.with_extension("mtl");
    let archivo_mtl = nombre_mtl.file_name().unwrap().to_string_lossy().to_string();

    let mut mtl = String::new();
    for (nombre, [r, g, b]) in MATERIALES {
        writeln!(mtl, "newmtl {}\nKa 1.000000 1.000000 1.000000\nKd {:.6} {:.6} {:.6}\nKs 0.000000 0.000000 0.000000\nd 1.000000\nillum 1\n", nombre, r, g, b).unwrap();
    }
    fs::write(&nombre_mtl, mtl)?;

    let vertices: Vec<Vec3> = mesh.vertices.iter().map(|&p| a_obj(p)).collect();
    let mut min = Vec3::repeat(f32::MAX);
    let mut max = Vec3::repeat(f32::MIN);
    for p in &vertices {
        min = min.inf(p);
        max = max.sup(p);
    }
    let rango = (max - min).sup(&Vec3::repeat(1e-6));

    let mut obj = String::new();
    writeln!(obj, "mtllib {}\no Nave", archivo_mtl).unwrap();
    for p in &vertices {
        writeln!(obj, "v {:.6} {:.6} {:.6}", p.x, p.y, p.z).unwrap();
    }
    for p in &vertices {
        writeln!(obj, "vt {:.6} {:.6}", (p.x - min.x) / rango.x, (p.z - min.z) / rango.z).unwrap();
    }

    let triangulos = mesh.triangulos();
    for (t, _) in &triangulos {
        let [a, b, c] = t.map(|i| vertices[i]);
        let n = (b - a).cross(&(c - a)).normalize();
        writeln!(obj, "vn {:.4} {:.4} {:.4}", n.x, n.y, n.z).unwrap();
    }

    for (material, (nombre, _)) in MATERIALES.iter().enumerate() {
        writeln!(obj, "usemtl {}", nombre).unwrap();
        for (k, (t, m)) in triangulos.iter().enumerate() {
            if *m == material {
                let [a, b, c] = t.map(|i| i + 1);
                writeln!(obj, "f {a}/{a}/{n} {b}/{b}/{n} {c}/{c}/{n}", n = k + 1).unwrap();
            }
        }
    }
    fs::write(ruta, obj)?;
    Ok((vertices.len(), triangulos.len()))
}
