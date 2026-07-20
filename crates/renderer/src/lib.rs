#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use crate::backend::SpriteRaw;
use image::GenericImageView as _;
use vyxen_geometry::{Shape, ShapeType, shape_type_from_shape};
use vyxen_math::{Matrix4, Vector2};
use winit::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    platform::windows::WindowAttributesExtWindows,
    window::{Fullscreen, Icon, Window, WindowAttributes},
};

/// Backend components for the renderer. Not recommended for use, may be short of documentation.
pub mod backend;
mod color;
mod texture;

pub use crate::backend::Camera;
pub use color::Color;
pub use texture::Texture;

/// Defines the draw types for a sprite
///
/// # Examples
/// ## Texture
/// ```rust, ignore
/// use vyxen_renderer::{DrawType, Texture};
///
/// let texture = DrawType::Texture(
///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
/// );
/// ```
/// ## Color
/// ```rust
/// use vyxen_renderer::{DrawType, Color};
///
/// let color = DrawType::Color(
///     Color::from_rgba(0.2, 0.1, 0.9, 1.0)
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DrawType {
    Texture(Texture),
    Color(Color),
    None,
}

/// Gives a sprite to render to the screen.
///
/// # Examples
/// ## Texture
/// ```rust, ignore
/// use vyxen_renderer::{Texture, Sprite};
/// use vyxen_geometry::Box;
///
/// let mut sprite = Sprite::with_texture(
///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
/// );
/// sprite.set_shape(Box::new(20.0, 2.0));
/// ```
/// ## Color
/// ```rust
/// use vyxen_renderer::{Color, Sprite};
/// use vyxen_geometry::Box;
///
/// let mut sprite = Sprite::with_color(
///     Color::from_rgba(0.2, 0.1, 0.9, 1.0)
/// );
/// sprite.set_shape(Box::new(20.0, 2.0));
/// ```
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
    /// Gives a sprite to render to the screen.
    ///
    /// # Examples
    /// ## Texture
    /// ```rust, ignore
    /// use vyxen_renderer::{DrawType, Texture, Sprite};
    /// use vyxen_geometry::Box;
    ///
    /// let mut sprite = Sprite::new();
    /// sprite.set_shape(Box::new(20.0, 2.0));
    /// sprite.set_draw_type(DrawType::Texture(
    ///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
    /// ));
    /// ```
    /// ## Color
    /// ```rust
    /// use vyxen_renderer::{DrawType, Color, Sprite};
    /// use vyxen_geometry::Box;
    ///
    /// let mut sprite = Sprite::new();
    /// sprite.set_shape(Box::new(20.0, 2.0));
    /// sprite.set_draw_type(DrawType::Color(
    ///     Color::from_rgba(0.2, 0.1, 0.9, 1.0)
    /// ));
    /// ```
    pub fn new() -> Self {
        Sprite {
            draw_type: DrawType::None,
            vertices: None,
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            z: 0.0,
        }
    }

    /// Short for `Sprite::new().set_draw_type(DrawType::Color(..))`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::{Color, Sprite};
    /// use vyxen_geometry::Box;
    ///
    /// let mut sprite = Sprite::with_color(
    ///     Color::from_rgba(0.2, 0.1, 0.9, 1.0)
    /// );
    /// sprite.set_shape(Box::new(20.0, 2.0));
    /// ```
    pub fn with_color(color: Color) -> Self {
        Sprite {
            draw_type: DrawType::Color(color),
            vertices: None,
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            z: 0.0,
        }
    }

    /// Short for `Sprite::new().set_draw_type(DrawType::Texture(..))`
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen_renderer::{Texture, Sprite};
    /// use vyxen_geometry::Box;
    ///
    /// let mut sprite = Sprite::with_texture(
    ///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
    /// );
    /// sprite.set_shape(Box::new(20.0, 2.0));
    /// ```
    pub fn with_texture(texture: Texture) -> Self {
        Sprite {
            draw_type: DrawType::Texture(texture),
            vertices: None,
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            z: 0.0,
        }
    }

    /// Sets how this sprite should be rendered.
    ///
    /// This replaces the current `DrawType` of the sprite.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::{Color, DrawType, Sprite};
    ///
    /// let mut sprite = Sprite::new();
    /// sprite.set_draw_type(DrawType::Color(Color::from_rgba(0.2, 0.1, 0.9, 1.0)));
    /// ```
    pub fn set_draw_type(&mut self, draw_type: DrawType) {
        self.draw_type = draw_type;
    }

    /// Sets the shape used to render this sprite.
    ///
    /// # Examples
    /// ## Box
    /// ```rust
    /// use vyxen_geometry::Box;
    /// use vyxen_renderer::Sprite;
    /// use vyxen_math::Vector2;
    ///
    /// let mut sprite = Sprite::new();
    /// sprite.set_shape(Box::new(64.0, 64.0));
    /// ```
    /// ## Circle
    /// ```rust
    /// use vyxen_geometry::Circle;
    /// use vyxen_renderer::Sprite;
    /// use vyxen_math::Vector2;
    ///
    /// let mut sprite = Sprite::new();
    /// sprite.set_shape(Circle::new(64.0));
    /// ```
    /// ## Polygon
    /// ```rust
    /// use vyxen_geometry::Polygon;
    /// use vyxen_renderer::Sprite;
    /// use vyxen_math::Vector2;
    ///
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    ///
    /// let mut sprite = Sprite::new();
    /// sprite.set_shape(Polygon::new(&[v1, v2, v3]));
    /// ```
    pub fn set_shape<T>(&mut self, shape: T)
    where
        T: Shape,
    {
        self.vertices = Some(shape_type_from_shape(shape));
    }

    /// Sets the world position of this sprite. Used mainly in the backend.
    pub fn set_position(&mut self, position: Vector2) {
        self.position_ref = position;
    }

    /// Sets the rotation of this sprite. Used mainly in the backend.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation_ref = rotation;
    }

    /// Returns `Some` if the shape is assigned, `None` if not.
    pub fn get_vertices(&self) -> Option<&ShapeType> {
        self.vertices.as_ref()
    }

    /// Returns the current draw type of this sprite.
    pub fn get_draw_type(&self) -> &DrawType {
        &self.draw_type
    }

    /// Converts this sprite into the raw equivalent. Used mainly in the backend.
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

