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
    /// Creates a new Vector2 are 0, 0.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v = Vector2::zero();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// ```
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Returns the length of the vector.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v = Vector2 { x: 3.0, y: 4.0 };
    /// let length = v.length();
    /// assert_eq!(length, 5.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the length squared of the vector.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v = Vector2 { x: 3.0, y: 4.0 };
    /// let length = v.length();
    /// assert_eq!(length, 5.0);
    /// let length_squared = v.length_squared();
    /// assert_eq!(length_squared, 25.0);
    /// assert_eq!(length * length, length_squared);
    /// ```
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the distance between this vector and another vector.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v1 = Vector2 { x: 1.0, y: 2.0 };
    /// let v2 = Vector2 { x: 4.0, y: 6.0 };
    /// let distance = v1.distance(&v2);
    /// assert_eq!(distance, 5.0);
    /// ```
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the distance between this vector and another vector.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v1 = Vector2 { x: 1.0, y: 2.0 };
    /// let v2 = Vector2 { x: 4.0, y: 6.0 };
    /// let distance = v1.distance(&v2);
    /// assert_eq!(distance, 5.0);
    /// let distance_squared = v1.distance_squared(&v2);
    /// assert_eq!(distance_squared, 25.0);
    /// assert_eq!(distance * distance, distance_squared);
    /// ```
    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Returns the normalized vector.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v = Vector2 { x: 3.0, y: 4.0 };
    /// let normalized = v.normalize();
    /// assert_eq!(normalized.length(), 1.0);
    /// ```
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    /// Returns the dot product of this vector and another vector.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v1 = Vector2 { x: 1.0, y: 2.0 };
    /// let v2 = Vector2 { x: 3.0, y: 4.0 };
    /// let dot = v1.dot(&v2);
    /// assert_eq!(dot, 1.0 * 3.0 + 2.0 * 4.0);
    /// ```
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the cross product of this vector and another vector.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v1 = Vector2 { x: 1.0, y: 2.0 };
    /// let v2 = Vector2 { x: 3.0, y: 4.0 };
    /// let cross = v1.cross(&v2);
    /// assert_eq!(cross, 1.0 * 4.0 - 2.0 * 3.0);
    /// ```
    pub fn cross(&self, other: &Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Transforms this vector by a Transform
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::{Vector2, Transform};
    /// 
    /// let v = Vector2 { x: 5.0, y: 5.0 };
    /// let t = Transform::new(Vector2 {x: 5.0, y: 2.0}, 45.0);
    /// let transformed = v.transform(&t);

    /// assert!(v != transformed);
    /// ```
    pub fn transform(&self, transform: &Transform) -> Self {
        Self {
            x: self.x * transform.cos - self.y * transform.sin + transform.pos_x,
            y: self.x * transform.sin + self.y * transform.cos + transform.pos_y
        }
    }

    /// Returns if two `Vector2`s are nealy equal (0.0005).
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// 
    /// let v1 = Vector2 { x: 5.0, y: 5.0 };
    /// let v2 = Vector2 { x: 4.9999, y: 4.9999 };
    /// 
    /// assert!(v1.is_nearly_equal(&v2));
    /// ```
    pub fn is_nearly_equal(&self, other: &Self) -> bool {
        is_nearly_equal(self.x, other.x) && is_nearly_equal(self.y, other.y)
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

/// A Transform Matrix
pub struct Transform {
    pos_x: f32,
    pos_y: f32,
    sin: f32,
    cos: f32,
}

impl Transform {
    /// Creates a new Transform Matrix
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::{Vector2, Transform};
    /// 
    /// let transform = Transform::new(Vector2 {x: 5.0, y: 2.0}, 45.0);
    /// ```
    pub fn new(position: Vector2, angle: f32) -> Self {
        Self {
            pos_x: position.x,
            pos_y: position.y,
            sin: angle.sin(),
            cos: angle.cos(),
        }
    }

    /// Creates a zero-ed Transform Matrix
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::{Vector2, Transform};
    /// 
    /// let transform = Transform::zero();
    /// ```
    pub fn zero() -> Self {
        Self::new(Vector2 { x: 0.0, y: 0.0 }, 0.0)
    }
}

/// Returns if two `f32`s are nealy equal (0.0005).
/// 
/// # Examples
/// ```rust
/// use vyxen_math::is_nearly_equal;
/// 
/// let f1 = 5.0;
/// let f2 = 4.9999;
/// 
/// assert!(is_nearly_equal(f1, f2));
/// ```
pub fn is_nearly_equal(a: f32, b: f32) -> bool {
    (a-b).abs() < 0.0005
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
        assert_eq!(v1.length_squared(), 1.0_f32 * 1.0 + 2.0_f32 * 2.0);
        assert_eq!(v1.distance(&v2), (2.0_f32 * 2.0 + 2.0_f32 * 2.0).sqrt());
        assert_eq!(v1.distance_squared(&v2), 2.0_f32 * 2.0 + 2.0_f32 * 2.0);
        assert_eq!(v1.normalize(), Vector2 { x: 1.0 / (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt(), y: 2.0 / (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt() });
        assert_eq!(v1.dot(&v2), 1.0 * 3.0 + 2.0 * 4.0);
        assert_eq!(v1.cross(&v2), 1.0 * 4.0 - 2.0 * 3.0);
        assert!(v1.is_nearly_equal(&Vector2 { x: 1.0, y: 1.9999 }));
    }

    #[test]
    fn test_transform() {
        let transform = Transform::new(Vector2 { x: 1.0, y: 2.0 }, 45.0);
        assert_eq!(transform.pos_x, 1.0);
        assert_eq!(transform.pos_y, 2.0);
        assert_eq!(transform.cos, 45_f32.cos());
        assert_eq!(transform.sin, 45_f32.sin());
    }

    #[test]
    fn test_is_nearly_equal() {
        let f1 = 1.0;
        let f2 = 1.0000001;
        let f3 = 0.9999;
        let f4 = 0.0;

        assert!(is_nearly_equal(f1, f2));
        assert!(is_nearly_equal(f1, f3));
        assert!(!is_nearly_equal(f1, f4));

        assert!(is_nearly_equal(f2, f3));
        assert!(!is_nearly_equal(f2, f4));

        assert!(!is_nearly_equal(f3, f4));
    }
}