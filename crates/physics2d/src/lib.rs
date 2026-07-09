//! A 2D physics engine for Vyxen.

mod collision;
mod rigid;
mod soft;

pub use collision::{Collision, ContactPoints, Manifold};
pub use rigid::RigidBody;
pub use soft::{
    PointMass, SHAPE_DAMPING, SHAPE_RECONSTRUCTION_STIFFNESS, SPRING_DAMPING, SPRING_FORCE,
    SoftBody, Spring,
};
