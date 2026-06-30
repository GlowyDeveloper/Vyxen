pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn a(&self) -> f32 {
        self.a
    }

    pub fn set_r(&mut self, r: f32) {
        self.r = r;
    }

    pub fn set_b(&mut self, b: f32) {
        self.b = b;
    }

    pub fn set_g(&mut self, g: f32) {
        self.g = g;
    }

    pub fn set_a(&mut self, a: f32) {
        self.a = a;
    }
}