/// Config used when creating a renderer window.
///
/// Modifiable fields:
/// - size
/// - minimium size
/// - maximium size
/// - title
/// - resizable
/// - position
/// - maximized
/// - visible
/// - decorations
/// - icon
/// - fullscreen
/// - render mode
/// - background color
///
/// # Examples
///
/// ```rust
/// use vyxen_renderer::{WindowConfig, Color};
/// use vyxen_math::Vector2;
///
/// let mut config = WindowConfig::new();
///
/// config.set_title("Foo".into());
/// config.set_size(Vector2 { x: 1920.0, y: 1080.0 });
/// config.set_resizable(false);
/// config.set_background_color(Color::from_rgb(0.1, 0.1, 0.15));
/// ```
#[derive(Debug, Clone)]
pub struct WindowConfig {
    size: Vector2,
    min_size: Vector2,
    max_size: Vector2,
    title: String,
    resizable: bool,
    position: Option<Vector2>,
    maximized: bool,
    visible: bool,
    decorations: bool,
    icon: Option<Icon>,
    fullscreen: bool,
    render_mode: RenderMode,
    background_color: Color,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowConfig {
    /// Creates a new window config using defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::{WindowConfig, Color};
    /// use vyxen_math::Vector2;
    ///
    /// let mut config = WindowConfig::new();
    ///
    /// config.set_title("Foo".into());
    /// config.set_size(Vector2 { x: 1920.0, y: 1080.0 });
    /// config.set_resizable(false);
    /// config.set_background_color(Color::from_rgb(0.1, 0.1, 0.15));
    /// ```
    pub fn new() -> Self {
        WindowConfig {
            size: Vector2 { x: 800.0, y: 600.0 },
            min_size: Vector2 { x: 100.0, y: 100.0 },
            max_size: Vector2 {
                x: f32::INFINITY,
                y: f32::INFINITY,
            },
            title: "Vyxen Application".to_string(),
            resizable: true,
            position: None,
            maximized: false,
            visible: true,
            decorations: true,
            icon: None,
            fullscreen: false,
            render_mode: RenderMode::Best,
            background_color: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }

    /// Sets the initial window size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    /// use vyxen_math::Vector2;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_size(Vector2 { x: 1920.0, y: 1080.0 });
    /// ```
    pub fn set_size(&mut self, size: Vector2) {
        self.size = size;
    }

    /// Sets the minimum allowed window size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    /// use vyxen_math::Vector2;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_min_size(Vector2 { x: 640.0, y: 480.0 });
    /// ```
    pub fn set_min_size(&mut self, min_size: Vector2) {
        self.min_size = min_size;
    }

    /// Sets the maximum allowed window size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    /// use vyxen_math::Vector2;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_max_size(Vector2 { x: 3840.0, y: 2160.0 });
    /// ```
    pub fn set_max_size(&mut self, max_size: Vector2) {
        self.max_size = max_size;
    }

    /// Sets the window title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_title("Foo".into());
    /// ```
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Enables or disables window resizing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_resizable(false);
    /// ```
    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    /// Sets the initial window position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    /// use vyxen_math::Vector2;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_position(Vector2 { x: 100.0, y: 100.0 });
    /// ```
    pub fn set_position(&mut self, position: Vector2) {
        self.position = Some(position);
    }

    /// Sets whether the window should start maximized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_maximized(true);
    /// ```
    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }

    /// Sets whether the window should be visible when created.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_visible(false);
    /// ```
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Enables or disables window decorations (title bar and borders).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_decorations(false);
    /// ```
    pub fn set_decorations(&mut self, decorations: bool) {
        self.decorations = decorations;
    }

    /// Sets the window icon and taskbar icon.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut config = WindowConfig::new();
    /// let icon = image::open("assets/icon.png")?;
    /// config.set_icon(&icon);
    /// ```
    pub fn set_icon(&mut self, icon: &image::DynamicImage) {
        let (width, height) = icon.dimensions();
        let rgba = icon.to_rgba8().into_vec();
        self.icon = Some(Icon::from_rgba(rgba, width, height).unwrap());
    }

    /// Enables or disables borderless fullscreen mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_fullscreen(true);
    /// ```
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    /// Selects the preferred graphics backend.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::{WindowConfig, RenderMode};
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_render_mode(RenderMode::Vulkan);
    /// ```
    pub fn set_render_mode(&mut self, render_mode: RenderMode) {
        self.render_mode = render_mode;
    }

    /// Sets the window background color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_renderer::{WindowConfig, Color};
    ///
    /// let mut config = WindowConfig::new();
    /// config.set_background_color(Color::from_rgb(0.2, 0.3, 0.4));
    /// ```
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
}

impl From<WindowConfig> for WindowAttributes {
    fn from(value: WindowConfig) -> Self {
        let inner_size = Size::Logical(LogicalSize::new(value.size.x.into(), value.size.y.into()));
        let min_size = Size::Logical(LogicalSize::new(
            value.min_size.x.into(),
            value.min_size.y.into(),
        ));
        let max_size = Size::Logical(LogicalSize::new(
            value.max_size.x.into(),
            value.max_size.y.into(),
        ));
        let fullscreen = if value.fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };

