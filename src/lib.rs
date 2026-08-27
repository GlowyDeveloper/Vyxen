#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

/// Collection of example colors for quick use.
pub mod colors;
/// Input system and input types.
pub mod inputs;
/// 2D collision and physics types.
pub mod physics2d;
/// Common types to be quickly used by `vyxen::prelude::*`.
pub mod prelude;
/// Functions and enums for shape types.
pub mod shape_type;
/// UI types and utilities.
pub mod ui;

pub(crate) mod geometry;
pub(crate) mod node;
pub(crate) mod renderer;
pub(crate) mod resource;

mod game;
mod math;
mod scene;

pub use game::{Context, Event, Game};
pub use geometry::{AABB, Box, Circle, Polygon};
pub use math::{Matrix4, Random, Transform, Vector2, is_nearly_equal};
pub use node::{Collider, Node};
pub use renderer::{Camera, DrawType, RenderMode, Sprite, WindowConfig};
pub use resource::{color::Color, font::Font, load_data, load_path, texture::Texture};
pub use scene::Scene;
