//! **A godot-style game engine written in Rust**
//!
//! ## Creating a world
//!
//! Vyxen doesn't use scenes (yet).
//!
//! Vyxen uses worlds to hold all nodes.
//!
//! ```rust
//! use vyxen::World;
//!
//! let mut world = World::new();
//! ```
//!
//! ## Adding nodes
//!
//! Nodes are the main focus of Vyxen.
//!
//! Nodes are generic. There's no pre-made nodes.
//!
//! ```rust
//! use vyxen::{Node, World};
//!
//! let mut world = World::new();
//!
//! let mut node = Node::new("Foo".to_string());
//! world.add_node(node);
//! ```
//!
//! ## Components
//!
//! Components are used to add behavior and data, such as colliders, to a node.
//!
//! The currently implemented components are:
//!  - Collider
//!  - RigidBody
//!  - SoftBody
//!
//! ```rust
//! use vyxen::{Node, Collider, geometry::Circle};
//!
//! let mut node = Node::new("Foo".to_string());
//! let collider = Collider::new(Circle::new(1.0));
//!
//! node.add_component(collider);
//! ```
//!
//! ## Scripts
//!
//! Scripts let you customize node behavior.
//!
//! The overridable methods are:
//!  - on_ready
//!  - process
//!  - physics_process
//!  - on_collision
//!
//! ```rust
//! use vyxen::{Node, Script, World};
//!
//! struct ExampleScript;
//! impl Script for ExampleScript {
//!     fn physics_process(&mut self, _: &mut Node, _: &mut World, _: f32) {
//!         println!("Processing...");
//!     }
//! }
//!
//! let mut node = Node::new("Foo".to_string());
//! node.set_script(ExampleScript);
//! ```
//!
//! ## License
//!
//! This project is licensed under either of
//!
//!  - Apache License, Version 2.0, (<https://www.apache.org/licenses/LICENSE-2.0>)
//!  - MIT license (<https://opensource.org/license/mit>)
//!
//! at your option.

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

/// The whole engine renderer
pub mod renderer {
    pub use vyxen_renderer::*;
}

pub use vyxen_core::*;
