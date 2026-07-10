/// Texture/Image type for sprites
///
/// # Examples
/// ## Raw bytes
/// ```rust, ignore
/// use vyxen_renderer::Texture;
///
/// let bytes = include_bytes!("test-img.png");
///
/// Texture::from_bytes(bytes, "image").unwrap();
/// ```
/// ## [Image crate](https://crates.io/crates/image)
/// ```rust, ignore
/// use vyxen_renderer::Texture;
///
/// let image = image::open("test-img.png").unwrap();
///
/// Texture::from_image(image, "image");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    image: image::DynamicImage,
    label: &'static str,
}

impl Texture {
    /// Sets the image of the texture with bytes
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen_renderer::Texture;
    ///
    /// let bytes = include_bytes!("test-img.png");
    ///
    /// Texture::from_bytes(bytes, "image").unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// May panic if the bytes are invalid.
    pub fn from_bytes(bytes: &[u8], label: &'static str) -> anyhow::Result<Self> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self { image, label })
    }

    /// Sets the image of the texture
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen_renderer::Texture;
    ///
    /// let image = image::open("test-img.png").unwrap();
    ///
    /// Texture::from_image(image, "image");
    /// ```
    ///
    /// # Panics
    ///
    /// May panic if the bytes are invalid.
    pub fn from_image(image: image::DynamicImage, label: &'static str) -> Self {
        Self { image, label }
    }

    /// Gets the image of the texture
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen_renderer::Texture;
    ///
    /// let image = image::open("test-img.png").unwrap();
    ///
    /// let texture = Texture::from_image(image.clone(), "image");
    ///
    /// let image_copy = texture.get_image();
    ///
    /// assert_eq!(&image, image_copy);
    /// ```
    pub fn get_image(&self) -> &image::DynamicImage {
        &self.image
    }

    /// Gets the label of the texture
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen_renderer::Texture;
    ///
    /// let image = image::open("test-img.png").unwrap();
    ///
    /// let texture = Texture::from_image(image, "image");
    ///
    /// let label = texture.get_label();
    ///
    /// assert_eq!(label, "image");
    /// ```
    pub fn get_label(&self) -> &'static str {
        self.label
    }
}
