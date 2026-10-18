use nalgebra_glm::Vec3;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Cara {
    pub indices: [usize; 3],
    pub color: Vec3,
}

pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub caras: Vec<Cara>,
}

fn cargar_materiales(ruta: &Path) -> HashMap<String, Vec3> {
    let mut materiales = HashMap::new();
    let Ok(texto) = fs::read_to_string(ruta) else {
        return materiales;
    };
    let mut actual = String::new();
    for linea in texto.lines() {
        let partes: Vec<&str> = linea.split_whitespace().collect();
        match partes.as_slice() {
            ["newmtl", nombre] => actual = nombre.to_string(),
            ["Kd", r, g, b] => {
                let color = Vec3::new(r.parse().unwrap_or(1.0), g.parse().unwrap_or(1.0), b.parse().unwrap_or(1.0));
                materiales.insert(actual.clone(), color);
            }
            _ => {}
        }
    }
    materiales
}

impl Obj {
    pub fn cargar(ruta: &str) -> Result<Obj, String> {
        let texto = fs::read_to_string(ruta).map_err(|e| format!("{}: {}", ruta, e))?;
        let carpeta = Path::new(ruta).parent().unwrap_or(Path::new("."));
        let mut materiales = HashMap::new();
        let mut color = Vec3::new(1.0, 1.0, 1.0);
        let mut vertices = Vec::new();
        let mut caras = Vec::new();

        for linea in texto.lines() {
            let partes: Vec<&str> = linea.split_whitespace().collect();
            match partes.first() {
                Some(&"mtllib") if partes.len() > 1 => materiales = cargar_materiales(&carpeta.join(partes[1])),
                Some(&"usemtl") if partes.len() > 1 => {
                    color = *materiales.get(partes[1]).unwrap_or(&Vec3::new(1.0, 1.0, 1.0));
                }
                Some(&"v") if partes.len() >= 4 => {
                    let p: Vec<f32> = partes[1..4].iter().map(|s| s.parse().unwrap_or(0.0)).collect();
                    vertices.push(Vec3::new(p[0], p[1], p[2]));
                }
                Some(&"f") if partes.len() >= 4 => {
                    let idx: Vec<usize> = partes[1..]
                        .iter()
                        .map(|s| s.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                        .collect();
                    for i in 1..idx.len() - 1 {
                        caras.push(Cara { indices: [idx[0], idx[i], idx[i + 1]], color });
                    }
                }
                _ => {}
            }
        }
        Ok(Obj { vertices, caras })
    }

    pub fn centro_y_tamano(&self) -> (Vec3, f32) {
        let mut min = Vec3::repeat(f32::MAX);
        let mut max = Vec3::repeat(f32::MIN);
        for v in &self.vertices {
            min = min.inf(v);
            max = max.sup(v);
        }
        ((min + max) * 0.5, (max - min).max())
    }
}
