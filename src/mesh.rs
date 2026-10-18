use nalgebra_glm::{self as glm, Vec3};
use std::collections::HashMap;

pub struct Cara {
    pub indices: Vec<usize>,
    pub material: usize,
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub caras: Vec<Cara>,
    pub material: usize,
}

impl Mesh {
    pub fn cubo(tamano: f32) -> Mesh {
        let h = tamano / 2.0;
        let vertices = vec![
            Vec3::new(-h, -h, -h),
            Vec3::new(h, -h, -h),
            Vec3::new(h, h, -h),
            Vec3::new(-h, h, -h),
            Vec3::new(-h, -h, h),
            Vec3::new(h, -h, h),
            Vec3::new(h, h, h),
            Vec3::new(-h, h, h),
        ];
        let caras = [
            [0, 3, 2, 1],
            [4, 5, 6, 7],
            [0, 1, 5, 4],
            [2, 3, 7, 6],
            [0, 4, 7, 3],
            [1, 2, 6, 5],
        ]
        .iter()
        .map(|c| Cara { indices: c.to_vec(), material: 0 })
        .collect();
        Mesh { vertices, caras, material: 0 }
    }

    pub fn centro(&self, cara: usize) -> Vec3 {
        let idx = &self.caras[cara].indices;
        idx.iter().fold(Vec3::zeros(), |s, &i| s + self.vertices[i]) / idx.len() as f32
    }

    pub fn normal(&self, cara: usize) -> Vec3 {
        let idx = &self.caras[cara].indices;
        let mut n = Vec3::zeros();
        for k in 0..idx.len() {
            let a = self.vertices[idx[k]];
            let b = self.vertices[idx[(k + 1) % idx.len()]];
            n += Vec3::new((a.y - b.y) * (a.z + b.z), (a.z - b.z) * (a.x + b.x), (a.x - b.x) * (a.y + b.y));
        }
        if n.norm() > 0.0 { n.normalize() } else { n }
    }

    pub fn buscar(&self, direccion: Vec3, filtro: impl Fn(Vec3) -> bool) -> usize {
        let d = direccion.normalize();
        (0..self.caras.len())
            .filter(|&c| self.normal(c).dot(&d) > 0.9 && filtro(self.centro(c)))
            .max_by(|&a, &b| self.centro(a).dot(&d).partial_cmp(&self.centro(b).dot(&d)).unwrap())
            .expect("no se encontro la cara")
    }

    pub fn mover(&mut self, verts: &[usize], desplazamiento: Vec3) {
        for &v in verts {
            self.vertices[v] += desplazamiento;
        }
    }

    pub fn escalar(&mut self, verts: &[usize], escala: Vec3, centro: Vec3) {
        for &v in verts {
            self.vertices[v] = centro + (self.vertices[v] - centro).component_mul(&escala);
        }
    }

    pub fn rotar(&mut self, verts: &[usize], eje: Vec3, angulo: f32, centro: Vec3) {
        let r = glm::rotate(&glm::Mat4::identity(), angulo, &eje);
        for &v in verts {
            let p = self.vertices[v] - centro;
            let q = r * glm::vec4(p.x, p.y, p.z, 1.0);
            self.vertices[v] = centro + Vec3::new(q.x, q.y, q.z);
        }
    }

    fn duplicar_anillo(&mut self, cara: usize, nuevos: Vec<Vec3>) -> Vec<usize> {
        let viejos = self.caras[cara].indices.clone();
        let inicio = self.vertices.len();
        self.vertices.extend(nuevos);
        let anillo: Vec<usize> = (inicio..self.vertices.len()).collect();
        for k in 0..viejos.len() {
            let s = (k + 1) % viejos.len();
            self.caras.push(Cara {
                indices: vec![viejos[k], viejos[s], anillo[s], anillo[k]],
                material: self.material,
            });
        }
        self.caras[cara].indices = anillo.clone();
        self.caras[cara].material = self.material;
        anillo
    }

    pub fn extruir(&mut self, cara: usize, distancia: f32, escala: Vec3, desplazamiento: Vec3) -> usize {
        let n = self.normal(cara);
        let copia: Vec<Vec3> = self.caras[cara].indices.iter().map(|&i| self.vertices[i]).collect();
        let anillo = self.duplicar_anillo(cara, copia);
        self.mover(&anillo, n * distancia + desplazamiento);
        let c = self.centro(cara);
        self.escalar(&anillo, escala, c);
        cara
    }

    pub fn inset(&mut self, cara: usize, grosor: f32) -> usize {
        let n = self.normal(cara);
        let idx = self.caras[cara].indices.clone();
        let total = idx.len();
        let interiores: Vec<Vec3> = (0..total)
            .map(|k| {
                let p = self.vertices[idx[k]];
                let antes = self.vertices[idx[(k + total - 1) % total]];
                let despues = self.vertices[idx[(k + 1) % total]];
                let in1 = n.cross(&(p - antes)).normalize();
                let in2 = n.cross(&(despues - p)).normalize();
                let bisectriz = (in1 + in2).normalize();
                p + bisectriz * (grosor / bisectriz.dot(&in1).max(0.2))
            })
            .collect();
        self.duplicar_anillo(cara, interiores);
        cara
    }

