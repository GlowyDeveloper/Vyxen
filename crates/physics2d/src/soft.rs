use vyxen_geometry::{Box, Circle, Polygon, Shape, ShapeType};
use vyxen_math::{Vector2, is_nearly_equal};

use crate::shape_type_from_shape;

/// Default for the spring force
pub const SPRING_FORCE: f32 = 100.0;
/// Default for the spring damping
pub const SPRING_DAMPING: f32 = 10.0;
/// Default for shape matching stiffness
pub const SHAPE_RECONSTRUCTION_STIFFNESS: f32 = 80.0;
/// Default for shape matching damping
pub const SHAPE_DAMPING: f32 = 5.0;

/// A struct representing a soft body in the physics simulation.
///
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::SoftBody;
/// use vyxen_geometry::Circle;
///
/// let circle = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
/// assert_eq!(circle.get_density(), 1.0);
/// assert_eq!(circle.get_restitution(), 0.5);
/// assert_eq!(circle.get_static_friction(), 0.6);
/// assert_eq!(circle.get_dynamic_friction(), 0.4);
/// assert_eq!(circle.get_points().len(), 8); // Circles get turned into an octagon
/// ```
pub struct SoftBody {
    density: f32,
    mass: f32,
    inverse_mass: f32,
    restitution: f32,
    area: f32,

    inertia: f32,
    inverse_inertia: f32,

    static_friction: f32,
    dynamic_friction: f32,

    original_points: Vec<Vector2>,
    points: Vec<PointMass>,
    springs: Vec<Spring>,
}

