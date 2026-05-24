use vyxen_math::{Transform, Vector2};

pub mod aabb;

/// A simple circle struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    radius: f32
}

impl Circle {
    /// Creates a new circle
    pub fn new(radius: f32) -> Self {
        Circle {
            radius
        }
    }

    /// Gets the radius of the circle
    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    /// Gets the rotational inertia of the circle
    pub fn rotational_inertia(&self, mass: f32) -> f32 {
        (1.0 / 2.0) * mass * self.radius * self.radius
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

    /// Sets if a transform is required
    pub fn set_transform_required(&mut self, required: bool) {
        self.transform_required = required;
    }

    /// Gets the rotational inertia of the circle
    pub fn rotational_inertia(&self, mass: f32) -> f32 {
        (1.0 / 12.0) * mass * (self.width * self.width + self.height * self.height)
    }

    /// Gets the transformed vertices
    pub fn get_transformed_vertices(&mut self, position: Vector2, rotation: f32) -> &[Vector2; 4] {
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
}

/// A simple polygon struct
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    vertices: Vec<Vector2>,
    transformed_vertices: Vec<Vector2>,
    transform_required: bool
}

impl Polygon {
    /// Creates a new polygon
    /// 
    /// # Note
    /// 
    ///  - The vertices should be the vertices placed around the world. Not relative.
    ///  - Position should be the position of the polygon at the time of creation.
    pub fn new(vertices: Vec<Vector2>, position: Vector2) -> Self { 
        let relative_vertices = Self::relative_vertices(vertices.clone(), position);

        Polygon {
            vertices: relative_vertices.clone(),
            transformed_vertices: relative_vertices,
            transform_required: true,
        }
    }

    /// Creates a new polygon from pre-calculated relative vertices
    pub fn new_from_relative_vertices(vertices: Vec<Vector2>) -> Self { 
        Polygon {
            vertices: vertices.clone(),
            transformed_vertices: vertices,
            transform_required: true,
        }
    }

    pub fn get_vertices(&self) -> &Vec<Vector2> {
        &self.vertices
    }

    /// Sets if a transform is required
    pub fn set_transform_required(&mut self, required: bool) {
        self.transform_required = required;
    }

    /// Gets the rotational inertia of the circle
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

    fn relative_vertices(vertices: Vec<Vector2>, position: Vector2) -> Vec<Vector2> {
        let mut relative_vec = Vec::with_capacity(vertices.len());

        for i in 0..vertices.len() {
            let vertex = vertices[i];
            let relative = vertex - position;
            relative_vec.push(relative);
        }

        relative_vec
    }
}