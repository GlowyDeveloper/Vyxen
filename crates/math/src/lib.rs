//! A math library made for Vyxen.

use std::{
    ops::{Add, Div, Mul, Neg, Range, Sub},
    time::{SystemTime, UNIX_EPOCH},
};

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
            y: self.x * transform.sin + self.y * transform.cos + transform.pos_y,
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
        self.distance(other) < 0.0005
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Vector2 {
            x: self.x + other,
            y: self.y + other,
        }
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

impl Sub<f32> for Vector2 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Vector2 {
            x: self.x - other,
            y: self.y - other,
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
    (a - b).abs() < 0.0005
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Random {
    state: u64,
}

impl Random {
    /// Creates a new Random with a given seed.
    ///
    /// # Note
    ///
    /// If the seed is 0, it will be generated from the current time.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::new(12345);
    ///
    /// let value1 = rng.next_u32();
    /// let value2 = rng.next_u32();
    /// ```
    pub fn new(seed: u64) -> Self {
        if seed == 0 {
            Self::from_time()
        } else {
            Self { state: seed }
        }
    }

    /// Creates a new Random from the current time.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.next_u32();
    /// let value2 = rng.next_u32();
    /// ```
    pub fn from_time() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        Self { state: seed }
    }

    /// Returns the current seed of the Random.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::new(12345);
    /// let seed = rng.seed();
    /// assert_eq!(seed, 12345);
    /// ```
    pub fn seed(&self) -> u64 {
        self.state
    }

    /// Resets the Random with a new seed.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::new(12345);
    /// let seed = rng.seed();
    /// assert_eq!(seed, 12345);
    ///
    /// rng.reseed(67890);
    ///
    /// let new_seed = rng.seed();
    /// assert_eq!(new_seed, 67890);
    /// ```
    pub fn reseed(&mut self, seed: u64) {
        self.state = seed;
    }

    /// Generates a random u64.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.next_u64();
    /// let value2 = rng.next_u64();
    /// ```
    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;

        x
    }

    /// Generates a random u32.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.next_u32();
    /// let value2 = rng.next_u32();
    /// ```
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Generates a random f32.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.next_f32();
    /// let value2 = rng.next_f32();
    /// ```
    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    /// Generates a random f64.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.next_f64();
    /// let value2 = rng.next_f64();
    /// ```
    pub fn next_f64(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }

    /// Generates a random bool.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.next_bool();
    /// let value2 = rng.next_bool();
    /// ```
    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 0
    }

    /// Generates a random u32 in the given range.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.range_u32(0..10);
    /// let value2 = rng.range_u32(0..100);
    /// ```
    pub fn range_u32(&mut self, range: Range<u32>) -> u32 {
        let width = range.end - range.start;

        range.start + ((self.next_u64() % width as u64) as u32)
    }

    /// Generates a random u64 in the given range.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.range_u64(0..10);
    /// let value2 = rng.range_u64(0..100);
    /// ```
    pub fn range_u64(&mut self, range: Range<u64>) -> u64 {
        let width = range.end - range.start;

        range.start + (self.next_u64() % width)
    }

    /// Generates a random f32 in the given range.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.range_f32(0.0..10.0);
    /// let value2 = rng.range_f32(0.0..100.0);
    /// ```
    pub fn range_f32(&mut self, range: Range<f32>) -> f32 {
        let width = range.end - range.start;

        range.start + self.next_f32() * width
    }

    /// Generates a random f64 in the given range.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Random;
    ///
    /// let mut rng = Random::from_time();
    ///
    /// let value1 = rng.range_f64(0.0..10.0);
    /// let value2 = rng.range_f64(0.0..100.0);
    /// ```
    pub fn range_f64(&mut self, range: Range<f64>) -> f64 {
        let width = range.end - range.start;

        range.start + self.next_f64() * width
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::from_time()
    }
}

