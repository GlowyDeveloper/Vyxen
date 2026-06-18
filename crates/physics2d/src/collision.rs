use vyxen_geometry::{Polygon, ShapeType};
use vyxen_math::{Vector2, is_nearly_equal};

/// Information about a collision between two rigid bodies.
///  - `normal` is the direction of the collision
///  - `depth` is how much the two bodies are penetrating each other.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collision {
    pub normal: Vector2,
    pub depth: f32,
}

impl Collision {
    /// Checks if 2 bodies collide
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::{Rigid, Collision};
    /// use vyxen_geometry::Circle;
    /// 
    /// let mut body1 = Rigid::generate_shape_type_from_shape(Circle::new(1.0));
    /// let mut body2 = Rigid::generate_shape_type_from_shape(Circle::new(1.0));
    /// 
    /// let collision = Collision::collide(&mut body1, Vector2 { x: 0.0, y: 0.0 }, 0.0,
    ///     &mut body2, Vector2 { x: 0.5, y: 0.5 }, 0.0);
    /// 
    /// assert!(collision.len() > 0);
    /// ```
    pub fn collide(shape_a: &mut ShapeType, position_a: Vector2, rotation_a: f32, shape_b: &mut ShapeType, position_b: Vector2, rotation_b: f32) -> Vec<Collision> {
        match (shape_a, shape_b) {
            (ShapeType::Circle(c1), ShapeType::Circle(c2)) =>
                Self::intersect_circles(position_a, c1.get_radius(), position_b, c2.get_radius()),
            (ShapeType::Box(a), ShapeType::Box(b)) =>
                Self::intersect_polygons(a.get_transformed_vertices(position_a, rotation_a), position_a,
                    b.get_transformed_vertices(position_b, rotation_b), position_b),
            (ShapeType::Polygon(a), ShapeType::Polygon(b)) =>
                Self::intersect_polygons(a.get_transformed_vertices(position_a, rotation_a), position_a,
                    b.get_transformed_vertices(position_b, rotation_b), position_b),

            (ShapeType::Box(b), ShapeType::Polygon(p)) =>
                Self::intersect_polygons(b.get_transformed_vertices(position_a, rotation_a), position_a,
                    p.get_transformed_vertices(position_b, rotation_b), position_b),
            (ShapeType::Polygon(p), ShapeType::Box(b)) =>
                Self::intersect_polygons(p.get_transformed_vertices(position_a, rotation_a), position_a,
                    b.get_transformed_vertices(position_b, rotation_b), position_b),

            (ShapeType::Box(b), ShapeType::Circle(c)) =>
                Self::intersect_polygon_circle(position_b, c.get_radius(),
                    b.get_transformed_vertices(position_a, rotation_a), position_a)
                    .into_iter()
                    .map(|c| Collision { normal: -c.normal, depth: c.depth })
                    .collect(),
            (ShapeType::Polygon(p), ShapeType::Circle(c)) =>
                Self::intersect_polygon_circle(position_b, c.get_radius(),
                    p.get_transformed_vertices(position_a, rotation_a), position_a)
                    .into_iter()
                    .map(|c| Collision { normal: -c.normal, depth: c.depth })
                    .collect(),
            (ShapeType::Circle(c), ShapeType::Box(b)) =>
                Self::intersect_polygon_circle(position_a, c.get_radius(),
                    b.get_transformed_vertices(position_b, rotation_b), position_b),
            (ShapeType::Circle(c), ShapeType::Polygon(p)) =>
                Self::intersect_polygon_circle(position_a, c.get_radius(),
                    p.get_transformed_vertices(position_b, rotation_b), position_b),

            (ShapeType::Circle(c), ShapeType::Concave(p)) =>
                Self::collide_concave_with_circle(&p, position_a, c.get_radius(), position_b, rotation_b),

            (ShapeType::Concave(p), ShapeType::Circle(c)) =>
                Self::collide_concave_with_circle(&p, position_b, c.get_radius(), position_a, rotation_a)
                    .into_iter()
                    .map(|c| Collision { normal: -c.normal, depth: c.depth })
                    .collect(),

            (ShapeType::Concave(a), ShapeType::Concave(b)) =>
                Self::collide_concave_with_concave(&a, &b, position_a, position_b, rotation_a, rotation_b),

            (ShapeType::Concave(a), ShapeType::Polygon(p)) =>
                Self::collide_concave_with_polygon(&a, p.get_transformed_vertices(position_b, rotation_b),
                    position_a, position_b, rotation_a),
            (ShapeType::Concave(a), ShapeType::Box(b)) =>
                Self::collide_concave_with_polygon(&a, b.get_transformed_vertices(position_b, rotation_b),
                    position_a, position_b, rotation_a),
            (ShapeType::Polygon(p), ShapeType::Concave(b)) =>
                Self::collide_concave_with_polygon(&b, p.get_transformed_vertices(position_a, rotation_a),
                    position_b, position_a, rotation_b)
                    .into_iter()
                    .map(|c| Collision { normal: -c.normal, depth: c.depth })
                    .collect(),
            (ShapeType::Box(b), ShapeType::Concave(c)) =>
                Self::collide_concave_with_polygon(&c, b.get_transformed_vertices(position_a, rotation_a),
                    position_b, position_a, rotation_b)
                    .into_iter()
                    .map(|c| Collision { normal: -c.normal, depth: c.depth })
                    .collect(),
        }
    }