        let attr = Window::default_attributes()
            .with_inner_size(inner_size)
            .with_min_inner_size(min_size)
            .with_max_inner_size(max_size)
            .with_title(value.title)
            .with_resizable(value.resizable)
            .with_maximized(value.maximized)
            .with_visible(value.visible)
            .with_decorations(value.decorations)
            .with_window_icon(value.icon.clone())
            .with_taskbar_icon(value.icon)
            .with_fullscreen(fullscreen);

        if let Some(vector) = value.position {
            let position =
                Position::Logical(LogicalPosition::new(vector.x.into(), vector.y.into()));
            attr.with_position(position)
        } else {
            attr
        }
    }
}

/// Preferred graphics backend used by the renderer.
#[derive(Debug, Clone, Copy)]
pub enum RenderMode {
    Best,
    Vulkan,
    Metal,
    DX12,
    GL,
    WebGPU,
}

impl From<RenderMode> for wgpu::Backends {
    fn from(value: RenderMode) -> Self {
        match value {
            RenderMode::Best => wgpu::Backends::PRIMARY,
            RenderMode::Vulkan => wgpu::Backends::VULKAN,
            RenderMode::Metal => wgpu::Backends::METAL,
            RenderMode::DX12 => wgpu::Backends::DX12,
            RenderMode::GL => wgpu::Backends::GL,
            RenderMode::WebGPU => wgpu::Backends::BROWSER_WEBGPU,
        }
    }
}
