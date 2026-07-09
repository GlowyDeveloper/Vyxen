use crate::backend::SpriteRaw;
use vyxen_geometry::{Shape, ShapeType, shape_type_from_shape};
use vyxen_math::{Matrix4, Vector2};

pub use color::Color;
pub use texture::Texture;

/// Backend components for the renderer. Not recommended for use.
pub mod backend;
mod color;
mod texture;

pub use crate::backend::Camera;

#[derive(Debug, Clone, PartialEq)]
pub enum DrawType {
    Texture(Texture),
    Color(Color),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sprite {
    draw_type: DrawType,
    vertices: Option<ShapeType>,
    position_ref: Vector2,
    rotation_ref: f32,
    z: f32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self::new()
    }
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            draw_type: DrawType::None,
            vertices: None,
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            z: 0.0,
        }
    }

    pub fn with_color(color: Color) -> Self {
        Sprite {
            draw_type: DrawType::Color(color),
            vertices: None,
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            z: 0.0,
        }
    }

    pub fn with_texture(texture: Texture) -> Self {
        Sprite {
            draw_type: DrawType::Texture(texture),
            vertices: None,
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            z: 0.0,
        }
    }

    pub fn set_draw_type(&mut self, draw_type: DrawType) {
        self.draw_type = draw_type;
    }

    pub fn set_shape<T>(&mut self, shape: T)
    where
        T: Shape,
    {
        self.vertices = Some(shape_type_from_shape(shape));
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position_ref = position;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation_ref = rotation;
    }

    pub fn get_vertices(&self) -> Option<&ShapeType> {
        self.vertices.as_ref()
    }

    pub fn get_draw_type(&self) -> &DrawType {
        &self.draw_type
    }

    pub fn to_raw(&self) -> SpriteRaw {
        let color: [f32; 4] = match &self.draw_type {
            DrawType::Texture(_) => [1.0, 1.0, 1.0, 1.0],
            DrawType::Color(color) => (*color).into(),
            DrawType::None => [1.0, 1.0, 1.0, 1.0],
        };

        let pos = self.position_ref;
        let rot = self.rotation_ref;

        SpriteRaw {
            matrix: (Matrix4::translation(pos.x, pos.y, self.z) * Matrix4::rotate(rot)).into(),
            color,
        }
    }
}
