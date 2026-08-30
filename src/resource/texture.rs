use crate::{
    Vector2,
    resource::{Resource, color::Color},
};
use png::{ColorType, Decoder, Transformations};
use std::{any::Any, io::Cursor};
use zune_jpeg::zune_core::{bytestream::ZCursor, colorspace::ColorSpace, options::DecoderOptions};

/// Texture/Image type
///
/// # Examples
/// ## Raw bytes
/// ```rust, ignore
/// use vyxen::{Texture, load_bytes};
///
/// let bytes = include_bytes!("test-img.png");
///
/// let texture = load_bytes::<Texture>(bytes).unwrap();
/// ```
/// ## Path
/// ```rust, ignore
/// use vyxen::{Texture, load_path};
///
/// let texture = load_path::<Texture>("test-img.png").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    dim: Vector2,
    rgba: Vec<u8>,
    tint: Option<Color>,
}

impl Resource for Texture {
    fn load(data: &[u8]) -> anyhow::Result<Self> {
        if data.len() < 8 {
            return Err(anyhow::anyhow!("Invalid data length"));
        }

        if data.starts_with(b"\x89PNG\r\n\x1a\n") {
            let mut decoder = Decoder::new(Cursor::new(data));
            decoder.set_transformations(Transformations::EXPAND | Transformations::STRIP_16);

            let mut reader = decoder.read_info()?;

            let mut buf = vec![
                0;
                reader
                    .output_buffer_size()
                    .ok_or_else(|| anyhow::anyhow!("Unknown PNG output size"))?
            ];

            let info = reader.next_frame(&mut buf)?;

            let width = info.width;
            let height = info.height;

            buf.truncate(info.buffer_size());

            let rgba = match info.color_type {
                ColorType::Rgba => buf,

                ColorType::Rgb => {
                    let mut out = Vec::with_capacity((width * height * 4) as usize);

                    for rgb in buf.chunks_exact(3) {
                        out.push(rgb[0]);
                        out.push(rgb[1]);
                        out.push(rgb[2]);
                        out.push(255);
                    }

                    out
                }

                ColorType::Grayscale => {
                    let mut out = Vec::with_capacity((width * height * 4) as usize);

                    for &g in &buf {
                        out.push(g);
                        out.push(g);
                        out.push(g);
                        out.push(255);
                    }

                    out
                }

                ColorType::GrayscaleAlpha => {
                    let mut out = Vec::with_capacity((width * height * 4) as usize);

                    for ga in buf.chunks_exact(2) {
                        let g = ga[0];
                        let a = ga[1];

                        out.push(g);
                        out.push(g);
                        out.push(g);
                        out.push(a);
                    }

                    out
                }

                ColorType::Indexed => {
                    anyhow::bail!("PNG should've been expanded by EXPAND transformation")
                }
            };

            Ok(Self {
                dim: Vector2 {
                    x: width as f32,
                    y: height as f32,
                },
                rgba,
                tint: None,
            })
        } else if data.starts_with(b"\xff\xd8\xff") {
            let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);
            let mut decoder = zune_jpeg::JpegDecoder::new_with_options(ZCursor::new(data), options);
            let pixels = decoder.decode().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            let (width, height) = decoder.dimensions().unwrap();

            Ok(Self {
                dim: Vector2 {
                    x: width as f32,
                    y: height as f32,
                },
                rgba: pixels,
                tint: None,
            })
        } else {
            Err(anyhow::anyhow!("Unsupported image format"))
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Texture {
    /// Creates a new texture from raw RGBA data.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// use vyxen::Texture;
    ///
    /// let texture = Texture::from_raw(Vector2::new(2.0, 1.0), vec![0,0,0,255, 0,0,0,255]);
    ///
    /// assert_eq!(texture.get_dimensions(), Vector2::new(2.0, 1.0));
    /// assert_eq!(texture.get_rgba(), &[0,0,0,255, 0,0,0,255]);
    /// ```
    pub fn from_raw(dim: Vector2, rgba: Vec<u8>) -> Self {
        Self {
            dim,
            rgba,
            tint: None,
        }
    }

    /// Returns the texture's dimensions.
    pub fn get_dimensions(&self) -> Vector2 {
        self.dim
    }

    /// Returns the texture's RGBA data.
    pub fn get_rgba(&self) -> &[u8] {
        &self.rgba
    }

    /// Sets the texture's tint color.
    pub fn set_tint(&mut self, color: Color) {
        self.tint = Some(color);
    }

    /// Returns the texture's tint color.
    pub fn get_tint(&self) -> Option<Color> {
        self.tint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_from_bytes() {
        let bytes = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x56, 0x28, 0xB5, 0xBF, 0x00, 0x00, 0x00, 0x01, 0x73, 0x52, 0x47, 0x42, 0x00,
            0xAE, 0xCE, 0x1C, 0xE9, 0x00, 0x00, 0x00, 0x04, 0x67, 0x41, 0x4D, 0x41, 0x00, 0x00,
            0xB1, 0x8F, 0x0B, 0xFC, 0x61, 0x05, 0x00, 0x00, 0x00, 0x09, 0x70, 0x48, 0x59, 0x73,
            0x00, 0x00, 0x0E, 0xC3, 0x00, 0x00, 0x0E, 0xC3, 0x01, 0xC7, 0x6F, 0xA8, 0x64, 0x00,
            0x00, 0x00, 0x19, 0x74, 0x45, 0x58, 0x74, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72,
            0x65, 0x00, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x2E, 0x4E, 0x45, 0x54, 0x20, 0x35, 0x2E,
            0x31, 0x2E, 0x31, 0x31, 0x8A, 0x08, 0x16, 0xCE, 0x00, 0x00, 0x00, 0xB8, 0x65, 0x58,
            0x49, 0x66, 0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00, 0x05, 0x00, 0x1A, 0x01,
            0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4A, 0x00, 0x00, 0x00, 0x1B, 0x01, 0x05, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x52, 0x00, 0x00, 0x00, 0x28, 0x01, 0x03, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x31, 0x01, 0x02, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x5A, 0x00, 0x00, 0x00, 0x69, 0x87, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x6C, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x60, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x2E,
            0x4E, 0x45, 0x54, 0x20, 0x35, 0x2E, 0x31, 0x2E, 0x31, 0x31, 0x00, 0x00, 0x03, 0x00,
            0x00, 0x90, 0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x30, 0x32, 0x33, 0x30, 0x01, 0xA0,
            0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0xA0, 0x04, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x96, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x00, 0x00, 0x52, 0x39, 0x38, 0x00, 0x02, 0x00,
            0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x30, 0x31, 0x30, 0x30, 0x00, 0x00, 0x00, 0x00,
            0x06, 0x35, 0xD4, 0x73, 0xB2, 0x8F, 0x72, 0x3B, 0x00, 0x00, 0x00, 0x14, 0x49, 0x44,
            0x41, 0x54, 0x18, 0x57, 0x63, 0xFC, 0xFF, 0xFF, 0xFF, 0x7F, 0x06, 0x28, 0x60, 0x82,
            0x31, 0x30, 0x38, 0x00, 0x92, 0x0A, 0x04, 0x02, 0xEC, 0xA4, 0x99, 0xDE, 0x00, 0x00,
            0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];

        let expected_bytes = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];

        let texture = Texture::load(&bytes).unwrap();
        assert_eq!(texture.get_dimensions(), Vector2 { x: 3.0, y: 3.0 });
        assert_eq!(texture.get_rgba(), &expected_bytes);
    }

    #[test]
    fn test_from_raw() {
        let expected_bytes = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];

        let texture = Texture::from_raw(Vector2 { x: 3.0, y: 3.0 }, expected_bytes.to_vec());
        assert_eq!(texture.get_dimensions(), Vector2 { x: 3.0, y: 3.0 });
        assert_eq!(texture.get_rgba(), &expected_bytes);
    }

    #[test]
    fn test_magic_numbers() {
        let png_chunk = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
        ];
        let png = Texture::load(&png_chunk);
        assert_ne!(png.err().unwrap().to_string(), "Unsupported image format");

        let jpeg_chunk = [
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01,
            0x00, 0x60,
        ];
        let jpeg = Texture::load(&jpeg_chunk);
        assert_ne!(jpeg.err().unwrap().to_string(), "Unsupported image format");

        let webp_chunk = [
            0x52, 0x49, 0x46, 0x46, 0x88, 0x0D, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50, 0x56, 0x50,
            0x38, 0x58,
        ];
        let webp = Texture::load(&webp_chunk);
        assert_eq!(webp.err().unwrap().to_string(), "Unsupported image format");
    }
}
