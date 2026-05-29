use std::any::Any;

use vyxen_math::{Transform, Vector2};

pub trait Shape {
    fn as_any(&self) -> &dyn Any;
}

/// A simple circle struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    radius: f32
}

impl Shape for Circle {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Circle {
    /// Creates a new circle
    pub fn new(radius: f32) -> Self {
        Circle {
            radius
        }
    }

    /// Gets the rotational inertia of the circle
    pub fn rotational_inertia(&self, mass: f32) -> f32 {
        (1.0 / 2.0) * mass * self.radius * self.radius
    }

    /// Gets the radius of the circle
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

/// A simple box struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box {
    height: f32,
    width: f32,

    vertices: [Vector2; 4],
    transformed_vertices: [Vector2; 4],
    transform_required: bool,
}

impl Shape for Box {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Box {
    /// Creates a new box
    pub fn new(width: f32, height: f32) -> Self {
        let vertices = Self::create_box_vertices(width, height);

        Box {
            height,
            width,
            vertices,
            transformed_vertices: vertices,
            transform_required: true
        }
    }

    /// Gets the height
    pub fn get_height(&self) -> f32 {
        self.height
    }

    /// Gets the width
    pub fn get_width(&self) -> f32 {
        self.width
    }

    /// Gets the rotational inertia of the box
    pub fn rotational_inertia(&self, mass: f32) -> f32 {
        (1.0 / 12.0) * mass * (self.width * self.width + self.height * self.height)
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

    /// Sets if a transform is required
    pub fn set_transform_required(&mut self, required: bool) {
        self.transform_required = required;
    }

    /// Gets the transformed vertices
    pub fn get_transformed_vertices(&mut self, position: Vector2, rotation: f32) -> &[Vector2] {
        if self.transform_required {
            let transform = Transform::new(position, rotation);

            for i in 0..4 {
                let vertex = self.vertices[i];
                self.transformed_vertices[i] = vertex.transform(&transform);
            }
            self.transform_required = false;
        }
        &self.transformed_vertices
    }

    /// Gets the vertices of the box
    pub fn get_vertices(&self) -> &[Vector2] {
        &self.vertices
    }
}

/// A simple polygon struct
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    vertices: Vec<Vector2>,
    transformed_vertices: Vec<Vector2>,
    transform_required: bool
}

impl Shape for Polygon {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Polygon {
    /// Creates a new polygon
    /// 
    /// # Note
    /// 
    ///  - The vertices should be the vertices placed around the world. Not relative.
    ///  - Position should be the position of the polygon at the time of creation.
    pub fn new(vertices: &[Vector2]) -> Self {
        let mut vertices = vertices.to_vec();

        Self::relative_vertices(&mut vertices);

        Self::ensure_counter_clockwise(&mut vertices);

        Polygon {
            vertices: vertices.to_vec(),
            transformed_vertices: vertices.to_vec(),
            transform_required: true,
        }
    }

    /// Creates a new polygon from pre-calculated relative vertices
    pub fn new_from_relative_vertices(vertices: &[Vector2]) -> Self {
        let mut vertices = vertices.to_vec();

        Self::ensure_counter_clockwise(&mut vertices);

        Polygon {
            vertices: vertices.to_vec(),
            transformed_vertices: vertices.to_vec(),
            transform_required: true,
        }
    }

    /// Gets the rotational inertia of the Polygon
    pub fn rotational_inertia(&self, mass: f32) -> f32 {
        let mut area2 = 0.0_f32;
        let mut inertia_sum = 0.0_f32;

        for i in 0..self.vertices.len() {
            let p0 = self.vertices[i];
            let p1 = self.vertices[(i + 1) % self.vertices.len()];

            let cross = p0.x * p1.y - p1.x * p0.y;

            area2 += cross;

            inertia_sum += cross * (
                p0.x * p0.x +
                p0.x * p1.x +
                p1.x * p1.x +
                p0.y * p0.y +
                p0.y * p1.y +
                p1.y * p1.y
            );
        }

        let area = area2.abs() * 0.5;

        (mass * inertia_sum.abs()) / (6.0 * area)
    }

    /// Gets the vertices of the polygon
    pub fn get_vertices(&self) -> &[Vector2] {
        &self.vertices
    }

    /// Sets if a transform is required
    pub fn set_transform_required(&mut self, required: bool) {
        self.transform_required = required;
    }

    /// Gets the transformed vertices
    pub fn get_transformed_vertices(&mut self, position: Vector2, rotation: f32) -> &[Vector2] {
        if self.transform_required {
            let transform = Transform::new(position, rotation);

            for i in 0..self.vertices.len() {
                let vertex = self.vertices[i];
                self.transformed_vertices[i] = vertex.transform(&transform);
            }
            self.transform_required = false;
        }
        &self.transformed_vertices
    }

    fn relative_vertices(vertices: &mut [Vector2]) {
        let mut sum = Vector2::zero();

        for vertex in &*vertices {
            sum = sum + *vertex;
        }

        let center = sum / (vertices.len() as f32);

        for vertex in vertices {
            *vertex = *vertex - center;
        }
    }

