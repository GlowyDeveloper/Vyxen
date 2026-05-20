//! A math library made for Vyxen.

use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 2D vector with x and y components.
/// 
/// # Examples
/// 
/// ```rust
/// use vyxen_math::Vector2;
/// 
/// let v1 = Vector2 { x: 1.0, y: 2.0 };
/// assert_eq!(v1.x, 1.0);
/// assert_eq!(v1.y, 2.0);
/// 
/// let v2 = Vector2 { x: 3.0, y: 4.0 };
/// assert_eq!(v2.x, 3.0);
/// assert_eq!(v2.y, 4.0);
/// 
/// let length = v1.length();
/// assert_eq!(length, (1.0f32 * 1.0 + 2.0 * 2.0).sqrt());
/// 
/// let normalized = v1.normalize();
/// assert_eq!(normalized.x, 1.0 / (1.0f32 * 1.0 + 2.0 * 2.0).sqrt());
/// assert_eq!(normalized.y, 2.0 / (1.0f32 * 1.0 + 2.0 * 2.0).sqrt());
/// 
/// let dot = v1.dot(&v2);
/// assert_eq!(dot, 1.0 * 3.0 + 2.0 * 4.0);
/// 
/// let cross = v1.cross(&v2);
/// assert_eq!(cross, 1.0 * 4.0 - 2.0 * 3.0);
/// ```
#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vector2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vector2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_operations() {
        let v1 = Vector2 { x: 1.0, y: 2.0 };
        let v2 = Vector2 { x: 3.0, y: 4.0 };

        assert_eq!(v1 + v2, Vector2 { x: 4.0, y: 6.0 });
        assert_eq!(v1 - v2, Vector2 { x: -2.0, y: -2.0 });
        assert_eq!(v1 * v2, Vector2 { x: 3.0, y: 8.0 });
        assert_eq!(v1 / v2, Vector2 { x: 1.0 / 3.0, y: 0.5 });
        assert_eq!(-v1, Vector2 { x: -1.0, y: -2.0 });
        assert_eq!(v1.length(), (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt());
        assert_eq!(v1.distance(&v2), (2.0_f32 * 2.0 + 2.0_f32 * 2.0).sqrt());
        assert_eq!(v1.normalize(), Vector2 { x: 1.0 / (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt(), y: 2.0 / (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt() });
        assert_eq!(v1.dot(&v2), 1.0 * 3.0 + 2.0 * 4.0);
        assert_eq!(v1.cross(&v2), 1.0 * 4.0 - 2.0 * 3.0);
    }
}