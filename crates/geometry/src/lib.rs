pub mod aabb;

/// A simple circle struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f32
}

/// A simple box struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box {
    pub height: f32,
    pub width: f32
}