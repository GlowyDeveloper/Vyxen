use vyxen_math::{Transform, Vector2};

/// An enum representing the type of a rigid body.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RigidType {
    Circle,
    Box
}

/// A struct representing a rigid body in the physics simulation.
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::bodies::{Rigid, RigidType};
/// 
/// let circle = Rigid::new_circle(1.0, Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5);
/// assert_eq!(circle.get_shape_type(), RigidType::Circle);
/// assert_eq!(circle.get_radius(), 1.0);
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

    density: f32,
    mass: f32,
    restitution: f32,
    area: f32,

    is_static: bool,

    shape_type: RigidType,
    radius: f32,
    width: f32,
    height: f32,

    vertices: [Vector2; 4],
    transformed_vertices: [Vector2; 4],
    triangles: [i32; 6],
    transform_required: bool,
}

impl Rigid {
    /// A getter for the position of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
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
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
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
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
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
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// assert_eq!(rigid.get_rotational_velocity(), 0.0);
    /// ```
    pub fn get_rotational_velocity(&self) -> f32 {
        self.rotational_velocity
    }
    /// A getter for the density of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
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
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// let area = std::f32::consts::PI;
    /// let expected_mass = area * 1.0; // area * density
    /// 
    /// assert_eq!(rigid.get_mass(), expected_mass);
    /// ```
    pub fn get_mass(&self) -> f32 {
        self.mass
    }
    /// A getter for the restitution of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
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
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
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
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// assert_eq!(rigid.is_static(), false);
    /// ```
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    /// A getter for the shape type of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// assert_eq!(rigid.get_shape_type(), RigidType::Circle);
    /// ```
    pub fn get_shape_type(&self) -> RigidType {
        self.shape_type
    }
    /// A getter for the radius of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// assert_eq!(rigid.get_radius(), 1.0);
    /// ```
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    /// A getter for the width of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let rigid = Rigid::new_box(1.0, 2.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// assert_eq!(rigid.get_width(), 1.0);
    /// ```
    pub fn get_width(&self) -> f32 {
        self.width
    }
    /// A getter for the height of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let rigid = Rigid::new_box(1.0, 2.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// assert_eq!(rigid.get_height(), 2.0);
    /// ```
    pub fn get_height(&self) -> f32 {
        self.height
    }
    /// A getter for the triangle indices of the rigid body.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let triangles = [0, 1, 2, 0, 2, 3];
    /// 
    /// let mut rigid = Rigid::new_box(1.0, 1.0, Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5);
    /// assert_eq!(rigid.get_triangles(), &triangles);
    /// ```
    pub fn get_triangles(&mut self) -> &[i32; 6] {
        &self.triangles
    }
}

impl Rigid {
    fn new(position: Vector2, density: f32, mass: f32, restitution: f32, area: f32, is_static: bool, shape_type: RigidType, radius: f32, width: f32, height: f32) -> Self {
        let vertices = match shape_type {
            RigidType::Circle => [Vector2::zero(); 4],
            RigidType::Box => Self::create_box_vertices(width, height),
        };
        let triangles = match shape_type {
            RigidType::Circle => [0; 6],
            RigidType::Box => [0, 1, 2, 0, 2, 3],
        };

        Rigid {
            position,
            linear_velocity: Vector2::zero(),
            rotation: 0.0,
            rotational_velocity: 0.0,
            density,
            mass,
            restitution,
            area,
            is_static,
            shape_type,
            radius,
            width,
            height,
            vertices,
            transformed_vertices: vertices,
            triangles,
            transform_required: true,
        }
    }

    /// A constructor for a circular rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let radius = 1.0;
    /// let position = Vector2 { x: 2.0, y: 3.0 };
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// 
    /// let rigid = Rigid::new_circle(radius, position, density, is_static, restitution);
    /// ```
    pub fn new_circle(radius: f32, position: Vector2, density: f32, is_static: bool, restitution: f32) -> Self {
        let area = std::f32::consts::PI * radius * radius;

        Rigid::new(
            position,
            density,
            area * density,
            restitution.clamp(0.0, 1.0),
            area,
            is_static,
            RigidType::Circle,
            radius,
            0.0,
            0.0,
        )
    }

