use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Write};

pub fn guardar(framebuffer: &Framebuffer, ruta: &str) -> io::Result<()> {
    let (w, h) = (framebuffer.width, framebuffer.height);
    let fila = (w * 3 + 3) & !3;
    let tamano = 54 + fila * h;
    let mut datos = Vec::with_capacity(tamano);
    datos.extend_from_slice(b"BM");
    datos.extend_from_slice(&(tamano as u32).to_le_bytes());
    datos.extend_from_slice(&0u32.to_le_bytes());
    datos.extend_from_slice(&54u32.to_le_bytes());
    datos.extend_from_slice(&40u32.to_le_bytes());
    datos.extend_from_slice(&(w as i32).to_le_bytes());
    datos.extend_from_slice(&(h as i32).to_le_bytes());
    datos.extend_from_slice(&1u16.to_le_bytes());
    datos.extend_from_slice(&24u16.to_le_bytes());
    datos.extend_from_slice(&[0u8; 24]);
    for y in (0..h).rev() {
        for x in 0..w {
            let c = framebuffer.buffer[y * w + x];
            datos.extend_from_slice(&[(c & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, ((c >> 16) & 0xFF) as u8]);
        }
        datos.resize(datos.len() + fila - w * 3, 0);
    }
    File::create(ruta)?.write_all(&datos)
}
