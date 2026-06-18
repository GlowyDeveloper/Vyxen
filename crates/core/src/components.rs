use std::any::Any;

use vyxen_geometry::{AABB, Shape, ShapeType};
use vyxen_math::Vector2;
use vyxen_physics2d::Rigid;

/// Component trait for attaching arbitrary data to `Node`s.
pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Component for Rigid {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

/// Allows nodes to collide with eachother.
/// 
/// # Examples
/// ```rust
/// use vyxen_core::{Node, components::Collider};
/// use vyxen_geometry::Circle;
/// 
/// let mut node = Node::new("Foo".to_string());
/// node.add_component(Collider::new(Circle::new(5.0)));
/// ```
pub struct Collider {
    hitbox: ShapeType,
    aabb: AABB,
    old_pos: Vector2,
    old_rot: f32,
    aabb_initialized: bool,
}

impl Collider {
    /// Creates a collider
    /// 
    /// # Examples
    /// ## Circles:
    /// ```rust
    /// use vyxen_core::{Node, components::Collider};
    /// use vyxen_geometry::Circle;
    /// 
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Circle::new(5.0)));
    /// ```
    /// ## Boxes:
    /// ```rust
    /// use vyxen_core::{Node, components::Collider};
    /// use vyxen_geometry::Box;
    /// 
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Box::new(5.0, 5.0)));
    /// ```
    /// ## Polygons:
    /// ```rust
    /// use vyxen_core::{Node, components::Collider};
    /// use vyxen_geometry::Polygon;
    /// use vyxen_math::Vector2;
    /// 
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// 
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Polygon::new(&[v1, v2, v3])));
    /// ```
    pub fn new<T>(hitbox: T) -> Self
    where
        T: Shape
    {
        Self {
            hitbox: Rigid::generate_shape_type_from_shape(hitbox),
            aabb: AABB::new_from_uncalculated(std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN),
            old_pos: Vector2::zero(),
            old_rot: 0.0,
            aabb_initialized: false,
        }
    }

    /// Gets the hitbox
    /// 
    /// For a mutable reference, refer to `get_hitbox_mut()`
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Node, components::Collider};
    /// use vyxen_geometry::Circle;
    /// 
    /// let mut node = Node::new("Foo".to_string());
    /// let collider = Collider::new(Circle::new(5.0));
    /// node.add_component(collider);
    /// 
    /// let collider = node.get_component::<Collider>().unwrap();
    /// 
    /// let hitbox = collider.get_hitbox();
    /// ```
    pub fn get_hitbox(&self) -> &ShapeType {
        &self.hitbox
    }

    /// Gets the hitbox as a mutable reference
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Node, components::Collider};
    /// use vyxen_geometry::Circle;
    /// 
    /// let mut node = Node::new("Foo".to_string());
    /// let collider = Collider::new(Circle::new(5.0));
    /// node.add_component(collider);
    /// 
    /// let mut collider = node.get_component_mut::<Collider>().unwrap();
    /// 
    /// let hitbox = collider.get_hitbox_mut();
    /// ```
    pub fn get_hitbox_mut(&mut self) -> &mut ShapeType {
        &mut self.hitbox
    }

    /// Gets the hitbox as a mutable reference
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Node, components::Collider};
    /// use vyxen_geometry::Circle;
    /// use vyxen_math::Vector2;
    /// 
    /// let mut node = Node::new("Foo".to_string());
    /// let collider = Collider::new(Circle::new(5.0));
    /// node.add_component(collider);
    /// 
    /// let mut collider = node.get_component_mut::<Collider>().unwrap();
    /// 
    /// let aabb = collider.get_aabb(Vector2 { x: 0.0, y: 0.0 }, 45.0);
    /// ```
    pub fn get_aabb(&mut self, pos: Vector2, rot: f32) -> AABB {
        if !self.aabb_initialized || self.old_pos != pos || self.old_rot != rot {
            let aabb = match &mut self.hitbox {
                ShapeType::Circle(c) => {
                    AABB::new_from_uncalculated(
                        pos.x - c.get_radius(),
                        pos.y - c.get_radius(),
                        pos.x + c.get_radius(),
                        pos.y + c.get_radius(),
                    )
                }
                ShapeType::Box(b) => {
                    b.set_transform_required(true);
                    let vertices = b.get_transformed_vertices(pos, rot);
                    let mut min_x = std::f32::MAX;
                    let mut max_x = std::f32::MIN;
                    let mut min_y = std::f32::MAX;
                    let mut max_y = std::f32::MIN;

                    for i in 0..vertices.len() {
                        let vertex = vertices[i];
                        if vertex.x < min_x {
                            min_x = vertex.x;
                        }
                        if vertex.x > max_x {
                            max_x = vertex.x;
                        }
                        if vertex.y < min_y {
                            min_y = vertex.y;
                        }
                        if vertex.y > max_y {
                            max_y = vertex.y;
                        }
                    }

                    AABB::new_from_uncalculated(min_x, min_y, max_x, max_y)
                }
                ShapeType::Polygon(p) => {
                    p.set_transform_required(true);
                    let vertices = p.get_transformed_vertices(pos, rot);
                    let mut min_x = std::f32::MAX;
                    let mut max_x = std::f32::MIN;
                    let mut min_y = std::f32::MAX;
                    let mut max_y = std::f32::MIN;

                    for i in 0..vertices.len() {
                        let vertex = vertices[i];
                        if vertex.x < min_x {
                            min_x = vertex.x;
                        }
                        if vertex.x > max_x {
                            max_x = vertex.x;
                        }
                        if vertex.y < min_y {
                            min_y = vertex.y;
                        }
                        if vertex.y > max_y {
                            max_y = vertex.y;
                        }
                    }

                    AABB::new_from_uncalculated(min_x, min_y, max_x, max_y)
                }
                ShapeType::Concave(p) => {
                    p.iter_mut().for_each(|p| p.set_transform_required(true));
                    let mut vertices = vec![];
                    p.iter_mut().for_each(|p| p.get_transformed_vertices(pos, rot).iter().for_each(|v| vertices.push(*v)));

                    let mut min_x = std::f32::MAX;
                    let mut max_x = std::f32::MIN;
                    let mut min_y = std::f32::MAX;
                    let mut max_y = std::f32::MIN;

                    for i in 0..vertices.len() {
                        let vertex = vertices[i];
                        if vertex.x < min_x {
                            min_x = vertex.x;
                        }
                        if vertex.x > max_x {
                            max_x = vertex.x;
                        }
                        if vertex.y < min_y {
                            min_y = vertex.y;
                        }
                        if vertex.y > max_y {
                            max_y = vertex.y;
                        }
                    }

                    AABB::new_from_uncalculated(min_x, min_y, max_x, max_y)
                }
            };
            self.old_pos = pos;
            self.old_rot = rot;
            self.aabb = aabb;
            self.aabb_initialized = true;
            self.aabb
        } else {
            self.aabb
        }
    }
}

impl Component for Collider {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}