    fn vecinos(&self, v: usize) -> Vec<usize> {
        let mut r = Vec::new();
        for c in &self.caras {
            if let Some(k) = c.indices.iter().position(|&i| i == v) {
                let t = c.indices.len();
                for w in [c.indices[(k + 1) % t], c.indices[(k + t - 1) % t]] {
                    if !r.contains(&w) {
                        r.push(w);
                    }
                }
            }
        }
        r
    }

    pub fn biselar(&mut self, cara: usize, ancho: f32) -> usize {
        let exteriores = self.caras[cara].indices.clone();
        let destinos: Vec<Option<usize>> = exteriores
            .iter()
            .map(|&v| self.vecinos(v).into_iter().find(|w| !exteriores.contains(w)))
            .collect();
        self.inset(cara, ancho);
        for (k, &v) in exteriores.iter().enumerate() {
            if let Some(w) = destinos[k] {
                let d = (self.vertices[w] - self.vertices[v]).normalize();
                self.vertices[v] += d * ancho;
            }
        }
        cara
    }

    pub fn subdividir(&mut self, cara: usize) -> usize {
        let idx = self.caras[cara].indices.clone();
        assert_eq!(idx.len(), 4);
        let centro = self.centro(cara);
        let mut medios = Vec::new();
        for k in 0..4 {
            let (a, b) = (idx[k], idx[(k + 1) % 4]);
            let m = self.vertices.len();
            self.vertices.push((self.vertices[a] + self.vertices[b]) * 0.5);
            for c in self.caras.iter_mut() {
                let t = c.indices.len();
                if let Some(p) = (0..t).find(|&p| c.indices[p] == b && c.indices[(p + 1) % t] == a) {
                    c.indices.insert(p + 1, m);
                }
            }
            medios.push(m);
        }
        let ce = self.vertices.len();
        self.vertices.push(centro);
        let material = self.caras[cara].material;
        self.caras[cara].indices = vec![idx[0], medios[0], ce, medios[3]];
        for k in 1..4 {
            self.caras.push(Cara { indices: vec![idx[k], medios[k], ce, medios[k - 1]], material });
        }
        ce
    }

    pub fn triangulos(&self) -> Vec<([usize; 3], usize)> {
        let mut r = Vec::new();
        for c in &self.caras {
            let t = c.indices.len();
            let inicio = (0..t)
                .max_by(|&a, &b| self.angulo(&c.indices, a).partial_cmp(&self.angulo(&c.indices, b)).unwrap())
                .unwrap();
            let rot: Vec<usize> = (0..t).map(|k| c.indices[(inicio + k) % t]).collect();
            for k in 1..t - 1 {
                r.push(([rot[0], rot[k], rot[k + 1]], c.material));
            }
        }
        r
    }

    fn angulo(&self, idx: &[usize], k: usize) -> f32 {
        let t = idx.len();
        let p = self.vertices[idx[k]];
        let a = (self.vertices[idx[(k + t - 1) % t]] - p).normalize();
        let b = (self.vertices[idx[(k + 1) % t]] - p).normalize();
        a.dot(&b).clamp(-1.0, 1.0).acos()
    }

    pub fn bordes_abiertos(&self) -> usize {
        let mut conteo: HashMap<(usize, usize), i32> = HashMap::new();
        for c in &self.caras {
            let t = c.indices.len();
            for k in 0..t {
                let (a, b) = (c.indices[k], c.indices[(k + 1) % t]);
                *conteo.entry((a.min(b), a.max(b))).or_insert(0) += if a < b { 1 } else { -1 };
            }
        }
        conteo.values().filter(|&&v| v != 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubo_cerrado_con_normales_hacia_afuera() {
        let m = Mesh::cubo(2.0);
        assert_eq!(m.bordes_abiertos(), 0);
        for c in 0..m.caras.len() {
            assert!(m.normal(c).dot(&m.centro(c)) > 0.0);
        }
    }

    #[test]
    fn herramientas_mantienen_la_malla_cerrada() {
        let mut m = Mesh::cubo(2.0);
        let frente = m.buscar(Vec3::new(0.0, 0.0, 1.0), |_| true);
        m.inset(frente, 0.2);
        m.extruir(frente, 1.0, Vec3::new(0.5, 0.5, 1.0), Vec3::zeros());
        m.subdividir(frente);
        let arriba = m.buscar(Vec3::new(0.0, 1.0, 0.0), |_| true);
        m.extruir(arriba, 0.5, Vec3::new(1.0, 1.0, 1.0), Vec3::zeros());
        m.biselar(arriba, 0.1);
        let atras = m.buscar(Vec3::new(0.0, 0.0, -1.0), |c| c.y < 0.9);
        m.inset(atras, 0.2);
        m.extruir(atras, -0.3, Vec3::new(1.0, 1.0, 1.0), Vec3::zeros());
        assert!((m.centro(atras).z + 0.7).abs() < 1e-4);
        assert!(m.centro(arriba).y > 1.49);
        assert_eq!(m.bordes_abiertos(), 0);
        for (t, _) in m.triangulos() {
            let [a, b, c] = t.map(|i| m.vertices[i]);
            assert!((b - a).cross(&(c - a)).norm() > 1e-6);
        }
    }
}