/// A 4×4 matrix used for 2D and 3D transformations.
///
/// # Examples
/// ```
/// use vyxen_math::Matrix4;
///
/// let matrix = Matrix4::new();
///
/// assert_eq!(matrix, Matrix4 {m: [[0.0; 4]; 4]});
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Matrix4 {
    /// The matrix elements stored in columns.
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    /// Creates a new `Matrix4`. All elements are initilized to `0.0`
    ///
    /// # Examples
    /// ```
    /// use vyxen_math::Matrix4;
    ///
    /// let matrix = Matrix4::new();
    ///
    /// assert_eq!(matrix, Matrix4 {m: [[0.0; 4]; 4]});
    /// ```
    pub fn new() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    /// Creates the identity matrix.
    ///
    /// # Examples
    /// ```
    /// use vyxen_math::Matrix4;
    ///
    /// let identity = Matrix4::identity();
    ///
    /// assert_eq!(
    ///     identity,
    ///     Matrix4 {
    ///         m: [
    ///             [1.0, 0.0, 0.0, 0.0],
    ///             [0.0, 1.0, 0.0, 0.0],
    ///             [0.0, 0.0, 1.0, 0.0],
    ///             [0.0, 0.0, 0.0, 1.0],
    ///         ],
    ///     }
    /// );
    /// ```
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// The returned matrix translates by `x`, `y`, and `z`.
    ///
    /// # Examples
    /// ```
    /// use vyxen_math::Matrix4;
    ///
    /// let x = 2.0;
    /// let y = 4.0;
    /// let z = 6.0;
    ///
    /// let translation = Matrix4::translation(x, y, z);
    ///
    /// assert_eq!(
    ///     translation,
    ///     Matrix4 {
    ///         m: [
    ///             [1.0, 0.0, 0.0, 0.0],
    ///             [0.0, 1.0, 0.0, 0.0],
    ///             [0.0, 0.0, 1.0, 0.0],
    ///             [x, y, z, 1.0],
    ///         ],
    ///     }
    /// );
    /// ```
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }

    /// The returned matrix scales each axis independently.
    ///
    /// # Examples
    /// ```
    /// use vyxen_math::Matrix4;
    ///
    /// let x = 2.0;
    /// let y = 4.0;
    /// let z = 6.0;
    ///
    /// let scale = Matrix4::scale(x, y, z);
    ///
    /// assert_eq!(
    ///     scale,
    ///     Matrix4 {
    ///         m: [
    ///             [x, 0.0, 0.0, 0.0],
    ///             [0.0, y, 0.0, 0.0],
    ///             [0.0, 0.0, z, 0.0],
    ///             [0.0, 0.0, 0.0, 1.0],
    ///         ],
    ///     }
    /// );
    /// ```
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            m: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Maps the cube defined by bounds into space without perspective.
    ///
    /// # Examples
    /// ```
    /// use vyxen_math::Matrix4;
    ///
    /// let projection = Matrix4::orthographic(
    ///     -1.0, 1.0,
    ///     -1.0, 1.0,
    ///     -1.0, 1.0,
    /// );
    ///
    /// assert_eq!(
    ///     projection,
    ///     Matrix4 {
    ///         m: [
    ///             [1.0, 0.0,  0.0, 0.0],
    ///             [0.0, 1.0,  0.0, 0.0],
    ///             [0.0, 0.0, -1.0, 0.0],
    ///             [0.0, 0.0,  0.0, 1.0],
    ///         ],
    ///     }
    /// );
    /// ```
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            m: [
                [2.0 / (right - left), 0.0, 0.0, 0.0],
                [0.0, 2.0 / (top - bottom), 0.0, 0.0],
                [0.0, 0.0, -2.0 / (far - near), 0.0],
                [
                    -(right + left) / (right - left),
                    -(top + bottom) / (top - bottom),
                    -(far + near) / (far - near),
                    1.0,
                ],
            ],
        }
    }

    /// The returned matrix is rotated around the Z axis.
    ///
    /// # Examples
    /// ```
    /// use vyxen_math::Matrix4;
    ///
    /// let rotation = Matrix4::rotation_z(std::f32::consts::PI);
    ///
    /// assert_eq!(
    ///     rotation,
    ///     Matrix4 {
    ///         m: [
    ///             [ 0.0,  1.0, 0.0, 0.0],
    ///             [-1.0,  0.0, 0.0, 0.0],
    ///             [ 0.0,  0.0, 1.0, 0.0],
    ///             [ 0.0,  0.0, 0.0, 1.0],
    ///         ],
    ///     }
    /// );
    /// ```
    pub fn rotate(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            m: [
                [c, s, 0.0, 0.0],
                [-s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let mut result = Matrix4 { m: [[0.0; 4]; 4] };

        for c in 0..4 {
            for r in 0..4 {
                result.m[c][r] = 0.0;
                for i in 0..4 {
                    result.m[c][r] += self.m[i][r] * rhs.m[c][i];
                }
            }
        }

        result
    }
}

impl From<Matrix4> for [[f32; 4]; 4] {
    fn from(matrix: Matrix4) -> Self {
        matrix.m
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
        assert_eq!(
            v1 / v2,
            Vector2 {
                x: 1.0 / 3.0,
                y: 0.5
            }
        );
        assert_eq!(-v1, Vector2 { x: -1.0, y: -2.0 });
        assert_eq!(v1.length(), (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt());
        assert_eq!(v1.length_squared(), 1.0_f32 * 1.0 + 2.0_f32 * 2.0);
        assert_eq!(v1.distance(&v2), (2.0_f32 * 2.0 + 2.0_f32 * 2.0).sqrt());
        assert_eq!(v1.distance_squared(&v2), 2.0_f32 * 2.0 + 2.0_f32 * 2.0);
        assert_eq!(
            v1.normalize(),
            Vector2 {
                x: 1.0 / (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt(),
                y: 2.0 / (1.0_f32 * 1.0 + 2.0_f32 * 2.0).sqrt()
            }
        );
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