    fn intersect_circles(center_a: Vector2, radius_a: f32, center_b: Vector2, radius_b: f32) -> Vec<Collision> {
        let distance = center_a.distance(&center_b);
        let radius_sum = radius_a + radius_b;

        if distance >= radius_sum {
            return vec![];
        }

        let mut normal = center_b - center_a;
        if normal.length_squared() <= f32::EPSILON {
            normal = Vector2 { x: 1.0, y: 0.0 };
        } else {
            normal = normal / normal.length();
        }
        let depth = radius_sum - distance;

        return vec![Collision { normal, depth }];
    }

    fn intersect_polygons(vertices_a: &[Vector2], vertices_center_a: Vector2, vertices_b: &[Vector2], vertices_center_b: Vector2) -> Vec<Collision> {
        let mut depth = std::f32::MAX;
        let mut normal = Vector2::zero();

        for i in 0..vertices_a.len() {
            let va = vertices_a[i];
            let vb = vertices_a[(i + 1) % vertices_a.len()];
            let edge = vb - va;
            let mut axis = Vector2 { x: -edge.y, y: edge.x };
            if axis.length_squared() <= f32::EPSILON { continue; }
            axis = axis / axis.length();

            let (min_a, max_a) = Self::project_vertices(vertices_a, axis);
            let (min_b, max_b) = Self::project_vertices(vertices_b, axis);

            if min_a > max_b || min_b > max_a {
                return vec![];
            }

            let axis_depth = f32::min(max_a, max_b) - f32::max(min_a, min_b);

            if axis_depth < depth {
                depth = axis_depth;
                normal = axis;
            }
        }

        for i in 0..vertices_b.len() {
            let va = vertices_b[i];
            let vb = vertices_b[(i + 1) % vertices_b.len()];
            let edge = vb - va;
            let mut axis = Vector2 { x: -edge.y, y: edge.x };
            if axis.length_squared() <= f32::EPSILON { continue; }
            axis = axis / axis.length();

            let (min_a, max_a) = Self::project_vertices(vertices_a, axis);
            let (min_b, max_b) = Self::project_vertices(vertices_b, axis);

            if min_a > max_b || min_b > max_a {
                return vec![];
            }

            let axis_depth = f32::min(max_a, max_b) - f32::max(min_a, min_b);

            if axis_depth < depth {
                depth = axis_depth;
                normal = axis;
            }
        }

        let direction = vertices_center_b - vertices_center_a;

        if direction.dot(&normal) < 0.0 {
            normal = -normal;
        }

        vec![Collision { normal, depth }]
    }

    fn intersect_polygon_circle(circle_center: Vector2, circle_radius: f32, vertices: &[Vector2], vertices_center: Vector2) -> Vec<Collision> {
        let mut depth = std::f32::MAX;
        let mut normal = Vector2::zero();

        for i in 0..vertices.len() {
            let va = vertices[i];
            let vb = vertices[(i + 1) % vertices.len()];
            let edge = vb - va;
            let mut axis = Vector2 { x: -edge.y, y: edge.x };
            if axis.length_squared() <= f32::EPSILON { continue; }
            axis = axis / axis.length();

            let (min_a, max_a) = Self::project_vertices(vertices, axis);
            let (min_b, max_b) = Self::project_circle(circle_center, circle_radius, axis);

            if min_a > max_b || min_b > max_a {
                return vec![];
            }

            let axis_depth = f32::min(max_a, max_b) - f32::max(min_a, min_b);

            if axis_depth < depth {
                depth = axis_depth;
                normal = axis;
            }
        }

        let closest_point_index_opt = Self::find_closest_point_on_polygon(circle_center, vertices);
        let closest_point_index = match closest_point_index_opt {
            Some(i) => i,
            None => return vec![],
        };

        let closest_point = vertices[closest_point_index];

        let axis_vec = closest_point - circle_center;

        if axis_vec.length_squared() <= f32::EPSILON {
            return vec![Collision { normal: Vector2 { x: 1.0, y: 0.0 }, depth: circle_radius }];
        }

        let mut axis = axis_vec;
        if axis.length_squared() <= f32::EPSILON {
            return vec![Collision { normal: Vector2 { x: 1.0, y: 0.0 }, depth: circle_radius }];
        }
        axis = axis / axis.length();

        let (min_a, max_a) = Self::project_vertices(vertices, axis);
        let (min_b, max_b) = Self::project_circle(circle_center, circle_radius, axis);

        if min_a > max_b || min_b > max_a {
            return vec![];
        }

        let axis_depth = f32::min(max_a, max_b) - f32::max(min_a, min_b);

        if axis_depth < depth {
            depth = axis_depth;
            normal = axis;
        }

        let direction = vertices_center - circle_center;

        if direction.dot(&normal) < 0.0 {
            normal = -normal;
        }

        vec![Collision { normal, depth, }]
    }