    /// Returns if a polygon is convex
    /// 
    /// # Example
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::shapes::Polygon;
    /// 
    /// let v1 = Vector2 { x: 2.0, y: -2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 2.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// let v4 = Vector2 { x: -2.0, y: -2.0 };
    /// let v5 = Vector2 { x: 0.0, y: 0.0 };
    /// 
    /// let concave = Polygon::new(&[v1, v2, v3, v4, v5]);
    /// 
    /// assert!(!concave.is_convex());
    /// 
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// 
    /// let convex = Polygon::new(&[v1, v2, v3]);
    /// 
    /// assert!(convex.is_convex());
    /// ```
    pub fn is_convex(&self) -> bool {
        let len = self.vertices.len();

        if len < 3 {
            return false;
        }

        let mut sign = 0.0_f32;

        for i in 0..len {
            let a = self.vertices[i];
            let b = self.vertices[(i + 1) % len];
            let c = self.vertices[(i + 2) % len];

            let ab = b - a;
            let bc = c - b;

            let cross = ab.cross(&bc);

            if cross.abs() < 0.0005 {
                continue;
            }

            if sign == 0.0 {
                sign = cross;
            } else if cross.signum() != sign.signum() {
                return false;
            }
        }

        true
    }

    /// Turns a `Polygon` into a `Vec<Polygon>`
    /// 
    /// # Example
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::shapes::Polygon;
    /// 
    /// let v1 = Vector2 { x: 2.0, y: -2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 2.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// let v4 = Vector2 { x: -2.0, y: -2.0 };
    /// let v5 = Vector2 { x: 0.0, y: 0.0 };
    /// 
    /// let triangulated = Polygon::triangulate(&[v1, v2, v3, v4, v5]);
    /// assert_eq!(triangulated.len(), 3);
    /// ```
    pub fn triangulate(vertices: &[Vector2]) -> Vec<Polygon> {
        let mut result = Vec::new();

        if vertices.len() < 3 {
            return result;
        }

        if vertices.len() == 3 {
            return vec![Polygon::new(vertices)];
        }

        let mut verts = vertices.to_vec();

        if Self::is_clockwise(vertices) {
            verts.reverse();
        }

        while verts.len() > 3 {
            let len = verts.len();
            let mut ear_found = false;

            for i in 0..len {
                let prev = verts[(i + len - 1) % len];
                let curr = verts[i];
                let next = verts[(i + 1) % len];

                let cross = (curr - prev).cross(&(next - curr));

                if cross <= 0.0 {
                    continue;
                }

                let mut contains_point = false;

                for j in 0..len {
                    if j == i || j == (i + 1) % len || j == (i + len - 1) % len {
                        continue;
                    }

                    if Self::point_in_triangle(verts[j], prev, curr, next) {
                        contains_point = true;
                        break;
                    }
                }

                if contains_point {
                    continue;
                }

                result.push(Polygon::new_from_relative_vertices(&[prev, curr, next]));

                verts.remove(i);

                ear_found = true;
                break;
            }

            if !ear_found {
                break;
            }
        }

        if verts.len() == 3 {
            result.push(Polygon::new_from_relative_vertices(&[verts[0], verts[1], verts[2]]));
        }

        result
    }

    fn point_in_triangle(p: Vector2, a: Vector2, b: Vector2, c: Vector2) -> bool {
        let ab = (b - a).cross(&(p - a));
        let bc = (c - b).cross(&(p - b));
        let ca = (a - c).cross(&(p - c));

        let has_neg = ab < 0.0 || bc < 0.0 || ca < 0.0;
        let has_pos = ab > 0.0 || bc > 0.0 || ca > 0.0;

        !(has_neg && has_pos)
    }

    /// Returns if vertices are clockwise
    /// 
    /// # Example
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::shapes::Polygon;
    /// 
    /// let v1 = Vector2 { x: 2.0, y: -2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 2.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// let v4 = Vector2 { x: -2.0, y: -2.0 };
    /// let v5 = Vector2 { x: 0.0, y: 0.0 };
    /// 
    /// let counter_clockwise = Polygon::is_clockwise(&[v1, v2, v3, v4, v5]);
    /// assert_eq!(counter_clockwise, false);
    /// 
    /// let clockwise = Polygon::is_clockwise(&[v5, v4, v3, v2, v1]);
    /// assert!(clockwise);
    /// ```
    pub fn is_clockwise(vertices: &[Vector2]) -> bool {
        let mut area = 0.0;

        for i in 0..vertices.len() {
            let a = vertices[i];
            let b = vertices[(i + 1) % vertices.len()];

            area += a.cross(&b);
        }

        (area * 0.5) < 0.0
    }

    /// Ensures the polygons are counter clockwise
    /// 
    /// # Example
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::shapes::Polygon;
    /// 
    /// let v1 = Vector2 { x: 0.0, y: 0.0 };
    /// let v2 = Vector2 { x: -2.0, y: -2.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// let v4 = Vector2 { x: 2.0, y: 2.0 };
    /// let v5 = Vector2 { x: 2.0, y: -2.0 };
    /// 
    /// let mut vertices = vec![v1, v2, v3, v4, v5];
    /// let reversed = vec![v5, v4, v3, v2, v1];
    /// 
    /// Polygon::ensure_counter_clockwise(&mut vertices);
    /// assert!(vertices == reversed)
    /// ```
    pub fn ensure_counter_clockwise(vertices: &mut Vec<Vector2>) {
        if Self::is_clockwise(vertices) {
            vertices.reverse();
        }
    }
}