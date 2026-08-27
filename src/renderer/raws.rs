use crate::{
    Camera, DrawType, Matrix4, Sprite,
    ui::{UiElement, UiType},
};

pub const OPENGL_TO_WGPU_MATRIX: Matrix4 = Matrix4 {
    m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.5, 1.0],
    ],
};

/// Maximum size of the sprite vertex buffer.
pub const MAX_SPRITE_VERTEX_BUFFER_SIZE: u64 = 1 << 20;
/// Maximum size of the sprite index buffer.
pub const MAX_SPRITE_INDEX_BUFFER_SIZE: u64 = 1 << 20;
/// Maximum number of sprites.
pub const MAX_SPRITES: usize = 8192;
/// Maximum number of UI elements.
pub const MAX_UI_ELEMENTS: usize = 8192;
/// Maximum number of glyph instances.
pub const MAX_GLYPH_INSTANCES: usize = 65536;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteRaw {
    pub matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl SpriteRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SpriteRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }

    pub fn gen_raw(sprite: &Sprite) -> SpriteRaw {
        let color: [f32; 4] = match &sprite.draw_type {
            DrawType::Texture(texture) if texture.get_tint().is_some() => {
                texture.get_tint().unwrap().into()
            }
            DrawType::Color(color) => (*color).into(),
            _ => [1.0, 1.0, 1.0, 1.0],
        };

        let pos = sprite.position_ref;
        let rot = sprite.rotation_ref;

        SpriteRaw {
            matrix: (Matrix4::translation(pos.x, pos.y, sprite.z) * Matrix4::rotate(rot)).into(),
            color,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        let view = Matrix4::translation(-camera.position.x, -camera.position.y, 0.0);

        let half_width = camera.width / camera.zoom / 2.0;
        let half_height = camera.height / camera.zoom / 2.0;

        let left = -half_width;
        let right = half_width;
        let bottom = -half_height;
        let top = half_height;

        let projection = Matrix4::orthographic(left, right, bottom, top, -1.0, 1.0);

        let proj = OPENGL_TO_WGPU_MATRIX * projection * view;
        self.view_proj = proj.into();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UiCameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl UiCameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        let view = Matrix4::identity();

        let left = 0.0;
        let right = camera.width / camera.zoom;
        let bottom = 0.0;
        let top = camera.height / camera.zoom;

        let projection = Matrix4::orthographic(left, right, bottom, top, -1.0, 1.0);

        let proj = OPENGL_TO_WGPU_MATRIX * projection * view;
        self.view_proj = proj.into();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UiRaw {
    pub matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl UiRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<UiRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }

    pub fn gen_raw(ui: &UiElement) -> UiRaw {
        let color: [f32; 4] = match ui.get_ui_type() {
            UiType::Image(texture) if texture.get_tint().is_some() => {
                texture.get_tint().unwrap().into()
            }
            UiType::Text(text) if text.get_tint().is_some() => text.get_tint().unwrap().into(),
            _ => [1.0, 1.0, 1.0, 1.0],
        };

        let pos = ui.get_position();
        let rot = ui.get_rotation();

        UiRaw {
            matrix: (Matrix4::translation(pos.x, pos.y, ui.get_z()) * Matrix4::rotate(rot)).into(),
            color,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphRaw {
    pub rect: [f32; 4],
    pub uv_rect: [f32; 4],
}

impl GlyphRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<GlyphRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