    fn collide_concave_with_polygon(concave: &[Polygon], convex_vertices: &[Vector2], concave_pos: Vector2, convex_pos: Vector2, concave_rotation: f32) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for poly in concave {
            let mut transformed = poly.clone();

            let verts = transformed.get_transformed_vertices(concave_pos, concave_rotation);

            let collision = Collision::intersect_polygons(verts, concave_pos, convex_vertices, convex_pos);
            collision.iter().for_each(|c| collisions.push((*c).clone()));
        }

        collisions
    }

    fn collide_concave_with_circle(concave: &[Polygon], circle_center: Vector2, radius: f32, concave_pos: Vector2, concave_rotation: f32) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for poly in concave {
            let mut transformed = poly.clone();

            let verts = transformed.get_transformed_vertices(concave_pos, concave_rotation);

            let collision = Collision::intersect_polygon_circle(circle_center, radius, verts, concave_pos);
            collision.iter().for_each(|c| collisions.push((*c).clone()));
        }

        collisions
    }

    fn collide_concave_with_concave(a: &[Polygon], b: &[Polygon], pos_a: Vector2, pos_b: Vector2, rotation_a: f32, rotation_b: f32) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for polygon_a in a {
            let mut ta = polygon_a.clone();
            let vertices_a = ta.get_transformed_vertices(pos_a, rotation_a);

            for poly_b in b {
                let mut tb = poly_b.clone();
                let vertices_b = tb.get_transformed_vertices(pos_b, rotation_b);

                let collision = Collision::intersect_polygons(vertices_a, pos_a, vertices_b, pos_b);
                collision.iter().for_each(|c| collisions.push((*c).clone()));
            }
        }

        collisions
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

    fn find_closest_point_on_polygon(circle_center: Vector2, vertices: &[Vector2]) -> Option<usize> {
        if vertices.is_empty() {
            return None;
        }

        let mut found = false;
        let mut result: usize = 0;
        let mut min_distance = std::f32::MAX;

        for (i, &v) in vertices.iter().enumerate() {
            let distance = v.distance(&circle_center);
            if distance.is_nan() {
                continue;
            }
            if !found || distance < min_distance {
                min_distance = distance;
                result = i;
                found = true;
            }
        }

        if found { Some(result) } else { None }
    }
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

/// The contact points of 2 bodies
pub struct ContactPoints {
    pub contact_1: Option<Vector2>,
    pub contact_2: Option<Vector2>
}

