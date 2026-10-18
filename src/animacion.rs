use crate::framebuffer::Framebuffer;
use gif::{Encoder, Frame, Repeat};
use std::fs::File;
use std::io;

pub struct Gif {
    encoder: Encoder<File>,
    ancho: u16,
    alto: u16,
}

impl Gif {
    pub fn crear(ruta: &str, ancho: usize, alto: usize) -> io::Result<Gif> {
        let archivo = File::create(ruta)?;
        let mut encoder = Encoder::new(archivo, ancho as u16, alto as u16, &[]).map_err(io::Error::other)?;
        encoder.set_repeat(Repeat::Infinite).map_err(io::Error::other)?;
        Ok(Gif { encoder, ancho: ancho as u16, alto: alto as u16 })
    }

    pub fn agregar(&mut self, framebuffer: &Framebuffer, centesimas: u16) -> io::Result<()> {
        let mut rgb = Vec::with_capacity(framebuffer.buffer.len() * 3);
        for &c in &framebuffer.buffer {
            rgb.extend_from_slice(&[(c >> 16) as u8, (c >> 8) as u8, c as u8]);
        }
        let mut cuadro = Frame::from_rgb_speed(self.ancho, self.alto, &rgb, 10);
        cuadro.delay = centesimas;
        self.encoder.write_frame(&cuadro).map_err(io::Error::other)
    }
}