impl SoftBody {
    /// A constructor for a soft body.
    ///
    /// # Examples
    ///
    /// ## Circle
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let radius = 1.0;
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// let static_friction = 0.6;
    /// let dynamic_friction = 0.4;
    ///
    /// let soft = SoftBody::new(density, is_static, restitution, Circle::new(radius), static_friction, dynamic_friction);
    /// ```
    ///
    /// ## Box
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Box;
    ///
    /// let width = 1.0;
    /// let height = 2.0;
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// let static_friction = 0.6;
    /// let dynamic_friction = 0.4;
    ///
    /// let soft = SoftBody::new(density, is_static, restitution, Box::new(width, height), static_friction, dynamic_friction);
    /// ```
    ///
    /// ## Polygon
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Polygon;
    ///
    /// let width = 1.0;
    /// let height = 2.0;
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// let static_friction = 0.6;
    /// let dynamic_friction = 0.4;
    ///
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    ///
    /// let soft = SoftBody::new(density, is_static, restitution, Polygon::new(&[v1, v2, v3]), static_friction, dynamic_friction);
    /// ```
    pub fn new<T>(
        density: f32,
        is_static: bool,
        restitution: f32,
        shape: T,
        static_friction: f32,
        dynamic_friction: f32,
    ) -> Self
    where
        T: Shape,
    {
        let points = match () {
            _ if shape.as_any().is::<Circle>() => {
                if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
                    let mut points = Vec::with_capacity(8);

                    for i in 0..8 {
                        let angle = i as f32 * (2.0 * std::f32::consts::PI / 8.0);

                        let x = circle.get_radius() * angle.cos();
                        let y = circle.get_radius() * angle.sin();

                        points.push(Vector2 { x, y });
                    }

                    points
                } else {
                    Vec::new()
                }
            }
            _ if shape.as_any().is::<Box>() => {
                if let Some(bx) = shape.as_any().downcast_ref::<Box>() {
                    vec![
                        Vector2 {
                            x: bx.get_width() / 2.0,
                            y: bx.get_height() / 2.0,
                        },
                        Vector2 {
                            x: -bx.get_width() / 2.0,
                            y: bx.get_height() / 2.0,
                        },
                        Vector2 {
                            x: -bx.get_width() / 2.0,
                            y: -bx.get_height() / 2.0,
                        },
                        Vector2 {
                            x: bx.get_width() / 2.0,
                            y: -bx.get_height() / 2.0,
                        },
                    ]
                } else {
                    Vec::new()
                }
            }
            _ if shape.as_any().is::<Polygon>() => {
                if let Some(polygon) = shape.as_any().downcast_ref::<Polygon>() {
                    polygon.get_vertices().to_vec()
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        };

        let mut point_masses = vec![];
        for point in points.iter() {
            point_masses.push(PointMass::new(*point));
        }

        let mut springs = vec![];
        for i in 0..points.len() {
            let point_a_index = i;
            let point_b_index = (i + 1) % points.len();
            let point_a = points[point_a_index];
            let point_b = points[point_b_index];
            let distance = point_a.distance(&point_b);
            springs.push(Spring::new(point_a_index, point_b_index, distance));
        }

        let shape_type = shape_type_from_shape(shape.clone());

        let area = match &shape_type {
            ShapeType::Circle(circle) => {
                std::f32::consts::PI * circle.get_radius() * circle.get_radius()
            }
            ShapeType::Box(bx) => bx.get_width() * bx.get_height(),
            ShapeType::Polygon(polygon) => {
                let vertices = polygon.get_vertices();

                let mut products_1 = 0.0_f32;
                let mut products_2 = 0.0_f32;

                for i in 0..vertices.len() {
                    let current = vertices[i];
                    let next = vertices[(i + 1) % vertices.len()];

                    products_1 += current.x * next.y;
                    products_2 += current.y * next.x;
                }

                (products_1 - products_2).abs() * 0.5
            }
            ShapeType::Concave(concave) => {
                let mut sum = 0.0_f32;
                for polygon in concave {
                    let vertices = polygon.get_vertices();

                    let mut products_1 = 0.0_f32;
                    let mut products_2 = 0.0_f32;

                    for i in 0..vertices.len() {
                        let current = vertices[i];
                        let next = vertices[(i + 1) % vertices.len()];

                        products_1 += current.x * next.y;
                        products_2 += current.y * next.x;
                    }

                    sum += (products_1 - products_2).abs() * 0.5;
                }
                sum
            }
        };

        let mass = area * if density <= 0.0 { 0.00001 } else { density };

        let inertia = match &shape_type {
            ShapeType::Circle(c) => c.rotational_inertia(mass),
            ShapeType::Box(b) => b.rotational_inertia(mass),
            ShapeType::Polygon(p) => p.rotational_inertia(mass),
            ShapeType::Concave(v) => {
                let mut added = vec![];
                v.iter()
                    .for_each(|p| added.push(p.rotational_inertia(mass)));

                added.iter().sum::<f32>() / added.len() as f32
            }
        };

        Self {
            original_points: points,
            points: point_masses,
            springs,
            density,
            mass,
            inverse_mass: if is_static { 0.0 } else { 1.0 / mass },
            restitution: restitution.clamp(0.0, 1.0),
            area,
            inertia,
            inverse_inertia: if is_static { 0.0 } else { 1.0 / inertia },
            static_friction,
            dynamic_friction,
        }
    }

    pub fn solve_springs(&mut self, dt: f32) {
        for spring in &self.springs {
            spring.calculate(&mut self.points, dt);
        }

        for i in 0..self.points.len() {
            let original_pos = self.original_points[i];
            let point = &mut self.points[i];

            let current_pos = point.get_position();
            let current_vel = point.get_velocity();

            let to_home = original_pos - current_pos;

            let restoration_force = to_home * SHAPE_RECONSTRUCTION_STIFFNESS;
            let damping_force = current_vel * SHAPE_DAMPING;

            let new_vel = current_vel + (restoration_force - damping_force) * dt;
            point.set_velocity(new_vel);
        }
    }

    /// A getter for the points of the soft body.
    ///
    /// If you want the mutable version, refer to `get_points_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::{Circle, ShapeType};
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let points = soft.get_points();
    /// assert_eq!(points.len(), 8);
    /// ```
    pub fn get_points(&self) -> &Vec<PointMass> {
        &self.points
    }
    /// A getter for the points of the soft body muttably.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::{Circle, ShapeType};
    ///
    /// let mut soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let mut points = soft.get_points_mut();
    /// assert_eq!(points.len(), 8);
    /// ```
    pub fn get_points_mut(&mut self) -> &mut Vec<PointMass> {
        &mut self.points
    }
    /// A getter for the springs of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::{Circle, ShapeType};
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let springs = soft.get_springs();
    /// assert_eq!(springs.len(), 8); // There's a spring for each edge
    /// ```
    pub fn get_springs(&self) -> &Vec<Spring> {
        &self.springs
    }
    /// A getter for the density of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(soft.get_density(), 1.0);
    /// ```
    pub fn get_density(&self) -> f32 {
        self.density
    }
    /// A getter for the mass of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let area = std::f32::consts::PI;
    /// let expected_mass = area * 1.0; // area * density
    ///
    /// assert_eq!(soft.get_mass(), expected_mass);
    /// ```
    pub fn get_mass(&self) -> f32 {
        self.mass
    }
    /// A getter for the inverted mass of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let area = std::f32::consts::PI;
    /// let expected_mass = area * 1.0; // area * density
    /// let expected_inverted_mass = 1.0 / expected_mass;
    ///
    /// assert_eq!(soft.get_inverse_mass(), expected_inverted_mass);
    /// ```
    pub fn get_inverse_mass(&self) -> f32 {
        self.inverse_mass
    }
    /// A getter for the restitution of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(soft.get_restitution(), 0.5);
    /// ```
    pub fn get_restitution(&self) -> f32 {
        self.restitution
    }
    /// A getter for the area of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(soft.get_area(), std::f32::consts::PI);
    /// ```
    pub fn get_area(&self) -> f32 {
        self.area
    }
    /// A getter for the rotational inertia of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let expected_mass = std::f32::consts::PI * 1.0; // area * density
    /// let expected_inertia = (1.0 / 2.0) * expected_mass * 1.0 * 1.0; // (1/2) * mass * radius * radius
    ///
    /// assert_eq!(soft.get_inertia(), expected_inertia);
    /// ```
    pub fn get_inertia(&self) -> f32 {
        self.inertia
    }
    /// A getter for the inverted rotational inertia of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let expected_mass = std::f32::consts::PI * 1.0; // area * density
    /// let expected_inertia = (1.0 / 2.0) * expected_mass * 1.0 * 1.0; // (1/2) * mass * radius * radius
    ///
    /// assert_eq!(soft.get_inertia(), expected_inertia);
    ///
    /// let expected_inverted_inertia = 1.0 / expected_inertia;
    ///
    /// assert_eq!(soft.get_inverse_inertia(), expected_inverted_inertia);
    /// ```
    pub fn get_inverse_inertia(&self) -> f32 {
        self.inverse_inertia
    }
    /// A getter for the static friction of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::{Circle, ShapeType};
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(soft.get_static_friction(), 0.6);
    /// ```
    pub fn get_static_friction(&self) -> f32 {
        self.static_friction
    }
    /// A getter for the dynamic friction of the soft body.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::SoftBody;
    /// use vyxen_geometry::{Circle, ShapeType};
    ///
    /// let soft = SoftBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(soft.get_dynamic_friction(), 0.4);
    /// ```
    pub fn get_dynamic_friction(&self) -> f32 {
        self.dynamic_friction
    }
}

/// A point with a position and a velocity
pub struct PointMass {
    position: Vector2,
    velocity: Vector2,
}

impl PointMass {
    /// Creates a new PointMass
    pub fn new(position: Vector2) -> Self {
        PointMass {
            position,
            velocity: Vector2::zero(),
        }
    }

