use std::{any::Any, path::Path};

use crate::error::Error;

pub mod color;
pub mod font;
pub mod texture;

/// Represents a resource that can be loaded.
pub trait Resource: Sized {
    fn as_any(&self) -> &dyn Any;
    fn load(data: &[u8]) -> Result<Self, Error>;
}

/// Loads a resource from a file path.
///
/// # Examples
/// ```rust, ignore
/// use vyxen::{Texture, load_path};
///
/// let texture = load_path::<Texture>("test-img.png").unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the file could not be read or the data could not be parsed.
#[allow(unused)]
pub fn load_path<T: Resource>(path: impl AsRef<Path>) -> Result<T, Error> {
    #[cfg(target_arch = "wasm32")]
    return Err(Error::NoFileSystem);

    let data = std::fs::read(path)?;
    load_data::<T>(&data)
}

/// Loads a resource from a byte array.
///
/// # Examples
/// ```rust, ignore
/// use vyxen::{Texture, load_bytes};
///
/// let bytes = include_bytes!("test-img.png");
///
/// let texture = load_bytes::<Texture>(bytes).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the data could not be parsed.
pub fn load_data<T: Resource>(data: &[u8]) -> Result<T, Error> {
    T::load(data)
}
