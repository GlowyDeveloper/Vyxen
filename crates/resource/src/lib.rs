use std::{any::Any, path::Path};

mod color;
mod font;
mod texture;

pub use color::Color;
pub use font::{Font, GlyphMap, GlyphRect};
pub use texture::Texture;

/// Represents a resource that can be loaded.
pub trait Resource: Sized {
    fn as_any(&self) -> &dyn Any;
    fn load(data: &[u8]) -> anyhow::Result<Self>;
}

/// Loads a resource from a file path.
///
/// # Examples
/// ```rust, ignore
/// use vyxen_resource::{Texture, load_path};
///
/// let texture = load_path::<Texture>("test-img.png").unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the file could not be read or the data could not be parsed.
pub fn load_path<T: Resource>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    let data = std::fs::read(path)?;
    load_data::<T>(&data)
}

/// Loads a resource from a byte array.
///
/// # Examples
/// ```rust, ignore
/// use vyxen_resource::{Texture, load_bytes};
///
/// let bytes = include_bytes!("test-img.png");
///
/// let texture = load_bytes::<Texture>(bytes).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the data could not be parsed.
pub fn load_data<T: Resource>(data: &[u8]) -> anyhow::Result<T> {
    T::load(data)
}