    /// Steps the PointMass
    pub fn step(&mut self, dt: f32) {
        self.velocity = self.velocity * 0.98;
        self.position = self.position + self.velocity * dt;
    }

    /// Gets the position
    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    /// Gets the velocity
    pub fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    /// Sets the position
    pub fn set_position(&mut self, pos: Vector2) {
        self.position = pos;
    }

    /// Sets the velocity
    pub fn set_velocity(&mut self, vel: Vector2) {
        self.velocity = vel;
    }
}

/// Springs to connect to points together
pub struct Spring {
    point_a: usize,
    point_b: usize,
    rest_distance: f32,
}

impl Spring {
    /// Creates a new spring
    pub fn new(point_a: usize, point_b: usize, rest_distance: f32) -> Self {
        Self {
            point_a,
            point_b,
            rest_distance,
        }
    }

    /// Updates the spring and points
    pub fn calculate(&self, points: &mut [PointMass], dt: f32) {
        let (point_a, point_b) = if self.point_a < self.point_b {
            let (l, r) = points.split_at_mut(self.point_b);
            (&mut l[self.point_a], &mut r[0])
        } else {
            let (l, r) = points.split_at_mut(self.point_a);
            (&mut l[self.point_b], &mut r[0])
        };

        let delta = point_a.get_position() - point_b.get_position();
        let distance = delta.length();

        if is_nearly_equal(distance, 0.0) {
            return;
        }

        let direction = delta / distance;

        let displacement = distance - self.rest_distance;
        let spring_force = direction * (displacement * SPRING_FORCE);

        point_a.set_velocity(point_a.get_velocity() - spring_force * dt);
        point_b.set_velocity(point_b.get_velocity() + spring_force * dt);

        let vrel = (point_b.get_velocity() - point_a.get_velocity()).dot(&direction);
        let damping_factor = (-SPRING_DAMPING * dt).exp();
        let new_vrel = vrel * damping_factor;
        let vrel_delta = new_vrel - vrel;

        let damping_force = direction * (vrel_delta / 2.0);
        point_a.set_velocity(point_a.get_velocity() - damping_force);
        point_b.set_velocity(point_b.get_velocity() + damping_force);
    }
}
