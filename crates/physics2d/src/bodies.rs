use vyxen_geometry::{Box, Circle, aabb::AABB};
use vyxen_math::{Transform, Vector2};

/// An enum representing the type of a rigid body.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RigidType {
    Circle(Circle),
    Box(Box)
}

/// A struct representing a rigid body in the physics simulation.
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::bodies::{Rigid, RigidType};
/// use vyxen_geometry::Circle;
/// 
/// let circle = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
/// assert!(circle.get_circle().is_some());
/// assert_eq!(circle.get_circle().unwrap().radius, 1.0);
/// assert_eq!(circle.get_position(), Vector2 { x: 0.0, y: 0.0 });
/// assert_eq!(circle.get_density(), 1.0);
/// assert_eq!(circle.is_static(), false);
/// assert_eq!(circle.get_restitution(), 0.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rigid {
    position: Vector2,
    linear_velocity: Vector2,
    rotation: f32,
    rotational_velocity: f32,

    force: Vector2,

    density: f32,
    mass: f32,
    inverse_mass: f32,
    restitution: f32,
    area: f32,

    is_static: bool,

    shape: RigidType,
    aabb: AABB,

    vertices: [Vector2; 4],
    transformed_vertices: [Vector2; 4],
    triangles: [i32; 6],
    transform_required: bool,
    aabb_required: bool,
}

impl Rigid {
    /// A getter for the position of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_position(), Vector2 { x: 2.0, y: 3.0 });
    /// ```
    pub fn get_position(&self) -> Vector2 {
        self.position
    }
    /// A getter for the linear velocity of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_linear_velocity(), Vector2 { x: 0.0, y: 0.0 });
    /// ```
    pub fn get_linear_velocity(&self) -> Vector2 {
        self.linear_velocity
    }
    /// A getter for the rotation of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_rotation(), 0.0);
    /// ```
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    /// A getter for the rotational velocity of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_rotational_velocity(), 0.0);
    /// ```
    pub fn get_rotational_velocity(&self) -> f32 {
        self.rotational_velocity
    }
    /// A getter for the force of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_force(), Vector2::zero());
    /// ```
    pub fn get_force(&self) -> Vector2 {
        self.force
    }
    /// A getter for the density of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_density(), 1.0);
    /// ```
    pub fn get_density(&self) -> f32 {
        self.density
    }
    /// A getter for the mass of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// let area = std::f32::consts::PI;
    /// let expected_mass = area * 1.0; // area * density
    /// 
    /// assert_eq!(rigid.get_mass(), expected_mass);
    /// ```
    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    /// A getter for the inverted mass of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// let area = std::f32::consts::PI;
    /// let expected_mass = area * 1.0; // area * density
    /// let expected_inverted_mass = 1.0 / expected_mass;
    /// 
    /// assert_eq!(rigid.get_inverse_mass(), expected_inverted_mass);
    /// ```
    pub fn get_inverse_mass(&self) -> f32 {
        self.inverse_mass
    }
    /// A getter for the restitution of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_restitution(), 0.5);
    /// ```
    pub fn get_restitution(&self) -> f32 {
        self.restitution
    }
    /// A getter for the area of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.get_area(), std::f32::consts::PI);
    /// ```
    pub fn get_area(&self) -> f32 {
        self.area
    }
    /// A getter for the static status of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert_eq!(rigid.is_static(), false);
    /// ```
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    /// A getter for the shape of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::Circle;
    /// 
    /// let rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert!(rigid.get_circle().is_some());
    /// ```
    pub fn get_shape(&self) -> RigidType {
        self.shape
    }
    /// Returns `None` if the shape is a box, return `Some(Circle)` if the shape is a circle
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::{Box, Circle};
    /// 
    /// let circle = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert!(circle.get_circle().is_some());
    /// 
    /// let bx = Rigid::new_box(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// assert!(bx.get_circle().is_none());
    /// ```
    pub fn get_circle(&self) -> Option<Circle> {
        match self.shape {
            RigidType::Circle(c) => Some(c),
            _ => None,
        }
    }
    /// Returns `None` if the shape is a circle, return `Some(Box)` if the shape is a box
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::{Box, Circle};
    /// 
    /// let bx = Rigid::new_box(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// assert!(bx.get_box().is_some());
    /// 
    /// let circle = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// assert!(circle.get_box().is_none());
    /// ```
    pub fn get_box(&self) -> Option<Box> {
        match self.shape {
            RigidType::Box(b) => Some(b),
            _ => None,
        }
    }
    /// A getter for the triangle indices of the rigid body.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    /// 
    /// let triangles = [0, 1, 2, 0, 2, 3];
    /// 
    /// let mut rigid = Rigid::new_box(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// assert_eq!(rigid.get_triangles(), &triangles);
    /// ```
    pub fn get_triangles(&mut self) -> &[i32; 6] {
        &self.triangles
    }

