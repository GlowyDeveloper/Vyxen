use vyxen_math::Vector2;
use vyxen_physics2d::{bodies::{Rigid, RigidType}, collision::{Collision, Manifold, find_contact_points, intersect_aabb, intersect_circles, intersect_polygon_circle, intersect_polygons}};

pub use vyxen_math as math;
pub use vyxen_geometry as geometry;
pub use vyxen_physics2d as physics2d;

/// World struct used throughout the engine
/// 
/// # Examples
/// ```rust
/// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
/// 
/// let mut world = World::new();
/// 
/// let body = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
/// world.add_body(body);
/// 
/// let len = world.get_bodies_len();
/// assert_eq!(len, 1);
/// 
/// world.remove_body(&body);
/// 
/// let len = world.get_bodies_len();
/// assert_eq!(len, 0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct World {
    bodies: Vec<Rigid>,
    contacts: Vec<Manifold>,
    /// A Vector of points where bodies collide.
    /// Only meant for debugging, may be removed later.
    pub contact_points: Vec<Vector2>,
    gravity: Vector2,
}

impl World {
    /// Generates a new world
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// world.add_body(body);
    /// 
    /// let len = world.get_bodies_len();
    /// assert_eq!(len, 1);
    /// 
    /// world.remove_body(&body);
    /// 
    /// let len = world.get_bodies_len();
    /// assert_eq!(len, 0);
    /// ```
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            contacts: Vec::new(),
            contact_points: Vec::new(),
            gravity: Vector2 { x: 0.0, y: -9.81 }
        }
    }

    /// Adds a body to the world
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// world.add_body(body);
    /// 
    /// let len = world.get_bodies_len();
    /// assert_eq!(len, 1);
    /// ```
    pub fn add_body(&mut self, body: Rigid) {
        self.bodies.push(body);
    }

    /// Removes body at index `i`
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// world.add_body(body);
    /// 
    /// let len = world.get_bodies_len();
    /// assert_eq!(len, 1);
    /// 
    /// world.remove_body(&body);
    /// 
    /// let len = world.get_bodies_len();
    /// assert_eq!(len, 0);
    /// ```
    pub fn remove_body(&mut self, body: &Rigid) {
        if let Some(index) = self.bodies.iter().position(|b| b == body) {
            self.bodies.remove(index);
        }
    }

    /// Returns body at index `i`
    /// 
    /// If you want the mutable version, refer to `get_body_mut()`
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body1 = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// world.add_body(body1);
    /// 
    /// let body2 = world.get_body(0);
    /// 
    /// assert!(body2.is_some());
    /// assert_eq!(body1, *body2.unwrap());
    /// ```
    pub fn get_body(&self, index: usize) -> Option<&Rigid> {
        self.bodies.get(index)
    }

    /// Returns body at index `i`
    /// 
    /// If you want the mutable version, refer to `get_body_mut()`
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body1 = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// world.add_body(body1);
    /// 
    /// let mut body2 = world.get_body_mut(0);
    /// 
    /// assert!(body2.is_some());
    /// assert_eq!(body1, *body2.unwrap());
    /// ```
    pub fn get_body_mut(&mut self, index: usize) -> Option<&mut Rigid> {
        self.bodies.get_mut(index)
    }

    /// Returns the amount of bodies in the world
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// world.add_body(body);
    /// 
    /// let len = world.get_bodies_len();
    /// assert_eq!(len, 1);
    /// ```
    pub fn get_bodies_len(&self) -> usize {
        self.bodies.len()
    }

    /// Returns the gravity of the world
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, World};
    /// 
    /// let world = World::new();
    /// 
    /// assert_eq!(Vector2 { x: 0.0, y: -9.81 }, world.get_gravity());
    /// ```
    pub fn get_gravity(&self) -> Vector2 {
        self.gravity
    }

    /// Sets the gravity of the world
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// assert_eq!(Vector2 { x: 0.0, y: -9.81 }, world.get_gravity());
    /// 
    /// world.set_gravity(Vector2 { x: 0.0, y: 9.81 });
    /// 
    /// assert_eq!(Vector2 { x: 0.0, y: 9.81 }, world.get_gravity());
    /// ```
    pub fn set_gravity(&mut self, g: Vector2) {
        self.gravity = g;
    }

    /// Calculates a single physics step
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// world.add_body(body);
    /// 
    /// world.step(0.1, 10);
    /// ```
    pub fn step(&mut self, dt: f32, iterations: usize) {
        self.contact_points.clear();

        for _ in 0..iterations {
            let len = self.bodies.len();
            for i in 0..len {
                let body = &mut self.bodies[i];
                body.step(dt / iterations as f32, self.gravity);
            }

            self.contacts.clear();

            for i in 0..len {
                let (left, right) = self.bodies.split_at_mut(i+1);
                let body_a = &mut left[i];
                let body_a_aabb = body_a.get_aabb();

                for j in i + 1..len {
                    let body_b = &mut right[j-i-1];
                    let body_b_aabb = body_b.get_aabb();

                    if body_a.is_static() && body_b.is_static() {
                        continue;
                    }

                    if !intersect_aabb(body_a_aabb, body_b_aabb) {
                        continue;
                    }

                    if let Some(collision) = Self::collide(body_a, body_b) {
                        if body_a.is_static() {
                            body_b.move_by(collision.normal * collision.depth);
                        } else if body_b.is_static() {
                            body_a.move_by(-collision.normal * collision.depth);
                        } else {
                            body_a.move_by(-collision.normal * collision.depth / 2.0);
                            body_b.move_by(collision.normal * collision.depth / 2.0);
                        }

                        let (contact_a, contact_b) = find_contact_points(body_a, body_b);
                        let contact = Manifold::new(i, j, collision.normal, collision.depth, contact_a, contact_b);
                        self.contacts.push(contact);
                    }
                }
            }

            for i in 0..self.contacts.len() {
                let manifold = self.contacts[i];
                self.resolve_collision(manifold);

                if manifold.get_contact_1().is_some() {
                    self.contact_points.push(manifold.get_contact_1().unwrap());
                }
                if manifold.get_contact_2().is_some() {
                    self.contact_points.push(manifold.get_contact_2().unwrap());
                }
            }
        }
    }

    /// Checks if 2 bodies collide
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, World, geometry::Circle};
    /// 
    /// let mut world = World::new();
    /// 
    /// let mut body1 = Rigid::new_circle(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// let mut body2 = Rigid::new_circle(Vector2 { x: 0.5, y: 0.5 }, 1.0, false, 0.5, Circle { radius: 1.0 });
    /// 
    /// world.add_body(body1);
    /// world.add_body(body2);
    /// 
    /// let collision = World::collide(&mut body1, &mut body2);
    /// assert!(collision.is_some());
    /// ```
    pub fn collide(body_a: &mut Rigid, body_b: &mut Rigid) -> Option<Collision> {
        match (body_a.get_shape(), body_b.get_shape()) {
            (RigidType::Circle(c1), RigidType::Circle(c2)) => intersect_circles(body_a.get_position(), c1.radius, body_b.get_position(), c2.radius),
            (RigidType::Box(_), RigidType::Box(_)) => intersect_polygons(body_a.get_transformed_vertices(), body_b.get_transformed_vertices()),
            (RigidType::Box(_), RigidType::Circle(c)) => intersect_polygon_circle(body_b.get_position(), c.radius, body_a.get_transformed_vertices()).map(|c| Collision { normal: -c.normal, depth: c.depth }),
            (RigidType::Circle(c), RigidType::Box(_)) => intersect_polygon_circle(body_a.get_position(), c.radius, body_b.get_transformed_vertices()),
        }
    }

    fn resolve_collision(&mut self, contact: Manifold) {
        let (left, right) = self.bodies.split_at_mut(contact.get_body_b_index());
        let body_a = &mut left[contact.get_body_a_index()];
        let body_b = &mut right[0];
        let normal = contact.get_normal();
        let _depth = contact.get_depth();

        let relative_velocity = body_b.get_linear_velocity() - body_a.get_linear_velocity();

        if relative_velocity.dot(&normal) > 0.0 {
            return;
        }

        let e = body_a.get_restitution().min(body_b.get_restitution());
        let mut j = -(1.0 + e) * relative_velocity.dot(&normal);
        j /= body_a.get_inverse_mass() + body_b.get_inverse_mass();

        let impulse = normal * j;

        body_a.set_linear_velocity(body_a.get_linear_velocity() - impulse * body_a.get_inverse_mass());
        body_b.set_linear_velocity(body_b.get_linear_velocity() + impulse * body_b.get_inverse_mass());
    }
}