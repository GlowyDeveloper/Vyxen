//! A 2D physics engine for Vyxen.

mod rigid;
mod collision;

pub use rigid::Rigid;
pub use collision::{Collision, ContactPoints, Manifold};