    /// A setter for the linear velocity of the rigid body.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    /// 
    /// let start_pos = Vector2 { x: 0.0, y: 0.0 };
    /// 
    /// let mut rigid = Rigid::new_box(start_pos, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// rigid.set_linear_velocity(Vector2 { x: 5.0, y: 0.0 });
    /// rigid.step(1.0, Vector2 { x: 0.0, y: -9.81 });
    /// 
    /// assert!(rigid.get_position() != start_pos);
    /// ```
    pub fn set_linear_velocity(&mut self, velocity: Vector2) {
        self.linear_velocity = velocity;
    }
}

impl Rigid {
    fn new(position: Vector2, density: f32, mass: f32, restitution: f32, area: f32, is_static: bool, shape: RigidType) -> Self {
        let vertices = match shape {
            RigidType::Circle(_) => [Vector2::zero(); 4],
            RigidType::Box(b) => Self::create_box_vertices(b.width, b.height),
        };
        let triangles = match shape {
            RigidType::Circle(_) => [0; 6],
            RigidType::Box(_) => [0, 1, 2, 0, 2, 3],
        };

        Rigid {
            position,
            linear_velocity: Vector2::zero(),
            rotation: 0.0,
            rotational_velocity: 0.0,
            force: Vector2::zero(),
            density,
            mass,
            inverse_mass: if is_static { 0.0 } else { 1.0 / mass },
            restitution,
            area,
            is_static,
            shape,
            vertices,
            transformed_vertices: vertices,
            triangles,
            transform_required: true,
            aabb: AABB::new_from_uncalculated(std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN),
            aabb_required: true,
        }
    }

    /// A constructor for a circular rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let radius = 1.0;
    /// let position = Vector2 { x: 2.0, y: 3.0 };
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// 
    /// let rigid = Rigid::new_circle(position, density, is_static, restitution, Circle { radius });
    /// ```
    pub fn new_circle(position: Vector2, density: f32, is_static: bool, restitution: f32, circle: Circle) -> Self {
        let area = std::f32::consts::PI * circle.radius * circle.radius;

        Rigid::new(
            position,
            density,
            area * density,
            restitution.clamp(0.0, 1.0),
            area,
            is_static,
            RigidType::Circle(circle),
        )
    }

    /// A constructor for a rectangular rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    /// 
    /// let width = 1.0;
    /// let height = 2.0;
    /// let position = Vector2 { x: 2.0, y: 3.0 };
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// 
    /// let rigid = Rigid::new_box(position, density, is_static, restitution, Box { width, height });
    /// ```
    pub fn new_box(position: Vector2, density: f32, is_static: bool, restitution: f32, box_shape: Box) -> Self {
        let area = box_shape.width * box_shape.height;

        Rigid::new(
            position,
            density,
            area * density,
            restitution.clamp(0.0, 1.0),
            area,
            is_static,
            RigidType::Box(box_shape),
        )
    }

    fn create_box_vertices(width: f32, height: f32) -> [Vector2; 4] {
        let left = -width / 2.0;
        let right = left + width;
        let bottom = -height / 2.0;
        let top = bottom + height;

        [
            Vector2 { x: left, y: top },
            Vector2 { x: right, y: top },
            Vector2 { x: right, y: bottom },
            Vector2 { x: left, y: bottom },
        ]
    }

