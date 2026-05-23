use vyxen_geometry::aabb::AABB;
use vyxen_math::{Vector2, is_nearly_equal};

use crate::bodies::{Rigid, RigidType};

/// Information about a collision between two rigid bodies.
///  - `normal` is the direction of the collision
///  - `depth` is how much the two bodies are penetrating each other.
pub struct Collision {
    pub normal: Vector2,
    pub depth: f32,
}

/// A collision manifold
/// Used to store data about a collision
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Manifold {
    body_a_index: usize,
    body_b_index: usize,
    normal: Vector2,
    depth: f32,
    contact_1: Option<Vector2>,
    contact_2: Option<Vector2>
}

impl Manifold {
    /// Creates a new collision manifold
    pub fn new(body_a_index: usize, body_b_index: usize, normal: Vector2, depth: f32, contact_1: Option<Vector2>, contact_2: Option<Vector2>) -> Self {
        Manifold { body_a_index, body_b_index, normal, depth, contact_1, contact_2 }
    }

    /// Gets the index of body a
    pub fn get_body_a_index(&self) -> usize {
        self.body_a_index
    }
    /// Gets the index of body b
    pub fn get_body_b_index(&self) -> usize {
        self.body_b_index
    }
    /// Gets the normal of the collision
    pub fn get_normal(&self) -> Vector2 {
        self.normal
    }
    /// Gets the depth of the collision
    pub fn get_depth(&self) -> f32 {
        self.depth
    }
    /// Gets the first `Vector2` contact point of the collision
    pub fn get_contact_1(&self) -> Option<Vector2> {
        self.contact_1
    }
    /// Gets the second `Vector2` contact point of the collision
    pub fn get_contact_2(&self) -> Option<Vector2> {
        self.contact_2
    }
}

/// Finds the contact points bodies
/// 
/// # Examples
/// 
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::{bodies::Rigid, collision::find_contact_points};
/// use vyxen_geometry::Circle;
/// 
/// let mut rigid_1 = Rigid::new_circle(Vector2 { x: 2.0, y: 3.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
/// let mut rigid_2 = Rigid::new_circle(Vector2 { x: 2.0, y: 4.0 }, 1.0, false, 0.5, Circle { radius: 1.0 });
/// 
/// let (point_1, point_2) = find_contact_points(&mut rigid_1, &mut rigid_2);
/// assert!(point_1.is_some());
/// assert!(point_2.is_none()); // Circles can only have 1 contact point.
/// ```
pub fn find_contact_points(body_a: &mut Rigid, body_b: &mut Rigid) -> (Option<Vector2>, Option<Vector2>) {
    match (body_a.get_shape(), body_b.get_shape()) {
        (RigidType::Circle(c1), RigidType::Circle(_)) => {
            let contact = find_contact_point_circle_to_circle(body_a.get_position(), c1.radius, body_b.get_position());
            return (Some(contact), None)
        },
        (RigidType::Box(_), RigidType::Box(_)) => {
            return find_contact_points_polygon_to_polygon(body_a.get_transformed_vertices(), body_b.get_transformed_vertices());
        },
        (RigidType::Box(_), RigidType::Circle(_)) => {
            let contact = find_contact_point_circle_to_polygon(body_b.get_position(), body_a.get_transformed_vertices());
            return (Some(contact), None);
        },
        (RigidType::Circle(_), RigidType::Box(_)) => {
            let contact = find_contact_point_circle_to_polygon(body_a.get_position(), body_b.get_transformed_vertices());
            return (Some(contact), None);
        },
    }
}

