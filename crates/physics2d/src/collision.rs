use vyxen_geometry::aabb::AABB;
use vyxen_math::Vector2;

/// Information about a collision between two rigid bodies.
///  - `normal` is the direction of the collision
///  - `depth` is how much the two bodies are penetrating each other.
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::collision::Collision;
/// 
/// let collision = Collision {
///   normal: Vector2 { x: 1.0, y: 0.0 },
///   depth: 1.0,
/// };
/// ```
pub struct Collision {
    pub normal: Vector2,
    pub depth: f32,
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