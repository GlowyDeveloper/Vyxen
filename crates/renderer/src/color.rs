/// Color type for sprites
///
/// # Examples
/// ```rust
/// use vyxen_renderer::Color;
///
/// let color1 = Color::from_rgba(1.0, 1.0, 1.0, 1.0); // White
/// let color2 = Color::from_rgb(0.5, 0.5, 0.5); // Gray
///
/// assert_eq!(color1.r(), 1.0);
/// assert_eq!(color1.b(), 1.0);
/// assert_eq!(color1.g(), 1.0);
/// assert_eq!(color2.a(), 1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    /// Generates a color with `rgba`.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let color = Color::from_rgba(1.0, 1.0, 1.0, 1.0); // White
    ///
    /// assert_eq!(color.r(), 1.0);
    /// assert_eq!(color.b(), 1.0);
    /// assert_eq!(color.g(), 1.0);
    /// assert_eq!(color.a(), 1.0);
    /// ```
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Generates a color with `rgb`.
    ///
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let color = Color::from_rgb(0.5, 0.5, 0.5); // Gray
    ///
    /// assert_eq!(color.r(), 0.5);
    /// assert_eq!(color.b(), 0.5);
    /// assert_eq!(color.g(), 0.5);
    /// assert_eq!(color.a(), 1.0);
    /// ```
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Gets the red element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.r(), 1.0);
    /// ```
    pub fn r(&self) -> f32 {
        self.r
    }

    /// Gets the blue element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.b(), 1.0);
    /// ```
    pub fn b(&self) -> f32 {
        self.b
    }

    /// Gets the green element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.g(), 1.0);
    /// ```
    pub fn g(&self) -> f32 {
        self.g
    }

    /// Gets the alpha element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.a(), 1.0);
    /// ```
    pub fn a(&self) -> f32 {
        self.a
    }

    /// Sets the red element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let mut color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.r(), 1.0);
    ///
    /// color.set_r(0.5);
    ///
    /// assert_eq!(color.r(), 0.5);
    /// ```
    pub fn set_r(&mut self, r: f32) {
        self.r = r;
    }

    /// Sets the blue element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let mut color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.b(), 1.0);
    ///
    /// color.set_b(0.5);
    ///
    /// assert_eq!(color.b(), 0.5);
    /// ```
    pub fn set_b(&mut self, b: f32) {
        self.b = b;
    }

    /// Sets the green element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let mut color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.g(), 1.0);
    ///
    /// color.set_g(0.5);
    ///
    /// assert_eq!(color.g(), 0.5);
    /// ```
    pub fn set_g(&mut self, g: f32) {
        self.g = g;
    }

    /// Sets the alpha element of the color.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_renderer::Color;
    ///
    /// let mut color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(color.a(), 1.0);
    ///
    /// color.set_a(0.5);
    ///
    /// assert_eq!(color.a(), 0.5);
    /// ```
    pub fn set_a(&mut self, a: f32) {
        self.a = a;
    }
}

impl From<Color> for [f32; 4] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}