    /// A constructor for a rectangular rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let width = 1.0;
    /// let height = 2.0;
    /// let position = Vector2 { x: 2.0, y: 3.0 };
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// 
    /// let rigid = Rigid::new_box(width, height, position, density, is_static, restitution);
    /// ```
    pub fn new_box(width: f32, height: f32, position: Vector2, density: f32, is_static: bool, restitution: f32) -> Self {
        let area = width * height;

        Rigid::new(
            position,
            density,
            area * density,
            restitution.clamp(0.0, 1.0),
            area,
            is_static,
            RigidType::Box,
            0.0,
            width,
            height,
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
    ///
    /// let mut rigid = Rigid::new_box(2.0, 2.0, Vector2 { x: 1.0, y: 1.0 }, 1.0, false, 0.5);
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
    /// 
    /// let mut rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// rigid.move_by(Vector2 { x: 1.0, y: 1.0 });
    /// assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    /// ```
    pub fn move_by(&mut self, amount: Vector2) {
        self.position = self.position + amount;
        self.transform_required = true;
    }

    /// Moves the rigid body to a given position.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let mut rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// rigid.move_to(Vector2 { x: 3.0, y: 4.0 });
    /// assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    /// ```
    pub fn move_to(&mut self, position: Vector2) {
        self.position = position;
        self.transform_required = true;
    }

    /// Rotates the rigid body by a given amount
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let mut rigid = Rigid::new_box(1.0, 1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
    /// rigid.rotate_by(45.0);
    /// assert_eq!(rigid.get_rotation(), 45.0);
    /// ```
    pub fn rotate_by(&mut self, amount: f32) {
        self.rotation += amount;
        self.transform_required = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vyxen_math::Vector2;

    #[test]
    fn test_move_by() {
        let mut rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
        rigid.move_by(Vector2 { x: 1.0, y: 1.0 });
        assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_move_to() {
        let mut rigid = Rigid::new_circle(1.0, Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5);
        rigid.move_to(Vector2 { x: 3.0, y: 4.0 });
        assert_eq!(rigid.get_position(), Vector2 { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_rotate_by() {
        let mut rigid = Rigid::new_box(
            1.0,
            1.0,
            Vector2::zero(),
            1.0,
            false,
            0.5,
        );

        rigid.rotate_by(90.0);

        assert_eq!(rigid.get_rotation(), 90.0);
    }

    #[test]
    fn test_new_circle_properties() {
        let rigid = Rigid::new_circle(
            2.0,
            Vector2 { x: 1.0, y: 2.0 },
            3.0,
            true,
            0.8,
        );

        assert_eq!(rigid.get_shape_type(), RigidType::Circle);
        assert_eq!(rigid.get_radius(), 2.0);
        assert_eq!(rigid.get_position(), Vector2 { x: 1.0, y: 2.0 });
        assert_eq!(rigid.get_density(), 3.0);
        assert_eq!(rigid.is_static(), true);
        assert_eq!(rigid.get_restitution(), 0.8);
    }

    #[test]
    fn test_new_box_properties() {
        let rigid = Rigid::new_box(
            4.0,
            2.0,
            Vector2 { x: 5.0, y: 6.0 },
            1.5,
            false,
            0.3,
        );

        assert_eq!(rigid.get_shape_type(), RigidType::Box);
        assert_eq!(rigid.get_width(), 4.0);
        assert_eq!(rigid.get_height(), 2.0);
        assert_eq!(rigid.get_position(), Vector2 { x: 5.0, y: 6.0 });
        assert_eq!(rigid.get_density(), 1.5);
        assert_eq!(rigid.is_static(), false);
        assert_eq!(rigid.get_restitution(), 0.3);
    }

    #[test]
    fn test_circle_area_and_mass() {
        let rigid = Rigid::new_circle(
            1.0,
            Vector2::zero(),
            2.0,
            false,
            0.5,
        );

        let expected_area = std::f32::consts::PI;
        let expected_mass = expected_area * 2.0;

        assert_eq!(rigid.get_area(), expected_area);
        assert_eq!(rigid.get_mass(), expected_mass);
    }

    #[test]
    fn test_box_area_and_mass() {
        let rigid = Rigid::new_box(
            4.0,
            2.0,
            Vector2::zero(),
            3.0,
            false,
            0.5,
        );

        assert_eq!(rigid.get_area(), 8.0);
        assert_eq!(rigid.get_mass(), 24.0);
    }

    #[test]
    fn test_get_transformed_vertices_without_rotation() {
        let mut rigid = Rigid::new_box(
            2.0,
            2.0,
            Vector2 { x: 1.0, y: 1.0 },
            1.0,
            false,
            0.5,
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
            1.0,
            Vector2::zero(),
            1.0,
            false,
            5.0,
        );

        assert_eq!(rigid.get_restitution(), 1.0);
    }

    #[test]
    fn test_negative_restitution_clamped() {
        let rigid = Rigid::new_circle(
            1.0,
            Vector2::zero(),
            1.0,
            false,
            -5.0,
        );

        assert_eq!(rigid.get_restitution(), 0.0);
    }
}