    /// Transformes the vertices of a box type body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    ///
    /// let mut rigid = Rigid::new_box(Vector2 { x: 1.0, y: 1.0 }, 1.0, false, 0.5, Box { width: 2.0, height: 2.0 });
    /// let vertices = rigid.get_transformed_vertices();
    ///
    /// assert_eq!(vertices[0], Vector2 { x: 0.0, y: 2.0 });
    /// assert_eq!(vertices[1], Vector2 { x: 2.0, y: 2.0 });
    /// assert_eq!(vertices[2], Vector2 { x: 2.0, y: 0.0 });
    /// assert_eq!(vertices[3], Vector2 { x: 0.0, y: 0.0 });
    /// ```
    pub fn get_transformed_vertices(&mut self) -> &[Vector2; 4] {
        if self.transform_required {
            let transform = Transform::new(self.position, self.rotation);

            for i in 0..4 {
                let vertex = self.vertices[i];
                self.transformed_vertices[i] = vertex.transform(&transform);
            }
            self.transform_required = false;
        }
        &self.transformed_vertices
    }

    /// Moves the rigid body by a given amount.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let mut rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// rigid.move_by(Vector2 { x: 1.0, y: 1.0 });
    /// assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    /// ```
    pub fn move_by(&mut self, amount: Vector2) {
        self.position = self.position + amount;
        self.transform_required = true;
        self.aabb_required = true;
    }

    /// Moves the rigid body to a given position.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Circle;
    /// 
    /// let mut rigid = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// rigid.move_to(Vector2 { x: 3.0, y: 4.0 });
    /// assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    /// ```
    pub fn move_to(&mut self, position: Vector2) {
        self.position = position;
        self.transform_required = true;
        self.aabb_required = true;
    }

    /// Rotates the rigid body by a given amount
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    /// 
    /// let mut rigid = Rigid::new_box(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// rigid.rotate_by(45.0);
    /// assert_eq!(rigid.get_rotation(), 45.0);
    /// ```
    pub fn rotate_by(&mut self, amount: f32) {
        self.rotation += amount;
        self.transform_required = true;
        self.aabb_required = true;
    }

    /// Processes a single physics frame of the body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    /// 
    /// let force = Vector2 { x: 5.0, y: 0.0 };
    /// 
    /// let mut rigid = Rigid::new_box(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// rigid.add_force(force);
    /// 
    /// assert_eq!(rigid.get_force(), force);
    /// 
    /// rigid.add_force(force);
    /// 
    /// assert_eq!(rigid.get_force(), force * 2.0);
    /// ```
    pub fn add_force(&mut self, force: Vector2) {
        self.force = self.force + force;
    }

    /// Processes a single physics frame of the body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    /// 
    /// let force = Vector2 { x: 5.0, y: 0.0 };
    /// 
    /// let mut rigid = Rigid::new_box(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// rigid.set_force(force);
    /// 
    /// assert_eq!(rigid.get_force(), force);
    /// ```
    pub fn set_force(&mut self, force: Vector2) {
        self.force = force;
    }

    /// Processes a single physics frame of the body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::Box;
    /// 
    /// let start_pos = Vector2 { x: 0.0, y: 0.0 };
    /// 
    /// let mut rigid = Rigid::new_box(start_pos, 1.0, false, 0.5, Box { width: 1.0, height: 1.0 });
    /// rigid.set_linear_velocity(Vector2 { x: 5.0, y: 0.0 });
    /// rigid.step(1.0, Vector2 { x: 0.0, y: -9.81 });
    /// 
    /// assert!(rigid.get_position() != start_pos);
    /// ```
    pub fn step(&mut self, dt: f32, gravity: Vector2) {
        if self.is_static {
            return;
        }
        self.linear_velocity = self.linear_velocity + gravity * dt;
        self.position = self.position + self.linear_velocity * dt;
        self.rotation += self.rotational_velocity * dt;
        self.force = Vector2::zero();
        self.transform_required = true;
        self.aabb_required = true;
    }

