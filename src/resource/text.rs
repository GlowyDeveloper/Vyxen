use crate::{Color, Font};

/// Represents a text element to be rendered on the UI.
///
/// ```rust, ignore
/// use vyxen::{Font, load_path, Text};
///
/// let font = load_path("path/to/font.ttf").unwrap();
/// let text = Text::new("Hello, World!", font, 16.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    text: String,
    font: Font,
    size: f32,
    tint: Option<Color>,
    anchor: TextAnchor,
}

impl Text {
    /// Creates a new `Text` element.
    ///
    /// ```rust, ignore
    /// use vyxen::{Font, load_path, Text};
    ///
    /// let font = load_path("path/to/font.ttf").unwrap();
    /// let text = Text::new("Hello, World!", font, 16.0);
    /// ```
    pub fn new(text: String, font: Font, size: f32) -> Self {
        Self {
            text,
            font,
            size,
            tint: None,
            anchor: TextAnchor::default(),
        }
    }

    /// Returns the text of this `Text`.
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Returns the font of this `Text`.
    pub fn get_font(&self) -> &Font {
        &self.font
    }

    /// Returns the size of this `Text`.
    pub fn get_size(&self) -> f32 {
        self.size
    }

    /// Returns the tint of this `Text`.
    pub fn get_tint(&self) -> Option<Color> {
        self.tint
    }

    /// Sets the tint of this `Text`.
    pub fn set_tint(&mut self, tint: Color) {
        self.tint = Some(tint);
    }

    /// Sets the text of this `Text`.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    /// Sets the font of this `Text`.
    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }

    /// Sets the size of this `Text`.
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    /// Sets the anchor type of this `Text`.
    pub fn set_anchor(&mut self, anchor: TextAnchor) {
        self.anchor = anchor;
    }

    /// Gets the anchor of this `Text`.
    pub fn get_anchor(&self) -> &TextAnchor {
        &self.anchor
    }
}

/// Sets if the text is anchored in the center, left or right.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum TextAnchor {
    Left,
    #[default]
    Center,
    Right,
}
