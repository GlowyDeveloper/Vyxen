use crate::{
    Color, Font, Text, Texture,
    geometry::Shape,
    node::Component,
    shape_type::{ShapeType, shape_type_from_shape},
};
use std::any::Any;

/// The type of a `UIElement or `Sprite`.
#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Color(Color),
    Text(Text),
    Texture(Texture),
    None,
}

/// A UI element.
///
/// # Examples
/// ```rust, ignore
/// use vyxen::{Box, UiElement};
///
/// let mut element = UiElement::with_image(Texture::new(
///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
/// ));
/// element.set_shape(Box::new(100.0, 20.0));
/// ```
#[derive(Debug, Clone)]
pub struct UiElement {
    element_type: ElementType,
    vertices: Option<ShapeType>,
    z: f32,
}

impl Default for UiElement {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for UiElement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl UiElement {
    /// Creates a new UI element.
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen::{Box, UiElement};
    ///
    /// let mut element = UiElement::with_image(Texture::new(
    ///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
    /// ));
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn new() -> Self {
        Self {
            element_type: ElementType::None,
            vertices: None,
            z: 0.0,
        }
    }

    /// Short for `UiElement::new().set_element_type(ElementType::Text(..))`
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen::{Box, Font, load_path, UiElement};
    ///
    /// let font = load_path::<Font>("path/to/font.ttf");
    /// let mut element = UiElement::with_text("Hello World!", font, 32.0);
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn with_text(text: String, font: Font, size: f32) -> Self {
        Self {
            element_type: ElementType::Text(Text::new(text, font, size)),
            vertices: None,
            z: 0.0,
        }
    }

    /// Short for `UiElement::new().set_element_type(ElementType::Texture(..))`
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen::{Box, UiElement};
    ///
    /// let mut element = UiElement::with_texture(Texture::new(
    ///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
    /// ));
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn with_texture(texture: Texture) -> Self {
        Self {
            element_type: ElementType::Texture(texture),
            vertices: None,
            z: 0.0,
        }
    }

    /// Short for `UiElement::new().set_element_type(ElementType::Color(..))`
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen::{Box, UiElement, Color};
    ///
    /// let mut element = UiElement::with_color(Color::new(Color::rgb(1.0, 0.0, 0.0)));
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn with_color(color: Color) -> Self {
        Self {
            element_type: ElementType::Color(color),
            vertices: None,
            z: 0.0,
        }
    }

    /// Sets how this element should be rendered.
    pub fn set_element_type(&mut self, element_type: ElementType) {
        self.element_type = element_type;
    }

    /// Returns the current type of this element.
    pub fn get_element_type(&self) -> &ElementType {
        &self.element_type
    }

    /// Sets the z-coordinate of this element.
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    /// Returns the current z-coordinate of this element.
    pub fn get_z(&self) -> f32 {
        self.z
    }

    /// Sets the vertices of this element.
    pub fn set_vertices(&mut self, vertices: Option<ShapeType>) {
        self.vertices = vertices;
    }

    /// Returns the current vertices of this element.
    pub fn get_vertices(&self) -> Option<&ShapeType> {
        self.vertices.as_ref()
    }

    /// Sets the shape used to render this element.
    ///
    /// # Examples
    /// ## Box
    /// ```rust
    /// use vyxen::{Box, UiElement};
    ///
    /// let mut element = UiElement::new();
    /// element.set_shape(Box::new(64.0, 64.0));
    /// ```
    /// ## Circle
    /// ```rust
    /// use vyxen::{Circle, UiElement};
    ///
    /// let mut element = UiElement::new();
    /// element.set_shape(Circle::new(64.0));
    /// ```
    /// ## Polygon
    /// ```rust
    /// use vyxen::{Polygon, Vector2, UiElement};
    ///
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    ///
    /// let mut element = UiElement::new();
    /// element.set_shape(Polygon::new(&[v1, v2, v3]));
    /// ```
    pub fn set_shape<T>(&mut self, shape: T)
    where
        T: Shape,
    {
        self.vertices = Some(shape_type_from_shape(shape));
    }
}
