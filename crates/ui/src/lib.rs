use vyxen_geometry::{Shape, ShapeType, shape_type_from_shape};
use vyxen_math::Vector2;
use vyxen_resource::{Font, Texture};

/// The type of UI element.
#[derive(Debug, Clone)]
pub enum UiType {
    Button,
    Text(Text),
    Image(Texture),
    None,
}

/// A UI element.
///
/// # Examples
/// ```rust
/// use vyxen_ui::UiElement;
/// use vyxen_geometry::Box;
///
/// let mut element = UiElement::with_button();
/// element.set_shape(Box::new(100.0, 20.0));
/// ```
#[derive(Debug, Clone)]
pub struct UiElement {
    position_ref: Vector2,
    rotation_ref: f32,
    ui_type: UiType,
    vertices: Option<ShapeType>,
    z: f32,
}

impl UiElement {
    /// Creates a new UI element with the given type.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_ui::UiElement;
    /// use vyxen_geometry::Box;
    ///
    /// let mut element = UiElement::with_button();
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn new() -> Self {
        Self {
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            ui_type: UiType::None,
            vertices: None,
            z: 0.0,
        }
    }

    /// Short for `UiElement::new().set_ui_type(UiType::Button)`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_ui::{UiElement, UiType};
    /// use vyxen_geometry::Box;
    ///
    /// let mut element = UiElement::with_button();
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn with_button() -> Self {
        Self {
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            ui_type: UiType::Button,
            vertices: None,
            z: 0.0,
        }
    }

    /// Short for `UiElement::new().set_ui_type(UiType::Text)`
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen_ui::{UiElement, UiType};
    /// use vyxen_geometry::Box;
    /// use vyxen_resource::{Font, load_path};
    ///
    /// let font = load_path::<Font>("path/to/font.ttf");
    /// let mut element = UiElement::with_text("Hello World!", font, 32.0);
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn with_text(text: String, font: Font, size: f32) -> Self {
        Self {
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            ui_type: UiType::Text(Text { text, font, size }),
            vertices: None,
            z: 0.0,
        }
    }

    /// Short for `UiElement::new().set_ui_type(UiType::Image(..))`
    ///
    /// # Examples
    /// ```rust, ignore
    /// use vyxen_ui::{UiElement, UiType};
    /// use vyxen_geometry::Box;
    ///
    /// let mut element = UiElement::with_image(Texture::new(
    ///     Texture::from_bytes(include_bytes!("test-img.png"), "image").unwrap()
    /// ));
    /// element.set_shape(Box::new(100.0, 20.0));
    /// ```
    pub fn with_image(texture: Texture) -> Self {
        Self {
            position_ref: Vector2::zero(),
            rotation_ref: 0.0,
            ui_type: UiType::Image(texture),
            vertices: None,
            z: 0.0,
        }
    }

    /// Sets the ui type of this element.
    pub fn set_ui_type(&mut self, ui_type: UiType) {
        self.ui_type = ui_type;
    }

    /// Sets the position of this element. Used mainly in the backend.
    pub fn set_position(&mut self, position: Vector2) {
        self.position_ref = position;
    }

    /// Sets the rotation of this element. Used mainly in the backend.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation_ref = rotation;
    }

    /// Returns the current position of this element.
    pub fn get_position(&self) -> Vector2 {
        self.position_ref
    }

    /// Returns the current rotation of this element.
    pub fn get_rotation(&self) -> f32 {
        self.rotation_ref
    }

    /// Returns the current ui type of this element.
    pub fn get_ui_type(&self) -> &UiType {
        &self.ui_type
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
    /// use vyxen_geometry::Box;
    /// use vyxen_ui::UiElement;
    /// use vyxen_math::Vector2;
    ///
    /// let mut element = UiElement::new();
    /// element.set_shape(Box::new(64.0, 64.0));
    /// ```
    /// ## Circle
    /// ```rust
    /// use vyxen_geometry::Circle;
    /// use vyxen_ui::UiElement;
    /// use vyxen_math::Vector2;
    ///
    /// let mut element = UiElement::new();
    /// element.set_shape(Circle::new(64.0));
    /// ```
    /// ## Polygon
    /// ```rust
    /// use vyxen_geometry::Polygon;
    /// use vyxen_ui::UiElement;
    /// use vyxen_math::Vector2;
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

/// Represents a text element to be rendered on the UI.
///
/// ```rust, ignore
/// use vyxen_resources::{Font, load_path};
/// use vyxen_ui::Text;
///
/// let font = load_path("path/to/font.ttf").unwrap();
/// let text = Text::new("Hello, World!", font, 16.0);
/// ```
#[derive(Debug, Clone)]
pub struct Text {
    text: String,
    font: Font,
    size: f32,
}

impl Text {
    /// Creates a new `Text` element.
    ///
    /// ```rust, ignore
    /// use vyxen_resources::{Font, load_path};
    /// use vyxen_ui::Text;
    ///
    /// let font = load_path("path/to/font.ttf").unwrap();
    /// let text = Text::new("Hello, World!", font, 16.0);
    /// ```
    pub fn new(text: String, font: Font, size: f32) -> Self {
        Self { text, font, size }
    }

    /// Returns the text of this `Text`.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the font of this `Text`.
    pub fn font(&self) -> &Font {
        &self.font
    }

    /// Returns the size of this `Text`.
    pub fn size(&self) -> f32 {
        self.size
    }
}
