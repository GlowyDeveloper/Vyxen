#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    image: image::DynamicImage,
    label: &'static str,
}

impl Texture {
    pub fn from_bytes(bytes: &[u8], label: &'static str) -> anyhow::Result<Self> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self { image, label })
    }

    pub fn from_image(image: image::DynamicImage, label: &'static str) -> Self {
        Self { image, label }
    }

    pub fn get_image(&self) -> &image::DynamicImage {
        &self.image
    }

    pub fn get_label(&self) -> &'static str {
        self.label
    }
}
