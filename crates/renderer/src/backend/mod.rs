use vyxen_math::{Matrix4, Vector2};

mod gpu_texture;
pub mod shape_geometry;
mod state;
pub mod winit_reexports;

pub use gpu_texture::GpuTexture;
pub use state::State;

pub const OPENGL_TO_WGPU_MATRIX: Matrix4 = Matrix4 {
    m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.5, 1.0],
    ],
};

pub const MAX_SPRITE_VERTEX_BUFFER_SIZE: u64 = 1 << 20;
pub const MAX_SPRITE_INDEX_BUFFER_SIZE: u64 = 1 << 20;
pub const MAX_SPRITES: usize = 8192;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteRaw {
    pub matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl SpriteRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
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
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
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

/// Camera struct for the scene.
///
/// # Examples
/// ```rust
/// use vyxen_renderer::Camera;
/// use vyxen_math::Vector2;
///
/// let mut camera = Camera::new(800.0, 600.0);
/// camera.set_width(600.0);
/// camera.set_height(300.0);
/// camera.set_zoom(2.0);
/// camera.set_position(Vector2 { x: 100.0, y: 100.0 });
/// camera.set_rotation(90.0);
///
/// assert_eq!(camera.get_width(), 600.0);
/// assert_eq!(camera.get_height(), 300.0);
/// assert_eq!(camera.get_zoom(), 2.0);
/// assert_eq!(camera.get_position(), Vector2 { x: 100.0, y: 100.0 });
/// assert_eq!(camera.get_rotation(), 90.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    position: Vector2,
    rotation: f32,
    zoom: f32,
    width: f32,
    height: f32,
}

impl Camera {
    /// Makes a new camera.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    /// use vyxen_math::Vector2;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    /// camera.set_width(600.0);
    /// camera.set_height(300.0);
    /// camera.set_zoom(2.0);
    /// camera.set_position(Vector2 { x: 100.0, y: 100.0 });
    /// camera.set_rotation(90.0);
    ///
    /// assert_eq!(camera.get_width(), 600.0);
    /// assert_eq!(camera.get_height(), 300.0);
    /// assert_eq!(camera.get_zoom(), 2.0);
    /// assert_eq!(camera.get_position(), Vector2 { x: 100.0, y: 100.0 });
    /// assert_eq!(camera.get_rotation(), 90.0);
    /// ```
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            zoom: 1.0,
            width,
            height,
        }
    }

    /// Gets the width.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_width(), 800.0);
    ///
    /// camera.set_width(600.0);
    ///
    /// assert_eq!(camera.get_width(), 600.0);
    /// ```
    pub fn get_width(&self) -> f32 {
        self.width
    }

    /// Gets the height.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_height(), 600.0);
    ///
    /// camera.set_height(400.0);
    ///
    /// assert_eq!(camera.get_height(), 400.0);
    /// ```
    pub fn get_height(&self) -> f32 {
        self.height
    }

    /// Gets the zoom.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_zoom(), 1.0);
    ///
    /// camera.set_zoom(2.0);
    ///
    /// assert_eq!(camera.get_zoom(), 2.0);
    /// ```
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    /// Gets the position.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    /// use vyxen_math::Vector2;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_position(), Vector2 { x: 0.0, y: 0.0 });
    ///
    /// camera.set_position(Vector2 { x: 100.0, y: 100.0 });
    ///
    /// assert_eq!(camera.get_position(), Vector2 { x: 100.0, y: 100.0 });
    /// ```
    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    /// Gets the rotation.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_rotation(), 0.0);
    ///
    /// camera.set_rotation(90.0);
    ///
    /// assert_eq!(camera.get_rotation(), 90.0);
    /// ```
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    /// Sets the width.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_width(), 800.0);
    ///
    /// camera.set_width(600.0);
    ///
    /// assert_eq!(camera.get_width(), 600.0);
    /// ```
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    /// Sets the height.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_height(), 600.0);
    ///
    /// camera.set_height(400.0);
    ///
    /// assert_eq!(camera.get_height(), 400.0);
    /// ```
    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    /// Sets the zoom.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_zoom(), 1.0);
    ///
    /// camera.set_zoom(2.0);
    ///
    /// assert_eq!(camera.get_zoom(), 2.0);
    /// ```
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    /// Sets the position.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    /// use vyxen_math::Vector2;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_position(), Vector2 { x: 0.0, y: 0.0 });
    ///
    /// camera.set_position(Vector2 { x: 100.0, y: 100.0 });
    ///
    /// assert_eq!(camera.get_position(), Vector2 { x: 100.0, y: 100.0 });
    /// ```
    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    /// Sets the rotation.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Camera;
    ///
    /// let mut camera = Camera::new(800.0, 600.0);
    ///
    /// assert_eq!(camera.get_rotation(), 0.0);
    ///
    /// camera.set_rotation(90.0);
    ///
    /// assert_eq!(camera.get_rotation(), 90.0);
    /// ```
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn build_view_projection_matrix(&self) -> Matrix4 {
        let view = Matrix4::translation(-self.position.x, -self.position.y, 0.0);

        let half_width = self.width / self.zoom / 2.0;
        let half_height = self.height / self.zoom / 2.0;

        let left = -half_width;
        let right = half_width;
        let bottom = -half_height;
        let top = half_height;

        let projection = Matrix4::orthographic(left, right, bottom, top, -1.0, 1.0);

        OPENGL_TO_WGPU_MATRIX * projection * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
