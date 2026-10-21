use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, BufWriter};

pub fn guardar_png(framebuffer: &Framebuffer, ruta: &str) -> io::Result<()> {
    let archivo = BufWriter::new(File::create(ruta)?);
    let mut encoder = png::Encoder::new(archivo, framebuffer.width as u32, framebuffer.height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(io::Error::other)?;
    let mut rgb = Vec::with_capacity(framebuffer.buffer.len() * 3);
    for &c in &framebuffer.buffer {
        rgb.extend_from_slice(&[(c >> 16) as u8, (c >> 8) as u8, c as u8]);
    }
    writer.write_image_data(&rgb).map_err(io::Error::other)
}