    pub fn get_aabb(&mut self) -> AABB {
        if self.aabb_required {
            match self.shape {
                RigidType::Circle(c) => {
                    self.aabb = AABB::new_from_uncalculated(
                        self.position.x - c.radius,
                        self.position.y - c.radius,
                        self.position.x + c.radius,
                        self.position.y + c.radius,
                    );
                }
                RigidType::Box(_) => {
                    let vertices = self.get_transformed_vertices();
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

                    self.aabb = AABB::new_from_uncalculated(min_x, min_y, max_x, max_y);
                }
            } 

            self.aabb_required = false;
        }

        self.aabb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vyxen_math::Vector2;

    #[test]
    fn test_move_by() {
        let mut rigid = Rigid::new_circle(
            Vector2 { x: 2.0, y: 3.0 },
            1.0,
            false,
            0.5,
            Circle { radius: 1.0 },
        );

        rigid.move_by(Vector2 { x: 1.0, y: 1.0 });

        assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_move_to() {
        let mut rigid = Rigid::new_circle(
            Vector2 { x: 2.0, y: 3.0 },
            1.0,
            false,
            0.5,
            Circle { radius: 1.0 },
        );

        rigid.move_to(Vector2 { x: 3.0, y: 4.0 });

        assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_rotate_by() {
        let mut rigid = Rigid::new_box(
            Vector2::zero(),
            1.0,
            false,
            0.5,
            Box {
                width: 1.0,
                height: 1.0,
            },
        );

        rigid.rotate_by(90.0);

        assert_eq!(rigid.get_rotation(), 90.0);
    }

    #[test]
    fn test_new_circle_properties() {
        let rigid = Rigid::new_circle(
            Vector2 { x: 1.0, y: 2.0 },
            3.0,
            true,
            0.8,
            Circle { radius: 2.0 },
        );

        assert_eq!(
            rigid.get_shape(),
            RigidType::Circle(Circle { radius: 2.0 })
        );

        assert_eq!(rigid.get_position(), Vector2 { x: 1.0, y: 2.0 });
        assert_eq!(rigid.get_density(), 3.0);
        assert_eq!(rigid.is_static(), true);
        assert_eq!(rigid.get_restitution(), 0.8);
    }

    #[test]
    fn test_new_box_properties() {
        let rigid = Rigid::new_box(
            Vector2 { x: 5.0, y: 6.0 },
            1.5,
            false,
            0.3,
            Box {
                width: 4.0,
                height: 2.0,
            },
        );

        assert_eq!(
            rigid.get_shape(),
            RigidType::Box(Box {
                width: 4.0,
                height: 2.0,
            })
        );

        assert_eq!(rigid.get_position(), Vector2 { x: 5.0, y: 6.0 });
        assert_eq!(rigid.get_density(), 1.5);
        assert_eq!(rigid.is_static(), false);
        assert_eq!(rigid.get_restitution(), 0.3);
    }

    #[test]
    fn test_circle_area_and_mass() {
        let rigid = Rigid::new_circle(
            Vector2::zero(),
            2.0,
            false,
            0.5,
            Circle { radius: 1.0 },
        );

        let expected_area = std::f32::consts::PI;
        let expected_mass = expected_area * 2.0;

        assert_eq!(rigid.get_area(), expected_area);
        assert_eq!(rigid.get_mass(), expected_mass);
    }

    #[test]
    fn test_box_area_and_mass() {
        let rigid = Rigid::new_box(
            Vector2::zero(),
            3.0,
            false,
            0.5,
            Box {
                width: 4.0,
                height: 2.0,
            },
        );

        assert_eq!(rigid.get_area(), 8.0);
        assert_eq!(rigid.get_mass(), 24.0);
    }

    #[test]
    fn test_get_transformed_vertices_without_rotation() {
        let mut rigid = Rigid::new_box(
            Vector2 { x: 1.0, y: 1.0 },
            1.0,
            false,
            0.5,
            Box {
                width: 2.0,
                height: 2.0,
            },
        );

        let vertices = rigid.get_transformed_vertices();

        assert_eq!(vertices[0], Vector2 { x: 0.0, y: 2.0 });
        assert_eq!(vertices[1], Vector2 { x: 2.0, y: 2.0 });
        assert_eq!(vertices[2], Vector2 { x: 2.0, y: 0.0 });
        assert_eq!(vertices[3], Vector2 { x: 0.0, y: 0.0 });
    }

    #[test]
    fn test_restitution_clamped() {
        let rigid = Rigid::new_circle(
            Vector2::zero(),
            1.0,
            false,
            5.0,
            Circle { radius: 1.0 },
        );

        assert_eq!(rigid.get_restitution(), 1.0);
    }

    #[test]
    fn test_negative_restitution_clamped() {
        let rigid = Rigid::new_circle(
            Vector2::zero(),
            1.0,
            false,
            -5.0,
            Circle { radius: 1.0 },
        );

        assert_eq!(rigid.get_restitution(), 0.0);
    }
}