impl ContactPoints {
    /// Finds the contact points bodies
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::{Rigid, ContactPoints};
    /// use vyxen_geometry::Circle;
    /// 
    /// let mut rigid_1 = Rigid::generate_shape_type_from_shape(Circle::new(1.0));
    /// let mut rigid_2 = Rigid::generate_shape_type_from_shape(Circle::new(1.0));
    /// 
    /// let contact_points = ContactPoints::find_contact_points(&mut rigid_1, Vector2 { x: 2.0, y: 3.0 }, 0.0,
    ///     &mut rigid_2, Vector2 { x: 2.0, y: 4.0 }, 0.0);
    /// 
    /// assert!(contact_points.contact_1.is_some());
    /// assert!(contact_points.contact_2.is_none()); // Circles can only have 1 contact point.
    /// ```
    pub fn find_contact_points(shape_a: &mut ShapeType, position_a: Vector2, rotation_a: f32, shape_b: &mut ShapeType, position_b: Vector2, rotation_b: f32) -> Self {
        match (shape_a, shape_b) {
            (ShapeType::Circle(c1), ShapeType::Circle(_)) => {
                let contact = Self::find_contact_point_circle_to_circle(position_a, c1.get_radius(), position_b);
                return ContactPoints { contact_1: Some(contact), contact_2: None };
            },
            (ShapeType::Box(a), ShapeType::Box(b)) =>
                Self::find_contact_points_polygon_to_polygon(a.get_transformed_vertices(position_a, rotation_a),
                    b.get_transformed_vertices(position_b, rotation_b)),
            (ShapeType::Polygon(a), ShapeType::Polygon(b)) =>
                Self::find_contact_points_polygon_to_polygon(a.get_transformed_vertices(position_a, rotation_a),
                    b.get_transformed_vertices(position_b, rotation_b)),

            (ShapeType::Box(a), ShapeType::Polygon(b)) =>
                Self::find_contact_points_polygon_to_polygon(a.get_transformed_vertices(position_a, rotation_a),
                    b.get_transformed_vertices(position_b, rotation_b)),
            (ShapeType::Polygon(a), ShapeType::Box(b)) =>
                Self::find_contact_points_polygon_to_polygon(a.get_transformed_vertices(position_a, rotation_a),
                    b.get_transformed_vertices(position_b, rotation_b)),

            (ShapeType::Box(b), ShapeType::Circle(_)) => {
                let contact = Self::find_contact_point_circle_to_polygon(position_b, b.get_transformed_vertices(position_a, rotation_a));
                return ContactPoints { contact_1: Some(contact), contact_2: None };
            }
            (ShapeType::Polygon(p), ShapeType::Circle(_)) => {
                let contact = Self::find_contact_point_circle_to_polygon(position_b, p.get_transformed_vertices(position_a, rotation_a));
                return ContactPoints { contact_1: Some(contact), contact_2: None };
            }
            (ShapeType::Circle(_), ShapeType::Box(b)) => {
                let contact = Self::find_contact_point_circle_to_polygon(position_a, b.get_transformed_vertices(position_b, rotation_b));
                return ContactPoints { contact_1: Some(contact), contact_2: None };
            }
            (ShapeType::Circle(_), ShapeType::Polygon(p)) => {
                let contact = Self::find_contact_point_circle_to_polygon(position_a, p.get_transformed_vertices(position_b, rotation_b));
                return ContactPoints { contact_1: Some(contact), contact_2: None };
            }

            (ShapeType::Circle(_), ShapeType::Concave(p)) =>
                Self::find_contact_point_concave_to_circle(&p, position_b, position_a, rotation_b),
            (ShapeType::Concave(p), ShapeType::Circle(_)) =>
                Self::find_contact_point_concave_to_circle(&p, position_a, position_b, rotation_b),

            (ShapeType::Concave(a), ShapeType::Concave(b)) =>
                Self::find_contact_point_concave_to_concave(&a, &b, position_a, position_b, rotation_b, rotation_b),

            (ShapeType::Concave(a), ShapeType::Polygon(p)) =>
                Self::find_contact_point_concave_to_polygon(&a, p.get_transformed_vertices(position_b, rotation_b), position_a, rotation_b),
            (ShapeType::Concave(a), ShapeType::Box(b)) =>
                Self::find_contact_point_concave_to_polygon(&a, b.get_transformed_vertices(position_b, rotation_b), position_a, rotation_b),
            (ShapeType::Polygon(p), ShapeType::Concave(c)) =>
                Self::find_contact_point_concave_to_polygon(&c, p.get_transformed_vertices(position_a, rotation_a), position_b, rotation_b),
            (ShapeType::Box(b), ShapeType::Concave(c)) =>
                Self::find_contact_point_concave_to_polygon(&c, b.get_transformed_vertices(position_a, rotation_a), position_b, rotation_b)
        }
    }

