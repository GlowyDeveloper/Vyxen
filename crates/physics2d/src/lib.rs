//! A 2D physics engine for Vyxen.

use vyxen_geometry::{Box, Circle, Polygon, Shape, ShapeType};

mod collision;
mod rigid;
mod soft;

pub use collision::{Collision, ContactPoints, Manifold};
pub use rigid::RigidBody;
pub use soft::{
    PointMass, SHAPE_DAMPING, SHAPE_RECONSTRUCTION_STIFFNESS, SPRING_DAMPING, SPRING_FORCE,
    SoftBody, Spring,
};

/// Generates a shape type from shape
///
/// # Examples
/// ```rust
/// use vyxen_geometry::Circle;
/// use vyxen_physics2d::shape_type_from_shape;
///
/// let rigid_type = shape_type_from_shape(Circle::new(2.0));
/// ```
pub fn shape_type_from_shape<T>(shape: T) -> ShapeType
where
    T: Shape,
{
    match () {
        _ if shape.as_any().is::<Circle>() => {
            if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
                ShapeType::Circle(*circle)
            } else {
                ShapeType::Circle(Circle::new(1.0))
            }
        }
        _ if shape.as_any().is::<Box>() => {
            if let Some(bx) = shape.as_any().downcast_ref::<Box>() {
                ShapeType::Box(*bx)
            } else {
                ShapeType::Circle(Circle::new(1.0))
            }
        }
        _ if shape.as_any().is::<Polygon>() => {
            if let Some(polygon) = shape.as_any().downcast_ref::<Polygon>() {
                if polygon.is_convex() {
                    ShapeType::Polygon(polygon.clone())
                } else {
                    ShapeType::Concave(Polygon::triangulate(polygon.get_vertices()))
                }
            } else {
                ShapeType::Circle(Circle::new(1.0))
            }
        }
        _ => ShapeType::Circle(Circle::new(1.0)),
    }
}
