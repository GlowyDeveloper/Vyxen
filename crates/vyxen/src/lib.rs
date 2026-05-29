use vyxen_geometry::aabb::AABB;
use vyxen_math::Vector2;
use vyxen_physics2d::{bodies::Rigid, collision::{Collision, ContactPoints, Manifold}};

pub mod math {
    pub use vyxen_math::*;
}

pub mod geometry {
    pub use vyxen_geometry::*;
}

pub mod physics2d {
    pub use vyxen_physics2d::*;
}

/// World struct used throughout the engine
/// 
/// # Examples
/// ```rust
/// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
/// 
/// let mut world = World::new();
/// 
/// let body = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
/// world.add_body(body.clone());
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
    contact_pairs: Vec<(usize, usize)>,
    gravity: Vector2,
}

impl World {
    /// Generates a new world
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// world.add_body(body.clone());
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
            contact_pairs: Vec::new(),
            gravity: Vector2 { x: 0.0, y: -9.81 }
        }
    }

    /// Adds a body to the world
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// world.add_body(body.clone());
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
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// world.add_body(body.clone());
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
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body1 = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// world.add_body(body1.clone());
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
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body1 = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// world.add_body(body1.clone());
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
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
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
    /// use vyxen::{math::Vector2, physics2d::bodies::Rigid, geometry::shapes::Circle, World};
    /// 
    /// let mut world = World::new();
    /// 
    /// let body = Rigid::new(Vector2 { x: 0.0, y: 0.0 }, 1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// world.add_body(body);
    /// 
    /// world.step(0.1, 10);
    /// ```
    pub fn step(&mut self, dt: f32, iterations: usize) {
        for _ in 0..iterations {
            self.step_bodies(dt, iterations);
            self.contact_pairs.clear();
            self.broad_phase();
            self.narrow_phase();
        }
    }

    fn step_bodies(&mut self, dt: f32, iterations: usize) {
        for i in 0..self.bodies.len() {
            let body = &mut self.bodies[i];
            body.step(dt / iterations as f32, self.gravity);
        }
    }

    fn broad_phase(&mut self) {
        let len = self.bodies.len();
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

                if !AABB::intersect_aabb(body_a_aabb, body_b_aabb) {
                    continue;
                }

                self.contact_pairs.push((i, j));
            }
        }
    }

    fn narrow_phase(&mut self) {
        for i in 0..self.contact_pairs.len() {
            let (j, k) = self.contact_pairs[i];

            for manifold in self.generate_manifolds(j, k) {
                self.resolve_collision(manifold);

                let (left, right) = self.bodies.split_at_mut(k);
                let body_a = &mut left[j];
                let body_b = &mut right[0];

                Self::seperate_bodies(body_a, body_b, manifold.get_depth(), manifold.get_normal());
            }
        }
    }

    fn seperate_bodies(body_a: &mut Rigid, body_b: &mut Rigid, depth: f32, normal: Vector2) {
        const PERCENT: f32 = 0.8;
        const SLOP: f32 = 0.01;

        let correction_mag = (depth - SLOP).max(0.0) * PERCENT;
        let correction = normal * correction_mag;
        let total_inv_mass = body_b.get_inverse_mass() + body_a.get_inverse_mass();

        if body_a.is_static() {
            body_b.move_by(correction * body_b.get_inverse_mass() / total_inv_mass);
        } else if body_b.is_static() {
            body_a.move_by(-correction * body_a.get_inverse_mass() / total_inv_mass);
        } else {
            body_a.move_by(-correction * body_a.get_inverse_mass() / total_inv_mass);
            body_b.move_by(correction * body_b.get_inverse_mass() / total_inv_mass);
        }
    }

    fn generate_manifolds(&mut self, body_a_index: usize, body_b_index: usize) -> Vec<Manifold> {
        let (left, right) = self.bodies.split_at_mut(body_b_index);
        let body_a = &mut left[body_a_index];
        let body_b = &mut right[0];

        let collisions = Collision::collide(body_a, body_b);

        let mut manifolds = Vec::new();

        for collision in collisions {
            let contacts = ContactPoints::find_contact_points(body_a, body_b);

            manifolds.push(Manifold::new(body_a_index, body_b_index, collision.normal, collision.depth, contacts.contact_1, contacts.contact_2));
        }

        manifolds
    }

    fn resolve_collision(&mut self, contact: Manifold) {
        let (left, right) = self.bodies.split_at_mut(contact.get_body_b_index());
        let body_a = &mut left[contact.get_body_a_index()];
        let body_b = &mut right[0];
        let normal = contact.get_normal();
        let _depth = contact.get_depth();
        let contact_1 = contact.get_contact_1();
        let contact_2 = contact.get_contact_2();
        let contact_count = if contact_2.is_none() { 1 } else { 2 };

        let sf = (body_a.get_static_friction() + body_b.get_static_friction()) / 2.0;
        let df = (body_a.get_dynamic_friction() + body_b.get_dynamic_friction()) / 2.0;

        let mut impulse_vec: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];
        let mut friction_impulse_vec: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];

        let e = body_a.get_restitution().min(body_b.get_restitution());

        let mut ra: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];
        let mut rb: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];
        let mut js: Vec<f32> = vec![0.0, 0.0];

        for i in 0..contact_count {
            let contact = if i == 0 { contact_1 } else { contact_2 };
            if contact.is_none() {
                continue;
            }

            ra[i] = contact.unwrap() - body_a.get_position();
            rb[i] = contact.unwrap() - body_b.get_position();

            let ra_prep = Vector2 { x: -ra[i].y, y: ra[i].x };
            let rb_prep = Vector2 { x: -rb[i].y, y: rb[i].x };

            let rotation_velocity_body_a = ra_prep * body_a.get_rotational_velocity();
            let rotation_velocity_body_b = rb_prep * body_b.get_rotational_velocity();

            let relative_velocity = (body_b.get_linear_velocity() + rotation_velocity_body_b) - (body_a.get_linear_velocity() + rotation_velocity_body_a);

            let velocity_magnitude = relative_velocity.dot(&normal);

            if velocity_magnitude > 0.0 {
                continue;
            }

            let ra_prep_dot_n = ra_prep.dot(&normal);
            let rb_prep_dot_n = rb_prep.dot(&normal);

            let denomenator = body_a.get_inverse_mass() + body_b.get_inverse_mass() +
                (ra_prep_dot_n * ra_prep_dot_n) * body_a.get_inverse_inertia() +
                (rb_prep_dot_n * rb_prep_dot_n) * body_b.get_inverse_inertia();

            let mut j = -(1.0 + e) * velocity_magnitude;
            j /= denomenator;
            j /= contact_count as f32;

            js[i] = j;

            let impulse = normal * j;
            impulse_vec[i] = impulse
        }

        for i in 0..contact_count {
            let impulse = impulse_vec[i];
            body_a.set_linear_velocity(body_a.get_linear_velocity() + -impulse * body_a.get_inverse_mass());
            body_a.set_rotational_velocity(body_a.get_rotational_velocity() + -ra[i].cross(&impulse) * body_a.get_inverse_inertia());
            body_b.set_linear_velocity(body_b.get_linear_velocity() + impulse * body_b.get_inverse_mass());
            body_b.set_rotational_velocity(body_b.get_rotational_velocity() + rb[i].cross(&impulse) * body_b.get_inverse_inertia());
        }

        for i in 0..contact_count {
            let contact = if i == 0 { contact_1 } else { contact_2 };
            if contact.is_none() {
                continue;
            }

            ra[i] = contact.unwrap() - body_a.get_position();
            rb[i] = contact.unwrap() - body_b.get_position();

            let ra_prep = Vector2 { x: -ra[i].y, y: ra[i].x };
            let rb_prep = Vector2 { x: -rb[i].y, y: rb[i].x };

            let rotation_velocity_body_a = ra_prep * body_a.get_rotational_velocity();
            let rotation_velocity_body_b = rb_prep * body_b.get_rotational_velocity();

            let relative_velocity = (body_b.get_linear_velocity() + rotation_velocity_body_b) - (body_a.get_linear_velocity() + rotation_velocity_body_a);

            let tangent = relative_velocity - normal * relative_velocity.dot(&normal);
            if tangent.is_nearly_equal(&Vector2::zero()) {
                continue;
            }
            let tangent_normalized = tangent.normalize();

            let ra_prep_dot_t = ra_prep.dot(&tangent_normalized);
            let rb_prep_dot_t = rb_prep.dot(&tangent_normalized);

            let denomenator = body_a.get_inverse_mass() + body_b.get_inverse_mass() +
                (ra_prep_dot_t * ra_prep_dot_t) * body_a.get_inverse_inertia() +
                (rb_prep_dot_t * rb_prep_dot_t) * body_b.get_inverse_inertia();

            let mut jt = -relative_velocity.dot(&tangent_normalized);
            jt /= denomenator;
            jt /= contact_count as f32;

            let impulse = if jt.abs() <= js[i] * sf {
                tangent_normalized * jt
            } else {
                tangent_normalized * -js[i] * df
            };

            friction_impulse_vec[i] = impulse
        }

        for i in 0..contact_count {
            let impulse = friction_impulse_vec[i];
            body_a.set_linear_velocity(body_a.get_linear_velocity() + -impulse * body_a.get_inverse_mass());
            body_a.set_rotational_velocity(body_a.get_rotational_velocity() + -ra[i].cross(&impulse) * body_a.get_inverse_inertia());
            body_b.set_linear_velocity(body_b.get_linear_velocity() + impulse * body_b.get_inverse_mass());
            body_b.set_rotational_velocity(body_b.get_rotational_velocity() + rb[i].cross(&impulse) * body_b.get_inverse_inertia());
        }
    }
}