    fn find_contact_points_polygon_to_polygon(vertices_a: &[Vector2], vertices_b: &[Vector2]) -> ContactPoints {
        let mut contact_1: Option<Vector2> = None;
        let mut contact_2: Option<Vector2> = None;
        let mut min_distance_squared = std::f32::MAX;

        for i in 0..vertices_a.len() {
            let p = vertices_a[i];

            for j in 0..vertices_b.len() {
                let va = vertices_b[j];
                let vb = vertices_b[(j + 1) % vertices_b.len()];
                let (distance_squared, contact) = Self::point_segment_distance(p, va, vb);

                if is_nearly_equal(distance_squared, min_distance_squared) {
                    if !contact.is_nearly_equal(&contact_1.unwrap_or(Vector2::zero())) {
                        contact_2 = Some(contact);
                    }
                } else if distance_squared < min_distance_squared {
                    min_distance_squared = distance_squared;
                    contact_1 = Some(contact);
                    contact_2 = None;
                }
            }
        }

        for i in 0..vertices_b.len() {
            let p = vertices_b[i];
            for j in 0..vertices_a.len() {
                let va = vertices_a[j];
                let vb = vertices_a[(j + 1) % vertices_a.len()];
                let (distance_squared, contact) = Self::point_segment_distance(p, va, vb);

                if is_nearly_equal(distance_squared, min_distance_squared) {
                    if !contact.is_nearly_equal(&contact_1.unwrap_or(Vector2::zero())) {
                        contact_2 = Some(contact);
                    }
                } else if distance_squared < min_distance_squared {
                    min_distance_squared = distance_squared;
                    contact_1 = Some(contact);
                    contact_2 = None;
                }
            }
        }

        ContactPoints { contact_1, contact_2 }
    }

    fn find_contact_point_circle_to_polygon(circle_center: Vector2, vertices: &[Vector2]) -> Vector2 {
        let mut min_distance_squared = std::f32::MAX;
        let mut contact_point = Vector2::zero();
        
        for i in 0..vertices.len() {
            let va = vertices[i];
            let vb = vertices[(i + 1) % vertices.len()];

            let (distance_squared, contact) = Self::point_segment_distance(circle_center, va, vb);

            if distance_squared < min_distance_squared {
                min_distance_squared = distance_squared;
                contact_point = contact;
            }
        }

        contact_point
    }

    fn find_contact_point_concave_to_concave(polygons_a: &[Polygon], polygons_b: &[Polygon], pos_a: Vector2, pos_b: Vector2, rot_a: f32, rot_b: f32) -> ContactPoints {
        let mut contact_1 = None;
        let mut contact_2 = None;

        for polygon_a in polygons_a {
            let mut ta = polygon_a.clone();
            let vertices_a = ta.get_transformed_vertices(pos_a, rot_a);

            for poly_b in polygons_b {
                let mut tb = poly_b.clone();
                let vertices_b = tb.get_transformed_vertices(pos_b, rot_b);

                let contact = Self::find_contact_points_polygon_to_polygon(vertices_a, vertices_b);
                if contact_1 == None {
                    contact_1 = contact.contact_1;
                }
                if contact_2 == None {
                    contact_2 = contact.contact_2;
                }
            }
        }
        ContactPoints { contact_1, contact_2 }
    }

    fn find_contact_point_concave_to_polygon(polygons_a: &[Polygon], vertices_b: &[Vector2], polygons_pos: Vector2, polygons_rot: f32) -> ContactPoints {
        let mut contact_1 = None;
        let mut contact_2 = None;

        for polygon_a in polygons_a {
            let mut p = polygon_a.clone();
            let contact = Self::find_contact_points_polygon_to_polygon(p.get_transformed_vertices(polygons_pos, polygons_rot), vertices_b);
            if contact_1 == None {
                contact_1 = contact.contact_1;
            }
            if contact_2 == None {
                contact_2 = contact.contact_2;
            }
        }
        ContactPoints { contact_1, contact_2 }
    }

    fn find_contact_point_concave_to_circle(polygons: &[Polygon], circle_center: Vector2, polygons_pos: Vector2, polygons_rot: f32) -> ContactPoints {
        let mut contact: Vector2 = Vector2::zero();

        for polygon in polygons {
            let mut p = polygon.clone();
            let contact_point = Self::find_contact_point_circle_to_polygon(circle_center, p.get_transformed_vertices(polygons_pos, polygons_rot));
            contact = contact_point;
        }

        ContactPoints { contact_1: Some(contact), contact_2: None }
    }

    fn find_contact_point_circle_to_circle(center_a: Vector2, radius_a: f32, center_b: Vector2) -> Vector2 {
        let ab = center_b - center_a;
        let dir = ab.normalize();
        center_a + dir * radius_a
    }

    fn point_segment_distance(p: Vector2, a: Vector2, b: Vector2) -> (f32, Vector2) {
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

        let collision = Collision::intersect_circles(center_a, radius_a, center_b, radius_b);

        assert!(collision.len() > 0);
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

        let collision = Collision::intersect_polygons(&vertices_a, Vector2 { x: 0.5, y: 0.5 }, &vertices_b, Vector2 { x: 1.0, y: 1.0 });
        assert!(collision.len() > 0);
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

        let collision = Collision::intersect_polygon_circle(circle_center, circle_radius, &vertices, Vector2 { x: 0.5, y: 0.5 });
        assert!(collision.len() > 0);
    }
}