fn find_contact_points_polygon_to_polygon(vertices_a: &[Vector2], vertices_b: &[Vector2]) -> (Option<Vector2>, Option<Vector2>) {
    let mut contact_1: Option<Vector2> = None;
    let mut contact_2: Option<Vector2> = None;
    
    let mut min_distance_squared = std::f32::MAX;

    for i in 0..vertices_a.len() {
        let p = vertices_a[i];
        
        for j in 0..vertices_b.len() {
            let va = vertices_b[j];
            let vb = vertices_b[(j + 1) % vertices_b.len()];

            let (distance_squared, contact) = point_segement_distance(p, va, vb);

            if is_nearly_equal(distance_squared, min_distance_squared) {
                if !contact.is_nearly_equal(&contact_1.unwrap_or(Vector2::zero())) {
                    contact_2 = Some(contact);
                }
            } else if distance_squared < min_distance_squared {
                min_distance_squared = distance_squared;
                contact_1 = Some(contact);
            }
        }
    }

    for i in 0..vertices_b.len() {
        let p = vertices_b[i];
        
        for j in 0..vertices_a.len() {
            let va = vertices_a[j];
            let vb = vertices_a[(j + 1) % vertices_a.len()];

            let (distance_squared, contact) = point_segement_distance(p, va, vb);

            if is_nearly_equal(distance_squared, min_distance_squared) {
                if !contact.is_nearly_equal(&contact_1.unwrap_or(Vector2::zero())) {
                    contact_2 = Some(contact);
                }
            } else if distance_squared < min_distance_squared {
                min_distance_squared = distance_squared;
                contact_1 = Some(contact);
            }
        }
    }

    (contact_1, contact_2)
}

fn find_contact_point_circle_to_polygon(circle_center: Vector2, vertices: &[Vector2]) -> Vector2 {
    let mut min_distance_squared = std::f32::MAX;
    let mut contact_point = Vector2::zero();
    
    for i in 0..vertices.len() {
        let va = vertices[i];
        let vb = vertices[(i + 1) % vertices.len()];

        let (distance_squared, contact) = point_segement_distance(circle_center, va, vb);

        if distance_squared < min_distance_squared {
            min_distance_squared = distance_squared;
            contact_point = contact;
        }
    }

    contact_point
}

fn find_contact_point_circle_to_circle(center_a: Vector2, radius_a: f32, center_b: Vector2) -> Vector2 {
    let ab = center_b - center_a;
    let dir = ab.normalize();
    center_a + dir * radius_a
}

fn point_segement_distance(p: Vector2, a: Vector2, b: Vector2) -> (f32, Vector2) {
    let ab = b - a;
    let ap = p - a;

    let proj = ap.dot(&ab);
    let ab_len_squared = ab.length_squared();
    let d = proj / ab_len_squared;

    let contact = if d <= 0.0 {
        a
    } else if d >= 1.0 {
        b
    } else {
        a + ab * d
    }; 

    return (p.distance_squared(&contact), contact);
}

/// Checks for collision between two circles and returns the collision information if they are colliding.
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::collision::{Collision, intersect_circles};
/// 
/// let center_a = Vector2 { x: 0.0, y: 0.0 };
/// let radius_a = 1.0;
/// 
/// let center_b = Vector2 { x: 1.0, y: 0.0 };
/// let radius_b = 1.0;
/// 
/// let collision = intersect_circles(center_a, radius_a, center_b, radius_b);
/// assert!(collision.is_some());
/// 
/// let collision_info = collision.unwrap();
/// assert_eq!(collision_info.normal, Vector2 { x: 1.0, y: 0.0 });
/// assert_eq!(collision_info.depth, 1.0);
/// ```
pub fn intersect_circles(center_a: Vector2, radius_a: f32, center_b: Vector2, radius_b: f32) -> Option<Collision> {
    let distance = center_a.distance(&center_b);
    let radius_sum = radius_a + radius_b;

    if distance >= radius_sum {
        return None;
    }

    let normal = (center_b - center_a).normalize();
    let depth = radius_sum - distance;

    return Some(Collision { normal, depth });
}

/// Checks **if** two `AABB`s intersect eachother
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_geometry::aabb::AABB;
/// use vyxen_physics2d::collision::intersect_aabb;
///
/// let a = AABB::new(
///     Vector2 { x: 0.0, y: 0.0 },
///     Vector2 { x: 1.0, y: 1.0 },
/// );
/// 
/// let b = AABB::new(
///     Vector2 { x: 0.5, y: 0.5 },
///     Vector2 { x: 1.5, y: 1.5 },
/// );
/// 
/// assert!(intersect_aabb(a, b));
/// ```
pub fn intersect_aabb(aabb_a: AABB, aabb_b: AABB) -> bool {
    if aabb_a.get_max().x <= aabb_b.get_min().x ||
        aabb_b.get_max().x <= aabb_a.get_min().x ||
        aabb_a.get_max().y <= aabb_b.get_min().y ||
        aabb_b.get_max().y <= aabb_a.get_min().y {
        return false;
    }
    return true;
}

