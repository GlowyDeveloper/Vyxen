#![forbid(unsafe_code)]
#![doc = include_str!("../../../README.md")]

/// Collection of example colors for quick use.
pub mod colors;
/// Common types to be quickly used by `vyxen::prelude::*`.
pub mod prelude;

/// Core math types and utilities.
pub mod math {
    pub use vyxen_math::*;
}

/// Geometry types and helpers.
pub mod geometry {
    pub use vyxen_geometry::*;
}

/// 2D collision and physics types.
pub mod physics2d {
    pub use vyxen_physics2d::*;
}

/// The whole engine renderer.
pub mod renderer {
    pub use vyxen_renderer::*;
}

/// Input system and input types.
pub mod input {
    pub use vyxen_input::*;
}

pub use vyxen_core::*;
