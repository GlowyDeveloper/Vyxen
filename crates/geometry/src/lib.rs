use vyxen_math::{Transform, Vector2};

pub mod aabb;

/// A simple circle struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    radius: f32
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle {
            radius
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn rotational_inertia(&self, mass: f32) -> f32 {
        (1.0 / 2.0) * mass * self.radius * self.radius
    }
}

/// A simple box struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box {
    pub height: f32,
    pub width: f32,

    vertices: [Vector2; 4],
    transformed_vertices: [Vector2; 4],
    transform_required: bool,
}

impl Box {
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

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn set_transform_required(&mut self, required: bool) {
        self.transform_required = required;
    }

    pub fn rotational_inertia(&self, mass: f32) -> f32 {
        (1.0 / 12.0) * mass * (self.width * self.width + self.height * self.height)
    }

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