/// Checks for collision between two convex polygons and returns the collision information if they are colliding.
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::collision::{Collision, intersect_polygons};
/// 
/// let vertices_a = vec![
///     Vector2 { x: 0.0, y: 0.0 },
///     Vector2 { x: 1.0, y: 0.0 },
///     Vector2 { x: 1.0, y: 1.0 },
///     Vector2 { x: 0.0, y: 1.0 },
/// ];
/// 
/// let vertices_b = vec![
///     Vector2 { x: 0.5, y: 0.5 },
///     Vector2 { x: 1.5, y: 0.5 },
///     Vector2 { x: 1.5, y: 1.5 },
///     Vector2 { x: 0.5, y: 1.5 },
/// ];
/// 
/// let collision = intersect_polygons(&vertices_a, &vertices_b);
/// assert!(collision.is_some());
/// 
/// let collision_info = collision.unwrap();
/// assert_eq!(collision_info.normal, Vector2 { x: 0.0, y: 1.0 });
/// assert_eq!(collision_info.depth, 0.5);
/// ```
pub fn intersect_polygons(vertices_a: &[Vector2], vertices_b: &[Vector2]) -> Option<Collision> {
    let mut depth = std::f32::MAX;
    let mut normal = Vector2::zero();

    for i in 0..vertices_a.len() {
        let va = vertices_a[i];
        let vb = vertices_a[(i + 1) % vertices_a.len()];
        let edge = vb - va;
        let axis = Vector2 { x: -edge.y, y: edge.x }.normalize();

        let (min_a, max_a) = project_vertices(vertices_a, axis);
        let (min_b, max_b) = project_vertices(vertices_b, axis);

        if min_a >= max_b || min_b >= max_a {
            return None;
        }

        let axis_depth = (max_b - min_a).min(max_a - min_b);

        if axis_depth < depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    for i in 0..vertices_b.len() {
        let va = vertices_b[i];
        let vb = vertices_b[(i + 1) % vertices_b.len()];
        let edge = vb - va;
        let axis = Vector2 { x: -edge.y, y: edge.x }.normalize();

        let (min_a, max_a) = project_vertices(vertices_a, axis);
        let (min_b, max_b) = project_vertices(vertices_b, axis);

        if min_a >= max_b || min_b >= max_a {
            return None;
        }

        let axis_depth = (max_b - min_a).min(max_a - min_b);

        if axis_depth < depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    let center_a = find_mean(vertices_a);
    let center_b = find_mean(vertices_b);
    let direction = center_b - center_a;

    if direction.dot(&normal) < 0.0 {
        normal = -normal;
    }

    Some(Collision { normal, depth })
}

/// Checks for collision between a convex polygon and a circle and returns the collision information if they are colliding.
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::collision::{Collision, intersect_polygon_circle};
/// 
/// let vertices = vec![
///     Vector2 { x: 0.0, y: 0.0 },
///     Vector2 { x: 1.0, y: 0.0 },
///     Vector2 { x: 1.0, y: 1.0 },
///     Vector2 { x: 0.0, y: 1.0 },
/// ];
/// 
/// let circle_center = Vector2 { x: 0.0, y: 0.0 };
/// let circle_radius = 0.5;
/// 
/// let collision = intersect_polygon_circle(circle_center, circle_radius, &vertices);
/// assert!(collision.is_some());
/// 
/// let collision_info = collision.unwrap();
/// assert_eq!(collision_info.normal, Vector2 { x: 0.0, y: 1.0 });
/// assert_eq!(collision_info.depth, 0.5);
/// ```
pub fn intersect_polygon_circle(circle_center: Vector2, circle_radius: f32, vertices: &[Vector2]) -> Option<Collision> {
    let mut depth = std::f32::MAX;
    let mut normal = Vector2::zero();

    for i in 0..vertices.len() {
        let va = vertices[i];
        let vb = vertices[(i + 1) % vertices.len()];
        let edge = vb - va;
        let axis = Vector2 { x: -edge.y, y: edge.x }.normalize();

        let (min_a, max_a) = project_vertices(vertices, axis);
        let (min_b, max_b) = project_circle(circle_center, circle_radius, axis);

        if min_a >= max_b || min_b >= max_a {
            return None;
        }

        let axis_depth = (max_b - min_a).min(max_a - min_b);

        if axis_depth < depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    let closest_point_index = find_closest_point_on_polygon(circle_center, vertices);

    let closest_point = vertices[closest_point_index as usize];
    let axis = closest_point - circle_center;

    let (min_a, max_a) = project_vertices(vertices, axis);
    let (min_b, max_b) = project_circle(circle_center, circle_radius, axis);

    if min_a >= max_b || min_b >= max_a {
        return None;
    }

    let axis_depth = (max_b - min_a).min(max_a - min_b);

    if axis_depth < depth {
        depth = axis_depth;
        normal = axis;
    }

    let polygon_center = find_mean(vertices);
    let direction = polygon_center - circle_center;

    if direction.dot(&normal) < 0.0 {
        normal = -normal;
    }

    Some(Collision { normal, depth })
}

fn project_vertices(vertices: &[Vector2], axis: Vector2) -> (f32, f32) {
    let mut min = std::f32::MAX;
    let mut max = std::f32::MIN;

    for i in 0..vertices.len() {
        let projection = vertices[i].dot(&axis);
        if projection < min {
            min = projection;
        }
        if projection > max {
            max = projection;
        }
    }

    (min, max)
}

fn project_circle(center: Vector2, radius: f32, axis: Vector2) -> (f32, f32) {
    let direction = axis.normalize();
    let direction_and_radius = direction * radius;

    let p1 = center + direction_and_radius;
    let p2 = center - direction_and_radius;

    let min = p1.dot(&axis);
    let max = p2.dot(&axis);

    if min < max {
        (min, max)
    } else {
        (max, min)
    }
}

fn find_mean(vertices: &[Vector2]) -> Vector2 {
    let mut sum = Vector2::zero();
    for i in 0..vertices.len() {
        sum = sum + vertices[i];
    }
    sum / (vertices.len() as f32)
}

fn find_closest_point_on_polygon(circle_center: Vector2, vertices: &[Vector2]) -> i32 {
    let mut result = -1;
    let mut min_distance = std::f32::MAX;

    for i in 0..vertices.len() {
        let v = vertices[i];
        let distance = v.distance(&circle_center);
        if distance < min_distance {
            min_distance = distance;
            result = i as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use vyxen_math::Vector2;

    #[test]
    fn test_intersect_circles() {
        let center_a = Vector2 { x: 0.0, y: 0.0 };
        let radius_a = 1.0;

        let center_b = Vector2 { x: 1.0, y: 0.0 };
        let radius_b = 1.0;

        let collision = intersect_circles(center_a, radius_a, center_b, radius_b);

        assert!(collision.is_some());

        let collision_info = collision.unwrap();

        assert_eq!(collision_info.normal, Vector2 { x: 1.0, y: 0.0 });
        assert_eq!(collision_info.depth, 1.0);
    }

    #[test]
    fn test_intersect_polygons() {
        let vertices_a = vec![
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 1.0, y: 0.0 },
            Vector2 { x: 1.0, y: 1.0 },
            Vector2 { x: 0.0, y: 1.0 },
        ];

        let vertices_b = vec![
            Vector2 { x: 0.5, y: 0.5 },
            Vector2 { x: 1.5, y: 0.5 },
            Vector2 { x: 1.5, y: 1.5 },
            Vector2 { x: 0.5, y: 1.5 },
        ];

        let collision = intersect_polygons(&vertices_a, &vertices_b);
        assert!(collision.is_some());
    }

    #[test]
    fn test_intersect_polygon_circle() {
        let vertices = vec![
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 1.0, y: 0.0 },
            Vector2 { x: 1.0, y: 1.0 },
            Vector2 { x: 0.0, y: 1.0 },
        ];

        let circle_center = Vector2 { x: 0.5, y: 0.5 };
        let circle_radius = 0.5;

        let collision = intersect_polygon_circle(circle_center, circle_radius, &vertices);
        assert!(collision.is_some());
    }
}